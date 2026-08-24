#!/usr/bin/env python3
"""
API 字段核对工具：扫描代码字段、检测可疑模式、可选抓飞书文档对比。

用法：
    快速模式（默认，秒级）：python3 tools/verify_api_fields.py --crate openlark-workflow
    完整模式（抓文档）：    python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs
    强制重抓 / 超龄：       ... --fetch-docs --force-refresh | --max-age 7
    单 API 门禁：           python3 tools/verify_api_fields.py --api-id <id> --fetch-docs

设计文档：docs/superpowers/specs/2026-06-16-api-field-verify-tool-design.md
"""
from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Optional, Tuple

REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.models import ApiIdentity, FieldInfo, StructFields
from tools.api_contracts.official import load_api_identities
from tools.api_contracts.official_evidence import (
    EvidenceDimension,
    EvidenceStatus,
    FieldObservation,
    FreshOfficialPolicy,
    PreferSnapshotPolicy,
    compose_full,
)
from tools.api_contracts.report import (
    evidence_markdown_lines,
    evidence_to_jsonable,
)
from tools.api_contracts.rust_contract_resolution import (
    Ambiguous,
    Missing,
    ResolutionConfigurationError,
    Resolved,
    Unmapped,
    compose as compose_contract_resolution,
)
from tools.api_contracts.rust_source import (
    MULTIPART_FORM_STRUCT_NAME,
    RESPONSE_STRUCT_SUFFIXES,
    extract_structs,
    matches_struct_suffix,
)

DEFAULT_CSV = REPO_ROOT / "api_list_export.csv"

# 请求体 struct 后缀：REQUEST_STRUCT_SUFFIXES 的 body 子集——Query/Params 是查询
# 参数而非请求体，混入会把「GET+query 无 body」的 API 误报 doc_parse_empty。
_BODY_STRUCT_SUFFIXES = ("Body", "RequestBody")
# 响应对比优先取 *Response（信封 struct）；仅当文件没有 *Response 时才回落
# *Result/*Resp/*ResponseData（如 baike 的 MatchEntityResp、auth 的
# UserAccessTokenV1ResponseData——后者是文件内唯一响应 payload struct，不兜底
# 会让 full 模式响应对比静默关闭）。嵌套 Result 先于 Response 定义时不能抢位，
# 否则文档顶层字段会对到嵌套 payload 上。
_RESPONSE_PRIMARY_SUFFIXES = ("Response",)
_RESPONSE_FALLBACK_SUFFIXES = RESPONSE_STRUCT_SUFFIXES


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
    api: ApiIdentity, structs: List[StructFields], source: str
) -> List[FieldIssue]:
    """检测不抓文档就能发现的字段问题（三类红旗）。

    红旗依据：
      1. 用户级接口 Body 含 user_id/approval_code（弱启发式，info 级——
         /reference/ 路径也含管理员级接口，需人工判断）
      2. 必填 Vec 字段缺非空校验（认 validate_required_list! 或 is_empty()）
      3. GET 查询接口 Response 为空（可能漏建响应字段）
    """
    issues: List[FieldIssue] = []

    # 收集所有请求体 struct 的字段
    body_structs = [
        s for s in structs if matches_struct_suffix(s.name, _BODY_STRUCT_SUFFIXES)
    ]

    # 红旗 1：用户级接口含 user_id / approval_code
    # 注意：is_user_level 基于 /reference/ 路径，但 reference 也含管理员级接口，
    # 故降为 info 级提示，detail 说明"若用 tenant_token 则可忽略"。
    if "/reference/" in api.full_path:
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
    if api.official_method == "GET":
        resp_structs = [
            s for s in structs if matches_struct_suffix(s.name, _RESPONSE_PRIMARY_SUFFIXES)
        ]
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

    api: ApiIdentity
    file_path: str
    file_exists: bool
    structs: List[StructFields]
    issues: List[FieldIssue]
    evidence: List[dict] = field(default_factory=list)
    resolution: str = "resolved"
    target: str = ""
    checked_candidates: Tuple[str, ...] = ()


