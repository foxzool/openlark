"""Official API catalog loading and normalization."""

from __future__ import annotations

import csv
import json
import re
import time
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from .models import ApiIdentity, OfficialField


DOC_DETAIL_URL = "https://open.feishu.cn/document_portal/v1/document/get_detail"
DOC_MARKDOWN_BASE_URL = "https://open.feishu.cn"

# security.supportedAccessToken 与 .md Authorization 行里合法的 token 凭证名。
_KNOWN_ACCESS_TOKENS = {
    "tenant_access_token",
    "user_access_token",
    "app_access_token",
    "none_access_token",
}


def camel_to_snake(name: str) -> str:
    name = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    name = name.replace("-", "_")
    return name.lower()


def normalize_name_path(name_path: str) -> str:
    name_path = name_path.replace("#", "_")
    segments = [segment for segment in name_path.split("/") if segment]
    normalized: list[str] = []
    for segment in segments:
        if segment.startswith("_") and len(segment) > 1:
            normalized.append("_" + camel_to_snake(segment[1:]))
        else:
            normalized.append(camel_to_snake(segment))
    return "/".join(normalized)


def expected_file_path(row: dict[str, str]) -> str:
    biz_tag = row.get("bizTag", "")
    meta_version = row.get("meta.Version", "")
    meta_resource = row.get("meta.Resource", "")
    meta_name = row.get("meta.Name", "")

    if biz_tag == "meeting_room" and meta_version == "old" and meta_resource == "default":
        name_path = normalize_name_path(meta_name.replace(":", "_"))
        return f"meeting_room/{name_path}.rs"

    base = f"{biz_tag}/{row.get('meta.Project', '')}"
    resource_path = meta_resource.replace(".", "/")
    name_path = normalize_name_path(meta_name.replace(":", "_").rstrip("/"))
    return f"{base}/{meta_version}/{resource_path}/{name_path}.rs"


def load_api_identities(
    csv_path: Path,
    filter_tags: list[str] | None = None,
    skip_old_versions: bool = True,
) -> list[ApiIdentity]:
    rows: list[ApiIdentity] = []
    tag_filter = set(filter_tags or [])

    with csv_path.open("r", encoding="utf-8-sig", newline="") as handle:
        reader = csv.DictReader(handle)
        for row in reader:
            if tag_filter and row.get("bizTag", "") not in tag_filter:
                continue
            if skip_old_versions and row.get("meta.Version", "") == "old":
                continue
            rows.append(
                ApiIdentity(
                    api_id=row.get("id", ""),
                    name=row.get("name", ""),
                    biz_tag=row.get("bizTag", ""),
                    meta_project=row.get("meta.Project", ""),
                    meta_version=row.get("meta.Version", ""),
                    meta_resource=row.get("meta.Resource", ""),
                    meta_name=row.get("meta.Name", ""),
                    url=row.get("url", ""),
                    doc_path=row.get("docPath", ""),
                    expected_file=expected_file_path(row),
                    full_path=row.get("fullPath", ""),
                )
            )
    return rows


def detail_full_path(api: ApiIdentity) -> str:
    full_path = api.full_path.strip()
    if full_path.startswith("/document/"):
        return full_path.removeprefix("/document")
    if full_path == "/document":
        return ""
    return full_path


def fetch_detail_payload(api: ApiIdentity, timeout: int, retries: int) -> dict[str, Any]:
    full_path = detail_full_path(api)
    if not full_path:
        raise ValueError(f"API {api.api_id} has no fullPath")

    url = DOC_DETAIL_URL + "?" + urllib.parse.urlencode({"fullPath": full_path})
    last_error: Exception | None = None
    for attempt in range(retries + 1):
        try:
            request = urllib.request.Request(
                url,
                headers={"User-Agent": "openlark-api-contract-validator/1.0"},
            )
            with urllib.request.urlopen(request, timeout=timeout) as response:
                raw = response.read()
            payload = json.loads(raw.decode("utf-8"))
            if isinstance(payload, dict):
                return payload
            raise ValueError("official detail payload is not a JSON object")
        except Exception as exc:  # noqa: BLE001 - command-line validator reports fetch failures.
            last_error = exc
            if attempt < retries:
                time.sleep(min(2**attempt, 8))
    raise RuntimeError(str(last_error))


