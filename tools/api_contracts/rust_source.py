"""Rust source scanning for endpoint contract validation."""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

from .models import RustApiContract, RustEndpointCall, RustField


REQUEST_STRUCT_SUFFIXES = ("Body", "Query", "Params", "RequestBody")
RESPONSE_STRUCT_SUFFIXES = ("Response", "Result")


@dataclass(frozen=True)
class EndpointResolver:
    constants: dict[str, str]
    enum_endpoints: dict[str, str] = field(default_factory=dict)

    def resolve(self, argument: str) -> tuple[str, str]:
        expression = strip_wrappers(argument)
        if not expression:
            return "", "empty endpoint argument"

        literal = parse_string_literal(expression)
        if literal:
            return literal, "literal"
        # 支持 "...".to_string() 模式
        to_string_match = re.fullmatch(r'"([^"]*)"\.to_string\(\)', expression)
        if to_string_match:
            return to_string_match.group(1), "literal"

        # 支持 String::from("...") 模式
        string_from_match = re.fullmatch(r'String::from\("([^"]*)"\)', expression)
        if string_from_match:
            return string_from_match.group(1), "literal"

        # 支持已知端点函数调用（cardkit 等）
        known_function_match = re.match(
            r'(cardkit_v1_card(?:_element(?:s|_content)?|_settings|_batch_update)?)\(([^)]*)\)',
            expression,
        )
        if known_function_match:
            func_name = known_function_match.group(1)
            known_functions = {
                "cardkit_v1_card": "/open-apis/cardkit/v1/cards/{param}",
                "cardkit_v1_card_settings": "/open-apis/cardkit/v1/cards/{param}/settings",
                "cardkit_v1_card_batch_update": "/open-apis/cardkit/v1/cards/{param}/batch_update",
                "cardkit_v1_card_elements": "/open-apis/cardkit/v1/cards/{param}/elements",
                "cardkit_v1_card_element": "/open-apis/cardkit/v1/cards/{param}/elements/{param2}",
                "cardkit_v1_card_element_content": "/open-apis/cardkit/v1/cards/{param}/elements/{param2}/content",
            }
            if func_name in known_functions:
                return known_functions[func_name], f"func:{func_name}"

        if expression in self.constants:
            return self.constants[expression], f"const:{expression}"

        # 支持 CONSTANT.replace("...", value) 模式，包括后面的字符串拼接
        replace_match = re.search(r'([A-Z_][A-Z0-9_]*)\s*\.replace\("([^"]*)",[^)]*\)', expression, re.DOTALL)
        if replace_match:
            constant_name = replace_match.group(1)
            if constant_name in self.constants:
                base = self.constants[constant_name]
                # 处理所有的 .replace("...", ...) 调用
                for replace_call in re.finditer(r'\.replace\("([^"]*)",[^)]*\)', expression, re.DOTALL):
                    placeholder = replace_call.group(1)
                    base = base.replace(placeholder, "{param}")
                # 如果后面有 + "/" + ... 拼接，在末尾添加 /{param}
                if re.search(r'\+?\s*"/"\s*\+\s*&?\w+', expression):
                    base = base.rstrip("/") + "/{param}"
                return base, "constant_replace"


 
        # 枚举端点解析（支持 .to_url() 和 .path()）
        enum_endpoint = resolve_enum_to_url_expression(expression, self.enum_endpoints)
        if enum_endpoint:
            # 去除 .to_url() 或 .path() 后缀
            enum_reference = expression
            for suffix in (".to_url()", ".path()"):
                if enum_reference.endswith(suffix):
                    enum_reference = enum_reference[: -len(suffix)]
                    break
            return enum_endpoint, f"enum:{enum_reference}"

        # format! 表达式可能在参数中包含 .to_url()，优先处理
        if expression.startswith("format!"):
            resolved = resolve_format_expression(expression, self.constants)
            if resolved:
                return resolved, "format"
            return "", "format! endpoint could not be resolved"

        if ".to_url()" in expression or ".path()" in expression:
            return "", "endpoint enum to_url() expression could not be resolved"

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
        elif re.search(r"(self\.|_id\b|token\b|\.to_url\(\)|\.path\(\)|\.join\()", arg_expr):
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


