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
    # 嵌套 struct：只渲染 body 可达的（MVP 不渲染 response struct 及其 nested，避免 dead code）
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
    lines = ["use openlark_core::{"]
    core_items = ["SDKResult", "api::ApiRequest", "config::Config", "http::Transport"]
    if _needs_validate_required(ir):
        core_items.append("validate_required")
    lines.append("    " + ", ".join(core_items) + ",")
    lines.append("};")
    if has_body:
        lines.append("use serde::{Deserialize, Serialize};")
    lines.append("")
    lines.append("use crate::{")
    utils = ["extract_response_data"]
    if has_body:
        utils.append("serialize_params")
    lines.append("    common::api_utils::{" + ", ".join(utils) + "},")
    lines.append(f"    endpoints::{ir.endpoint_const_name},")
    lines.append("};")
    return "\n".join(lines)


def _render_struct(ir: ApiIR, struct: StructDef) -> str:
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
        lines.extend(_field_lines(field))
    lines.append("}")
    return "\n".join(lines)


def _field_lines(field: FieldDef) -> list[str]:
    out: list[str] = []
    if field.description:
        out.append(f"    /// {_oneliner(field.description)}")
    if not field.required:
        out.append('    #[serde(skip_serializing_if = "Option::is_none")]')
    rust_t = _rust_type(field.type_expr, field.required)
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
    lines.append(f"    pub async fn execute{exec_sig} -> SDKResult<serde_json::Value> {{")
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
    lines.append("    ) -> SDKResult<serde_json::Value> {")
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
    lines.append(f"        let req: ApiRequest<serde_json::Value> = ApiRequest::{method}({url_expr})")
    # body
    if has_body:
        lines.append(f'            .body(serialize_params(&body, "{ir.name}")?);')
    else:
        # 无 body：链式终止，末尾分号。把上一行末尾补分号
        lines[-1] = lines[-1] + ";"
    # query params（required 用 query(key, String)；optional 用 query_opt）
    if ir.query_params:
        # 上一行去掉分号，改链式
        lines[-1] = lines[-1].rstrip(";")
        for p in ir.query_params:
            if p.required:
                lines.append(f'            .query("{p.name}", {p.rust_name})')
            else:
                lines.append(f'            .query_opt("{p.name}", self.{p.rust_name})')
        lines[-1] = lines[-1] + ";"
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


def _reachable_struct_keys(ir: ApiIR) -> set[str]:
    """从 body_struct 出发可达的 struct key 集合（MVP 只渲染 body 子树，排除 response）。"""
    visited: set[str] = set()
    if ir.body_struct is None:
        return visited
    stack = [ir.body_struct.key]
    while stack:
        key = stack.pop()
        if key in visited:
            continue
        visited.add(key)
        struct = ir.structs.get(key)
        if struct is None:
            continue
        for f in struct.fields:
            if (
                isinstance(f.type_expr, TypeStructRef)
                and f.type_expr.struct_key in ir.structs
            ):
                stack.append(f.type_expr.struct_key)
    return visited