class FieldResolutionBlocked(RuntimeError):
    """A target resolution cannot safely be consumed by field verification."""


def run_quick_mode(
    csv_path: Path,
    repository_root: Path,
    resolver,
    output_md: Optional[Path] = None,
    output_json: Optional[Path] = None,
    filter_tags: Optional[List[str]] = None,
) -> str:
    """快速模式：扫描代码字段 + 可疑模式检测，不抓文档。返回报告文本。"""
    apis = load_api_identities(csv_path, filter_tags)
    reports: List[ApiFieldReport] = []

    for api in apis:
        rel_path = api.expected_file
        resolution = resolver.resolve(api)
        if isinstance(resolution, Unmapped):
            raise FieldResolutionBlocked(
                f"Catalog Entry {api.api_id} ({api.biz_tag}) is Unmapped"
            )
        if isinstance(resolution, Ambiguous):
            raise FieldResolutionBlocked(_ambiguous_message(resolution))
        if isinstance(resolution, Missing):
            reports.append(
                ApiFieldReport(
                    api=api, file_path=rel_path, file_exists=False,
                    structs=[], issues=[],
                    resolution="missing",
                    checked_candidates=tuple(
                        sorted(path.as_posix() for path in resolution.checked_candidates)
                    ),
                )
            )
            continue
        full_path = repository_root / Path(resolution.target.repository_path.as_posix())
        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)
        reports.append(
            ApiFieldReport(
                api=api, file_path=rel_path, file_exists=True,
                structs=structs, issues=issues,
                target=resolution.target.repository_path.as_posix(),
            )
        )

    if not any(report.file_exists for report in reports):
        raise FieldResolutionBlocked("scan produced zero Resolved Rust Contract Targets")

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
            if r.checked_candidates:
                lines.append(
                    "  - checked: "
                    + ", ".join(f"`{path}`" for path in r.checked_candidates)
                )
        lines.append("")

    lines.extend(
        evidence_markdown_lines(
            [
                (report.api.name, dimension)
                for report in reports
                for dimension in report.evidence
            ]
        )
    )

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
                "resolution": r.resolution,
                "target": r.target,
                "checked_candidates": list(r.checked_candidates),
                "issues": [
                    {"severity": i.severity, "category": i.category, "detail": i.detail}
                    for i in r.issues
                ],
                "evidence": r.evidence,
            }
            for r in reports
        ],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")




@dataclass
class FieldDiff:
    """代码字段与文档字段的对比结果。"""

    matched: List[str] = field(default_factory=list)  # 两边都有
    missing: List[str] = field(default_factory=list)  # 文档有、代码无
    extra: List[str] = field(default_factory=list)  # 代码有、文档无
    required_mismatches: List[FieldIssue] = field(default_factory=list)
    type_mismatches: List[FieldIssue] = field(default_factory=list)


# 文档类型 → 可接受的 Rust 核心类型名（不含 Option/Vec 外壳）
_DOC_TYPE_TO_RUST: dict[str, set[str]] = {
    "string": {"String"},
    "int": {"i32", "i64", "u32", "u64", "isize", "usize"},
    "integer": {"i32", "i64", "u32", "u64", "isize", "usize"},
    "boolean": {"bool"},
    "bool": {"bool"},
    "number": {"f64", "f32", "i32", "i64"},
    "float": {"f32", "f64"},
    "double": {"f64"},
}

# 仅当代码侧是这些已知原始类型时才做类型对比；
# 自定义 enum/newtype（如 RecognitionModel）对文档 string/string[] 跳过，避免假绿反向的假警告。
_KNOWN_RUST_PRIMITIVES: frozenset[str] = frozenset().union(*_DOC_TYPE_TO_RUST.values())


