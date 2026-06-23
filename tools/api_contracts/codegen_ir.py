"""Codegen IR：飞书 apiSchema dict → 与风格 A 渲染对齐的中间表示。

与 models.py 的 OfficialField（扁平 4 字段 diff 叶子）**共存不改**。本模块是单向
（schema→IR），带递归类型树 + query/path 参数拆分 + 完整 OpenAPI 节点保留，
专供代码生成。验证器（compare.py）继续用 OfficialField，两者职责不重叠。

Step 0 勘察结论（见 tools/schema_cache/SCHEMA_FINDINGS.md）：
- properties 是 list 形式 [{name,...}]（iter_properties 兼容 dict/list）
- 枚举用 options:[{name,value,description}]（不是 OpenAPI enum）
- $ref 0 次（不实现）
- query/path 参数在顶层 parameters[in=query/path]
- Token 类型在 security.supportedAccessToken
"""

from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Any, Union

from .models import ApiIdentity
from .official import camel_to_snake

# --------------------------------------------------------------------------- #
# 类型表达式
# --------------------------------------------------------------------------- #


@dataclass(frozen=True)
class TypePrimitive:
    """基础类型：String/i64/bool/f64/serde_json::Value 等。"""

    rust: str
    schema_type: str = ""
    schema_format: str = ""


@dataclass(frozen=True)
class TypeArray:
    item: "TypeExpr"


@dataclass(frozen=True)
class TypeMap:
    """HashMap<String, V>（schema additionalProperties 形态）。"""

    value: "TypeExpr"


@dataclass(frozen=True)
class TypeStructRef:
    """指向同文件 ApiIR.structs 里的嵌套 StructDef。"""

    struct_key: str


@dataclass(frozen=True)
class TypeEnumRef:
    """指向同文件 ApiIR.enums 里的枚举（MVP 默认不启用，退化 String）。"""

    enum_key: str


@dataclass(frozen=True)
class TypeOpaque:
    """退化为 serde_json::Value。reason 供渲染注释。"""

    reason: str


TypeExpr = Union[TypePrimitive, TypeArray, TypeMap, TypeStructRef, TypeEnumRef, TypeOpaque]


# --------------------------------------------------------------------------- #
# 结构 / 参数 / API
# --------------------------------------------------------------------------- #


@dataclass(frozen=True)
class FieldDef:
    name: str  # 官方原始名（camelCase 或 snake_case）
    rust_name: str  # snake_case Rust 字段名
    type_expr: TypeExpr
    required: bool
    description: str = ""
    max_length: int | None = None  # 校验约束（供 G6）
    options: tuple[tuple[str, str], ...] = ()  # 枚举可选值 (serde_value, description)


@dataclass(frozen=True)
class StructDef:
    key: str  # 全局唯一 key（同 ApiIR.structs 的键）
    rust_name: str  # PascalCase
    fields: tuple[FieldDef, ...]
    origin: str  # "request_body" | "response_data" | "nested:<path>"
    serde_rename_all: str = "camelCase"
    object_name: str = ""  # 官方 objectName（如 CreateMessageResp），命名参考


@dataclass(frozen=True)
class EnumDef:
    key: str
    rust_name: str
    variants: tuple[tuple[str, str], ...]  # (rust_variant, serde_value)


@dataclass(frozen=True)
class ParamDef:
    """query / path 参数（来自 apiSchema.parameters）。"""

    name: str
    rust_name: str
    type_expr: TypeExpr
    required: bool
    location: str  # "query" | "path"
    description: str = ""
    default: str = ""
    options: tuple[tuple[str, str], ...] = ()


@dataclass
class ApiIR:
    api_id: str
    name: str  # 官方中文名（doc 用）
    doc_path: str
    method: str  # POST/GET/...
    endpoint_path: str  # 原始 path（含 :param）
    endpoint_const_name: str  # IM_V1_MESSAGES（base path 派生，同 base 复用）
    endpoint_const_value: str  # /open-apis/im/v1/messages（第一个参数前的 base，常量值）
    endpoint_format_template: str  # ""=无参；"{}/{}"=有参（首个 {} 占常量位）
    needs_format: bool  # endpoint_format_template 非空 → format!()
    request_struct_name: str  # CreateMessageRequest（parse 时派生）
    path_params: tuple[ParamDef, ...] = ()
    query_params: tuple[ParamDef, ...] = ()
    body_struct: StructDef | None = None
    response_struct: StructDef | None = None
    structs: dict[str, StructDef] = field(default_factory=dict)
    enums: dict[str, EnumDef] = field(default_factory=dict)
    supported_access_tokens: tuple[str, ...] = ()  # G5 数据源
    notes: list[str] = field(default_factory=list)