def resolve_enum_to_url_expression(expression: str, enum_endpoints: dict[str, str]) -> str:
    expr = expression.strip()
    # 支持完整模块路径如 crate::common::api_endpoints::EnumName::VariantName
    # 匹配末尾的 EnumName::VariantName
    match = re.search(
        r"([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)\b(?!::)",
        expr,
    )
    if not match:
        return ""
    enum_name = match.group(1)
    variant = match.group(2)
    # 检查后面是否跟着 .to_url() 或 .path()
    pos = match.end()
    rest = expr[pos:].strip()
    # 跳过可选的括号参数 ( ... )
    if rest.startswith("("):
        paren_open = expr.find("(", pos)
        if paren_open >= 0:
            paren_close = find_matching_paren(expr, paren_open)
            if paren_close >= 0:
                pos = paren_close + 1
                rest = expr[pos:].strip()
    # 检查后面是否跟着 .to_url() 或 .path()
    if rest and not (rest.startswith(".to_url()") or rest.startswith(".path()")):
        return ""
    return enum_endpoints.get(f"{enum_name}::{variant}", "")

def extract_enum_endpoint_aliases(text: str, enum_endpoints: dict[str, str]) -> dict[str, str]:
    aliases: dict[str, str] = {}
    if not enum_endpoints:
        return aliases

    # 匹配 let <var> = <optional_module_path>::EnumName::VariantName(<optional_args>);
    # 支持完整路径如 crate::common::api_endpoints::ApprovalApiV4::ApprovalCreate
    assignment_pattern = re.compile(
        r"let\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"
        r"((?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*)"
        r"(?:\s*\([^;]*?\))?\s*(?:\.\w+\s*\([^)]*\)\s*)*\s*;",
        re.DOTALL,
    )
    for match in assignment_pattern.finditer(text):
        expression = re.sub(r"\s+", " ", match.group(2).strip())
        # 提取最后的 EnumName::VariantName 部分
        enum_match = re.search(r"([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)(?:\s*\(|$)", expression)
        if enum_match and f"{enum_match.group(1)}::{enum_match.group(2)}" in enum_endpoints:
            aliases[match.group(1)] = expression
    # 匹配 let <var> = <known_alias>.to_url() 或 <known_alias>.path()
    chain_pattern = re.compile(
        r"let\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"
        r"([A-Za-z_][A-Za-z0-9_]*)\.(to_url|path)\(\)\s*;",
        re.DOTALL,
    )
    for match in chain_pattern.finditer(text):
        var_name = match.group(2)
        if var_name in aliases:
            aliases[match.group(1)] = aliases[var_name]

    return aliases

def find_variable_assignment(text: str, variable: str) -> str:
    """查找文件中变量的赋值表达式，支持 format! 和直接字符串。"""
    pattern = re.compile(
        rf"let\s+(?:mut\s+)?{re.escape(variable)}\s*=\s*(.*?)\s*;",
        re.DOTALL,
    )
    match = pattern.search(text)
    if match:
        assignment = match.group(1).strip()
        # 只处理 format!、直接字符串字面量或常量.replace()调用
        if assignment.startswith("format!") or assignment.startswith('"') or re.search(r'[A-Z_][A-Z0-9_]*\s*\.replace\(', assignment, re.DOTALL) or re.search(r'[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*', assignment) or re.search(r'[A-Za-z_][A-Za-z0-9_]*\(', assignment) or assignment.startswith("String::from("):
            return assignment
    return ""


