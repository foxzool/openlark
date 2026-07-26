"""Rust source scanning for endpoint contract validation."""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

from .models import DEFAULT_ACCESS_TOKEN_TYPES, RustApiContract, RustEndpointCall, RustField


REQUEST_STRUCT_SUFFIXES = ("Body", "Query", "Params", "RequestBody")
RESPONSE_STRUCT_SUFFIXES = ("Response", "Result", "Resp")

# AccessTokenType 枚举变体 → 飞书凭证名（与 constants.rs 的 as_str/Display 一致）。
# 用于把 Rust 声明的 token 类型翻译成可与官方 supportedAccessToken 直接比对的形态。
_ACCESS_TOKEN_VARIANT_TO_FEISHU: dict[str, str] = {
    "None": "none_access_token",
    "App": "app_access_token",
    "Tenant": "tenant_access_token",
    "User": "user_access_token",
}


@dataclass(frozen=True)
class EndpointResolver:
    constants: dict[str, str]
    enum_endpoints: dict[str, str] = field(default_factory=dict)
    enum_methods: dict[str, str] = field(default_factory=dict)

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


 
        # CatalogEndpoint::to_request()（docs 域主路径，#568）
        to_request_path = resolve_enum_to_request_expression(expression, self.enum_endpoints)
        if to_request_path:
            enum_key = enum_key_from_expression(expression)
            return to_request_path, f"to_request:{enum_key or expression}"

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

        if ".to_request" in expression:
            return "", "endpoint enum to_request() expression could not be resolved"

        return "", f"unresolved endpoint expression: {expression}"

    def resolve_method(self, argument: str, fallback: str = "") -> str:
        """从 CatalogEndpoint method() 表解析 HTTP method；失败时回落 fallback。"""
        expression = strip_wrappers(argument)
        enum_key = enum_key_from_expression(expression)
        if enum_key and enum_key in self.enum_methods:
            return self.enum_methods[enum_key]
        bare = resolve_enum_key(expression)
        if bare and bare in self.enum_methods:
            return self.enum_methods[bare]
        return fallback


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
        elif parse_string_literal(arg_expr):
            value = parse_string_literal(arg_expr)
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


def resolve_enum_key(expression: str) -> str:
    """从表达式中提取 EnumName::Variant 键（忽略参数与后缀方法调用）。"""
    expr = expression.strip()
    match = re.search(
        r"([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)\b(?!::)",
        expr,
    )
    if not match:
        return ""
    return f"{match.group(1)}::{match.group(2)}"


def enum_key_from_expression(expression: str) -> str:
    """兼容 .to_url/.path/.to_request 后缀的 EnumName::Variant 键提取。"""
    expr = expression.strip()
    for suffix in (".to_request()", ".to_url()", ".path()"):
        if expr.endswith(suffix):
            expr = expr[: -len(suffix)].strip()
            break
    with_call = re.match(
        r"(.+?)\.to_request(?:_with_url)?(?:\s*::\s*<[^()]+>)?\s*\(.*\)\s*$",
        expr,
        re.DOTALL,
    )
    if with_call:
        expr = with_call.group(1).strip()
    return resolve_enum_key(expr)


_TO_REQUEST_CALL_RE = re.compile(
    r"\.to_request(?:_with_url)?(?:\s*::\s*<[^()]+>)?\s*\(",
)


def resolve_enum_to_request_expression(expression: str, enum_endpoints: dict[str, str]) -> str:
    """解析 Enum::Variant(...).to_request() / .to_request_with_url(...) 的路径。"""
    expr = expression.strip()
    if not _TO_REQUEST_CALL_RE.search(expr):
        key = resolve_enum_key(expr)
        if key and key in enum_endpoints and ("::" in expr) and ".to_" not in expr:
            if re.match(
                r"^(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*::[A-Za-z_][A-Za-z0-9_]*"
                r"(?:\s*\(.*\))?\s*$",
                expr,
                re.DOTALL,
            ):
                return enum_endpoints[key]
        return ""

    match = re.search(
        r"([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)\b(?!::)",
        expr,
    )
    if not match:
        return ""
    enum_name = match.group(1)
    variant = match.group(2)
    pos = match.end()
    rest = expr[pos:].strip()
    if rest.startswith("("):
        paren_open = expr.find("(", pos)
        if paren_open >= 0:
            paren_close = find_matching_paren(expr, paren_open)
            if paren_close >= 0:
                pos = paren_close + 1
                rest = expr[pos:].strip()
    if not _TO_REQUEST_CALL_RE.match(rest):
        return ""
    return enum_endpoints.get(f"{enum_name}::{variant}", "")


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


