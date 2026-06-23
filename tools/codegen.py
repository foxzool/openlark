#!/usr/bin/env python3
"""飞书 API → Rust 代码生成器（风格 A）CLI。

把 api_list_export.csv 里的 API 经 apiSchema → Codegen IR → 渲染成风格 A Rust 文件，
落盘到 openlark-communication。生成后可自动跑 fmt + clippy 闭环校验。

复用：
- official.load_api_identities（CSV 真理源 + expected_file 路径公式）
- schema_cache.cache.get_or_fetch（带缓存的取数）
- codegen_ir.parse_api_schema_to_ir + codegen_render.render_api_file

安全：--write 只覆盖带「由 codegen 自动生成」标记或新文件，绝不覆盖手工精写文件。
半自动：mod.rs 与 endpoints 常量打印 [MANUAL] 提示（不自动改，避免破坏现有导出）。
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.codegen_ir import parse_api_schema_to_ir  # noqa: E402
from tools.api_contracts.codegen_render import (  # noqa: E402
    CODEGEN_MARKER,
    render_api_file,
    render_endpoint_const_snippet,
)
from tools.api_contracts.official import load_api_identities  # noqa: E402
from tools.schema_cache.cache import DEFAULT_CACHE_DIR, get_or_fetch, record_error  # noqa: E402

MVP_CRATE = "openlark-communication"
CRATE_SRC = REPO_ROOT / "crates" / MVP_CRATE / "src"


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="飞书 API → Rust 代码生成器（风格 A）")
    sel = p.add_mutually_exclusive_group(required=True)
    sel.add_argument("--api-id", help="单个 API（CSV id）")
    sel.add_argument("--tag", help="单个 bizTag（如 im）")
    sel.add_argument("--crate", help="整个 crate（api_coverage.toml 映射的 biz_tags）")
    p.add_argument("--write", action="store_true", help="写入文件（默认 dry-run 打印）")
    p.add_argument("--refresh", action="store_true", help="强制重新拉取 schema（忽略缓存）")
    p.add_argument("--no-validate", action="store_true", help="跳过 fmt+clippy 闭环")
    p.add_argument("--csv", default="api_list_export.csv")
    p.add_argument("--timeout", type=int, default=30)
    p.add_argument("--retries", type=int, default=2)
    return p.parse_args()


def load_biz_tags(crate_name: str) -> list[str]:
    import tomllib

    data = tomllib.loads((REPO_ROOT / "tools" / "api_coverage.toml").read_text(encoding="utf-8"))
    return list(data.get("crates", {}).get(crate_name, {}).get("biz_tags") or [])


def is_codegen_file(path: Path) -> bool:
    """True = 可安全覆盖（新文件或带 codegen 标记）；False = 手工文件，勿覆盖。"""
    if not path.exists():
        return True
    for line in path.read_text(encoding="utf-8").splitlines()[:6]:
        if CODEGEN_MARKER in line:
            return True
    return False


def endpoint_const_exists(const_name: str) -> bool:
    pat = re.compile(rf"pub\s+const\s+{re.escape(const_name)}\s*:")
    for p in (CRATE_SRC / "endpoints").glob("**/*.rs"):
        if pat.search(p.read_text(encoding="utf-8")):
            return True
    return False


def main() -> int:
    args = parse_args()
    csv_path = REPO_ROOT / args.csv

    if args.crate or args.tag:
        if args.crate and args.crate != MVP_CRATE:
            print(f"[WARN] MVP 仅支持 {MVP_CRATE}，忽略 --crate {args.crate}")
        tags = [args.tag] if args.tag else load_biz_tags(MVP_CRATE)
        apis = load_api_identities(csv_path, filter_tags=tags, skip_old_versions=True)
    else:
        apis = [
            a for a in load_api_identities(csv_path, skip_old_versions=True)
            if a.api_id == args.api_id
        ]
        if not apis:
            print(f"[ERROR] api_id {args.api_id} 不在 CSV")
            return 1

    print(f"[INFO] 待生成 {len(apis)} 个 API（crate={MVP_CRATE}）")
    stats = {"generated": 0, "skipped": 0, "errors": 0}
    manual_consts: list[str] = []
    manual_mods: list[str] = []

    for api in apis:
        try:
            cache = get_or_fetch(
                api, refresh=args.refresh, timeout=args.timeout, retries=args.retries
            )
            ir = parse_api_schema_to_ir(api, cache.api_schema)
            content = render_api_file(ir)
            target = CRATE_SRC / api.expected_file
            rel = target.relative_to(REPO_ROOT)

            if args.write:
                if target.exists() and not is_codegen_file(target):
                    print(f"[SKIP ] {rel}（手工文件，不覆盖）")
                    stats["skipped"] += 1
                    continue
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(content, encoding="utf-8")
                stats["generated"] += 1
                print(f"[WRITE] {rel}")
                # 端点常量提示
                if not endpoint_const_exists(ir.endpoint_const_name):
                    snippet = render_endpoint_const_snippet(ir)
                    manual_consts.append(snippet)
                    print(f"  [MANUAL] endpoints/{api.biz_tag}.rs 追加: {snippet}")
                # mod.rs 提示（直接父目录）
                mod_rel = target.parent.relative_to(REPO_ROOT)
                mod_line = f"pub mod {target.stem};"
                manual_mods.append(f"{mod_rel}/mod.rs: {mod_line}")
            else:
                print(f"\n{'='*70}\n=== {rel}\n{'='*70}")
                print(content)
        except Exception as exc:  # noqa: BLE001
            record_error(DEFAULT_CACHE_DIR, api, exc)
            stats["errors"] += 1
            print(f"[ERROR] {api.api_id} {api.name}: {exc}")

    if args.write:
        if manual_mods:
            print("\n[MANUAL] 需在 mod.rs 追加（增量，勿重写）：")
            seen = set()
            for m in manual_mods:
                if m not in seen:
                    seen.add(m)
                    print(f"  {m}")
        rc = 0
        if stats["generated"] and not args.no_validate:
            rc = run_closed_loop()
        print(
            f"\n[DONE] 生成 {stats['generated']}，跳过 {stats['skipped']}，"
            f"错误 {stats['errors']}"
        )
        return rc if stats["generated"] else (1 if stats["errors"] else 0)
    return 0


def run_closed_loop() -> int:
    """生成后跑 fmt + clippy，验证可编译无 warning。"""
    rc = 0
    commands = [
        ["cargo", "fmt", "--all"],
        [
            "cargo", "clippy", "-p", MVP_CRATE, "--all-targets", "--all-features",
            "--", "-Dwarnings", "-A", "missing_docs",
        ],
    ]
    for cmd in commands:
        print(f"[VALIDATE] {' '.join(cmd)}")
        result = subprocess.run(cmd, cwd=REPO_ROOT)
        if result.returncode != 0:
            print(f"[FAIL] {' '.join(cmd)} (exit {result.returncode})")
            rc = result.returncode
    return rc


if __name__ == "__main__":
    sys.exit(main())
