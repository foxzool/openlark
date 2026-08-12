#!/usr/bin/env python3
"""Validate OpenLark typed API endpoint contracts against the official API list."""

from __future__ import annotations

import argparse
import sys
from dataclasses import replace
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None

REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.compare import (
    compare_access_token_types,
    compare_endpoint,
    compare_request_fields,
    compare_response_fields,
    finding,
)
from tools.api_contracts.models import ContractReport, OfficialField
from tools.api_contracts.official import load_api_identities
from tools.api_contracts.official_evidence import (
    EndpointObservation,
    EvidenceDimension,
    EvidenceStatus,
    FieldObservation,
    FreshOfficialPolicy,
    TokenObservation,
    compose,
)
from tools.api_contracts.report import evidence_to_jsonable, write_report, write_summary
from tools.api_contracts.rust_source import (
    load_endpoint_constants,
    load_enum_endpoints,
    load_enum_methods,
    scan_api_file,
)


def implementation_path_candidates(expected_file: str, crate_config: dict) -> list[str]:
    """Return the strict path plus explicitly registered legacy implementation paths."""
    candidates = [expected_file]
    alias = (crate_config.get("implementation_path_aliases") or {}).get(expected_file)
    if alias:
        candidates.append(str(alias))
    for rewrite in crate_config.get("implementation_path_rewrites") or []:
        source_prefix = str(rewrite.get("from", ""))
        target_prefix = str(rewrite.get("to", ""))
        if source_prefix and expected_file.startswith(source_prefix):
            candidates.append(target_prefix + expected_file[len(source_prefix) :])
    return list(dict.fromkeys(candidates))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate endpoint-level API contracts.")
    parser.add_argument("--csv", default="api_list_export.csv", help="Official API CSV path")
    parser.add_argument("--mapping", default="tools/api_coverage.toml", help="crate to bizTag mapping")
    parser.add_argument("--crate", dest="crate_name", help="Validate one mapped crate")
    parser.add_argument("--all-crates", action="store_true", help="Validate all mapped crates")
    parser.add_argument(
        "--biz-tag",
        action="append",
        help="进一步过滤到指定 bizTag（可多次传入）；覆盖 crate 的 biz_tags，用于子模块级 gate",
    )
    parser.add_argument(
        "--api-id",
        action="append",
        help="仅验证显式列入 gate inventory 的 API ID（可多次传入，需配合 --crate）",
    )
    parser.add_argument("--report-dir", default="reports/api_contracts", help="Report directory")
    parser.add_argument("--include-old", dest="skip_old", action="store_false", help="Include meta.Version=old APIs")
    parser.add_argument("--skip-old", dest="skip_old", action="store_true", default=True, help="Skip old APIs")
    parser.add_argument(
        "--live-endpoints",
        action="store_true",
        help="Fetch each official detail page and compare endpoints against the current official schema",
    )
    parser.add_argument(
        "--fields",
        action="store_true",
        help="Compare Rust request body fields against official request body fields",
    )
    parser.add_argument(
        "--live-fields",
        action="store_true",
        help="Fetch official detail pages for request body field validation",
    )
    parser.add_argument(
        "--tokens",
        action="store_true",
        help=(
            "Verify Rust supported_access_token_types against the official "
            "security.supportedAccessToken annotation (always fetches detail payloads)"
        ),
    )
    parser.add_argument("--field-timeout", type=int, default=20, help="Official detail fetch timeout in seconds")
    parser.add_argument("--field-retries", type=int, default=1, help="Official detail fetch retries")
    parser.add_argument(
        "--max-field-apis",
        type=int,
        default=0,
        help="Limit live field validation to the first N APIs in each crate; 0 means no limit",
    )
    parser.add_argument(
        "--strict",
        default="",
        help="Comma-separated strict categories. Supported values: endpoint, fields, tokens",
    )
    return parser.parse_args()


def load_mapping(path: Path) -> dict[str, dict]:
    if tomllib is None:
        raise SystemExit("Python 3.11+ is required for tomllib")
    if not path.exists():
        raise SystemExit(f"Mapping file does not exist: {path}")
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    crates = data.get("crates", {})
    if not isinstance(crates, dict) or not crates:
        raise SystemExit(f"Mapping file lacks [crates.*] entries: {path}")
    return crates