def iter_api_endpoint_definition_files(crate_src: Path) -> list[Path]:
    """枚举端点定义文件：`api_endpoints.rs` 以及 `api_endpoints/**/*.rs` 子模块。

    docs 域把 catalog 拆到 `common/api_endpoints/{lingo,drive,...}.rs`；仅扫顶层
    `api_endpoints.rs` 会漏掉绝大部分路径/method 映射（#568）。
    """
    if not crate_src.exists():
        return []
    paths: list[Path] = []
    for path in crate_src.rglob("*.rs"):
        if "__pycache__" in path.parts:
            continue
        if path.name == "api_endpoints.rs" or "api_endpoints" in path.parts:
            paths.append(path)
    return sorted(set(paths))


def extract_endpoint_type_aliases(text: str) -> dict[str, str]:
    """解析 `type Alias = Target` 与 `pub use path::Target as Alias` 端点别名。"""
    aliases: dict[str, str] = {}
    for match in re.finditer(
        r"pub\s+type\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;",
        text,
    ):
        aliases[match.group(1)] = match.group(2)
    for match in re.finditer(
        r"pub\s+use\s+(?:[A-Za-z_][A-Za-z0-9_]*::)*([A-Za-z_][A-Za-z0-9_]*)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)\s*;",
        text,
    ):
        aliases[match.group(2)] = match.group(1)
    return aliases


def apply_enum_aliases(mapping: dict[str, str], type_aliases: dict[str, str]) -> dict[str, str]:
    """把 Target::Variant 映射复制到 Alias::Variant。"""
    result = dict(mapping)
    for alias, target_enum in type_aliases.items():
        for key, value in list(mapping.items()):
            if key.startswith(f"{target_enum}::"):
                alias_key = f"{alias}::{key[len(target_enum) + 2 :]}"
                result[alias_key] = value
    return result


def load_enum_endpoints(crate_src: Path, constants: dict[str, str] | None = None) -> dict[str, str]:
    enum_endpoints: dict[str, str] = {}
    all_constants = constants or {}
    type_aliases: dict[str, str] = {}

    for path in iter_api_endpoint_definition_files(crate_src):
        text = path.read_text(encoding="utf-8")
        enum_variants = parse_enum_variants(text)
        local_constants = dict(all_constants)
        for match in re.finditer(r'pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"\s*;', text):
            local_constants[match.group(1)] = match.group(2)
        for match in re.finditer(r"pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+)\s*;", text):
            if match.group(2) in local_constants:
                local_constants[match.group(1)] = local_constants[match.group(2)]
        type_aliases.update(extract_endpoint_type_aliases(text))
        for enum_name, variants in enum_variants.items():
            enum_endpoints.update(parse_enum_to_url_endpoints(text, enum_name, variants, local_constants))

    return apply_enum_aliases(enum_endpoints, type_aliases)


def load_enum_methods(crate_src: Path) -> dict[str, str]:
    """加载 CatalogEndpoint / inherent `method()` → EnumName::Variant → HTTP method。"""
    enum_methods: dict[str, str] = {}
    type_aliases: dict[str, str] = {}

    for path in iter_api_endpoint_definition_files(crate_src):
        text = path.read_text(encoding="utf-8")
        type_aliases.update(extract_endpoint_type_aliases(text))
        enum_variants = parse_enum_variants(text)
        for enum_name, variants in enum_variants.items():
            enum_methods.update(parse_enum_methods(text, enum_name, variants))

    return apply_enum_aliases(enum_methods, type_aliases)