# --------------------------------------------------------------------------- #
# 命名工具
# --------------------------------------------------------------------------- #

_RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
}


def to_pascal(name: str) -> str:
    """snake/camel/raw → PascalCase。空串返回 "Item"。"""
    snake = camel_to_snake(name).strip("_")
    parts = [p for p in snake.split("_") if p]
    if not parts:
        return "Item"
    return "".join(p[:1].upper() + p[1:] for p in parts)


def field_rust_name(raw: str) -> str:
    """raw 字段名 → snake_case，避开 Rust 关键字（加 _ 后缀）。"""
    snake = camel_to_snake(raw).strip("_")
    if snake in _RUST_KEYWORDS:
        snake += "_"
    return snake


# --------------------------------------------------------------------------- #
# endpoint 常量派生
# --------------------------------------------------------------------------- #


def derive_endpoint_const(raw_path: str) -> tuple[str, str, str, bool, list[str]]:
    """从 path 派生 (常量名, 常量值base, format模板, needs_format, path参数名)。

    常量值 = 第一个 path 参数前的 base（同 base 的 API 复用同一常量，对齐现有 endpoints/，
    如 message 的 create/get/reply 都用 IM_V1_MESSAGES）。format 模板 = "{}" + suffix
    （suffix 中 :param/{param} → {}），首个 {} 占常量位。

    /open-apis/im/v1/messages                          → IM_V1_MESSAGES, base, "",        False, []
    /open-apis/im/v1/messages/:message_id              → IM_V1_MESSAGES, base, "{}/{}",    True,  [message_id]
    /open-apis/im/v1/messages/:message_id/reply        → IM_V1_MESSAGES, base, "{}/{}/reply", True, [message_id]
    /open-apis/contact/v3/departments/:department_id/children → CONTACT_V3_DEPARTMENTS, ..., True, [department_id]
    """
    param_names = [
        a or b
        for a, b in re.findall(r":([A-Za-z_][A-Za-z0-9_]*)|\{([A-Za-z_][A-Za-z0-9_]*)\}", raw_path)
    ]
    first = re.search(r"/:[A-Za-z_][A-Za-z0-9_]*|/\{[A-Za-z_][A-Za-z0-9_]*\}", raw_path)
    if first:
        base = raw_path[: first.start()].rstrip("/")
        suffix = raw_path[first.start():]
    else:
        base = raw_path.rstrip("/")
        suffix = ""
    segs = [s for s in base.split("/") if s and s != "open-apis"]
    const_name = "_".join(camel_to_snake(s).upper() for s in segs)
    if suffix:
        tmpl = re.sub(r"/:[A-Za-z_][A-Za-z0-9_]*|/\{[A-Za-z_][A-Za-z0-9_]*\}", "/{}", suffix)
        format_template = "{}" + tmpl
    else:
        format_template = ""
    return const_name, base, format_template, bool(param_names), param_names


# --------------------------------------------------------------------------- #
# parser
# --------------------------------------------------------------------------- #

MAX_STRUCT_DEPTH = 3

PRIMITIVE_MAP: dict[tuple[str, str], str] = {
    ("string", ""): "String",
    ("string", "date-time"): "String",  # RFC3339 文本，不引 chrono
    ("string", "byte"): "String",  # base64 文本
    ("string", "int64"): "String",  # 罕见，保守 String
    ("string", "binary"): "String",  # multipart，MVP 退化
    ("integer", ""): "i64",
    ("integer", "int32"): "i32",
    ("integer", "int64"): "i64",
    ("number", ""): "f64",
    ("number", "double"): "f64",
    ("number", "float"): "f32",
    ("boolean", ""): "bool",
}


def iter_properties(container: Any) -> list[tuple[str, dict[str, Any]]]:
    """统一 dict/list 两种 properties 形式，返回 [(name, definition), ...]。"""
    result: list[tuple[str, dict[str, Any]]] = []
    if isinstance(container, dict):
        for name, definition in container.items():
            if isinstance(definition, dict):
                result.append((str(name), definition))
    elif isinstance(container, list):
        for definition in container:
            if isinstance(definition, dict) and isinstance(definition.get("name"), str):
                result.append((definition["name"], definition))
    return result