def _doc_type_core(type_name: str) -> Optional[Tuple[str, bool]]:
    """解析文档类型为 (核心类型小写, 是否数组)。空类型返回 None（跳过对比）。"""
    raw = (type_name or "").strip()
    if not raw:
        return None
    is_array = raw.endswith("[]")
    core = raw[:-2].strip() if is_array else raw
    if not core:
        return None
    return core.lower(), is_array


def _rust_type_basename(type_name: str) -> str:
    """取 Rust 类型末段（忽略路径/模块前缀）。"""
    return type_name.rsplit("::", 1)[-1].strip()


def compare_fields(
    code_fields: List[FieldInfo], doc_fields: List[FieldInfo]
) -> FieldDiff:
    """对比代码字段与文档字段（名字 + 必填性 + 类型）。

    必填性：文档 Yes + 代码 Option → error；文档 No + 代码非 Option → info（更严建模，不阻断）。
    类型：仅当代码侧是已知原始类型时对比；不匹配 → warning。
    空文档类型 / 未建模文档类型 / 代码自定义 enum·newtype → 跳过类型对比。
    """
    code_by_name = {f.effective_name: f for f in code_fields}
    doc_by_name = {f.effective_name: f for f in doc_fields}
    code_names = set(code_by_name)
    doc_names = set(doc_by_name)
    matched = sorted(code_names & doc_names)
    required_mismatches: List[FieldIssue] = []
    type_mismatches: List[FieldIssue] = []

    for name in matched:
        code_f = code_by_name[name]
        doc_f = doc_by_name[name]

        if doc_f.required is True and code_f.required is False:
            required_mismatches.append(
                FieldIssue(
                    severity="error",
                    category="required_mismatch",
                    detail=(
                        f"字段 {name} 文档必填(Yes) 但代码为 Option"
                    ),
                )
            )
        elif doc_f.required is False and code_f.required is True:
            # 代码比文档更严是常见有意建模（如 OCR image），不阻断门禁
            required_mismatches.append(
                FieldIssue(
                    severity="info",
                    category="required_mismatch",
                    detail=(
                        f"字段 {name} 文档选填(No) 但代码非 Option（代码更严，可能有意）"
                    ),
                )
            )

        parsed = _doc_type_core(doc_f.type_name)
        if parsed is None:
            continue
        doc_core, doc_is_array = parsed
        accepted = _DOC_TYPE_TO_RUST.get(doc_core)
        if accepted is None:
            # object/file/自定义等未建模类型：跳过，避免误报
            continue
        code_core = _rust_type_basename(code_f.type_name)
        if code_core not in _KNOWN_RUST_PRIMITIVES:
            # 代码侧是自定义 enum/newtype 等：跳过类型对比
            # （文档 string ↔ serde 枚举是合法建模，不能假警告阻断门禁）
            continue
        array_mismatch = doc_is_array != code_f.is_array
        core_mismatch = code_core not in accepted
        if array_mismatch or core_mismatch:
            doc_display = f"{doc_core}[]" if doc_is_array else doc_core
            code_display = (
                f"Vec<{code_f.type_name}>" if code_f.is_array else code_f.type_name
            )
            type_mismatches.append(
                FieldIssue(
                    severity="warning",
                    category="type_mismatch",
                    detail=(
                        f"字段 {name} 类型不一致：文档 {doc_display} vs 代码 {code_display}"
                    ),
                )
            )

    return FieldDiff(
        matched=matched,
        missing=sorted(doc_names - code_names),
        extra=sorted(code_names - doc_names),
        required_mismatches=required_mismatches,
        type_mismatches=type_mismatches,
    )


# ---------------------------------------------------------------------------
# CLI 入口
# ---------------------------------------------------------------------------


def _exit_code_for_issues(issues: List[FieldIssue]) -> int:
    """完整模式门禁：error/warning 导致非 0；info 不阻断。"""
    for i in issues:
        if i.severity in ("error", "warning"):
            return 1
    return 0


