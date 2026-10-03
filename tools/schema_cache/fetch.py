"""飞书官方 get_detail 取数栈（schema_cache 专用）。

自 tools/api_contracts/official.py 搬迁（2026-08-17，#635）：两个 CLI 调用方已切
official_evidence 深模块，本取数栈唯一生产消费者是 schema_cache（codegen / okr v2
批量预取），紧邻消费者安家。official.py 回归纯 catalog seam。
"""

from __future__ import annotations

import json
import time
import urllib.parse
import urllib.request
from typing import Any

from tools.api_contracts.models import ApiIdentity


DOC_DETAIL_URL = "https://open.feishu.cn/document_portal/v1/document/get_detail"


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
        except Exception as exc:  # noqa: BLE001 - schema_cache prefetch reports fetch failures per-API.
            last_error = exc
            if attempt < retries:
                time.sleep(min(2**attempt, 8))
    raise RuntimeError(str(last_error))


def extract_api_schema(payload: dict[str, Any]) -> dict[str, Any]:
    data = payload.get("data") or {}
    schema = data.get("schema") or {}
    api_schema = schema.get("apiSchema") if isinstance(schema, dict) else {}
    return api_schema if isinstance(api_schema, dict) else {}
