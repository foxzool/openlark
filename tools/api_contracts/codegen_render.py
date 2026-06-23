"""风格 A 渲染器：ApiIR → 可编译的 Rust 文件。

纯字符串模板（零依赖，与现有 tools 脚本一致）。对标
crates/openlark-communication/src/im/im/v1/message/{create,get,reply}.rs 的真实写法。

5 条核心契约逐条保证：
1. Config owned（render_request_struct 写 config: Config，无 Arc）
2. R=serde_json::Value + extract_response_data（绝不生成 XxxResponse{data:..} 包装）
3. 固定 Transport::request(req, &self.config, Some(option))（过 E003）
4. Token：MVP 留 // TODO(G5) 注释（issue41 不检 token）
5. 端点常量 + validate_required! + execute 委托 execute_with_options（过 E001/E002/W001）
"""

from __future__ import annotations

from .codegen_ir import (
    ApiIR,
    FieldDef,
    StructDef,
    TypeArray,
    TypeEnumRef,
    TypeMap,
    TypeOpaque,
    TypePrimitive,
    TypeStructRef,
)

CODEGEN_MARKER = "由 codegen 自动生成"


def render_api_file(ir: ApiIR) -> str:
    """渲染整个 .rs 文件。"""
    parts: list[str] = [
        _module_doc(ir),
        _imports(ir),
    ]
    # 嵌套 struct：渲染 body + response 可达的
    reachable = _reachable_struct_keys(ir)
    for struct in sorted(
        (
            s
            for s in ir.structs.values()
            if s.origin.startswith("nested:") and s.key in reachable
        ),
        key=lambda s: s.rust_name,
    ):
        parts.append(_render_struct(ir, struct))
    if ir.body_struct is not None:
        parts.append(_render_struct(ir, ir.body_struct))
    if ir.response_struct is not None:
        parts.append(_render_struct(ir, ir.response_struct, force_optional=True))
        parts.append(_render_api_response_trait(ir))
    parts.append(_request_struct(ir))
    parts.append(_request_impl(ir))
    parts.append(_tests(ir))
    return "\n\n".join(p for p in parts if p) + "\n"


def render_endpoint_const_snippet(ir: ApiIR) -> str:
    """要追加到 crate endpoints/<biz>.rs 的常量片段。"""
    return f'pub const {ir.endpoint_const_name}: &str = "{ir.endpoint_const_value}";'


# --------------------------------------------------------------------------- #
# 各段
# --------------------------------------------------------------------------- #


def _module_doc(ir: ApiIR) -> str:
    return (
        f"//! {ir.name}\n"
        f"//!\n"
        f"//! docPath: {ir.doc_path}\n"
        f"//!\n"
        f"//! {CODEGEN_MARKER}（风格 A）。手工修改将在下次生成时覆盖。"
    )


def _imports(ir: ApiIR) -> str:
    has_body = ir.body_struct is not None
    is_typed = ir.response_struct is not None
    lines = ["use openlark_core::{"]
    if is_typed:
        core_items = [
            "SDKResult",
            "api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat}",
            "config::Config",
            "http::Transport",
        ]
    else:
        core_items = ["SDKResult", "api::ApiRequest", "config::Config", "http::Transport"]
    if _needs_validate_required(ir):
        core_items.append("validate_required")
    if _emit_token_decl(ir.supported_access_tokens) is not None:
        core_items.append("constants::AccessTokenType")
    lines.append("    " + ", ".join(core_items) + ",")
    lines.append("};")
    if has_body or is_typed:
        lines.append("use serde::{Deserialize, Serialize};")
    lines.append("")
    lines.append("use crate::{")
    utils = []
    if not is_typed:
        utils.append("extract_response_data")
    if has_body:
        utils.append("serialize_params")
    if utils:
        lines.append("    common::api_utils::{" + ", ".join(utils) + "},")
    lines.append(f"    endpoints::{ir.endpoint_const_name},")
    lines.append("};")
    return "\n".join(lines)


