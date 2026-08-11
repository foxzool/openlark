"""Official API catalog loading and normalization."""

from __future__ import annotations

import csv
import json
import re
import time
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

from .models import ApiIdentity


DOC_DETAIL_URL = "https://open.feishu.cn/document_portal/v1/document/get_detail"


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





def extract_api_schema(payload: dict[str, Any]) -> dict[str, Any]:
    data = payload.get("data") or {}
    schema = data.get("schema") or {}
    api_schema = schema.get("apiSchema") if isinstance(schema, dict) else {}
    return api_schema if isinstance(api_schema, dict) else {}




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
