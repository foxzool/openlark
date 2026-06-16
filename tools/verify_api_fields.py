#!/usr/bin/env python3
"""
API 字段核对工具：扫描代码字段、检测可疑模式、可选抓飞书文档对比。

用法：
    快速模式（默认，秒级）：python3 tools/verify_api_fields.py --crate openlark-workflow
    完整模式（抓文档）：    python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs

设计文档：docs/superpowers/specs/2026-06-16-api-field-verify-tool-design.md
"""
from __future__ import annotations

import argparse
import csv
import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple

REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CSV = REPO_ROOT / "api_list_export.csv"


# ---------------------------------------------------------------------------
# 数据模型
# ---------------------------------------------------------------------------


@dataclass
class ApiRecord:
    """CSV 中一条 API 记录（只保留核对需要的字段）。"""

    api_id: str
    name: str
    biz_tag: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name: str
    url: str
    doc_path: str
    full_path: str

    @property
    def http_method(self) -> str:
        return self.url.partition(":")[0].upper()

    @property
    def endpoint_path(self) -> str:
        return self.url.partition(":")[2]

    @property
    def is_user_level(self) -> bool:
        """用户级接口：文档 fullPath 含 /reference/（新版用户级路径标识）。"""
        return "/reference/" in self.full_path


# ---------------------------------------------------------------------------
# 路径推断
# ---------------------------------------------------------------------------


def generate_expected_file_path(api: ApiRecord) -> str:
    """根据 API 元信息推断 .rs 文件相对路径（移植自 validate_apis.py）。

    规则：bizTag/project/version/resource/name.rs
      - resource 的 . 转为 /
      - name 的 : 转为 _
    """
    resource_path = api.meta_resource.replace(".", "/")
    name_path = api.meta_name.replace(":", "_").rstrip("/")
    return f"{api.biz_tag}/{api.meta_project}/{api.meta_version}/{resource_path}/{name_path}.rs"


# ---------------------------------------------------------------------------
# CSV 加载
# ---------------------------------------------------------------------------


def load_apis_from_csv(
    csv_path: Path, filter_tags: Optional[List[str]] = None
) -> List[ApiRecord]:
    """从 CSV 加载 API 记录，可按 bizTag 过滤。跳过 old 版本。"""
    apis: List[ApiRecord] = []
    with open(csv_path, encoding="utf-8-sig", newline="") as f:
        for row in csv.DictReader(f):
            if filter_tags and row.get("bizTag", "") not in filter_tags:
                continue
            if row.get("meta.Version") == "old":
                continue
            apis.append(
                ApiRecord(
                    api_id=row["id"],
                    name=row["name"],
                    biz_tag=row["bizTag"],
                    meta_project=row["meta.Project"],
                    meta_version=row["meta.Version"],
                    meta_resource=row["meta.Resource"],
                    meta_name=row["meta.Name"],
                    url=row["url"],
                    doc_path=row.get("docPath", ""),
                    full_path=row.get("fullPath", ""),
                )
            )
    return apis


# ---------------------------------------------------------------------------
# CLI 入口
# ---------------------------------------------------------------------------


def main() -> int:
    """CLI 入口（后续 task 逐步充实）。"""
    parser = argparse.ArgumentParser(description="API 字段核对工具")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="API 清单 CSV 路径")
    args = parser.parse_args()
    print(f"📂 CSV: {args.csv}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
