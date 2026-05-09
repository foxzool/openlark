"""Contract comparison rules."""

from __future__ import annotations

from .models import ApiIdentity, ContractFinding, OfficialField, RustApiContract
from .official import normalize_endpoint_path


def compare_endpoint(api: ApiIdentity, rust_contract: RustApiContract | None) -> list[ContractFinding]:
    findings: list[ContractFinding] = []
    official_method = api.official_method
    official_path = api.official_path
    official_text = f"{official_method}:{official_path}" if official_method and official_path else api.url

    if not official_method or not official_path:
        return [
            finding(
                "UNVERIFIED",
                "U_OFFICIAL_ENDPOINT_UNSTRUCTURED",
                "Official method/path is unavailable.",
                api,
                official=official_text,
            )
        ]

    if rust_contract is None:
        return [
            finding(
                "WARN",
                "W_IMPLEMENTATION_FILE_MISSING",
                "Expected implementation file does not exist.",
                api,
                official=official_text,
            )
        ]

    if not rust_contract.endpoint_calls:
        return [
            finding(
                "WARN",
                "W_ENDPOINT_UNRESOLVED",
                "No ApiRequest endpoint call was found in the implementation file.",
                api,
                official=official_text,
            )
        ]

    official_normalized_path = normalize_endpoint_path(official_path)
    resolved_calls = [call for call in rust_contract.endpoint_calls if call.is_resolved]
    unresolved_calls = [call for call in rust_contract.endpoint_calls if not call.is_resolved]

    for call in resolved_calls:
        if call.method == official_method and normalize_endpoint_path(call.resolved_path) == official_normalized_path:
            return []

    for call in resolved_calls:
        if normalize_endpoint_path(call.resolved_path) == official_normalized_path and call.method != official_method:
            findings.append(
                finding(
                    "ERROR",
                    "E_ENDPOINT_METHOD_MISMATCH",
                    "Rust ApiRequest method differs from the official method.",
                    api,
                    rust_line=call.line,
                    official=official_text,
                    rust=f"{call.method}:{call.resolved_path}",
                )
            )

    method_matches = [call for call in resolved_calls if call.method == official_method]
    if method_matches:
        rust_paths = ", ".join(sorted({call.resolved_path for call in method_matches}))
        findings.append(
            finding(
                "ERROR",
                "E_ENDPOINT_PATH_MISMATCH",
                "Rust endpoint path differs from the official path.",
                api,
                rust_line=method_matches[0].line,
                official=official_text,
                rust=f"{official_method}:{rust_paths}",
            )
        )
    elif resolved_calls and not findings:
        rust_methods = ", ".join(sorted({call.method for call in resolved_calls}))
        findings.append(
            finding(
                "ERROR",
                "E_ENDPOINT_METHOD_MISMATCH",
                "No resolved Rust ApiRequest call uses the official method.",
                api,
                rust_line=resolved_calls[0].line,
                official=official_text,
                rust=rust_methods,
            )
        )

    if not resolved_calls and unresolved_calls:
        findings.append(
            finding(
                "WARN",
                "W_ENDPOINT_UNRESOLVED",
                "Rust endpoint expression could not be resolved by the validator.",
                api,
                rust_line=unresolved_calls[0].line,
                official=official_text,
                rust="; ".join(sorted({call.unresolved_reason for call in unresolved_calls if call.unresolved_reason})),
            )
        )

    return findings


def compare_request_fields(
    api: ApiIdentity,
    official_fields: tuple[OfficialField, ...],
    rust_contract: RustApiContract | None,
) -> list[ContractFinding]:
    if not official_fields:
        return []
    if rust_contract is None:
        return [
            finding(
                "WARN",
                "W_IMPLEMENTATION_FILE_MISSING",
                "Expected implementation file does not exist.",
                api,
            )
        ]
    if not rust_contract.fields:
        return [
            finding(
                "WARN",
                "W_REQUEST_FIELDS_UNRESOLVED",
                "No request field struct was found in the implementation file.",
                api,
                official=", ".join(field.name for field in official_fields),
            )
        ]

    rust_by_name = {field.serialized_name: field for field in rust_contract.fields}
    findings: list[ContractFinding] = []
    for official_field in official_fields:
        rust_field = rust_by_name.get(official_field.name)
        official_text = official_field_text(official_field)
        if rust_field is None:
            severity = "ERROR" if official_field.required else "WARN"
            code = "E_REQUIRED_REQUEST_FIELD_MISSING" if official_field.required else "W_OPTIONAL_REQUEST_FIELD_MISSING"
            findings.append(
                finding(
                    severity,
                    code,
                    "Rust request field is missing an official request body field.",
                    api,
                    official=official_text,
                    rust=", ".join(sorted(rust_by_name)) or "<none>",
                )
            )
            continue
        if official_field.required and rust_field.optional:
            findings.append(
                finding(
                    "WARN",
                    "W_REQUIRED_REQUEST_FIELD_OPTIONAL",
                    "Official request body field is required but Rust models it as optional.",
                    api,
                    rust_line=rust_field.line,
                    official=official_text,
                    rust=f"{rust_field.struct_name}.{rust_field.field_name}: {rust_field.type_name}",
                )
            )
    return findings


def compare_response_fields(
    api: ApiIdentity,
    official_fields: tuple[OfficialField, ...],
    rust_contract: RustApiContract | None,
) -> list[ContractFinding]:
    if not official_fields:
        return []
    if rust_contract is None:
        return [
            finding(
                "WARN",
                "W_IMPLEMENTATION_FILE_MISSING",
                "Expected implementation file does not exist.",
                api,
            )
        ]
    if not rust_contract.response_fields:
        return [
            finding(
                "WARN",
                "W_RESPONSE_FIELDS_UNRESOLVED",
                "No response field struct was found in the implementation file.",
                api,
                official=", ".join(field.name for field in official_fields),
            )
        ]

    rust_names = {field.serialized_name for field in rust_contract.response_fields}
    findings: list[ContractFinding] = []
    for official_field in official_fields:
        if official_field.name in rust_names:
            continue
        findings.append(
            finding(
                "WARN",
                "W_RESPONSE_FIELD_MISSING",
                "Rust response model is missing an official response data field.",
                api,
                official=official_field_text(official_field),
                rust=", ".join(sorted(rust_names)) or "<none>",
            )
        )
    return findings


def official_field_text(field: OfficialField) -> str:
    required = "required" if field.required else "optional"
    suffix = f" {field.field_type}" if field.field_type else ""
    return f"{field.location} {field.name} {required}{suffix}"


def finding(
    severity: str,
    code: str,
    message: str,
    api: ApiIdentity,
    rust_line: int = 0,
    official: str = "",
    rust: str = "",
) -> ContractFinding:
    return ContractFinding(
        severity=severity,
        code=code,
        message=message,
        api_id=api.api_id,
        api_name=api.name,
        expected_file=api.expected_file,
        doc_path=api.doc_path,
        rust_line=rust_line,
        official=official,
        rust=rust,
    )
