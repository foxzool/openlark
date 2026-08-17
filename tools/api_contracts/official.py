"""Official API catalog loading and normalization."""

from __future__ import annotations

import csv
import re
from pathlib import Path

from .models import ApiIdentity


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


def normalize_endpoint_path(path: str) -> str:
    normalized = path.strip().rstrip("/")
    # 去掉查询参数部分
    query_pos = normalized.find("?")
    if query_pos >= 0:
        normalized = normalized[:query_pos]
    normalized = re.sub(r"\{[^}/]*\}", "{param}", normalized)
    normalized = re.sub(r":[A-Za-z_][A-Za-z0-9_]*", "{param}", normalized)
    return normalized