def parse_enum_methods(text: str, enum_name: str, variants: set[str]) -> dict[str, str]:
    """从 `impl CatalogEndpoint for Enum` / `impl Enum` 中的 `fn method` 解析 HTTP method。"""
    methods: dict[str, str] = {}
    impl_patterns = (
        rf"impl\s+CatalogEndpoint\s+for\s+{re.escape(enum_name)}\s*\{{",
        rf"impl\s+{re.escape(enum_name)}\s*\{{",
    )
    for impl_pattern in impl_patterns:
        for impl_match in re.finditer(impl_pattern, text):
            impl_open = text.find("{", impl_match.end() - 1)
            impl_close = find_matching_brace(text, impl_open)
            if impl_close < 0:
                continue
            impl_body = text[impl_open + 1 : impl_close]
            fn_match = re.search(
                r"(?:pub\s+)?fn\s+method\s*\([^)]*\)\s*->\s*HttpMethod\s*\{",
                impl_body,
            )
            if not fn_match:
                continue
            fn_open = impl_body.find("{", fn_match.end() - 1)
            fn_close = find_matching_brace(impl_body, fn_open)
            if fn_close < 0:
                continue
            fn_body = impl_body[fn_open + 1 : fn_close]
            parsed = _parse_method_fn_body(fn_body, enum_name, variants)
            if parsed:
                methods.update(parsed)
                return methods
    return methods


def _parse_method_fn_body(fn_body: str, enum_name: str, variants: set[str]) -> dict[str, str]:
    """解析 method() 函数体：常量返回或 match self 臂。"""
    stripped = fn_body.strip()
    simple = re.fullmatch(r"HttpMethod::(Get|Post|Put|Patch|Delete)\s*", stripped)
    if simple:
        method = simple.group(1).upper()
        return {f"{enum_name}::{variant}": method for variant in variants}

    match_pos = fn_body.find("match self")
    if match_pos < 0:
        return {}
    match_open = fn_body.find("{", match_pos)
    match_close = find_matching_brace(fn_body, match_open)
    if match_close < 0:
        return {}
    match_body = fn_body[match_open + 1 : match_close]

    methods: dict[str, str] = {}
    enum_re = re.escape(enum_name)
    arm_pattern = re.compile(
        rf"(?P<head>(?:(?:Self|{enum_re})::[A-Za-z_][A-Za-z0-9_]*(?:\s*\([^=>]*?\))?\s*\|\s*)*"
        rf"(?:Self|{enum_re})::[A-Za-z_][A-Za-z0-9_]*(?:\s*\([^=>]*?\))?)"
        r"\s*=>",
        re.DOTALL,
    )
    heads = list(arm_pattern.finditer(match_body))
    for index, arm in enumerate(heads):
        next_start = heads[index + 1].start() if index + 1 < len(heads) else len(match_body)
        arm_span = match_body[arm.start() : next_start]
        method_match = re.search(r"HttpMethod::(Get|Post|Put|Patch|Delete)", arm_span)
        if not method_match:
            continue
        method = method_match.group(1).upper()
        arm_variants = re.findall(
            rf"(?:Self|{re.escape(enum_name)})::([A-Za-z_][A-Za-z0-9_]*)",
            arm.group("head"),
        )
        for variant in arm_variants:
            if variants and variant not in variants:
                continue
            methods[f"{enum_name}::{variant}"] = method
    return methods