def extract_endpoint_from_detail_payload(payload: dict[str, Any]) -> tuple[str, str]:
    data = payload.get("data") or {}
    schema = data.get("schema") or {}
    api_schema = schema.get("apiSchema") if isinstance(schema, dict) else {}
    if not isinstance(api_schema, dict):
        return "", ""

    method = api_schema.get("httpMethod")
    path = api_schema.get("path")
    if isinstance(method, str) and isinstance(path, str):
        return method.strip().upper(), path.strip()
    return "", ""


def extract_request_fields_from_detail_payload(payload: dict[str, Any]) -> tuple[OfficialField, ...]:
    api_schema = extract_api_schema(payload)
    request_body = api_schema.get("requestBody") if isinstance(api_schema, dict) else {}
    if not isinstance(request_body, dict):
        return ()

    content = request_body.get("content") or {}
    if not isinstance(content, dict):
        return ()

    fields: dict[str, OfficialField] = {}
    for content_type, body in content.items():
        if not isinstance(body, dict):
            continue
        schema = body.get("schema") or {}
        if not isinstance(schema, dict):
            continue
        for item in extract_schema_properties(schema):
            if not item.name or item.name in fields:
                continue
            fields[item.name] = OfficialField(
                name=item.name,
                required=item.required,
                location=f"requestBody:{content_type}",
                field_type=item.field_type,
                source="official_api_schema",
            )
    return tuple(fields.values())


def extract_access_token_types_from_detail_payload(payload: dict[str, Any]) -> tuple[str, ...]:
    """从 detail payload 解析官方文档标注的可接受 token 类型。

    oracle 路径：``data.schema.apiSchema.security.supportedAccessToken``
    （如 ``["tenant_access_token", "user_access_token"]``，见
    ``tools/schema_cache/SCHEMA_FINDINGS.md``）。未标注时返回空元组，调用方据此
    降级为 UNVERIFIED（无法核对），而非误报为不匹配。
    """
    api_schema = extract_api_schema(payload)
    security = api_schema.get("security") if isinstance(api_schema, dict) else {}
    if not isinstance(security, dict):
        return ()
    tokens = security.get("supportedAccessToken")
    if not isinstance(tokens, list):
        return ()
    return tuple(str(token) for token in tokens if isinstance(token, str))


def fetch_doc_markdown(api: ApiIdentity, timeout: int) -> str:
    """抓取飞书文档 ``.md`` 源（SPA 页正文的真实来源，见 api-spec-accuracy-audit）。

    部分 ``server-docs`` 风格页（如 acs ``user/get``、``user/list``）的 detail payload
    不含 ``security.supportedAccessToken``，但其 ``.md`` 源的请求头 Authorization 行
    仍标注了凭证类型——作为 token oracle 的回退来源。失败/无 fullPath 返回空串。
    """
    full_path = detail_full_path(api)
    if not full_path:
        return ""
    url = DOC_MARKDOWN_BASE_URL + "/document" + full_path + ".md"
    try:
        request = urllib.request.Request(
            url,
            headers={"User-Agent": "openlark-api-contract-validator/1.0"},
        )
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return response.read().decode("utf-8", "replace")
    except Exception:  # noqa: BLE001 - 回退来源，失败即视作无标注。
        return ""


def parse_access_token_types_from_markdown(text: str) -> tuple[str, ...]:
    """从 ``.md`` 源的「请求头 Authorization」表头行解析可接受 token 类型。

    仅采集以 ``Authorization`` 开头的表头行内反引号包裹的 ``*_access_token`` 凭证名
    （如 ``Authorization | string | 是 | `tenant_access_token`、`user_access_token```），
    忽略正文里的散文提及。按出现顺序去重。
    """
    tokens: list[str] = []
    seen: set[str] = set()
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped.startswith("Authorization"):
            continue
        for token in re.findall(r"`([a-z_]+_access_token)`", stripped):
            if token in _KNOWN_ACCESS_TOKENS and token not in seen:
                seen.add(token)
                tokens.append(token)
    return tuple(tokens)