def expand_endpoint_alias(argument: str, enum_aliases: dict[str, str], file_text: str = "", enum_endpoints: dict[str, str] | None = None) -> str:
    expression = strip_wrappers(argument)
    
    # 如果变量名本身在别名中，返回其枚举表达式
    if expression in enum_aliases:
        return enum_aliases[expression]
    
    # 支持任何变量名的 .to_url() 或 .path() 调用
    match = re.fullmatch(r"([A-Za-z_][A-Za-z0-9_]*)\.(to_url|path)\(\)", expression)
    if match:
        enum_expression = enum_aliases.get(match.group(1))
        if enum_expression:
            prefix = "&" if argument.strip().startswith("&") else ""
            method = match.group(2)
            return f"{prefix}{enum_expression}.{method}()"
    
    # 支持链式调用如 var.to_url().replace(...) 或 var.path().to_string()
    chain_match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\.(to_url|path)\(\)(\..+)", expression)
    if chain_match:
        enum_expression = enum_aliases.get(chain_match.group(1))
        if enum_expression:
            prefix = "&" if argument.strip().startswith("&") else ""
            method = chain_match.group(2)
            rest = chain_match.group(3)
            return f"{prefix}{enum_expression}.{method}(){rest}"
    
    # 如果变量未被解析且 file_text 提供，尝试从文件文本中查找变量赋值
    if file_text and expression not in ("self",) and re.match(r"[A-Za-z_][A-Za-z0-9_]*$", expression):
        assignment = find_variable_assignment(file_text, expression)
        if assignment:
            # 如果赋值是 format! 且包含变量调用，尝试扩展变量
            if assignment.startswith("format!") and enum_endpoints:
                def replace_var_call(match: re.Match[str]) -> str:
                    var_name = match.group(1)
                    method = match.group(2)
                    enum_expr = enum_aliases.get(var_name)
                    if enum_expr:
                        endpoint = resolve_enum_to_url_expression(f"{enum_expr}.{method}()", enum_endpoints)
                        if endpoint:
                            return f'"{endpoint}"'
                    return match.group(0)
                assignment = re.sub(
                    r'([A-Za-z_][A-Za-z0-9_]*)\.(to_url|path)\(\)',
                    replace_var_call,
                    assignment,
                )
            return assignment
    
    return argument



def extract_endpoint_template(expression: str, constants: dict[str, str] | None = None) -> str:
    constants = constants or {}
    expr = expression.strip().rstrip(",").strip()
    if expr.startswith("{") and expr.endswith("}"):
        expr = expr[1:-1].strip().rstrip(",").strip()

    format_start = expr.find("format!(")
    if format_start >= 0:
        open_paren = expr.find("(", format_start)
        close_paren = find_matching_paren(expr, open_paren)
        if close_paren >= 0:
            return resolve_format_expression(expr[format_start : close_paren + 1], {})

    string_match = re.search(r'"(/open-apis/[^"]*)"\s*(?:\.to_string\(\))?', expr, re.DOTALL)
    if string_match:
        return string_match.group(1)

    # 支持常量引用（如 APP_ACCESS_TOKEN_INTERNAL_URL_PATH）
    constant_match = re.fullmatch(r'([A-Z_][A-Z0-9_]*)', expr.strip())
    if constant_match:
        return constants.get(constant_match.group(1), "")

    return ""
    if string_match:
        return string_match.group(1)

    return ""


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

    # 加载 openlark-core 的常量（其他 crate 可能导入这些常量）
    core_src = crate_src.parent.parent / "openlark-core" / "src"
    if core_src.exists():
        for path in iter_rust_files(core_src):
            text = path.read_text(encoding="utf-8")
            for match in re.finditer(r'pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"\s*;', text):
                constants[match.group(1)] = match.group(2)
            for match in re.finditer(r"pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+)\s*;", text):
                aliases[match.group(1)] = match.group(2)

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


def load_enum_endpoints(crate_src: Path, constants: dict[str, str] | None = None) -> dict[str, str]:
    enum_endpoints: dict[str, str] = {}
    all_constants = constants or {}
    type_aliases: dict[str, str] = {}

    for path in sorted(crate_src.rglob("api_endpoints.rs")):
        if "__pycache__" in path.parts:
            continue
        text = path.read_text(encoding="utf-8")
        enum_variants = parse_enum_variants(text)
        # 加载本地常量
        local_constants = dict(all_constants)
        for match in re.finditer(r'pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"\s*;', text):
            local_constants[match.group(1)] = match.group(2)
        for match in re.finditer(r"pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+)\s*;", text):
            if match.group(2) in local_constants:
                local_constants[match.group(1)] = local_constants[match.group(2)]
        # 解析 type 别名
        for match in re.finditer(r'pub\s+type\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;', text):
            type_aliases[match.group(1)] = match.group(2)
        for enum_name, variants in enum_variants.items():
            enum_endpoints.update(parse_enum_to_url_endpoints(text, enum_name, variants, local_constants))

    # 应用 type 别名到枚举端点映射
    for alias, target_enum in type_aliases.items():
        for key, value in list(enum_endpoints.items()):
            if key.startswith(f"{target_enum}::"):
                alias_key = key.replace(f"{target_enum}::", f"{alias}::", 1)
                enum_endpoints[alias_key] = value

    return enum_endpoints


