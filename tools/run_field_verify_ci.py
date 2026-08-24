#!/usr/bin/env python3
"""CI 入口：按固定 coverage 清单逐 crate 核对并合并报告（#638）。

Rust Contract Resolution 已让全仓扫描安全可用；CI 仍按 crate 调度，以保留
既有产物布局、失败隔离和进度粒度。本入口：

- 显式 `--crate`：`--output-dir` 就是产物根（`summary.json` + `<crate>.md`）
- 全仓：按固定 coverage 表逐 crate 调用，子目录再合并；清单为空则非零退出
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tomllib
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_VERIFY = REPO_ROOT / "tools" / "verify_api_fields.py"


def coverage_crates(repository_root: Path) -> list[str]:
    mapping = repository_root / "tools" / "api_coverage.toml"
    data = tomllib.loads(mapping.read_text(encoding="utf-8"))
    return sorted(data.get("crates", {}))


def require_coverage_crates(repository_root: Path) -> list[str]:
    crates = coverage_crates(repository_root)
    if not crates:
        print("coverage crate list is empty", file=sys.stderr)
        raise SystemExit(1)
    return crates


def _verify_command(crate: str, output_dir: Path, extra: list[str]) -> list[str]:
    injected = os.environ.get("FIELD_VERIFY_BIN")
    if injected:
        cmd = (
            [sys.executable, injected]
            if Path(injected).suffix == ".py"
            else [injected]
        )
    else:
        cmd = [sys.executable, str(DEFAULT_VERIFY)]
    cmd.extend(["--crate", crate, "--output-dir", str(output_dir), *extra])
    return cmd


def run_verify(crate: str, output_dir: Path, extra: list[str]) -> int:
    output_dir.mkdir(parents=True, exist_ok=True)
    cmd = _verify_command(crate, output_dir, extra)
    print("Running:", " ".join(cmd), flush=True)
    return subprocess.call(cmd)


def merge_summaries(root: Path, *, mode: str) -> dict:
    paths = sorted(root.glob("*/summary.json"))
    if not paths:
        print("missing per-crate summary.json after scan", file=sys.stderr)
        raise SystemExit(1)
    apis: list[dict] = []
    for path in paths:
        apis.extend(json.loads(path.read_text(encoding="utf-8")).get("apis", []))
    summary = {
        "mode": mode,
        "total_apis": len(apis),
        "apis_with_issues": sum(1 for api in apis if api.get("issues")),
        "apis": apis,
    }
    (root / "summary.json").write_text(
        json.dumps(summary, ensure_ascii=False, indent=2),
        encoding="utf-8",
    )
    return summary


def cmd_list(args: argparse.Namespace) -> int:
    for name in require_coverage_crates(args.repository_root):
        print(name)
    return 0


def cmd_quick(args: argparse.Namespace) -> int:
    crates = require_coverage_crates(args.repository_root)
    status = 0
    for crate in crates:
        crate_status = run_verify(crate, args.output_dir / crate, [])
        if crate_status != 0:
            status = crate_status
    merge_summaries(args.output_dir, mode="quick")
    return status


def cmd_full(args: argparse.Namespace) -> int:
    extra = ["--fetch-docs"]
    if args.max_age is not None:
        extra.extend(["--max-age", args.max_age])
    if args.force_refresh:
        extra.append("--force-refresh")

    if args.crate:
        # 与 main 上单 crate 调用相同：产物在 output-dir 根。
        return run_verify(args.crate, args.output_dir, extra)

    crates = require_coverage_crates(args.repository_root)
    status = 0
    for crate in crates:
        crate_status = run_verify(crate, args.output_dir / crate, extra)
        if crate_status != 0:
            status = crate_status
    merge_summaries(args.output_dir, mode="full")
    return status


def main(
    argv: list[str] | None = None,
    repository_root: Path | None = None,
) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    sub.add_parser("list", help="print coverage crates; empty list exits 1")

    quick = sub.add_parser("quick", help="weekly quick: per-crate --crate + merge")
    quick.add_argument("--output-dir", type=Path, required=True)

    full = sub.add_parser("full", help="full-mode: explicit crate or all-crates loop")
    full.add_argument("--output-dir", type=Path, required=True)
    full.add_argument("--crate", default="", help="single crate; omit = all coverage crates")
    full.add_argument("--max-age", default=None)
    full.add_argument("--force-refresh", action="store_true")

    args = parser.parse_args(argv)
    args.repository_root = (
        Path(repository_root) if repository_root is not None else REPO_ROOT
    )
    if args.command == "list":
        return cmd_list(args)
    if args.command == "quick":
        return cmd_quick(args)
    if args.command == "full":
        return cmd_full(args)
    parser.error(f"unknown command {args.command}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