def _compare_evidence_against_code(
    structs: List[StructFields],
    evidence,
    issues: List[FieldIssue],
) -> None:
    """消费顶层 Field Observations，保留既有 Rust comparison 语义。"""
    request = evidence.for_dimension(EvidenceDimension.REQUEST_FIELDS)
    response = evidence.for_dimension(EvidenceDimension.RESPONSE_FIELDS)
    nonpassing = [
        item
        for item in (request, response)
        if item.status is not EvidenceStatus.TRUSTED
    ]
    if nonpassing:
        hard_failure = any(
            item.status in (EvidenceStatus.UNAVAILABLE, EvidenceStatus.REJECTED)
            for item in nonpassing
        )
        diagnostics = ", ".join(
            diagnostic.code
            for item in nonpassing
            for diagnostic in item.diagnostics
        )
        issues.append(
            FieldIssue(
                "error" if hard_failure else "warning",
                "doc_fetch_failed" if hard_failure else "doc_parse_empty",
                "官方文档证据未通过 Strict Evidence Gate"
                f"（{diagnostics or '无 diagnostic'}）",
            )
        )

    if request.status is EvidenceStatus.TRUSTED:
        doc_req = [
            FieldInfo(
                name=item.path[0],
                type_name=item.field_type or "",
                required=item.required,
                is_array=(item.field_type or "").rstrip().endswith("[]"),
            )
            for item in request.observations
            if isinstance(item, FieldObservation)
            and len(item.path) == 1
            and item.location == "request_body"
        ]
        # 请求体 = 首个 body 后缀 struct 的字段 + multipart 合成分组的表单字段
        body_fields = next(
            (
                item.fields
                for item in structs
                if matches_struct_suffix(item.name, _BODY_STRUCT_SUFFIXES)
            ),
            [],
        )
        multipart_fields = next(
            (item.fields for item in structs if item.name == MULTIPART_FORM_STRUCT_NAME),
            [],
        )
        # Body struct（强类型）在前：compare_fields 按名字建 dict 后写覆盖先写，
        # 同名字段以 Body struct 的 required/类型语义为准，multipart 通道不静默盖掉
        code_body = list(multipart_fields) + list(body_fields)
        if not doc_req and code_body:
            issues.append(
                FieldIssue(
                    "warning",
                    "doc_parse_empty",
                    "Trusted Evidence 未观察到请求字段，但 Rust 存在请求体字段",
                )
            )
        elif doc_req and code_body:
            diff = compare_fields(code_body, doc_req)
            if diff.missing:
                issues.append(
                    FieldIssue(
                        "error",
                        "missing_field",
                        f"请求体缺字段: {', '.join(diff.missing)}",
                    )
                )
            if diff.extra:
                issues.append(
                    FieldIssue(
                        "warning",
                        "extra_field",
                        f"请求体多余字段: {', '.join(diff.extra)}",
                    )
                )
            issues.extend(diff.required_mismatches)
            issues.extend(diff.type_mismatches)

    if response.status is EvidenceStatus.TRUSTED:
        doc_response_names = {
            item.path[0]
            for item in response.observations
            if isinstance(item, FieldObservation) and len(item.path) == 1
        }
        code_response = next(
            (
                item.fields
                for item in structs
                if matches_struct_suffix(item.name, _RESPONSE_PRIMARY_SUFFIXES)
            ),
            next(
                (
                    item.fields
                    for item in structs
                    if matches_struct_suffix(item.name, _RESPONSE_FALLBACK_SUFFIXES)
                ),
                [],
            ),
        )
        if doc_response_names and code_response:
            code_names = {item.effective_name for item in code_response}
            missing = sorted(doc_response_names - code_names)
            if missing:
                issues.append(
                    FieldIssue(
                        "info",
                        "missing_response_field",
                        f"响应体可能缺字段: {', '.join(missing)}",
                    )
                )


