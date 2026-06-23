"""飞书 apiSchema 持久缓存。

复用 tools.api_contracts.official.fetch_detail_payload 做实际取数（零鉴权 urllib +
指数退避 min(2**attempt,8)），本模块只加：落盘 + mtime + refresh + 失败隔离。

缓存策略：默认永不过期，--refresh 强制重拉；损坏 JSON 静默重拉。
失败隔离：record_error 把单条失败 append 到 errors.json，不中断批量。
"""

from __future__ import annotations

import json
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official import extract_api_schema, fetch_detail_payload

DEFAULT_CACHE_DIR = Path(__file__).parent / ".cache"


@dataclass(frozen=True)
class CacheResult:
    """get_or_fetch 的返回。payload 是完整 detail 响应，api_schema 是提取出的 apiSchema dict。"""

    payload: dict[str, Any]
    api_schema: dict[str, Any]
    source: str  # "cache" | "fetch"


def get_or_fetch(
    api: ApiIdentity,
    *,
    cache_dir: Path = DEFAULT_CACHE_DIR,
    refresh: bool = False,
    timeout: int = 20,
    retries: int = 2,
) -> CacheResult:
    """命中缓存且未 refresh 则读盘；否则拉取并落盘。损坏文件静默重拉。"""
    cache_dir.mkdir(parents=True, exist_ok=True)
    path = cache_dir / f"{api.api_id}.json"
    if path.exists() and not refresh:
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
            if isinstance(payload, dict):
                return CacheResult(payload, extract_api_schema(payload), "cache")
        except (json.JSONDecodeError, OSError):
            pass  # 损坏则重拉
    payload = fetch_detail_payload(api, timeout=timeout, retries=retries)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return CacheResult(payload, extract_api_schema(payload), "fetch")


def record_error(cache_dir: Path, api: ApiIdentity, exc: Exception) -> None:
    """失败隔离：append 到 errors.json，不抛、不中断后续 API。"""
    cache_dir.mkdir(parents=True, exist_ok=True)
    errors_path = cache_dir / "errors.json"
    errors: list[dict[str, Any]] = []
    if errors_path.exists():
        try:
            loaded = json.loads(errors_path.read_text(encoding="utf-8"))
            if isinstance(loaded, list):
                errors = loaded
        except (json.JSONDecodeError, OSError):
            errors = []
    errors.append(
        {
            "api_id": api.api_id,
            "name": api.name,
            "full_path": api.full_path,
            "error": str(exc),
            "ts": time.time(),
        }
    )
    errors_path.write_text(json.dumps(errors, ensure_ascii=False, indent=2), encoding="utf-8")