def validate_crate(
    crate_name: str,
    crate_config: dict,
    csv_path: Path,
    report_dir: Path,
    skip_old: bool,
    collector,
    live_endpoints: bool = False,
    fields: bool = False,
    max_field_apis: int = 0,
    tokens: bool = False,
    biz_tag_filter: list[str] | None = None,
    api_ids: set[str] | None = None,
) -> ContractReport:
    """核对一个 crate；官方事实只通过 collect seam 获取。"""
    src_path = Path(crate_config["src"])
    biz_tags = biz_tag_filter if biz_tag_filter else list(crate_config.get("biz_tags") or [])
    apis = load_api_identities(csv_path, filter_tags=biz_tags, skip_old_versions=skip_old)
    if api_ids:
        available_ids = {api.api_id for api in apis}
        missing_ids = api_ids - available_ids
        if missing_ids:
            raise SystemExit(
                "指定 API ID 不在当前 crate/catalog 范围内: "
                + ", ".join(sorted(missing_ids))
            )
        apis = [api for api in apis if api.api_id in api_ids]
    constants = load_endpoint_constants(src_path)
    enum_endpoints = load_enum_endpoints(src_path, constants)
    enum_methods = load_enum_methods(src_path)
    report = ContractReport(crate_name=crate_name, total_apis=len(apis))

    field_checks = 0
    for api in apis:
        rust_contract = next(
            (
                contract
                for candidate in implementation_path_candidates(api.expected_file, crate_config)
                if (
                    contract := scan_api_file(
                        src_path, candidate, constants, enum_endpoints, enum_methods
                    )
                )
                is not None
            ),
            None,
        )
        if rust_contract is not None:
            report.checked_apis += 1

        should_check_fields = fields and (
            not max_field_apis or field_checks < max_field_apis
        )
        dimensions = []
        if live_endpoints:
            dimensions.append(EvidenceDimension.ENDPOINT)
        if should_check_fields:
            dimensions.extend(
                (
                    EvidenceDimension.REQUEST_FIELDS,
                    EvidenceDimension.RESPONSE_FIELDS,
                )
            )
        if tokens:
            dimensions.append(EvidenceDimension.TOKENS)
        evidence = None
        if dimensions:
            evidence = collector.collect(
                api, tuple(dimensions), FreshOfficialPolicy()
            )
            report.evidence.append(evidence_to_jsonable(evidence))

        endpoint_api = api
        if live_endpoints:
            endpoint = evidence.for_dimension(EvidenceDimension.ENDPOINT)
            if endpoint.status is EvidenceStatus.TRUSTED:
                observation = next(
                    (
                        item
                        for item in endpoint.observations
                        if isinstance(item, EndpointObservation)
                    ),
                    None,
                )
                if observation is not None:
                    endpoint_api = replace(
                        api, url=f"{observation.method}:{observation.path}"
                    )
            else:
                report.add(
                    _evidence_finding(
                        api, EvidenceDimension.ENDPOINT, endpoint
                    )
                )
        for item in compare_endpoint(endpoint_api, rust_contract):
            report.add(item)

        if tokens:
            token_evidence = evidence.for_dimension(EvidenceDimension.TOKENS)
            if token_evidence.status is EvidenceStatus.TRUSTED:
                official_tokens = tuple(
                    item.token
                    for item in token_evidence.observations
                    if isinstance(item, TokenObservation)
                )
                for item in compare_access_token_types(
                    api, official_tokens, rust_contract
                ):
                    report.add(item)
            else:
                report.add(
                    _evidence_finding(
                        api, EvidenceDimension.TOKENS, token_evidence
                    )
                )

        if should_check_fields:
            field_checks += 1
            request_evidence = evidence.for_dimension(
                EvidenceDimension.REQUEST_FIELDS
            )
            response_evidence = evidence.for_dimension(
                EvidenceDimension.RESPONSE_FIELDS
            )
            if request_evidence.status is EvidenceStatus.TRUSTED:
                request_fields = _official_fields(request_evidence.observations)
                for item in compare_request_fields(
                    api, request_fields, rust_contract
                ):
                    report.add(item)
            else:
                report.add(
                    _evidence_finding(
                        api,
                        EvidenceDimension.REQUEST_FIELDS,
                        request_evidence,
                    )
                )
            if response_evidence.status is EvidenceStatus.TRUSTED:
                response_fields = _official_fields(
                    response_evidence.observations
                )
                for item in compare_response_fields(
                    api, response_fields, rust_contract
                ):
                    report.add(item)
            else:
                report.add(
                    _evidence_finding(
                        api,
                        EvidenceDimension.RESPONSE_FIELDS,
                        response_evidence,
                    )
                )

    write_report(
        report,
        report_dir / "crates" / f"{crate_name}.md",
        report_dir / "crates" / f"{crate_name}.json",
    )
    return report


def _official_fields(observations) -> tuple[OfficialField, ...]:
    """当前 Rust comparison 只消费顶层字段，保持既有比较边界。"""
    return tuple(
        OfficialField(
            name=item.path[0],
            required=bool(item.required),
            location=item.location or "",
            field_type=item.field_type or "",
            source=item.source.value,
        )
        for item in observations
        if isinstance(item, FieldObservation) and len(item.path) == 1
    )


