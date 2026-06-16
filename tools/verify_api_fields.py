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
# Rust 源码字段提取
# ---------------------------------------------------------------------------


@dataclass
class FieldInfo:
    """单个字段信息。"""

    name: str  # Rust 字段名（rename 前的 snake_case）
    type_name: str  # 类型名（Vec<String> -> String，Option<i32> -> i32）
    required: bool  # 是否必填（Option -> False，其余 -> True）
    rename: Optional[str] = None  # serde rename 后的名字，无则 None

    @property
    def effective_name(self) -> str:
        """对比时用的名字：rename 优先。"""
        return self.rename or self.name


@dataclass
class StructFields:
    """一个 struct 提取出的字段集合。"""

    name: str  # struct 名
    fields: List[FieldInfo] = field(default_factory=list)


def extract_structs(source: str) -> List[StructFields]:
    """从 Rust 源码提取 Body/Response struct 的字段。

    只提取名字含 Body 或 Response 的 struct（请求体/响应体），
    跳过 Request struct（那是 builder，不是字段定义）。
    """
    results: List[StructFields] = []
    # 匹配 pub struct Name { ... }，非贪婪到第一个 }
    pattern = re.compile(r"pub\s+struct\s+(\w+)\s*\{([^}]*)\}", re.S)
    for m in pattern.finditer(source):
        name = m.group(1)
        if "Body" not in name and "Response" not in name:
            continue
        body = m.group(2)
        results.append(StructFields(name=name, fields=_extract_fields_from_block(body)))
    return results


def _extract_fields_from_block(block: str) -> List[FieldInfo]:
    """从 struct 体内提取字段列表。"""
    fields: List[FieldInfo] = []
    lines = block.split("\n")
    pending_rename: Optional[str] = None
    for line in lines:
        stripped = line.strip()
        if not stripped:
            continue
        # 收集 serde rename 属性
        rename_match = re.search(r'#\[serde\s*\([^)]*rename\s*=\s*"([^"]+)"', stripped)
        if rename_match:
            pending_rename = rename_match.group(1)
            continue
        # 跳过其他属性行和注释行
        if stripped.startswith("#[") or stripped.startswith("//"):
            continue
        # 匹配 pub field_name: Type,
        field_match = re.match(r"pub\s+(\w+)\s*:\s*(.+?),?\s*$", stripped)
        if not field_match:
            continue
        fname = field_match.group(1)
        raw_type = field_match.group(2).strip().rstrip(",")
        required, type_name = _parse_type(raw_type)
        fields.append(
            FieldInfo(
                name=fname,
                type_name=type_name,
                required=required,
                rename=pending_rename,
            )
        )
        pending_rename = None
    return fields


def _parse_type(raw: str) -> Tuple[bool, str]:
    """解析类型字符串，返回 (是否必填, 规范化类型名)。"""
    raw = raw.strip()
    # Option<T> -> 选填，内部类型
    opt_match = re.match(r"Option<(.+)>$", raw)
    if opt_match:
        inner = opt_match.group(1).strip()
        return False, _unwrap_generic(inner)
    # Vec<T> -> 必填，元素类型
    vec_match = re.match(r"Vec<(.+)>$", raw)
    if vec_match:
        return True, _unwrap_generic(vec_match.group(1).strip())
    # 裸类型
    return True, _unwrap_generic(raw)


def _unwrap_generic(type_str: str) -> str:
    """去掉外层泛型，取核心类型名（Vec<String> -> String，HashMap<K,V> -> K）。"""
    inner_match = re.match(r"\w+<(.+)>$", type_str)
    if inner_match:
        return inner_match.group(1).split(",")[0].strip()
    return type_str


# ---------------------------------------------------------------------------
# 可疑模式检测
# ---------------------------------------------------------------------------


@dataclass
class FieldIssue:
    """一个检测到的字段问题。"""

    severity: str  # "error" | "warning" | "info"
    category: str  # 问题类别标识
    detail: str  # 人可读的描述


def detect_suspicious_patterns(
    api: ApiRecord, structs: List[StructFields], source: str
) -> List[FieldIssue]:
    """检测不抓文档就能发现的字段问题（三类红旗）。

    红旗依据：
      1. 用户级接口 Body 含 user_id/approval_code（用户级从 token 推断，不应有）
      2. Vec 字段缺 validate_required_list! 校验
      3. GET 查询接口 Response 为空（可能漏建响应字段）
    """
    issues: List[FieldIssue] = []

    # 收集所有 Body struct 的字段
    body_structs = [s for s in structs if "Body" in s.name]

    # 红旗 1：用户级接口含 user_id / approval_code
    if api.is_user_level:
        for s in body_structs:
            for f in s.fields:
                if f.name in ("user_id", "approval_code"):
                    issues.append(
                        FieldIssue(
                            severity="warning",
                            category="user_level_extra_field",
                            detail=(
                                f"用户级接口 {s.name} 含 {f.name} 字段——"
                                "用户级接口的操作者身份从 user_access_token 推断，"
                                "请求体通常不应含此字段"
                            ),
                        )
                    )

    # 红旗 2：Vec 字段缺 validate_required_list!
    has_validate_list = "validate_required_list!" in source
    for s in body_structs:
        vec_fields = [f for f in s.fields if f.name.endswith("_ids") or f.name.endswith("_user_ids")]
        # 启发式：字段名暗示数组（_ids）但源码没用 list 校验
        if vec_fields and not has_validate_list:
            for f in vec_fields:
                issues.append(
                    FieldIssue(
                        severity="warning",
                        category="missing_validate_required_list",
                        detail=(
                            f"Body {s.name} 的 {f.name} 疑似数组字段，"
                            "但 execute_with_options 未使用 validate_required_list! 校验"
                        ),
                    )
                )

    # 红旗 3：GET 查询接口 Response 为空
    if api.http_method == "GET":
        resp_structs = [s for s in structs if "Response" in s.name]
        for s in resp_structs:
            if not s.fields:
                issues.append(
                    FieldIssue(
                        severity="info",
                        category="empty_get_response",
                        detail=(
                            f"GET 接口 {s.name} 无响应字段——"
                            "查询接口通常应返回数据，可能漏建响应体"
                        ),
                    )
                )

    return issues


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