def _collect_field_evidence(collector, api: ApiIdentity, policy):
    return collector.collect(
        api,
        (
            EvidenceDimension.REQUEST_FIELDS,
            EvidenceDimension.RESPONSE_FIELDS,
        ),
        policy,
    )


def _resolve_evidence_policy(
    *,
    force_refresh: bool,
    max_age_days: int,
    single_api: bool,
):
    """选择 Official Evidence 抓取策略。

    - --force-refresh：始终 FreshOfficialPolicy
    - 单 API + --fetch-docs：默认 FreshOfficialPolicy（门禁应对齐官网）
    - 批量完整模式：PreferSnapshotPolicy(max_age_days=...)
    """
    if force_refresh or single_api:
        return FreshOfficialPolicy()
    return PreferSnapshotPolicy(max_age_days=max_age_days)


def main(repository_root: Path | None = None) -> int:
    parser = argparse.ArgumentParser(description="API 字段核对工具")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="API 清单 CSV 路径")
    parser.add_argument("--crate", help="指定单个 crate（如 openlark-workflow）")
    parser.add_argument(
        "--fetch-docs",
        action="store_true",
        help=(
            "完整模式：抓飞书文档对比（慢）。"
            "批量模式默认复用未超龄快照；单 API 模式默认重抓"
        ),
    )
    parser.add_argument(
        "--force-refresh",
        action="store_true",
        help="忽略本地 Official Evidence 快照，强制重新抓取文档",
    )
    parser.add_argument(
        "--max-age",
        type=int,
        default=30,
        metavar="DAYS",
        help="批量 --fetch-docs 时快照最大年龄（天），超时重抓；默认 30",
    )
    parser.add_argument("--output-dir", default="reports/api_field_verify", help="报告输出目录")
    parser.add_argument("--api-id", help="只核对单个 API（调试用，按 CSV id 过滤）")
    args = parser.parse_args()
    root = Path(repository_root) if repository_root is not None else REPO_ROOT

    if args.max_age < 0:
        print("❌ --max-age 必须是非负整数")
        return 1

    csv_path = Path(args.csv)
    out_dir = Path(args.output_dir)
    evidence_policy = _resolve_evidence_policy(
        force_refresh=args.force_refresh,
        max_age_days=args.max_age,
        single_api=bool(args.api_id),
    )
    try:
        resolver = compose_contract_resolution(repository_root=root)
    except ResolutionConfigurationError as exc:
        print(f"❌ {exc}")
        return 1

    # 单 API 模式：按 id 过滤，目标只通过 Rust Contract Resolution 获取。
    if args.api_id:
        crate_label = f"api-{args.api_id}"
        all_apis = load_api_identities(csv_path)
        if args.fetch_docs:
            with compose_full(
                snapshot_directory=out_dir / "official_evidence",
                timeout_seconds=90,
                retries=1,
            ) as collector:
                return _run_single_api(
                    args.api_id,
                    all_apis,
                    root,
                    resolver,
                    out_dir,
                    crate_label,
                    True,
                    collector,
                    evidence_policy,
                )
        return _run_single_api(
            args.api_id,
            all_apis,
            root,
            resolver,
            out_dir,
            crate_label,
            False,
            None,
            evidence_policy,
        )

    # 确定 src 根目录和 bizTag 过滤
    if args.crate:
        source_display = root / "crates" / args.crate / "src"
        filter_tags = _load_crate_tags(args.crate, root)
        if not filter_tags:
            print(f"❌ api_coverage.toml 中找不到 crate: {args.crate}")
            return 1
        crate_label = args.crate
    else:
        source_display = root / "crates"
        filter_tags = None
        crate_label = "all"

    print(f"📂 CSV: {csv_path}")
    print(f"📁 源码根: {source_display}")
    print(f"🏷️  过滤 bizTag: {filter_tags or '(全部)'}")

    if args.fetch_docs:
        print("🐌 完整模式（通过 Official Document Evidence 收集）")
        if isinstance(evidence_policy, FreshOfficialPolicy):
            print("🔄 文档策略: FreshOfficialPolicy（强制重抓）")
        else:
            print(
                f"💾 文档策略: PreferSnapshotPolicy"
                f"（max_age={args.max_age} 天）"
            )
        with compose_full(
            snapshot_directory=out_dir / "official_evidence",
            timeout_seconds=90,
            retries=1,
        ) as collector:
            return _run_full_mode(
                csv_path,
                root,
                resolver,
                out_dir,
                crate_label,
                filter_tags,
                collector,
                evidence_policy,
            )

    # 快速模式
    print("⚡ 快速模式（代码自检）")
    try:
        run_quick_mode(
            csv_path=csv_path,
            repository_root=root,
            resolver=resolver,
            output_md=out_dir / f"{crate_label}.md",
            output_json=out_dir / "summary.json",
            filter_tags=filter_tags,
        )
    except FieldResolutionBlocked as exc:
        print(f"❌ Rust contract resolution blocked: {exc}")
        return 1
    print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    return 0