def _render_struct(ir: ApiIR, struct: StructDef, *, force_optional: bool = False) -> str:
    lines: list[str] = []
    if struct.origin == "request_body":
        lines.append(f"/// {ir.name}请求体。")
    elif struct.origin == "response_data":
        lines.append(f"/// {ir.name}响应数据。")
    elif struct.origin.startswith("nested:"):
        lines.append(f"/// {struct.rust_name}（嵌套结构）。")
    lines.append("#[derive(Debug, Clone, Serialize, Deserialize)]")
    lines.append('#[serde(rename_all = "camelCase")]')
    lines.append(f"pub struct {struct.rust_name} {{")
    for field in struct.fields:
        lines.extend(_field_lines(field, force_optional=force_optional))
    lines.append("}")
    return "\n".join(lines)


def _field_lines(field: FieldDef, *, force_optional: bool = False) -> list[str]:
    optional = (not field.required) or force_optional
    out: list[str] = []
    if field.description:
        out.append(f"    /// {_oneliner(field.description)}")
    if optional:
        out.append('    #[serde(skip_serializing_if = "Option::is_none")]')
    rust_t = _rust_type(field.type_expr, field.required and not force_optional)
    out.append(f"    pub {field.rust_name}: {rust_t},")
    return out


def _rust_type(t: object, required: bool) -> str:
    if isinstance(t, TypePrimitive):
        base = t.rust
    elif isinstance(t, TypeArray):
        base = f"Vec<{_rust_type(t.item, True)}>"
    elif isinstance(t, TypeMap):
        base = f"std::collections::HashMap<String, {_rust_type(t.value, True)}>"
    elif isinstance(t, (TypeStructRef, TypeEnumRef)):
        base = t.rust_name if hasattr(t, "rust_name") else (
            t.struct_key if isinstance(t, TypeStructRef) else t.enum_key
        )
    elif isinstance(t, TypeOpaque):
        base = "serde_json::Value"
    else:
        base = "serde_json::Value"
    return base if required else f"Option<{base}>"


def _request_struct(ir: ApiIR) -> str:
    lines = [f"/// {ir.name}请求。", f"pub struct {ir.request_struct_name} {{", "    config: Config,"]
    for p in ir.path_params:
        lines.append(f"    {p.rust_name}: String,")
    for p in ir.query_params:
        lines.append(f"    {p.rust_name}: Option<String>,")
    lines.append("}")
    return "\n".join(lines)