def parse_enum_variants(text: str) -> dict[str, set[str]]:
    variants_by_enum: dict[str, set[str]] = {}
    # 兼容 `pub enum` 与 `pub(crate) enum`（minutes Extra catalog 等）
    for match in re.finditer(
        r"pub(?:\s*\([^)]*\))?\s+enum\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{",
        text,
    ):
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
    """从 inherent `impl Enum` 或 `impl CatalogEndpoint for Enum` 的 to_url/path 解析路径。

    docs 的 MinutesExtraApiV1 等仅在 CatalogEndpoint impl 中实现 to_url（#568）。
    """
    impl_patterns = (
        rf"impl\s+{re.escape(enum_name)}\s*\{{",
        rf"impl\s+CatalogEndpoint\s+for\s+{re.escape(enum_name)}\s*\{{",
    )
    endpoints: dict[str, str] = {}
    for impl_pattern in impl_patterns:
        for impl_match in re.finditer(impl_pattern, text):
            impl_open = text.find("{", impl_match.end() - 1)
            impl_close = find_matching_brace(text, impl_open)
            if impl_close < 0:
                continue
            impl_body = text[impl_open + 1 : impl_close]

            fn_match = re.search(
                r"(?:pub(?:\s*\([^)]*\))?\s+)?fn\s+(to_url|path)\s*\([^)]*\)\s*->\s*(?:String|&'static\s+str)\s*\{",
                impl_body,
            )
            if not fn_match:
                continue
            fn_open = impl_body.find("{", fn_match.end() - 1)
            fn_close = find_matching_brace(impl_body, fn_open)
            if fn_close < 0:
                continue
            fn_body = impl_body[fn_open + 1 : fn_close]

            if re.search(rf"{re.escape(enum_name)}::(to_url|path)\s*\(", fn_body) and "match self" not in fn_body:
                continue

            match_pos = fn_body.find("match self")
            if match_pos < 0:
                template = extract_endpoint_template(fn_body, constants or {})
                if template:
                    for variant in variants:
                        endpoints[f"{enum_name}::{variant}"] = template
                    return endpoints
                continue
            match_open = fn_body.find("{", match_pos)
            match_close = find_matching_brace(fn_body, match_open)
            if match_close < 0:
                continue
            match_body = fn_body[match_open + 1 : match_close]

            arm_pattern = re.compile(
                rf"((?:\s*\|?\s*(?:{re.escape(enum_name)}|Self)::[A-Za-z_][A-Za-z0-9_]*(?:\s*\([^=]*?\))?)+)\s*=>",
                re.MULTILINE | re.DOTALL,
            )
            arms = list(arm_pattern.finditer(match_body))
            for index, arm in enumerate(arms):
                next_start = arms[index + 1].start() if index + 1 < len(arms) else len(match_body)
                arm_expression = match_body[arm.end() : next_start]
                template = extract_endpoint_template(arm_expression, constants or {})
                if not template:
                    continue
                arm_variants = re.findall(
                    rf"(?:{re.escape(enum_name)}|Self)::([A-Za-z_][A-Za-z0-9_]*)",
                    arm.group(1),
                )
                for variant in arm_variants:
                    if variants and variant not in variants:
                        continue
                    endpoints[f"{enum_name}::{variant}"] = template
            if endpoints:
                return endpoints
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

    calls.extend(extract_to_request_endpoint_calls(text, resolver, enum_aliases))
    return tuple(calls)