def _run_single_api(
    api_id,
    all_apis,
    repository_root,
    resolver,
    out_dir,
    crate_label,
    fetch_docs,
    collector,
    evidence_policy=None,
) -> int:
    """核对单个 API。--fetch-docs 时抓取失败或 error/warning 返回非 0。"""
    api = next((a for a in all_apis if a.api_id == api_id), None)
    if api is None:
        print(f"❌ CSV 中找不到 id={api_id} 的 API")
        return 1
    print(f"🔍 单 API 核对: {api.name} ({api.url})")
    rel_path = api.expected_file
    resolution = resolver.resolve(api)
    if isinstance(resolution, Missing):
        checked = ", ".join(
            sorted(path.as_posix() for path in resolution.checked_candidates)
        )
        print(f"❌ Rust Contract Target Missing: {checked}")
        return 1
    if isinstance(resolution, Unmapped):
        print(f"❌ Rust Contract Target Unmapped: bizTag={api.biz_tag}")
        return 1
    if isinstance(resolution, Ambiguous):
        print(f"❌ {_ambiguous_message(resolution)}")
        return 1
    full_path = repository_root / Path(resolution.target.repository_path.as_posix())
    source = full_path.read_text(encoding="utf-8")
    structs = extract_structs(source)
    issues = detect_suspicious_patterns(api, structs, source)

    mode = "quick"
    evidence_metadata = []
    if fetch_docs:
        mode = "full"
        policy = evidence_policy or FreshOfficialPolicy()
        evidence = _collect_field_evidence(collector, api, policy)
        evidence_metadata = evidence_to_jsonable(evidence)["dimensions"]
        _compare_evidence_against_code(structs, evidence, issues)

    report = ApiFieldReport(
        api=api,
        file_path=rel_path,
        file_exists=True,
        structs=structs,
        issues=issues,
        evidence=evidence_metadata,
        target=resolution.target.repository_path.as_posix(),
    )
    md = _render_report([report], mode=mode)
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / f"{crate_label}.md").write_text(md, encoding="utf-8")
    _write_summary_json([report], out_dir / "summary.json", mode=mode)
    if issues:
        print(f"⚠️ 发现 {len(issues)} 个问题:")
        for i in issues:
            print(f"  [{i.severity}] {i.detail}")
    else:
        print("✅ 字段核对通过" if fetch_docs else "✅ 无可疑模式")
    print(f"📄 报告: {out_dir / f'{crate_label}.md'}")
    return _exit_code_for_issues(issues) if fetch_docs else 0




