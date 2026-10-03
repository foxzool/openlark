"""CatalogEndpoint / enum endpoint resolution for contract validation.

从 `rust_source` 拆出 docs 域 CatalogEndpoint、`.to_request()` 与 enum→path/method
解析逻辑（#568），避免 `rust_source.py` 越过 1k 行维护阈值。
"""

from __future__ import annotations

import re
from pathlib import Path
from typing import Callable, Iterator

from .models import RustEndpointCall

_TO_REQUEST_CALL_RE = re.compile(
    r"\.to_request(?:_with_url)?(?:\s*::\s*<[^()]+>)?\s*\(",
)

_ENUM_VARIANT_RE = re.compile(
    r"([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)\b(?!::)",
)

_HTTP_METHOD_RE = re.compile(r"HttpMethod::(Get|Post|Put|Patch|Delete)")


# ---------------------------------------------------------------------------
# 共享：match self 臂遍历 + Enum::Variant 表达式拆解
# ---------------------------------------------------------------------------


def _find_matching_paren(text: str, open_paren_idx: int) -> int:
    from .rust_source import find_matching_paren

    return find_matching_paren(text, open_paren_idx)


def _find_matching_brace(text: str, open_brace_idx: int) -> int:
    from .rust_source import find_matching_brace

    return find_matching_brace(text, open_brace_idx)


def _extract_endpoint_template(expression: str, constants: dict[str, str] | None = None) -> str:
    from .rust_source import extract_endpoint_template

    return extract_endpoint_template(expression, constants)


def _expand_endpoint_alias(
    argument: str,
    enum_aliases: dict[str, str],
    file_text: str = "",
    enum_endpoints: dict[str, str] | None = None,
) -> str:
    from .rust_source import expand_endpoint_alias

    return expand_endpoint_alias(argument, enum_aliases, file_text, enum_endpoints)


def _line_of(text: str, index: int) -> int:
    from .rust_source import line_of

    return line_of(text, index)


def split_enum_variant_and_suffix(expression: str) -> tuple[str, str, str] | None:
    """拆解 ``Enum::Variant(...).<suffix>``。

    返回 ``(enum_name, variant, rest)``；``rest`` 为可选参数之后的后缀（如
    ``.to_url()`` / ``.to_request()``），无匹配时返回 ``None``。
    """
    expr = expression.strip()
    match = _ENUM_VARIANT_RE.search(expr)
    if not match:
        return None
    enum_name = match.group(1)
    variant = match.group(2)
    pos = match.end()
    rest = expr[pos:].strip()
    if rest.startswith("("):
        paren_open = expr.find("(", pos)
        if paren_open >= 0:
            paren_close = _find_matching_paren(expr, paren_open)
            if paren_close >= 0:
                pos = paren_close + 1
                rest = expr[pos:].strip()
    return enum_name, variant, rest


def iter_match_self_arms(fn_body: str, enum_name: str) -> Iterator[tuple[list[str], str]]:
    """遍历 ``match self { ... }`` 各臂，产出 ``(variants, rhs_text)``。

    供 ``method()`` 与 ``to_url()``/``path()`` 解析共用，避免重复实现 match 臂提取。
    """
    match_pos = fn_body.find("match self")
    if match_pos < 0:
        return
    match_open = fn_body.find("{", match_pos)
    match_close = _find_matching_brace(fn_body, match_open)
    if match_close < 0:
        return
    match_body = fn_body[match_open + 1 : match_close]

    enum_re = re.escape(enum_name)
    # 兼容 Self::V / Enum::V 以及 ``A | B =>`` 多 variant 臂
    arm_pattern = re.compile(
        rf"(?P<head>(?:\s*\|?\s*(?:Self|{enum_re})::[A-Za-z_][A-Za-z0-9_]*"
        rf"(?:\s*\([^=>]*?\))?)+)\s*=>",
        re.MULTILINE | re.DOTALL,
    )
    heads = list(arm_pattern.finditer(match_body))
    for index, arm in enumerate(heads):
        next_start = heads[index + 1].start() if index + 1 < len(heads) else len(match_body)
        rhs = match_body[arm.end() : next_start]
        variants = re.findall(
            rf"(?:Self|{enum_re})::([A-Za-z_][A-Za-z0-9_]*)",
            arm.group("head"),
        )
        yield variants, rhs