def extract_to_request_endpoint_calls(
    text: str,
    resolver: EndpointResolver,
    enum_aliases: dict[str, str],
) -> list[RustEndpointCall]:
    """提取 `.to_request()` / `.to_request_with_url(...)` 端点调用（docs 主构造路径）。"""
    calls: list[RustEndpointCall] = []
    for match in _TO_REQUEST_CALL_RE.finditer(text):
        prefix = text[max(0, match.start() - 40) : match.start()]
        if re.search(r"\bfn\s+to_request(?:_with_url)?\s*$", prefix.rstrip()):
            continue
        open_paren = text.find("(", match.end() - 1)
        close_paren = find_matching_paren(text, open_paren)
        if close_paren < 0:
            continue
        receiver = extract_to_request_receiver(text, match.start())
        if not receiver:
            continue
        call_span = text[match.start() : close_paren + 1]
        full_expr = f"{receiver}{call_span}"
        resolved_argument = expand_endpoint_alias(receiver, enum_aliases, text, resolver.enum_endpoints)
        if resolved_argument != receiver and ".to_request" not in resolved_argument:
            resolved_for_path = f"{resolved_argument}.to_request()"
        else:
            resolved_for_path = (
                expand_endpoint_alias(full_expr, enum_aliases, text, resolver.enum_endpoints)
                if resolved_argument == receiver
                else f"{resolved_argument}.to_request()"
            )

        resolved_path, source_or_reason = resolver.resolve(resolved_for_path)
        if not resolved_path:
            resolved_path, source_or_reason = resolver.resolve(
                expand_endpoint_alias(full_expr, enum_aliases, text, resolver.enum_endpoints)
            )
        method = resolver.resolve_method(
            resolved_for_path if resolved_path else full_expr,
            fallback="",
        )
        if not method and resolved_path:
            method = resolver.resolve_method(resolved_argument, fallback="")

        line = line_of(text, match.start())
        if resolved_path and method:
            calls.append(
                RustEndpointCall(
                    method=method,
                    argument=full_expr if len(full_expr) < 200 else receiver,
                    line=line,
                    resolved_path=resolved_path,
                    source=source_or_reason if source_or_reason.startswith("to_request") else f"to_request:{source_or_reason}",
                )
            )
        elif resolved_path and not method:
            calls.append(
                RustEndpointCall(
                    method="",
                    argument=receiver,
                    line=line,
                    unresolved_reason=f"resolved path but missing enum method(): {resolved_path}",
                )
            )
        else:
            calls.append(
                RustEndpointCall(
                    method=method or "",
                    argument=receiver,
                    line=line,
                    unresolved_reason=source_or_reason or "to_request endpoint could not be resolved",
                )
            )
    return calls


def extract_to_request_receiver(text: str, dot_index: int) -> str:
    """从 `.to_request` 的 '.' 位置向前提取 receiver 表达式。"""
    whitespace = set(" " + "\t" + "\r" + "\n")
    # decode escapes
    whitespace = {" ", chr(9), chr(10), chr(13)}
    i = dot_index - 1
    while i >= 0 and text[i] in whitespace:
        i -= 1
    if i < 0:
        return ""

    if text[i] == ")":
        depth = 0
        j = i
        while j >= 0:
            ch = text[j]
            if ch == ")":
                depth += 1
            elif ch == "(":
                depth -= 1
                if depth == 0:
                    k = j - 1
                    while k >= 0 and text[k] in whitespace:
                        k -= 1
                    while k >= 0 and (text[k].isalnum() or text[k] in "_:"):
                        k -= 1
                    return text[k + 1 : i + 1].strip()
            j -= 1
        return ""

    if text[i].isalnum() or text[i] == "_":
        k = i
        while k >= 0 and (text[k].isalnum() or text[k] in "_:"):
            k -= 1
        return text[k + 1 : i + 1].strip()
    return ""


def extract_rust_fields(text: str) -> tuple[RustField, ...]:
    return (
        extract_rust_struct_fields(text, REQUEST_STRUCT_SUFFIXES)
        + extract_file_content_fields(text)
    )


def extract_access_token_types(text: str) -> tuple[str, ...]:
    """解析 ``.with_supported_access_token_types(vec![...])`` 声明的 token 类型。

    未找到调用、或调用内无已知变体时，回落到 ``DEFAULT_ACCESS_TOKEN_TYPES``
    （与 ``ApiRequest`` 运行时默认一致）。变体翻译见 ``_ACCESS_TOKEN_VARIANT_TO_FEISHU``。
    """
    match = re.search(
        r"\.with_supported_access_token_types\s*\(\s*vec!\s*\[([^\]]*)\]",
        text,
        re.DOTALL,
    )
    if not match:
        return DEFAULT_ACCESS_TOKEN_TYPES
    variants = re.findall(r"AccessTokenType::([A-Za-z]+)", match.group(1))
    types = tuple(
        _ACCESS_TOKEN_VARIANT_TO_FEISHU[variant]
        for variant in variants
        if variant in _ACCESS_TOKEN_VARIANT_TO_FEISHU
    )
    return types or DEFAULT_ACCESS_TOKEN_TYPES