def _load_crate_tags(crate: str, repository_root: Path = REPO_ROOT) -> Optional[List[str]]:
    """从 tools/api_coverage.toml 读 crate 的 biz_tags。"""
    import tomllib  # Python 3.11+

    toml_path = repository_root / "tools" / "api_coverage.toml"
    if not toml_path.exists():
        return None
    with open(toml_path, "rb") as f:
        data = tomllib.load(f)
    crate_cfg = data.get("crates", {}).get(crate, {})
    return crate_cfg.get("biz_tags")


def _run_full_mode(
    csv_path,
    repository_root,
    resolver,
    out_dir,
    crate_label,
    filter_tags,
    collector,
    evidence_policy=None,
) -> int:
    """完整模式：复用同一个 collect 行为核对所有字段。"""
    import json

    apis = load_api_identities(csv_path, filter_tags)
    reports: List[ApiFieldReport] = []
    failed: List[Tuple[str, str]] = []
    policy = evidence_policy or PreferSnapshotPolicy(max_age_days=30)
    resolved_count = 0

    for idx, api in enumerate(apis, 1):
        rel_path = api.expected_file
        resolution = resolver.resolve(api)
        if isinstance(resolution, Unmapped):
            print(
                f"❌ Rust contract resolution blocked: Catalog Entry "
                f"{api.api_id} ({api.biz_tag}) is Unmapped"
            )
            return 1
        if isinstance(resolution, Ambiguous):
            print(f"❌ Rust contract resolution blocked: {_ambiguous_message(resolution)}")
            return 1
        if isinstance(resolution, Missing):
            reports.append(
                ApiFieldReport(
                    api=api,
                    file_path=rel_path,
                    file_exists=False,
                    structs=[],
                    issues=[],
                    resolution="missing",
                    checked_candidates=tuple(
                        sorted(path.as_posix() for path in resolution.checked_candidates)
                    ),
                )
            )
            continue
        resolved_count += 1
        full_path = repository_root / Path(resolution.target.repository_path.as_posix())

        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)
        evidence = _collect_field_evidence(collector, api, policy)
        dimensions = evidence_to_jsonable(evidence)["dimensions"]
        _compare_evidence_against_code(structs, evidence, issues)
        nonpassing = [
            item
            for item in evidence.dimensions
            if item.status is not EvidenceStatus.TRUSTED
        ]
        if nonpassing:
            diagnostics = ", ".join(
                diagnostic.code
                for item in nonpassing
                for diagnostic in item.diagnostics
            )
            failed.append((api.api_id, diagnostics))

        reports.append(
            ApiFieldReport(
                api=api,
                file_path=rel_path,
                file_exists=True,
                structs=structs,
                issues=issues,
                evidence=dimensions,
                target=resolution.target.repository_path.as_posix(),
            )
        )
        print(f"⏳ [{idx}/{len(apis)}] {api.name} ({len(issues)} 问题)")

    md = _render_report(reports, mode="full")
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / f"{crate_label}.md").write_text(md, encoding="utf-8")
    _write_summary_json(reports, out_dir / "summary.json", mode="full")

    if resolved_count == 0:
        print("❌ Rust contract resolution produced zero Resolved targets")
        return 1

    if failed:
        print(f"⚠️ {len(failed)} 个官方文档证据未通过，详见 failed.json")
        (out_dir / "failed.json").write_text(
            json.dumps(failed, ensure_ascii=False, indent=2), encoding="utf-8"
        )

    all_issues = [issue for report in reports for issue in report.issues]
    exit_code = _exit_code_for_issues(all_issues)
    if exit_code == 0:
        print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    else:
        print(
            f"❌ 核对未通过（error/warning），报告: "
            f"{out_dir / f'{crate_label}.md'}"
        )
    return exit_code


def _ambiguous_message(resolution: Ambiguous) -> str:
    candidates = ", ".join(
        sorted(target.repository_path.as_posix() for target in resolution.candidates)
    )
    return f"Catalog Entry {resolution.entry.api_id} is Ambiguous: {candidates}"


if __name__ == "__main__":
    raise SystemExit(main())