def _find_property(properties: Any, name: str) -> dict[str, Any] | None:
    for pname, pdef in iter_properties(properties):
        if pname == name:
            return pdef
    return None


def _extract_options(options_raw: Any) -> tuple[tuple[str, str], ...]:
    """options:[{name,value,description}] → ((serde_value, description),)。"""
    out: list[tuple[str, str]] = []
    if isinstance(options_raw, list):
        for opt in options_raw:
            if not isinstance(opt, dict):
                continue
            value = str(opt.get("value") or opt.get("name") or "")
            desc = str(opt.get("description") or "")
            if value:
                out.append((value, desc))
    return tuple(out)


def _parse_type_expr(
    ir: ApiIR, definition: dict[str, Any], name_hint: str, depth: int
) -> TypeExpr:
    ftype = str(definition.get("type") or "")
    fmt = str(definition.get("format") or "")

    if "$ref" in definition:
        ir.notes.append(f"{name_hint}: $ref 暂不支持，退化 Value")
        return TypeOpaque("unsupported_ref")

    if ftype == "array":
        items = definition.get("items") or {}
        if not isinstance(items, dict):
            return TypeOpaque("unknown_type")
        return TypeArray(_parse_type_expr(ir, items, name_hint + "_item", depth))

    if ftype == "object":
        props = definition.get("properties")
        if props is not None:  # 嵌套 struct
            if depth >= MAX_STRUCT_DEPTH:
                return TypeOpaque("max_depth_exceeded")
            struct = _parse_object_to_struct(
                definition, name_hint=to_pascal(name_hint),
                origin=f"nested:{name_hint}", depth=depth + 1, ir=ir,
            )
            ir.structs[struct.key] = struct
            return TypeStructRef(struct.key)
        addl = definition.get("additionalProperties")
        if isinstance(addl, dict):  # HashMap
            return TypeMap(_parse_type_expr(ir, addl, name_hint + "_val", depth))
        return TypeOpaque("unknown_type")

    rust = PRIMITIVE_MAP.get((ftype, fmt)) or PRIMITIVE_MAP.get((ftype, ""))
    if rust is None:
        return TypeOpaque("unknown_type")
    return TypePrimitive(rust, schema_type=ftype, schema_format=fmt)


def _parse_object_to_struct(
    schema: dict[str, Any],
    *,
    name_hint: str,
    origin: str,
    depth: int,
    ir: ApiIR,
) -> StructDef:
    required_raw = schema.get("required") or []
    required_set = {str(x) for x in required_raw} if isinstance(required_raw, list) else set()
    fields: list[FieldDef] = []
    for fname, fdef in iter_properties(schema.get("properties")):
        required = bool(fdef.get("required")) or fname in required_set
        type_expr = _parse_type_expr(ir, fdef, name_hint + "_" + fname, depth)
        max_len_raw = fdef.get("maxLength")
        fields.append(
            FieldDef(
                name=fname,
                rust_name=field_rust_name(fname),
                type_expr=type_expr,
                required=required,
                description=str(fdef.get("description") or ""),
                max_length=int(max_len_raw) if isinstance(max_len_raw, int) else None,
                options=_extract_options(fdef.get("options")),
            )
        )
    object_name = str(schema.get("objectName") or "")
    return StructDef(
        key=name_hint,
        rust_name=name_hint,
        fields=tuple(fields),
        origin=origin,
        serde_rename_all="camelCase",
        object_name=object_name,
    )


def _parse_parameters(ir: ApiIR, parameters: Any, path_param_names: list[str]) -> None:
    seen_path: set[str] = set()
    for param in parameters if isinstance(parameters, list) else []:
        if not isinstance(param, dict):
            continue
        location = param.get("in")
        schema = param.get("schema") if isinstance(param.get("schema"), dict) else {}
        name = param.get("name") or schema.get("name")
        if not isinstance(name, str) or not name:
            continue
        if location not in ("query", "path"):
            continue
        required = bool(param.get("required")) or location == "path"
        pd = ParamDef(
            name=name,
            rust_name=field_rust_name(name),
            type_expr=_parse_type_expr(ir, schema, name, 0),
            required=required,
            location=location,
            description=str(schema.get("description") or ""),
            default=str(schema.get("default") or ""),
            options=_extract_options(schema.get("options")),
        )
        if location == "path":
            seen_path.add(name)
            ir.path_params += (pd,)
        else:
            ir.query_params += (pd,)
    # path 字符串里声明、但 parameters 没给的 path 参数 → 默认 String 兜底
    for pname in path_param_names:
        if pname in seen_path:
            continue
        ir.path_params += (
            ParamDef(
                name=pname,
                rust_name=field_rust_name(pname),
                type_expr=TypePrimitive("String"),
                required=True,
                location="path",
            ),
        )


