#!/usr/bin/env python3
"""Evidence 健康绊线：防止官方 schema 格式变更静默致盲字段门禁（PR #750 教训）。

背景：官方 get_detail 的 apiSchema 若发生结构性变更（如 2026-10 的 properties
dict → list），解析器会把所有 evidence 降级 structure_incomplete → 非 TRUSTED
即跳过比对 → 门禁「0 error」假绿，且没有任何报警。本工具对 live 校验产物里
request_fields 维度的 evidence 状态做池化统计，trusted 率低于阈值即失败——
上游格式变更当天报警，而不是数月后被交叉审计揭穿。

用法（CI weekly；report-dir 为 validate_api_contracts --fields --live-fields 的产物）:
    python3 tools/validate_api_contracts.py --crate openlark-ai --fields \
        --live-fields --max-field-apis 40 --report-dir reports/evidence_health
    python3 tools/check_evidence_health.py --report-dir reports/evidence_health

正常基线（2026-10-26 全仓实测）：ai 100%、platform 58%，池化 ≈73%；
致盲模式（格式变更）下 trusted → ~0%。阈值取 40% + 最少 30 条 evidence，
对正常文档波动留足余量、对致盲零容忍。
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

DEFAULT_TRUSTED_FLOOR = 0.40
DEFAULT_MIN_EVIDENCE = 30


def collect_request_fields_status(report_dir: Path) -> list[str]:
    """聚合 report_dir 下各 crate 报告里 request_fields 维度的 evidence 状态。"""
    statuses: list[str] = []
    for path in sorted(report_dir.glob("crates/*.json")):
        payload = json.loads(path.read_text(encoding="utf-8"))
        for entry in payload.get("evidence", []):
            for dimension in entry.get("dimensions", []):
                if dimension.get("dimension") == "request_fields":
                    statuses.append(str(dimension.get("status", "unknown")))
    return statuses


def evaluate(
    statuses: list[str],
    *,
    trusted_floor: float,
    min_evidence: int,
) -> tuple[bool, str]:
    """返回 (通过?, 人可读结论)。"""
    total = len(statuses)
    trusted = sum(1 for status in statuses if status == "trusted")
    if total < min_evidence:
        return False, (
            f"evidence 样本不足：{total} < {min_evidence}——"
            "live 校验可能整体失败（抓取/网络），门禁未被证明健康"
        )
    rate = trusted / total
    if rate < trusted_floor:
        return False, (
            f"字段门禁疑似被上游格式变更致盲：request_fields trusted 率 "
            f"{rate:.0%}（{trusted}/{total}）< 阈值 {trusted_floor:.0%}。"
            "请核对 open.feishu.cn get_detail 的 apiSchema 结构与 "
            "tools/api_contracts/official_evidence 解析器（先例：2026-10 "
            "properties dict→list，PR #750）"
        )
    return True, f"evidence 健康：request_fields trusted {rate:.0%}（{trusted}/{total}）"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report-dir",
        type=Path,
        required=True,
        help="validate_api_contracts --fields --live-fields 的 --report-dir",
    )
    parser.add_argument(
        "--trusted-floor",
        type=float,
        default=DEFAULT_TRUSTED_FLOOR,
        help=f"池化 trusted 率下限（默认 {DEFAULT_TRUSTED_FLOOR}）",
    )
    parser.add_argument(
        "--min-evidence",
        type=int,
        default=DEFAULT_MIN_EVIDENCE,
        help=f"最少 evidence 条数（默认 {DEFAULT_MIN_EVIDENCE}）",
    )
    args = parser.parse_args(argv)

    statuses = collect_request_fields_status(args.report_dir)
    ok, message = evaluate(
        statuses,
        trusted_floor=args.trusted_floor,
        min_evidence=args.min_evidence,
    )
    print(("✅ " if ok else "❌ ") + message)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