def _evidence_finding(api, dimension, evidence):
    diagnostic_codes = {
        diagnostic.code for diagnostic in evidence.diagnostics
    }
    acquisition_failure_codes = {
        "snapshot_unavailable",
        "adapter_unavailable",
        "acquisition_timeout",
        "acquisition_failed",
        "document_not_found",
        "document_unhealthy",
    }
    if diagnostic_codes & acquisition_failure_codes:
        code = "U_OFFICIAL_DETAIL_FETCH_FAILED"
    else:
        code = {
            EvidenceDimension.ENDPOINT: "U_LIVE_OFFICIAL_ENDPOINT_UNAVAILABLE",
            EvidenceDimension.REQUEST_FIELDS: "U_OFFICIAL_DETAIL_FETCH_FAILED",
            EvidenceDimension.RESPONSE_FIELDS: "U_OFFICIAL_DETAIL_FETCH_FAILED",
            EvidenceDimension.TOKENS: "U_ACCESS_TOKEN_UNANNOTATED",
        }[dimension]
    diagnostics = ", ".join(
        diagnostic.code for diagnostic in evidence.diagnostics
    )
    return finding(
        "UNVERIFIED",
        code,
        "Official Document Evidence is not trusted.",
        api,
        official=f"{evidence.status.value}: {diagnostics}",
    )


def _strict_exit_code(
    reports: list[ContractReport],
    strict_categories: set[str],
    live_dimensions: set[str],
) -> int:
    """Strict Evidence Gate：requested Evidence 只有 Trusted 才通过。"""
    strict_dimensions = set()
    if "endpoint" in strict_categories:
        strict_dimensions.add(EvidenceDimension.ENDPOINT.value)
    if "fields" in strict_categories:
        strict_dimensions.update(
            (
                EvidenceDimension.REQUEST_FIELDS.value,
                EvidenceDimension.RESPONSE_FIELDS.value,
            )
        )
    if "tokens" in strict_categories:
        strict_dimensions.add(EvidenceDimension.TOKENS.value)
    if not strict_dimensions:
        return 0
    required_strict_dimensions = strict_dimensions & live_dimensions
    observed_dimensions = {
        dimension["dimension"]
        for report in reports
        for api_evidence in report.evidence
        for dimension in api_evidence["dimensions"]
    }
    if not required_strict_dimensions <= observed_dimensions:
        return 1
    has_nontrusted_evidence = any(
        dimension["dimension"] in strict_dimensions
        and dimension["status"] != EvidenceStatus.TRUSTED.value
        for report in reports
        for api_evidence in report.evidence
        for dimension in api_evidence["dimensions"]
    )
    has_contract_error = any(report.error_count for report in reports)
    return 1 if has_contract_error or has_nontrusted_evidence else 0


def main() -> int:
    args = parse_args()
    if args.fields and not args.live_fields:
        print("Field validation requires --live-fields so official fields come from current docs.", file=sys.stderr)
        return 1
    csv_path = Path(args.csv)
    if not csv_path.exists():
        print(f"CSV file does not exist: {csv_path}", file=sys.stderr)
        return 1

    mapping = load_mapping(Path(args.mapping))
    if not args.all_crates and not args.crate_name:
        print("Specify --crate <name> or --all-crates", file=sys.stderr)
        return 1
    if args.api_id and not args.crate_name:
        print("--api-id 必须配合 --crate 使用", file=sys.stderr)
        return 1

    if args.crate_name:
        if args.crate_name not in mapping:
            print(f"Unknown crate in mapping: {args.crate_name}", file=sys.stderr)
            return 1
        crate_names = [args.crate_name]
    else:
        crate_names = sorted(mapping.keys())

    report_dir = Path(args.report_dir)
    with compose(
        snapshot_directory=report_dir / "official_evidence",
        timeout_seconds=args.field_timeout,
        retries=args.field_retries,
    ) as collector:
        reports = [
            validate_crate(
                crate_name,
                mapping[crate_name],
                csv_path,
                report_dir,
                args.skip_old,
                collector,
                live_endpoints=args.live_endpoints,
                fields=args.fields,
                max_field_apis=args.max_field_apis,
                tokens=args.tokens,
                biz_tag_filter=args.biz_tag,
                api_ids=set(args.api_id or []),
            )
            for crate_name in crate_names
        ]
    write_summary(reports, report_dir / "summary.md", report_dir / "summary.json")

    total_errors = sum(report.error_count for report in reports)
    total_warnings = sum(report.warn_count for report in reports)
    print(
        "API contract validation complete: "
        f"{len(reports)} crate(s), {total_errors} error(s), {total_warnings} warning(s); "
        f"reports in {report_dir}"
    )

    strict_categories = {item.strip() for item in args.strict.split(",") if item.strip()}
    live_dimensions = set()
    if args.live_endpoints:
        live_dimensions.add(EvidenceDimension.ENDPOINT.value)
    if args.fields:
        live_dimensions.update(
            (
                EvidenceDimension.REQUEST_FIELDS.value,
                EvidenceDimension.RESPONSE_FIELDS.value,
            )
        )
    if args.tokens:
        live_dimensions.add(EvidenceDimension.TOKENS.value)
    return _strict_exit_code(reports, strict_categories, live_dimensions)


if __name__ == "__main__":
    raise SystemExit(main())