# 手动注入 ``Authorization: Bearer <self.field>`` 的 struct 字段 → 飞书凭证名。
# 声明 ``AccessTokenType::None`` 的端点（如 OIDC userinfo）自行管理鉴权：从 struct
# 字段取 token 并手设 Authorization header，bypass token cache。
_MANUAL_TOKEN_FIELD_TO_FEISHU: dict[str, str] = {
    "user_access_token": "user_access_token",
    "tenant_access_token": "tenant_access_token",
    "app_access_token": "app_access_token",
}


def extract_manual_auth_token(text: str) -> str:
    """检测 request 是否手动注入 ``Authorization: Bearer <self.token_field>``。

    用于声明 ``AccessTokenType::None``（bypass token cache）的端点：它们的实际 token
    由 struct 字段提供并手设 header，validator 据此把 ``none_access_token`` 替换为该
    实际类型核对，避免误报 disjoint ERROR（#515 的 auth/user_info 误报根因）。

    匹配 ``format!("Bearer ...", self.<field>)`` 形态；未检测到返回空串。
    """
    for field, feishu_name in _MANUAL_TOKEN_FIELD_TO_FEISHU.items():
        if re.search(
            r'format!\(\s*"Bearer\b[^"]*"\s*,\s*self\.' + field + r"\b",
            text,
        ):
            return feishu_name
    return ""


def has_flatten_value_passthrough(text: str) -> bool:
    """检测 request struct 是否有 ``#[serde(flatten)]`` 字段。

    flatten 字段把额外的官方 request 字段整体合并到请求体（例如 docx block 的
    ``update_*`` 操作，无论透传 ``serde_json::Value`` 还是 typed
    ``BlockUpdateOperation`` 枚举），扫描器无法逐字段对比这类 API 的 optional 字段，
    应在 contract 比较时跳过其 optional 字段缺失告警。
    """
    pattern = re.compile(r"#\s*\[\s*serde\s*\(\s*flatten\s*\)\s*\]")
    return bool(pattern.search(text))


def extract_rust_response_fields(text: str) -> tuple[RustField, ...]:
    return extract_rust_struct_fields(text, RESPONSE_STRUCT_SUFFIXES)


def extract_file_content_fields(text: str) -> tuple[RustField, ...]:
    """提取 multipart/form-data 接口的 request 字段。

    multipart 接口的请求体字段组织方式与普通 JSON 接口不同：
    - 文件二进制内容通过 ``.file_content(...)`` 传入（序列化为 ``file``）
    - 其余表单字段（``file_name``/``parent_node``/``size``/``checksum``/``name`` 等）通过
      局部 ``UploadMeta``/``PartMeta`` 结构体（``.json_body(&meta)``）或
      ``serde_json::json!({...})`` 字面量组织

    这三类都会被本函数识别，以消除 multipart 接口下大量 ``E_REQUIRED_REQUEST_FIELD_MISSING``
    与 ``W_OPTIONAL_REQUEST_FIELD_MISSING`` 的扫描器误报。
    """
    # 仅当文件使用了 .file_content(...) 才按 multipart 处理
    if ".file_content(" not in text:
        return ()

    fields: list[RustField] = []

    # 1) file 二进制字段（兼容旧逻辑：.file_content(body|self).field）
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
    # .file_content(self.file) / .file_content(file) 形式（无显式字段名）
    if not fields:
        for match in re.finditer(r"\.file_content\(\s*(?:self\.)?([A-Za-z_][A-Za-z0-9_]*)\s*\)", text):
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

    # 2) 局部 UploadMeta / PartMeta 结构体的字段（.json_body(&meta) 组织 multipart 表单）
    fields.extend(_extract_multipart_meta_fields(text))

    # 3) serde_json::json!({...}) 字面量中的字符串键（baike 风格 multipart）
    fields.extend(_extract_json_literal_fields(text))

    return tuple(fields)