def parse_api_schema_to_ir(api: ApiIdentity, api_schema: dict[str, Any]) -> ApiIR:
    """apiSchema dict → ApiIR。自适应：未知类型/深嵌套/$ref 退化 Value。"""
    method = str(api_schema.get("httpMethod") or "").upper()
    raw_path = str(api_schema.get("path") or "")
    const_name, const_value, format_template, needs_format, path_param_names = derive_endpoint_const(raw_path)

    ir = ApiIR(
        api_id=api.api_id,
        name=api.name,
        doc_path=api.doc_path,
        method=method,
        endpoint_path=raw_path,
        endpoint_const_name=const_name,
        endpoint_const_value=const_value,
        endpoint_format_template=format_template,
        needs_format=needs_format,
        request_struct_name=request_struct_name(api),
    )

    _parse_parameters(ir, api_schema.get("parameters"), path_param_names)

    # requestBody（取 application/json，其他 content 类型退化）
    request_body = api_schema.get("requestBody") if isinstance(api_schema.get("requestBody"), dict) else {}
    content = request_body.get("content") if isinstance(request_body, dict) else {}
    body_schema = _pick_content_schema(content, "application/json")
    if body_schema is not None:
        struct = _parse_object_to_struct(
            body_schema,
            name_hint=_body_struct_name(api),
            origin="request_body",
            depth=0,
            ir=ir,
        )
        ir.body_struct = struct
        ir.structs[struct.key] = struct
    elif request_body:
        ir.notes.append("requestBody 非 application/json（可能 multipart），body 退化 Value")

    # response.data（解析存起来供未来 typed response；MVP 渲染仍用 Value）
    data_schema = _pick_response_data_schema(api_schema)
    if data_schema is not None:
        struct = _parse_object_to_struct(
            data_schema,
            name_hint=_response_struct_name(api, data_schema),
            origin="response_data",
            depth=0,
            ir=ir,
        )
        ir.response_struct = struct
        ir.structs[struct.key] = struct

    # G5 Token 数据源
    security = api_schema.get("security") if isinstance(api_schema.get("security"), dict) else {}
    sat = security.get("supportedAccessToken")
    if isinstance(sat, list):
        ir.supported_access_tokens = tuple(str(x) for x in sat)

    return ir


def _pick_content_schema(content: Any, preferred: str) -> dict[str, Any] | None:
    if not isinstance(content, dict):
        return None
    body = content.get(preferred) or next(iter(content.values()), None)
    schema = body.get("schema") if isinstance(body, dict) else None
    return schema if isinstance(schema, dict) else None


def _pick_response_data_schema(api_schema: dict[str, Any]) -> dict[str, Any] | None:
    responses = api_schema.get("responses") or {}
    r200 = responses.get("200") or responses.get(200) or {}
    content = r200.get("content") if isinstance(r200, dict) else {}
    schema = _pick_content_schema(content, "application/json")
    if not isinstance(schema, dict):
        return None
    data_def = _find_property(schema.get("properties"), "data")
    return data_def if isinstance(data_def, dict) else None


def _body_struct_name(api: ApiIdentity) -> str:
    """create.rs 风格：动词 + 资源 + Body，如 CreateMessageBody。"""
    return to_pascal(api.meta_name.replace(":", "_")) + to_pascal(api.meta_resource) + "Body"


def _response_struct_name(api: ApiIdentity, data_schema: dict[str, Any]) -> str:
    """优先官方 objectName，否则动词+资源+Response。"""
    object_name = str(data_schema.get("objectName") or "")
    if object_name and object_name.replace("_", "").isalnum():
        return object_name
    return to_pascal(api.meta_name.replace(":", "_")) + to_pascal(api.meta_resource) + "Response"


def request_struct_name(api: ApiIdentity) -> str:
    """CreateMessageRequest 风格。"""
    return to_pascal(api.meta_name.replace(":", "_")) + to_pascal(api.meta_resource) + "Request"
