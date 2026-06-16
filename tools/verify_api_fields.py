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
      1. 用户级接口 Body 含 user_id/approval_code（弱启发式，info 级——
         /reference/ 路径也含管理员级接口，需人工判断）
      2. 必填 Vec 字段缺非空校验（认 validate_required_list! 或 is_empty()）
      3. GET 查询接口 Response 为空（可能漏建响应字段）
    """
    issues: List[FieldIssue] = []

    # 收集所有 Body struct 的字段
    body_structs = [s for s in structs if "Body" in s.name]

    # 红旗 1：用户级接口含 user_id / approval_code
    # 注意：is_user_level 基于 /reference/ 路径，但 reference 也含管理员级接口，
    # 故降为 info 级提示，detail 说明"若用 tenant_token 则可忽略"。
    if api.is_user_level:
        for s in body_structs:
            for f in s.fields:
                if f.name in ("user_id", "approval_code"):
                    issues.append(
                        FieldIssue(
                            severity="info",
                            category="user_level_extra_field",
                            detail=(
                                f"{s.name} 含 {f.name} 字段，且文档在 /reference/ 路径下——"
                                "若为用户级接口（user_access_token）此字段多余；"
                                "若为管理员级接口（tenant_access_token）则正常，可忽略"
                            ),
                        )
                    )

    # 红旗 2：必填 Vec 字段缺非空校验
    # 认两种校验写法：validate_required_list! 宏 或 if xxx.is_empty() 手写
    has_list_macro = "validate_required_list!" in source
    for s in body_structs:
        # 只检查必填的数组字段（Option<Vec> 是选填，不报）
        vec_fields = [
            f
            for f in s.fields
            if f.required  # 跳过 Option 字段
            and (
                f.name.endswith(("_ids", "_list", "_tokens", "_keys"))
                or f.name.endswith("_user_ids")
            )
        ]
        for f in vec_fields:
            # 检查字段名是否在 is_empty() 校验里出现
            has_manual_check = f"{f.name}.is_empty()" in source
            if not has_list_macro and not has_manual_check:
                issues.append(
                    FieldIssue(
                        severity="warning",
                        category="missing_list_validation",
                        detail=(
                            f"Body {s.name} 的必填数组字段 {f.name} "
                            "缺少非空校验（validate_required_list! 或 is_empty()）"
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
# 报告生成
# ---------------------------------------------------------------------------


@dataclass
class ApiFieldReport:
    """单个 API 的核对结果。"""

    api: ApiRecord
    file_path: str
    file_exists: bool
    structs: List[StructFields]
    issues: List[FieldIssue]


def run_quick_mode(
    csv_path: Path,
    src_root: Path,
    output_md: Optional[Path] = None,
    output_json: Optional[Path] = None,
    filter_tags: Optional[List[str]] = None,
) -> str:
    """快速模式：扫描代码字段 + 可疑模式检测，不抓文档。返回报告文本。"""
    apis = load_apis_from_csv(csv_path, filter_tags)
    reports: List[ApiFieldReport] = []

    for api in apis:
        rel_path = generate_expected_file_path(api)
        full_path = src_root / rel_path
        if not full_path.exists():
            reports.append(
                ApiFieldReport(
                    api=api, file_path=rel_path, file_exists=False,
                    structs=[], issues=[],
                )
            )
            continue
        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)
        reports.append(
            ApiFieldReport(
                api=api, file_path=rel_path, file_exists=True,
                structs=structs, issues=issues,
            )
        )

    md = _render_report(reports, mode="quick")
    if output_md:
        output_md.parent.mkdir(parents=True, exist_ok=True)
        output_md.write_text(md, encoding="utf-8")
    if output_json:
        _write_summary_json(reports, output_json, mode="quick")
    return md


def _render_report(reports: List[ApiFieldReport], mode: str) -> str:
    """渲染 Markdown 报告。"""
    import datetime

    total = len(reports)
    found = [r for r in reports if r.file_exists]
    missing = [r for r in reports if not r.file_exists]
    with_issues = [r for r in found if r.issues]

    lines = [
        "# API 字段核对报告",
        "",
        f"**生成时间**: {datetime.datetime.now().strftime('%Y-%m-%d %H:%M')}",
        f"**模式**: {mode}",
        "",
        "## 一、总体统计",
        "",
        "| 指标 | 数量 |",
        "|------|------|",
        f"| 核对 API 数 | {total} |",
        f"| 文件存在 | {len(found)} |",
        f"| 文件缺失 | {len(missing)} |",
        f"| 有问题 | {len(with_issues)} |",
        "",
    ]

    if with_issues:
        lines.append("## 二、问题详情（按严重度）")
        lines.append("")
        for sev, label in [("error", "🔴 硬错误"), ("warning", "🟡 警告"), ("info", "🟢 提示")]:
            sev_issues = [
                (r, i) for r in with_issues for i in r.issues if i.severity == sev
            ]
            if not sev_issues:
                continue
            lines.append(f"### {label}（{len(sev_issues)}）")
            lines.append("")
            lines.append("| API | 文件 | 问题 |")
            lines.append("|-----|------|------|")
            for r, i in sev_issues:
                lines.append(f"| {r.api.name} | `{r.file_path}` | {i.detail} |")
            lines.append("")

    if missing:
        lines.append("## 三、文件缺失（无法核对）")
        lines.append("")
        for r in missing:
            lines.append(f"- {r.api.name}: `{r.file_path}`")
        lines.append("")

    return "\n".join(lines)


def _write_summary_json(reports: List[ApiFieldReport], path: Path, mode: str) -> None:
    """写机器可读的 JSON 汇总。"""
    import json

    with_issues = sum(1 for r in reports if r.issues)
    data = {
        "mode": mode,
        "total_apis": len(reports),
        "apis_with_issues": with_issues,
        "apis": [
            {
                "id": r.api.api_id,
                "name": r.api.name,
                "url": r.api.url,
                "file": r.file_path,
                "file_exists": r.file_exists,
                "issues": [
                    {"severity": i.severity, "category": i.category, "detail": i.detail}
                    for i in r.issues
                ],
            }
            for r in reports
        ],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")


# ---------------------------------------------------------------------------
# 文档字段解析与对比（完整模式）
# ---------------------------------------------------------------------------


@dataclass
class FieldDiff:
    """代码字段与文档字段的对比结果。"""

    matched: List[str] = field(default_factory=list)  # 两边都有
    missing: List[str] = field(default_factory=list)  # 文档有、代码无
    extra: List[str] = field(default_factory=list)  # 代码有、文档无


def parse_doc_request_fields(doc_text: str, method: str) -> List[FieldInfo]:
    """从文档 innerText 提取请求体/查询参数字段。

    POST: Request body（第2次出现）→ Request example
    GET:  Query parameters → Request example
    """
    if method == "POST":
        section = _extract_section(doc_text, "Request body", "Request example", occurrence=2)
    else:
        section = _extract_section(doc_text, "Query parameters", "Request example", occurrence=1)
    if not section:
        return []
    return _parse_param_table(section)


def parse_doc_response_fields(doc_text: str) -> List[str]:
    """从响应示例 JSON 提取字段名集合。

    Response body 的 data 子字段在折叠区拿不到，从示例 JSON 反推。
    返回 data 内部的字段名列表。
    """
    section = _extract_section(doc_text, "Response body example", "Error code", occurrence=1)
    if not section:
        return []
    # 提取所有 "field": 的字段名（排除外层 code/msg/data）
    names = re.findall(r'"([a-z_]+)"\s*:', section)
    return [n for n in names if n not in ("code", "msg", "data")]


def _extract_section(text: str, start: str, end: str, occurrence: int = 1) -> str:
    """提取 start（第 occurrence 次出现）到 end 之间的文本。"""
    parts = text.split(start)
    if len(parts) <= occurrence:
        return ""
    chunk = start.join(parts[occurrence:])
    end_idx = chunk.find(end)
    if end_idx < 0:
        return chunk
    return chunk[:end_idx]


def _parse_param_table(section: str) -> List[FieldInfo]:
    """解析参数表（参数名/类型/必填交错成行）。"""
    lines = [l.strip() for l in section.split("\n")]
    results: List[FieldInfo] = []
    banned = {
        "parameter", "type", "required", "description", "authorization",
        "content", "value", "example",
    }
    i = 0
    while i < len(lines):
        line = lines[i]
        # 候选参数名：snake_case，非 banned 词
        if re.fullmatch(r"[a-z][a-z0-9_]*", line) and line not in banned and len(line) >= 2:
            # 往后找 Yes/No
            for j in range(i + 1, min(i + 8, len(lines))):
                if lines[j] in ("Yes", "No"):
                    required = lines[j] == "Yes"
                    results.append(FieldInfo(name=line, type_name="", required=required))
                    i = j + 1
                    break
            else:
                i += 1
        else:
            i += 1
    return results


def compare_fields(
    code_fields: List[FieldInfo], doc_fields: List[FieldInfo]
) -> FieldDiff:
    """对比代码字段与文档字段。"""
    code_names = {f.effective_name for f in code_fields}
    doc_names = {f.effective_name for f in doc_fields}
    return FieldDiff(
        matched=sorted(code_names & doc_names),
        missing=sorted(doc_names - code_names),
        extra=sorted(code_names - doc_names),
    )


# ---------------------------------------------------------------------------
# CLI 入口
# ---------------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description="API 字段核对工具")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="API 清单 CSV 路径")
    parser.add_argument("--crate", help="指定单个 crate（如 openlark-workflow）")
    parser.add_argument("--fetch-docs", action="store_true", help="完整模式：抓飞书文档对比（慢，默认跳过已缓存）")
    parser.add_argument("--output-dir", default="reports/api_field_verify", help="报告输出目录")
    parser.add_argument("--api-id", help="只核对单个 API（调试用，按 CSV id 过滤）")
    args = parser.parse_args()

    csv_path = Path(args.csv)
    out_dir = Path(args.output_dir)

    # 单 API 模式：按 id 过滤，src 根用 crates 目录（路径推断会定位到具体文件）
    if args.api_id:
        src_root = REPO_ROOT / "crates"
        crate_label = f"api-{args.api_id}"
        # 自定义过滤：只保留指定 id 的 API
        all_apis = load_apis_from_csv(csv_path)
        filter_tags = None  # 由 _filter_by_id 处理
        _run_single_api(args.api_id, all_apis, src_root, out_dir, crate_label, args.fetch_docs)
        return 0

    # 确定 src 根目录和 bizTag 过滤
    if args.crate:
        src_root = REPO_ROOT / "crates" / args.crate / "src"
        filter_tags = _load_crate_tags(args.crate)
        crate_label = args.crate
    else:
        # --all-crates 或无参数：扫描整个 crates 目录
        src_root = REPO_ROOT / "crates"
        filter_tags = None
        crate_label = "all"

    print(f"📂 CSV: {csv_path}")
    print(f"📁 源码根: {src_root}")
    print(f"🏷️  过滤 bizTag: {filter_tags or '(全部)'}")

    if args.fetch_docs:
        print("🐌 完整模式（抓文档，默认跳过已缓存）")
        return _run_full_mode(csv_path, src_root, out_dir, crate_label, filter_tags)

    # 快速模式
    print("⚡ 快速模式（代码自检）")
    md = run_quick_mode(
        csv_path=csv_path,
        src_root=src_root,
        output_md=out_dir / f"{crate_label}.md",
        output_json=out_dir / "summary.json",
        filter_tags=filter_tags,
    )
    print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    return 0


def _run_single_api(api_id, all_apis, src_root, out_dir, crate_label, fetch_docs):
    """核对单个 API（调试用）。"""
    api = next((a for a in all_apis if a.api_id == api_id), None)
    if api is None:
        print(f"❌ CSV 中找不到 id={api_id} 的 API")
        return
    print(f"🔍 单 API 核对: {api.name} ({api.url})")
    rel_path = generate_expected_file_path(api)
    # 单 API 模式：在所有 crate 的 src 目录下查找文件
    full_path = None
    for crate_dir in src_root.iterdir():
        candidate = crate_dir / "src" / rel_path
        if candidate.exists():
            full_path = candidate
            break
    if full_path is None:
        print(f"❌ 文件不存在（在所有 crate 中查找）: {rel_path}")
        return
    source = full_path.read_text(encoding="utf-8")
    structs = extract_structs(source)
    issues = detect_suspicious_patterns(api, structs, source)

    # 完整模式：抓文档对比字段
    mode = "quick"
    if fetch_docs and api.full_path:
        mode = "full"
        doc_text = _fetch_single_doc(api, out_dir)
        if doc_text:
            # 对比请求体字段
            doc_req = parse_doc_request_fields(doc_text, api.http_method)
            code_body = next((s.fields for s in structs if "Body" in s.name), [])
            if doc_req and code_body:
                diff = compare_fields(code_body, doc_req)
                if diff.missing:
                    issues.append(
                        FieldIssue("error", "missing_field",
                                   f"请求体缺字段: {', '.join(diff.missing)}")
                    )
                if diff.extra:
                    issues.append(
                        FieldIssue("warning", "extra_field",
                                   f"请求体多余字段: {', '.join(diff.extra)}")
                    )
            # 对比响应体字段
            doc_resp = parse_doc_response_fields(doc_text)
            code_resp = next((s.fields for s in structs if "Response" in s.name), [])
            if doc_resp and code_resp:
                code_resp_names = {f.effective_name for f in code_resp}
                missing_resp = sorted(set(doc_resp) - code_resp_names)
                if missing_resp:
                    issues.append(
                        FieldIssue("info", "missing_response_field",
                                   f"响应体可能缺字段: {', '.join(missing_resp)}")
                    )

    report = ApiFieldReport(
        api=api, file_path=rel_path, file_exists=True, structs=structs, issues=issues,
    )
    md = _render_report([report], mode=mode)
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / f"{crate_label}.md").write_text(md, encoding="utf-8")
    if issues:
        print(f"⚠️ 发现 {len(issues)} 个问题:")
        for i in issues:
            print(f"  [{i.severity}] {i.detail}")
    else:
        print("✅ 无可疑模式")
    print(f"📄 报告: {out_dir / f'{crate_label}.md'}")


def _fetch_single_doc(api: ApiRecord, out_dir: Path) -> str:
    """抓取单个 API 的文档（带缓存）。返回文档文本，失败返回空串。"""
    import subprocess

    fetch_script = (
        REPO_ROOT / ".agents" / "skills" / "openlark-api-field-verify" / "scripts" / "fetch_doc.js"
    )
    if not fetch_script.exists():
        print(f"⚠️ 找不到抓取脚本: {fetch_script}")
        return ""
    doc_cache = out_dir / "doc_cache"
    doc_cache.mkdir(parents=True, exist_ok=True)
    doc_file = doc_cache / f"{api.api_id}.txt"
    if not doc_file.exists():
        url = "https://open.feishu.cn" + api.full_path
        print(f"📄 抓取文档: {url}")
        try:
            subprocess.run(
                ["node", str(fetch_script), url, str(doc_file)],
                check=True, capture_output=True, timeout=90,
            )
        except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
            print("⚠️ 文档抓取失败")
            return ""
    return doc_file.read_text(encoding="utf-8") if doc_file.exists() else ""


def _load_crate_tags(crate: str) -> Optional[List[str]]:
    """从 tools/api_coverage.toml 读 crate 的 biz_tags。"""
    import tomllib  # Python 3.11+

    toml_path = REPO_ROOT / "tools" / "api_coverage.toml"
    if not toml_path.exists():
        return None
    with open(toml_path, "rb") as f:
        data = tomllib.load(f)
    crate_cfg = data.get("crates", {}).get(crate, {})
    return crate_cfg.get("biz_tags")


def _run_full_mode(csv_path, src_root, out_dir, crate_label, filter_tags):
    """完整模式：抓飞书文档对比字段（慢）。"""
    import json
    import subprocess

    apis = load_apis_from_csv(csv_path, filter_tags)
    fetch_script = (
        REPO_ROOT / ".agents" / "skills" / "openlark-api-field-verify" / "scripts" / "fetch_doc.js"
    )
    if not fetch_script.exists():
        print(f"❌ 找不到抓取脚本: {fetch_script}")
        return 1

    reports: List[ApiFieldReport] = []
    doc_cache = out_dir / "doc_cache"
    doc_cache.mkdir(parents=True, exist_ok=True)
    failed: List[Tuple[str, str]] = []

    for idx, api in enumerate(apis, 1):
        rel_path = generate_expected_file_path(api)
        full_path = src_root / rel_path
        if not full_path.exists():
            continue

        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)

        # 抓文档
        doc_text = ""
        if api.full_path:
            url = "https://open.feishu.cn" + api.full_path
            doc_file = doc_cache / f"{api.api_id}.txt"
            if not doc_file.exists():  # 简单 resume：文件存在则跳过
                try:
                    subprocess.run(
                        ["node", str(fetch_script), url, str(doc_file)],
                        check=True, capture_output=True, timeout=90,
                    )
                except (subprocess.CalledProcessError, subprocess.TimeoutExpired) as e:
                    failed.append((api.api_id, str(e)[:80]))
                else:
                    doc_text = (
                        doc_file.read_text(encoding="utf-8") if doc_file.exists() else ""
                    )
            else:
                doc_text = doc_file.read_text(encoding="utf-8")

            # 对比请求体字段
            if doc_text:
                doc_req = parse_doc_request_fields(doc_text, api.http_method)
                code_body = next((s.fields for s in structs if "Body" in s.name), [])
                if doc_req and code_body:
                    diff = compare_fields(code_body, doc_req)
                    if diff.missing:
                        issues.append(
                            FieldIssue(
                                "error", "missing_field",
                                f"请求体缺字段: {', '.join(diff.missing)}",
                            )
                        )
                    if diff.extra:
                        issues.append(
                            FieldIssue(
                                "warning", "extra_field",
                                f"请求体多余字段: {', '.join(diff.extra)}",
                            )
                        )

                # 对比响应体字段（文档示例 vs 代码 Response struct）
                doc_resp = parse_doc_response_fields(doc_text)
                code_resp = next((s.fields for s in structs if "Response" in s.name), [])
                if doc_resp and code_resp:
                    code_resp_names = {f.effective_name for f in code_resp}
                    doc_resp_set = set(doc_resp)
                    missing_resp = sorted(doc_resp_set - code_resp_names)
                    if missing_resp:
                        issues.append(
                            FieldIssue(
                                "info", "missing_response_field",
                                f"响应体可能缺字段: {', '.join(missing_resp)}",
                            )
                        )

        reports.append(
            ApiFieldReport(
                api=api, file_path=rel_path, file_exists=True,
                structs=structs, issues=issues,
            )
        )
        print(f"⏳ [{idx}/{len(apis)}] {api.name} ({len(issues)} 问题)")

    md = _render_report(reports, mode="full")
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / f"{crate_label}.md").write_text(md, encoding="utf-8")
    _write_summary_json(reports, out_dir / "summary.json", mode="full")

    if failed:
        print(f"⚠️ {len(failed)} 个文档抓取失败，详见 failed.json")
        (out_dir / "failed.json").write_text(
            json.dumps(failed, ensure_ascii=False, indent=2), encoding="utf-8"
        )
    print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