def extract_response_fields_from_detail_payload(payload: dict[str, Any]) -> tuple[OfficialField, ...]:
    api_schema = extract_api_schema(payload)
    responses = api_schema.get("responses") if isinstance(api_schema, dict) else {}
    if not isinstance(responses, dict):
        return ()

    response_200 = responses.get("200") or responses.get(200) or {}
    content = response_200.get("content") if isinstance(response_200, dict) else {}
    if not isinstance(content, dict):
        return ()

    fields: dict[str, OfficialField] = {}
    for content_type, body in content.items():
        if not isinstance(body, dict):
            continue
        schema = body.get("schema") or {}
        if not isinstance(schema, dict):
            continue
        for item in extract_response_data_properties(schema):
            if not item.name or item.name in fields:
                continue
            fields[item.name] = OfficialField(
                name=item.name,
                required=item.required,
                location=f"responseBody:{content_type}.data",
                field_type=item.field_type,
                source="official_api_schema",
            )
    return tuple(fields.values())


@dataclass(frozen=True)
class _SchemaProperty:
    name: str
    required: bool
    field_type: str


def extract_api_schema(payload: dict[str, Any]) -> dict[str, Any]:
    data = payload.get("data") or {}
    schema = data.get("schema") or {}
    api_schema = schema.get("apiSchema") if isinstance(schema, dict) else {}
    return api_schema if isinstance(api_schema, dict) else {}


def extract_schema_properties(schema: dict[str, Any]) -> tuple[_SchemaProperty, ...]:
    required_names = schema.get("required") or []
    required_set = {str(item) for item in required_names} if isinstance(required_names, list) else set()
    properties = schema.get("properties") or {}
    result: list[_SchemaProperty] = []

    if isinstance(properties, dict):
        for name, definition in properties.items():
            if not isinstance(definition, dict):
                continue
            result.append(schema_property(str(name), definition, str(name) in required_set))
    elif isinstance(properties, list):
        for definition in properties:
            if not isinstance(definition, dict):
                continue
            name = definition.get("name")
            if not isinstance(name, str):
                continue
            result.append(schema_property(name, definition, name in required_set))

    return tuple(result)


def extract_response_data_properties(schema: dict[str, Any]) -> tuple[_SchemaProperty, ...]:
    for item in extract_schema_properties(schema):
        if item.name != "data":
            continue
        data_schema = find_schema_property(schema, "data")
        if data_schema:
            return extract_schema_properties(data_schema)
    return ()


def find_schema_property(schema: dict[str, Any], property_name: str) -> dict[str, Any]:
    properties = schema.get("properties") or {}
    if isinstance(properties, dict):
        value = properties.get(property_name)
        return value if isinstance(value, dict) else {}
    if isinstance(properties, list):
        for item in properties:
            if isinstance(item, dict) and item.get("name") == property_name:
                return item
    return {}


def schema_property(name: str, definition: dict[str, Any], required_by_parent: bool) -> _SchemaProperty:
    required = bool(definition.get("required")) or required_by_parent
    field_type = str(definition.get("type") or "")
    fmt = definition.get("format")
    if isinstance(fmt, str) and fmt:
        field_type = f"{field_type}:{fmt}" if field_type else fmt
    return _SchemaProperty(name=name, required=required, field_type=field_type)


def split_method_path(url: str) -> tuple[str, str]:
    method, separator, path = url.partition(":")
    if not separator:
        return "", ""
    return method.strip().upper(), path.strip()


def normalize_endpoint_path(path: str) -> str:
    normalized = path.strip().rstrip("/")
    # 去掉查询参数部分
    query_pos = normalized.find("?")
    if query_pos >= 0:
        normalized = normalized[:query_pos]
    normalized = re.sub(r"\{[^}/]*\}", "{param}", normalized)
    normalized = re.sub(r"\{[^}/]*\}", "{param}", normalized)
    normalized = re.sub(r":[A-Za-z_][A-Za-z0-9_]*", "{param}", normalized)
    return normalized