def _extract_multipart_meta_fields(text: str) -> list[RustField]:
    """提取文件内局部定义的 multipart meta 结构体（UploadMeta / PartMeta 等）字段。

    这些结构体不以 Request 后缀（Body/Query/Params/RequestBody）结尾，
    常规 ``extract_rust_struct_fields`` 识别不到，需要单独扫描。
    局部结构体的字段通常没有 ``pub`` 前缀（私有），这里放宽匹配。
    """
    fields: list[RustField] = []
    meta_suffixes = ("Meta",)
    for match in re.finditer(r"struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{", text):
        struct_name = match.group(1)
        if not struct_name.endswith(meta_suffixes):
            continue
        open_brace = text.find("{", match.end() - 1)
        close_brace = find_matching_brace(text, open_brace)
        if close_brace < 0:
            continue
        body = text[open_brace + 1 : close_brace]
        base_line = line_of(text, open_brace + 1)
        fields.extend(_extract_meta_struct_fields(struct_name, body, base_line))
    return fields


def _extract_meta_struct_fields(struct_name: str, body: str, base_line: int) -> list[RustField]:
    """解析局部 meta 结构体字段（兼容 ``pub`` 与无 ``pub`` 的私有字段）。"""
    fields: list[RustField] = []
    pending_attrs: list[str] = []
    for offset, line in enumerate(body.splitlines()):
        stripped = line.strip()
        if not stripped:
            continue
        if stripped.startswith("#["):
            pending_attrs.append(stripped)
            continue
        # 兼容 "pub field: T," 与 "field: T," 两种写法
        match = re.match(r"(?:pub\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,]+),", stripped)
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
        serialized_name = rename_match.group(1) if rename_match else field_name
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


def _extract_json_literal_fields(text: str) -> list[RustField]:
    """提取 ``serde_json::json!({...})`` 字面量里的字符串键（multipart 表单字段）。

    典型场景：baike 文件上传用 ``serde_json::json!({"name": ..., "__file_name": ...})``
    组织 multipart 表单字段。这里只收集顶层字符串键作为 request 字段名，
    下划线前缀的内部字段（如 ``__file_name``）跳过。
    """
    fields: list[RustField] = []
    for match in re.finditer(r"json!\s*\(\s*\{", text):
        open_brace = text.find("{", match.end() - 1)
        close_brace = find_matching_brace(text, open_brace)
        if close_brace < 0:
            continue
        body = text[open_brace + 1 : close_brace]
        base_line = line_of(text, open_brace + 1)
        for offset, line in enumerate(body.splitlines()):
            key_match = re.search(r'"([A-Za-z_][A-Za-z0-9_]*)"\s*:', line)
            if not key_match:
                continue
            key = key_match.group(1)
            if key.startswith("__"):
                continue
            fields.append(
                RustField(
                    struct_name="JsonLiteral",
                    field_name=key,
                    serialized_name=key,
                    type_name="String",
                    optional=False,
                    line=base_line + offset,
                )
            )
    return fields


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
    enum_methods: dict[str, str] | None = None,
) -> RustApiContract | None:
    path = crate_src / expected_file
    if not path.exists():
        return None
    text = path.read_text(encoding="utf-8")
    resolved_constants = constants if constants is not None else load_endpoint_constants(crate_src)
    resolved_enums = (
        enum_endpoints if enum_endpoints is not None else load_enum_endpoints(crate_src, resolved_constants)
    )
    resolved_methods = enum_methods if enum_methods is not None else load_enum_methods(crate_src)
    resolver = EndpointResolver(
        resolved_constants,
        resolved_enums,
        resolved_methods,
    )
    access_token_types = extract_access_token_types(text)
    return RustApiContract(
        rel_path=expected_file,
        endpoint_calls=extract_endpoint_calls(text, resolver),
        fields=extract_rust_fields(text),
        response_fields=extract_rust_response_fields(text),
        has_flatten_value_passthrough=has_flatten_value_passthrough(text),
        access_token_types=access_token_types,
        manual_auth_token=extract_manual_auth_token(text),
    )
