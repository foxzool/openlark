"""Rust source scanning for endpoint contract validation."""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

from .models import RustApiContract, RustEndpointCall, RustField


REQUEST_STRUCT_SUFFIXES = ("Body", "Query", "Params", "RequestBody")
RESPONSE_STRUCT_SUFFIXES = ("Response", "Result")


@dataclass(frozen=True)
class EndpointResolver:
    constants: dict[str, str]

    def resolve(self, argument: str) -> tuple[str, str]:
        expression = strip_wrappers(argument)
        if not expression:
            return "", "empty endpoint argument"

        literal = parse_string_literal(expression)
        if literal:
            return literal, "literal"

        if expression in self.constants:
            return self.constants[expression], f"const:{expression}"

        if ".to_url()" in expression:
            return "", "endpoint enum to_url() resolution not implemented"

        if expression.startswith("format!("):
            resolved = resolve_format_expression(expression, self.constants)
            if resolved:
                return resolved, "format"
            return "", "format! endpoint could not be resolved"

        return "", f"unresolved endpoint expression: {expression}"


def line_of(text: str, index: int) -> int:
    return text.count("\n", 0, max(index, 0)) + 1


def find_matching_paren(text: str, open_paren_idx: int) -> int:
    if open_paren_idx < 0 or open_paren_idx >= len(text) or text[open_paren_idx] != "(":
        return -1
    depth = 0
    in_string = False
    escaped = False
    for index in range(open_paren_idx, len(text)):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
    return -1