def _request_impl(ir: ApiIR) -> str:
    name = ir.request_struct_name
    has_body = ir.body_struct is not None
    body_type = ir.body_struct.rust_name if has_body else None
    method = ir.method.lower()
    resp_type = _response_type(ir)
    is_typed = ir.response_struct is not None

    lines: list[str] = [f"impl {name} {{"]

    # new
    lines.append("    /// 创建请求构建器。")
    lines.append("    pub fn new(config: Config) -> Self {")
    lines.append("        Self {")
    lines.append("            config,")
    for p in ir.path_params:
        lines.append(f"            {p.rust_name}: String::new(),")
    for p in ir.query_params:
        lines.append(f"            {p.rust_name}: None,")
    lines.append("        }")
    lines.append("    }")

    # setters
    for p in ir.path_params:
        lines.append(_setter_doc(p))
        lines.append(f"    pub fn {p.rust_name}(mut self, v: impl Into<String>) -> Self {{")
        lines.append(f"        self.{p.rust_name} = v.into();")
        lines.append("        self")
        lines.append("    }")
    for p in ir.query_params:
        lines.append(_setter_doc(p))
        lines.append(f"    pub fn {p.rust_name}(mut self, v: impl Into<String>) -> Self {{")
        lines.append(f"        self.{p.rust_name} = Some(v.into());")
        lines.append("        self")
        lines.append("    }")

    # execute（委托）
    exec_sig = f"(self, body: {body_type})" if has_body else "(self)"
    lines.append("    /// 执行请求。")
    lines.append(f"    pub async fn execute{exec_sig} -> SDKResult<{resp_type}> {{")
    if has_body:
        lines.append(
            "        self.execute_with_options(body, "
            "openlark_core::req_option::RequestOption::default())"
        )
        lines.append("            .await")
    else:
        lines.append(
            "        self.execute_with_options("
            "openlark_core::req_option::RequestOption::default()).await"
        )
    lines.append("    }")

    # execute_with_options
    params_sig = ["self"]
    if has_body:
        params_sig.append(f"body: {body_type}")
    params_sig.append("option: openlark_core::req_option::RequestOption")
    lines.append("    /// 使用指定请求选项执行请求。")
    lines.append(f"    pub async fn execute_with_options(")
    for i, p in enumerate(params_sig):
        sep = "," if i < len(params_sig) - 1 else ","
        lines.append(f"        {p}{sep}")
    lines.append(f"    ) -> SDKResult<{resp_type}> {{")
    lines.append("        // === 必填字段验证 ===")
    # body required String 字段
    if has_body:
        for f in ir.body_struct.fields:
            if f.required and isinstance(f.type_expr, TypePrimitive) and f.type_expr.rust == "String":
                lines.append(
                    f'        validate_required!(body.{f.rust_name}, "{f.rust_name} 不能为空");'
                )
    # path param（String，validate_required! 直接可用）
    for p in ir.path_params:
        lines.append(f'        validate_required!(self.{p.rust_name}, "{p.rust_name} 不能为空");')
    # required query param（Option<String>，用 ok_or_else，对齐 create.rs）
    for p in ir.query_params:
        if p.required:
            lines.append(f"        let {p.rust_name} = self.{p.rust_name}.ok_or_else(|| {{")
            lines.append("            openlark_core::error::validation_error(")
            lines.append(f'                "{p.rust_name} 不能为空".to_string(),')
            lines.append(f'                "{ir.name}需要指定 {p.rust_name}".to_string(),')
            lines.append("            )")
            lines.append("        })?;")

    # url
    lines.append(f"        // url: {ir.method}:{ir.endpoint_path}")
    url_expr = _endpoint_url_expr(ir)
    lines.append(f"        let req: ApiRequest<{resp_type}> = ApiRequest::{method}({url_expr})")
    # body（无分号，链式续）
    if has_body:
        lines.append(f'            .body(serialize_params(&body, "{ir.name}")?)')
    # query params（required 用 query(key, String)；optional 用 query_opt）
    for p in ir.query_params:
        if p.required:
            lines.append(f'            .query("{p.name}", {p.rust_name})')
        else:
            lines.append(f'            .query_opt("{p.name}", self.{p.rust_name})')
    # G5: token 类型声明（非默认 {User, Tenant} 组合才生成，否则保持 core 默认）
    token_decl = _emit_token_decl(ir.supported_access_tokens)
    if token_decl is not None:
        lines.append(f"            .with_supported_access_token_types({token_decl})")
    # 统一收口：链尾补分号
    lines[-1] = lines[-1] + ";"
    if is_typed:
        lines.append(
            f"        let response: Response<{resp_type}> = "
            "Transport::request(req, &self.config, Some(option)).await?;"
        )
        lines.append("        response.data.ok_or_else(|| {")
        lines.append(
            '            openlark_core::error::validation_error("response", "响应数据为空")'
        )
        lines.append("        })")
    else:
        lines.append("        let resp = Transport::request(req, &self.config, Some(option)).await?;")
        lines.append(f'        extract_response_data(resp, "{ir.name}")')
    lines.append("    }")
    lines.append("}")
    return "\n".join(lines)


def _endpoint_url_expr(ir: ApiIR) -> str:
    """构造 ApiRequest::<method>(URL_EXPR) 的 URL_EXPR。"""
    if not ir.needs_format:
        return ir.endpoint_const_name
    args = ", ".join([ir.endpoint_const_name] + [f"self.{p.rust_name}" for p in ir.path_params])
    return f'format!("{ir.endpoint_format_template}", {args})'


def _tests(ir: ApiIR) -> str:
    """最小冒烟测试：builder 可构造、Body 可构造。不调 execute（需网络）。"""
    name = ir.request_struct_name
    lines = [
        "#[cfg(test)]",
        "#[allow(unused_imports)]",
        "mod tests {",
        "    use super::*;",
        "",
        "    #[test]",
        "    fn test_request_builder_default() {",
        "        let config = Config::default();",
        f"        let _req = {name}::new(config);",
        "    }",
    ]
    if ir.body_struct is not None:
        body_name = ir.body_struct.rust_name
        field_lines = [
            f"            {f.rust_name}: {_default_value(f.type_expr, f.required)},"
            for f in ir.body_struct.fields
        ]
        body_init = "{\n" + "\n".join(field_lines) + "\n        }"
        lines += [
            "",
            "    #[test]",
            "    fn test_body_construct() {",
            f"        let _body = {body_name} {body_init};",
            "    }",
        ]
    lines.append("}")
    return "\n".join(lines)