def _iter_impl_fn_bodies(
    text: str,
    enum_name: str,
    fn_patterns: tuple[str, ...],
    impl_patterns: tuple[str, ...] | None = None,
) -> Iterator[str]:
    """在 inherent / CatalogEndpoint impl 中查找目标 fn 并产出函数体文本。"""
    patterns = impl_patterns or (
        rf"impl\s+CatalogEndpoint\s+for\s+{re.escape(enum_name)}\s*\{{",
        rf"impl\s+{re.escape(enum_name)}\s*\{{",
    )
    for impl_pattern in patterns:
        for impl_match in re.finditer(impl_pattern, text):
            impl_open = text.find("{", impl_match.end() - 1)
            impl_close = _find_matching_brace(text, impl_open)
            if impl_close < 0:
                continue
            impl_body = text[impl_open + 1 : impl_close]
            for fn_pattern in fn_patterns:
                fn_match = re.search(fn_pattern, impl_body)
                if not fn_match:
                    continue
                fn_open = impl_body.find("{", fn_match.end() - 1)
                fn_close = _find_matching_brace(impl_body, fn_open)
                if fn_close < 0:
                    continue
                yield impl_body[fn_open + 1 : fn_close]


# ---------------------------------------------------------------------------
# Enum key / path 表达式解析
# ---------------------------------------------------------------------------


def resolve_enum_key(expression: str) -> str:
    """从表达式中提取 EnumName::Variant 键（忽略参数与后缀方法调用）。"""
    match = _ENUM_VARIANT_RE.search(expression.strip())
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

    parts = split_enum_variant_and_suffix(expr)
    if not parts:
        return ""
    enum_name, variant, rest = parts
    if not _TO_REQUEST_CALL_RE.match(rest):
        return ""
    return enum_endpoints.get(f"{enum_name}::{variant}", "")


def resolve_enum_to_url_expression(expression: str, enum_endpoints: dict[str, str]) -> str:
    """解析 Enum::Variant(...).to_url() / .path() 的路径。"""
    parts = split_enum_variant_and_suffix(expression)
    if not parts:
        return ""
    enum_name, variant, rest = parts
    if rest and not (rest.startswith(".to_url()") or rest.startswith(".path()")):
        return ""
    return enum_endpoints.get(f"{enum_name}::{variant}", "")


# ---------------------------------------------------------------------------
# 端点定义文件扫描与 type alias
# ---------------------------------------------------------------------------


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


def _load_local_constants(text: str, base: dict[str, str]) -> dict[str, str]:
    local = dict(base)
    for match in re.finditer(r'pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]+)"\s*;', text):
        local[match.group(1)] = match.group(2)
    for match in re.finditer(r"pub\s+const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*([A-Z0-9_]+)\s*;", text):
        if match.group(2) in local:
            local[match.group(1)] = local[match.group(2)]
    return local


def load_enum_endpoints(crate_src: Path, constants: dict[str, str] | None = None) -> dict[str, str]:
    enum_endpoints: dict[str, str] = {}
    all_constants = constants or {}
    type_aliases: dict[str, str] = {}

    for path in iter_api_endpoint_definition_files(crate_src):
        text = path.read_text(encoding="utf-8")
        enum_variants = parse_enum_variants(text)
        local_constants = _load_local_constants(text, all_constants)
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
    fn_pattern = r"(?:pub\s+)?fn\s+method\s*\([^)]*\)\s*->\s*HttpMethod\s*\{"
    for fn_body in _iter_impl_fn_bodies(text, enum_name, (fn_pattern,)):
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

    methods: dict[str, str] = {}
    for arm_variants, rhs in iter_match_self_arms(fn_body, enum_name):
        method_match = _HTTP_METHOD_RE.search(rhs)
        if not method_match:
            continue
        method = method_match.group(1).upper()
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
        close_brace = _find_matching_brace(text, open_brace)
        if close_brace < 0:
            continue
        body = text[open_brace + 1 : close_brace]
        variants = set(re.findall(r"^\s*([A-Za-z_][A-Za-z0-9_]*)(?:\s*\(|\s*,)", body, re.MULTILINE))
        variants_by_enum[enum_name] = variants
    return variants_by_enum


