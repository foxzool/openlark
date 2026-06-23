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
from tools.api_contracts.mod_tree import ensure_mod_chain  # noqa: E402
from tools.api_contracts.official import load_api_identities  # noqa: E402
from tools.schema_cache.cache import DEFAULT_CACHE_DIR, get_or_fetch, record_error  # noqa: E402

DEFAULT_CRATE = "openlark-communication"


def resolve_crate(name: str) -> tuple[str, Path]:
    """从 api_coverage.toml [crates.<name>].src 读 src，返回 (crate_name, crate_src)。"""
    import tomllib

    data = tomllib.loads((REPO_ROOT / "tools" / "api_coverage.toml").read_text(encoding="utf-8"))
    entry = data.get("crates", {}).get(name)
    if not entry or "src" not in entry:
        raise SystemExit(f"[ERROR] api_coverage.toml 无 [crates.{name}] 或缺 src 字段")
    return name, REPO_ROOT / entry["src"]


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="飞书 API → Rust 代码生成器（风格 A）")
    p.add_argument("--api-id", help="单个 API（CSV id）")
    p.add_argument("--tag", help="单个 bizTag（如 im），配合 --crate 过滤")
    p.add_argument("--crate", help="目标 crate（默认 communication；全量该 crate 的 biz_tags）")
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


def endpoint_const_exists(crate_src: Path, const_name: str) -> bool:
    ep_dir = crate_src / "endpoints"
    if not ep_dir.exists():
        return False  # 无 endpoints/ 目录（docs/hr/auth 等）→ 视为不存在
    pat = re.compile(rf"pub\s+const\s+{re.escape(const_name)}\s*:")
    for p in ep_dir.glob("**/*.rs"):
        if pat.search(p.read_text(encoding="utf-8")):
            return True
    return False


def main() -> int:
    args = parse_args()
    csv_path = REPO_ROOT / args.csv

    crate_name = args.crate or DEFAULT_CRATE
    crate_name, crate_src = resolve_crate(crate_name)
    if args.crate is None:
        print(f"[INFO] 未指定 --crate，默认 {crate_name}（如目标 API 属其他 crate，加 --crate <name>）")

    if args.api_id:
        apis = [
            a for a in load_api_identities(csv_path, skip_old_versions=True)
            if a.api_id == args.api_id
        ]
        if not apis:
            print(f"[ERROR] api_id {args.api_id} 不在 CSV")
            return 1
    elif args.tag or args.crate:
        tags = [args.tag] if args.tag else load_biz_tags(crate_name)
        apis = load_api_identities(csv_path, filter_tags=tags, skip_old_versions=True)
    else:
        print("[ERROR] 需指定 --api-id / --tag / --crate 之一")
        return 1

    print(f"[INFO] 待生成 {len(apis)} 个 API（crate={crate_name}）")
    stats = {"generated": 0, "skipped": 0, "errors": 0}
    manual_consts: list[str] = []
    mod_actions: list[str] = []

    for api in apis:
        try:
            cache = get_or_fetch(
                api, refresh=args.refresh, timeout=args.timeout, retries=args.retries
            )
            ir = parse_api_schema_to_ir(api, cache.api_schema)
            content = render_api_file(ir)
            target = crate_src / api.expected_file
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
                if not endpoint_const_exists(crate_src, ir.endpoint_const_name):
                    snippet = render_endpoint_const_snippet(ir)
                    manual_consts.append(snippet)
                    print(f"  [MANUAL] endpoints/{api.biz_tag}.rs 追加: {snippet}")
                # G4: mod.rs 多层增量补全（自动写盘，不重写现有内容）
                mod_actions.extend(ensure_mod_chain(crate_src, api.expected_file))
            else:
                print(f"\n{'='*70}\n=== {rel}\n{'='*70}")
                print(content)
        except Exception as exc:  # noqa: BLE001
            record_error(DEFAULT_CACHE_DIR, api, exc)
            stats["errors"] += 1
            print(f"[ERROR] {api.api_id} {api.name}: {exc}")

    if args.write:
        if mod_actions:
            print("\n[MOD-TREE] mod.rs 已增量补全：")
            for a in mod_actions:
                print(f"  {a}")
        if stats["generated"]:
            print(
                "\n[MANUAL] service 链（chain.rs）需手动加 client.<biz>...<api>() "
                "方法（G4-phase2，手工精写不自动生成）"
            )
        rc = 0
        if stats["generated"] and not args.no_validate:
            rc = run_closed_loop(crate_name)
        print(
            f"\n[DONE] 生成 {stats['generated']}，跳过 {stats['skipped']}，"
            f"错误 {stats['errors']}"
        )
        return rc if stats["generated"] else (1 if stats["errors"] else 0)
    return 0


def run_closed_loop(crate_name: str) -> int:
    """生成后跑 fmt + clippy，验证可编译无 warning。"""
    rc = 0
    commands = [
        ["cargo", "fmt", "--all"],
        [
            "cargo", "clippy", "-p", crate_name, "--all-targets", "--all-features",
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