def find_matching_brace(text: str, open_brace_idx: int) -> int:
    if open_brace_idx < 0 or open_brace_idx >= len(text) or text[open_brace_idx] != "{":
        return -1
    depth = 0
    in_string = False
    escaped = False
    for index in range(open_brace_idx, len(text)):
        char = text[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return index
    return -1


def strip_wrappers(argument: str) -> str:
    expr = argument.strip()
    while expr.startswith("&"):
        expr = expr[1:].strip()
    return expr


def parse_string_literal(expression: str) -> str:
    match = re.fullmatch(r'"([^"]*)"', expression.strip(), re.DOTALL)
    return match.group(1) if match else ""


def snake_to_camel(name: str) -> str:
    parts = name.split("_")
    return parts[0] + "".join(part[:1].upper() + part[1:] for part in parts[1:])


def apply_rename_rule(field_name: str, rename_all: str) -> str:
    if rename_all in {"camelCase", "lowerCamelCase"}:
        return snake_to_camel(field_name)
    if rename_all == "snake_case":
        return field_name
    return field_name


def resolve_format_expression(expression: str, constants: dict[str, str]) -> str:
    template_match = re.match(
        r'format!\(\s*"([^"]+)"\s*(?:,\s*(.*))?\)\s*$',
        expression,
        re.DOTALL,
    )
    if not template_match:
        return ""

    template = template_match.group(1)
    args = split_top_level_args(template_match.group(2) or "")
    if not args:
        return resolve_captured_format_template(template, constants)

    parts = template.split("{}")
    if len(parts) - 1 != len(args):
        return ""

    resolved = parts[0]
    for arg, suffix in zip(args, parts[1:], strict=True):
        arg_expr = strip_wrappers(arg)
        if arg_expr in constants:
            value = constants[arg_expr]
        elif re.search(r"(self\.|_id\b|token\b)", arg_expr):
            value = "{param}"
        else:
            return ""
        resolved += value + suffix
    return resolved


def resolve_captured_format_template(template: str, constants: dict[str, str]) -> str:
    def replace_capture(match: re.Match[str]) -> str:
        name = match.group(1)
        if name in constants:
            return constants[name]
        return "{param}"

    resolved = re.sub(r"\{([A-Za-z_][A-Za-z0-9_]*)\}", replace_capture, template)
    return resolved if resolved.startswith("/open-apis/") else ""


def split_top_level_args(text: str) -> list[str]:
    if not text.strip():
        return []
    args: list[str] = []
    start = 0
    depth = 0
    in_string = False
    escaped = False
    for index, char in enumerate(text):
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif char == "," and depth == 0:
            args.append(text[start:index].strip())
            start = index + 1
    tail = text[start:].strip()
    if tail:
        args.append(tail)
    return args


def load_endpoint_constants(crate_src: Path) -> dict[str, str]:
    constants: dict[str, str] = {}
    aliases: dict[str, str] = {}

    for path in iter_rust_files(crate_src):
        text = path.read_text(encoding="utf-8")
        for match in re.finditer(r'pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"\s*;', text):
            constants[match.group(1)] = match.group(2)
        for match in re.finditer(r"pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+)\s*;", text):
            aliases[match.group(1)] = match.group(2)

    changed = True
    while changed:
        changed = False
        for name, target in aliases.items():
            if name in constants:
                continue
            if target in constants:
                constants[name] = constants[target]
                changed = True

    return constants


def iter_rust_files(root: Path) -> Iterable[Path]:
    if not root.exists():
        return []
    return sorted(path for path in root.rglob("*.rs") if "__pycache__" not in path.parts)


def extract_endpoint_calls(text: str, resolver: EndpointResolver) -> tuple[RustEndpointCall, ...]:
    calls: list[RustEndpointCall] = []
    pattern = re.compile(r"ApiRequest(?:::\s*<[^>]+>)?::(get|post|put|patch|delete)\s*\(")
    for match in pattern.finditer(text):
        method = match.group(1).upper()
        open_paren = text.find("(", match.end() - 1)
        close_paren = find_matching_paren(text, open_paren)
        if close_paren < 0:
            continue
        argument = text[open_paren + 1 : close_paren].strip()
        first_argument = split_top_level_args(argument)[0] if argument else ""
        resolved_path, source_or_reason = resolver.resolve(first_argument)
        if resolved_path:
            calls.append(
                RustEndpointCall(
                    method=method,
                    argument=first_argument,
                    line=line_of(text, match.start()),
                    resolved_path=resolved_path,
                    source=source_or_reason,
                )
            )
        else:
            calls.append(
                RustEndpointCall(
                    method=method,
                    argument=first_argument,
                    line=line_of(text, match.start()),
                    unresolved_reason=source_or_reason,
                )
            )
    return tuple(calls)


def extract_rust_fields(text: str) -> tuple[RustField, ...]:
    return (
        extract_rust_struct_fields(text, REQUEST_STRUCT_SUFFIXES)
        + extract_file_content_fields(text)
    )


def extract_rust_response_fields(text: str) -> tuple[RustField, ...]:
    return extract_rust_struct_fields(text, RESPONSE_STRUCT_SUFFIXES)


def extract_file_content_fields(text: str) -> tuple[RustField, ...]:
    fields: list[RustField] = []
    pattern = re.compile(r"\.file_content\(\s*(?:body|self)\.([A-Za-z_][A-Za-z0-9_]*)")
    for match in pattern.finditer(text):
        fields.append(
            RustField(
                struct_name="MultipartFile",
                field_name=match.group(1),
                serialized_name="file",
                type_name="Vec<u8>",
                optional=False,
                line=line_of(text, match.start()),
            )
        )
    return tuple(fields)


def extract_rust_struct_fields(text: str, suffixes: tuple[str, ...]) -> tuple[RustField, ...]:
    fields: list[RustField] = []
    for match in re.finditer(r"pub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", text):
        struct_name = match.group(1)
        if not struct_name.endswith(suffixes):
            continue
        open_brace = text.find("{", match.end() - 1)
        close_brace = find_matching_brace(text, open_brace)
        if close_brace < 0:
            continue
        struct_attrs = preceding_attrs(text, match.start())
        rename_all = serde_rename_all(struct_attrs)
        body = text[open_brace + 1 : close_brace]
        base_line = line_of(text, open_brace + 1)
        fields.extend(extract_struct_fields(struct_name, body, base_line, rename_all))
    return tuple(fields)


def preceding_attrs(text: str, start_index: int) -> str:
    prefix = text[:start_index]
    lines = prefix.splitlines()
    attrs: list[str] = []
    for line in reversed(lines):
        stripped = line.strip()
        if stripped.startswith("#["):
            attrs.append(stripped)
            continue
        if not stripped:
            continue
        break
    return "\n".join(reversed(attrs))


def serde_rename_all(attrs: str) -> str:
    match = re.search(r'rename_all\s*=\s*"([^"]+)"', attrs)
    return match.group(1) if match else ""


def extract_struct_fields(
    struct_name: str,
    body: str,
    base_line: int,
    rename_all: str,
) -> list[RustField]:
    fields: list[RustField] = []
    pending_attrs: list[str] = []
    for offset, line in enumerate(body.splitlines()):
        stripped = line.strip()
        if not stripped:
            continue
        if stripped.startswith("#["):
            pending_attrs.append(stripped)
            continue
        match = re.match(r"pub\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,]+),", stripped)
        if not match:
            pending_attrs.clear()
            continue
        attrs = "\n".join(pending_attrs)
        pending_attrs.clear()
        if "skip_serializing" in attrs and "skip_serializing_if" not in attrs:
            continue
        field_name = match.group(1)
        type_name = match.group(2).strip()
        rename_match = re.search(r'rename\s*=\s*"([^"]+)"', attrs)
        serialized_name = rename_match.group(1) if rename_match else apply_rename_rule(field_name, rename_all)
        fields.append(
            RustField(
                struct_name=struct_name,
                field_name=field_name,
                serialized_name=serialized_name,
                type_name=type_name,
                optional=is_optional_type(type_name),
                line=base_line + offset,
            )
        )
    return fields


def is_optional_type(type_name: str) -> bool:
    return bool(re.match(r"(std::option::)?Option\s*<", type_name.strip()))


def scan_api_file(crate_src: Path, expected_file: str, constants: dict[str, str] | None = None) -> RustApiContract | None:
    path = crate_src / expected_file
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8")
    resolver = EndpointResolver(constants or load_endpoint_constants(crate_src))
    return RustApiContract(
        rel_path=expected_file,
        endpoint_calls=extract_endpoint_calls(text, resolver),
        fields=extract_rust_fields(text),
        response_fields=extract_rust_response_fields(text),
    )