def parse_enum_to_url_endpoints(
    text: str,
    enum_name: str,
    variants: set[str],
    constants: dict[str, str] | None = None,
) -> dict[str, str]:
    """从 inherent `impl Enum` 或 `impl CatalogEndpoint for Enum` 的 to_url/path 解析路径。

    docs 的 MinutesExtraApiV1 等仅在 CatalogEndpoint impl 中实现 to_url（#568）。
    """
    impl_patterns = (
        rf"impl\s+{re.escape(enum_name)}\s*\{{",
        rf"impl\s+CatalogEndpoint\s+for\s+{re.escape(enum_name)}\s*\{{",
    )
    fn_pattern = (
        r"(?:pub(?:\s*\([^)]*\))?\s+)?fn\s+(to_url|path)\s*\([^)]*\)\s*"
        r"->\s*(?:String|&'static\s+str)\s*\{"
    )
    endpoints: dict[str, str] = {}
    for fn_body in _iter_impl_fn_bodies(text, enum_name, (fn_pattern,), impl_patterns):
        if re.search(rf"{re.escape(enum_name)}::(to_url|path)\s*\(", fn_body) and "match self" not in fn_body:
            continue

        if "match self" not in fn_body:
            template = _extract_endpoint_template(fn_body, constants or {})
            if template:
                for variant in variants:
                    endpoints[f"{enum_name}::{variant}"] = template
                return endpoints
            continue

        for arm_variants, rhs in iter_match_self_arms(fn_body, enum_name):
            template = _extract_endpoint_template(rhs, constants or {})
            if not template:
                continue
            for variant in arm_variants:
                if variants and variant not in variants:
                    continue
                endpoints[f"{enum_name}::{variant}"] = template
        if endpoints:
            return endpoints
    return endpoints


# ---------------------------------------------------------------------------
# .to_request() 调用点提取
# ---------------------------------------------------------------------------


def _to_request_resolve_candidates(
    receiver: str,
    full_expr: str,
    enum_aliases: dict[str, str],
    file_text: str,
    enum_endpoints: dict[str, str],
) -> list[str]:
    """单一 alias 展开管道：按优先级产出待 resolve 的表达式候选。"""
    expanded = _expand_endpoint_alias(receiver, enum_aliases, file_text, enum_endpoints)
    candidates: list[str] = []
    if expanded != receiver:
        if ".to_request" in expanded:
            candidates.append(expanded)
        else:
            candidates.append(f"{expanded}.to_request()")
    expanded_full = _expand_endpoint_alias(full_expr, enum_aliases, file_text, enum_endpoints)
    for expr in (expanded_full, full_expr):
        if expr not in candidates:
            candidates.append(expr)
    return candidates


def extract_to_request_endpoint_calls(
    text: str,
    resolver: object,
    enum_aliases: dict[str, str],
) -> list[RustEndpointCall]:
    """提取 `.to_request()` / `.to_request_with_url(...)` 端点调用（docs 主构造路径）。"""
    # duck-typed: EndpointResolver.resolve / resolve_method / enum_endpoints
    resolve: Callable[[str], tuple[str, str]] = resolver.resolve  # type: ignore[attr-defined]
    resolve_method: Callable[..., str] = resolver.resolve_method  # type: ignore[attr-defined]
    enum_endpoints: dict[str, str] = resolver.enum_endpoints  # type: ignore[attr-defined]

    calls: list[RustEndpointCall] = []
    for match in _TO_REQUEST_CALL_RE.finditer(text):
        prefix = text[max(0, match.start() - 40) : match.start()]
        if re.search(r"\bfn\s+to_request(?:_with_url)?\s*$", prefix.rstrip()):
            continue
        open_paren = text.find("(", match.end() - 1)
        close_paren = _find_matching_paren(text, open_paren)
        if close_paren < 0:
            continue
        receiver = extract_to_request_receiver(text, match.start())
        if not receiver:
            continue
        call_span = text[match.start() : close_paren + 1]
        full_expr = f"{receiver}{call_span}"

        candidates = _to_request_resolve_candidates(
            receiver, full_expr, enum_aliases, text, enum_endpoints
        )
        resolved_path = ""
        source_or_reason = ""
        resolved_expr = full_expr
        for candidate in candidates:
            path, reason = resolve(candidate)
            if path:
                resolved_path, source_or_reason, resolved_expr = path, reason, candidate
                break
            source_or_reason = reason

        method = ""
        for method_expr in (resolved_expr, receiver, full_expr, *candidates):
            method = resolve_method(method_expr, fallback="")
            if method:
                break

        line = _line_of(text, match.start())
        if resolved_path and method:
            source = (
                source_or_reason
                if source_or_reason.startswith("to_request")
                else f"to_request:{source_or_reason}"
            )
            calls.append(
                RustEndpointCall(
                    method=method,
                    argument=full_expr if len(full_expr) < 200 else receiver,
                    line=line,
                    resolved_path=resolved_path,
                    source=source,
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
    whitespace = {" ", "\t", "\n", "\r"}
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