def parse_enum_variants(text: str) -> dict[str, set[str]]:
    variants_by_enum: dict[str, set[str]] = {}
    for match in re.finditer(r"pub\s+enum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", text):
        enum_name = match.group(1)
        open_brace = text.find("{", match.end() - 1)
        close_brace = find_matching_brace(text, open_brace)
        if close_brace < 0:
            continue
        body = text[open_brace + 1 : close_brace]
        variants = set(re.findall(r"^\s*([A-Za-z_][A-Za-z0-9_]*)(?:\s*\(|\s*,)", body, re.MULTILINE))
        variants_by_enum[enum_name] = variants
    return variants_by_enum


def parse_enum_to_url_endpoints(text: str, enum_name: str, variants: set[str], constants: dict[str, str] | None = None) -> dict[str, str]:
    impl_match = re.search(rf"impl\s+{re.escape(enum_name)}\s*\{{", text)
    if not impl_match:
        return {}
    impl_open = text.find("{", impl_match.end() - 1)
    impl_close = find_matching_brace(text, impl_open)
    if impl_close < 0:
        return {}
    impl_body = text[impl_open + 1 : impl_close]

    fn_match = re.search(r"pub\s+fn\s+(to_url|path)\s*\([^)]*\)\s*->\s*(?:String|&'static\s+str)\s*\{", impl_body)
    if not fn_match:
        return {}
    fn_open = impl_body.find("{", fn_match.end() - 1)
    fn_close = find_matching_brace(impl_body, fn_open)
    if fn_close < 0:
        return {}
    fn_body = impl_body[fn_open + 1 : fn_close]

    match_pos = fn_body.find("match self")
    if match_pos < 0:
        return {}
    match_open = fn_body.find("{", match_pos)
    match_close = find_matching_brace(fn_body, match_open)
    if match_close < 0:
        return {}
    match_body = fn_body[match_open + 1 : match_close]

    arm_pattern = re.compile(
        rf"((?:\s*\|?\s*{re.escape(enum_name)}::[A-Za-z_][A-Za-z0-9_]*(?:\s*\([^=]*?\))?)+)\s*=>",
        re.MULTILINE | re.DOTALL,
    )
    arms = list(arm_pattern.finditer(match_body))
    endpoints: dict[str, str] = {}
    for index, arm in enumerate(arms):
        next_start = arms[index + 1].start() if index + 1 < len(arms) else len(match_body)
        arm_expression = match_body[arm.end() : next_start]
        template = extract_endpoint_template(arm_expression, constants or {})
        if not template:
            continue
        arm_variants = re.findall(rf"{re.escape(enum_name)}::([A-Za-z_][A-Za-z0-9_]*)", arm.group(1))
        for variant in arm_variants:
            if variants and variant not in variants:
                continue
            endpoints[f"{enum_name}::{variant}"] = template
    return endpoints


def iter_rust_files(root: Path) -> Iterable[Path]:
    if not root.exists():
        return []
    return sorted(path for path in root.rglob("*.rs") if "__pycache__" not in path.parts)


def extract_endpoint_calls(text: str, resolver: EndpointResolver) -> tuple[RustEndpointCall, ...]:
    calls: list[RustEndpointCall] = []
    enum_aliases = extract_enum_endpoint_aliases(text, resolver.enum_endpoints)
    pattern = re.compile(r"ApiRequest(?:::\s*<[^>]+>>?)?::(get|post|put|patch|delete)\s*\(")
    for match in pattern.finditer(text):
        method = match.group(1).upper()
        open_paren = text.find("(", match.end() - 1)
        close_paren = find_matching_paren(text, open_paren)
        if close_paren < 0:
            continue
        argument = text[open_paren + 1 : close_paren].strip()
        first_argument = split_top_level_args(argument)[0] if argument else ""
        resolved_argument = expand_endpoint_alias(first_argument, enum_aliases, text, resolver.enum_endpoints)
        resolved_path, source_or_reason = resolver.resolve(resolved_argument)
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


def scan_api_file(
    crate_src: Path,
    expected_file: str,
    constants: dict[str, str] | None = None,
    enum_endpoints: dict[str, str] | None = None,
) -> RustApiContract | None:
    path = crate_src / expected_file
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8")
    resolver = EndpointResolver(
        constants or load_endpoint_constants(crate_src),
        enum_endpoints or load_enum_endpoints(crate_src),
    )
    return RustApiContract(
        rel_path=expected_file,
        endpoint_calls=extract_endpoint_calls(text, resolver),
        fields=extract_rust_fields(text),
        response_fields=extract_rust_response_fields(text),
    )
