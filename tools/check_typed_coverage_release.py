#!/usr/bin/env python3
"""根据仓库策略判定稳定版 typed API coverage 发布门禁。"""

from __future__ import annotations

import argparse
import json
from collections import namedtuple
from pathlib import Path
from typing import Any, Dict

import tomllib


GateResult = namedtuple("GateResult", ["status", "failures", "warnings"])


def evaluate_release_gate(
    policy: Dict[str, Any], summary: Dict[str, Any], core_dashboard: Dict[str, Any]
) -> GateResult:
    stable_release = policy["stable_release"]
    hard_gates = stable_release["hard_gates"]
    failures = []

    summary_rate = float(summary.get("completion_rate", 0.0))
    summary_min = float(hard_gates["summary_completion_rate_min"])
    if summary_rate < summary_min:
        failures.append(f"全仓完成率 {summary_rate:.1f}% < {summary_min:.1f}%")

    core_rate = float(core_dashboard.get("completion_rate", 0.0))
    core_min = float(hard_gates["core_business_completion_rate_min"])
    if core_rate < core_min:
        failures.append(f"核心业务完成率 {core_rate:.1f}% < {core_min:.1f}%")

    crate_min = float(hard_gates["core_crate_completion_rate_min"])
    for crate in core_dashboard.get("crates", []):
        crate_rate = float(crate.get("completion_rate", 0.0))
        if crate_rate < crate_min:
            failures.append(
                f"{crate.get('crate', '<unknown>')} 完成率 {crate_rate:.1f}% < {crate_min:.1f}%"
            )

    p0_missing = int(core_dashboard.get("priority_counts", {}).get("P0", 0))
    warnings = []
    if p0_missing > 0:
        warnings.append(f"核心业务仍有 {p0_missing} 个 P0 缺口")

    if failures:
        return GateResult("BLOCKED", failures, warnings)

    waiver_rule = stable_release.get("waiver_gates", {}).get("core_business_p0_missing")
    if p0_missing > 0 and waiver_rule == "waiver_required":
        return GateResult("WAIVER REQUIRED", [], warnings)

    return GateResult("PASS", [], warnings)


def render_markdown(result: GateResult) -> str:
    lines = ["## Typed API Coverage Release Gate", "", f"**Status: {result.status}**", ""]
    if result.failures:
        lines.extend(["### Hard gate failures", ""])
        lines.extend(f"- {failure}" for failure in result.failures)
        lines.append("")
    if result.warnings:
        lines.extend(["### Waiver signals", ""])
        lines.extend(f"- {warning}" for warning in result.warnings)
        lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--policy", default="tools/typed_coverage_release.toml")
    parser.add_argument("--summary", default="reports/api_validation/summary.json")
    parser.add_argument(
        "--core-dashboard",
        default="reports/api_validation/dashboards/core_business.json",
    )
    parser.add_argument("--output", default="")
    args = parser.parse_args()

    policy = tomllib.loads(Path(args.policy).read_text(encoding="utf-8"))
    summary = json.loads(Path(args.summary).read_text(encoding="utf-8"))
    dashboard = json.loads(Path(args.core_dashboard).read_text(encoding="utf-8"))
    result = evaluate_release_gate(policy, summary, dashboard)
    markdown = render_markdown(result)
    print(markdown)

    if args.output:
        output_path = Path(args.output)
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(markdown + "\n", encoding="utf-8")

    return 0 if result.status == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