def _default_value(t: object, required: bool) -> str:
    if not required:
        return "None"
    if isinstance(t, TypePrimitive):
        return {
            "String": '"".to_string()',
            "i64": "0",
            "i32": "0",
            "bool": "false",
            "f64": "0.0",
            "f32": "0.0",
        }.get(t.rust, "Default::default()")
    if isinstance(t, TypeArray):
        return "vec![]"
    if isinstance(t, TypeMap):
        return "std::collections::HashMap::new()"
    return "serde_json::Value::Null"


def _setter_doc(p) -> str:
    """setter 方法的 doc 注释行。"""
    if p.description:
        return f"    /// {_oneliner(p.description)}"
    loc = "路径参数" if p.location == "path" else "查询参数"
    return f"    /// {p.name}（{loc}）。"


def _oneliner(text: str, limit: int = 80) -> str:
    """description 取第一行非空文本，截断。"""
    for line in text.splitlines():
        s = line.strip()
        if s:
            return s[:limit] + ("…" if len(s) > limit else "")
    return ""


_TOKEN_MAP = {
    "user_access_token": "User",
    "tenant_access_token": "Tenant",
    "app_access_token": "App",
}


def _emit_token_decl(tokens: tuple[str, ...]) -> str | None:
    """supportedAccessToken 字符串 → "vec![...]" 或 None（默认/空时不生成）。

    集合 == {User, Tenant}（core getter 默认）→ None，省略显式声明（契约 4 由 core 兜底）。
    非默认 → 保留 dump 顺序输出。未知值跳过（不阻塞 codegen，演进友好）。
    绝不返回空 vec（core 会兜底成 [None]，导致不带 token）。
    """
    if not tokens:
        return None
    variants = [_TOKEN_MAP[t] for t in tokens if t in _TOKEN_MAP]
    if not variants:
        return None
    if set(variants) == {"User", "Tenant"}:
        return None
    return "vec![" + ", ".join(f"AccessTokenType::{v}" for v in variants) + "]"


def _response_type(ir: ApiIR) -> str:
    """execute 的返回类型：typed response struct 名，或 serde_json::Value（无 data 时回退）。"""
    return ir.response_struct.rust_name if ir.response_struct is not None else "serde_json::Value"


def _render_api_response_trait(ir: ApiIR) -> str:
    """渲染 impl ApiResponseTrait for XxxResp（契约 2：R 是 data 内容类型，非包装层）。"""
    name = ir.response_struct.rust_name
    return (
        f"impl ApiResponseTrait for {name} {{\n"
        f"    fn data_format() -> ResponseFormat {{\n"
        f"        ResponseFormat::Data\n"
        f"    }}\n"
        f"}}"
    )


def _needs_validate_required(ir: ApiIR) -> bool:
    """是否用到 validate_required!（path param 必用；body required String 字段用）。"""
    if ir.path_params:
        return True
    if ir.body_struct is not None:
        for f in ir.body_struct.fields:
            if (
                f.required
                and isinstance(f.type_expr, TypePrimitive)
                and f.type_expr.rust == "String"
            ):
                return True
    return False


def _collect_struct_refs(t: object):
    """从 TypeExpr 收集所有 TypeStructRef 的 struct_key（递归 array/map 内层）。"""
    if isinstance(t, TypeStructRef):
        yield t.struct_key
    elif isinstance(t, TypeArray):
        yield from _collect_struct_refs(t.item)
    elif isinstance(t, TypeMap):
        yield from _collect_struct_refs(t.value)


def _reachable_struct_keys(ir: ApiIR) -> set[str]:
    """从 body + response 出发可达的 struct key 集合（递归 array/map 内层 TypeStructRef）。"""
    visited: set[str] = set()
    roots = []
    if ir.body_struct is not None:
        roots.append(ir.body_struct.key)
    if ir.response_struct is not None:
        roots.append(ir.response_struct.key)
    stack = list(roots)
    while stack:
        key = stack.pop()
        if key in visited:
            continue
        visited.add(key)
        struct = ir.structs.get(key)
        if struct is None:
            continue
        for f in struct.fields:
            for ref_key in _collect_struct_refs(f.type_expr):
                if ref_key in ir.structs:
                    stack.append(ref_key)
    return visited

