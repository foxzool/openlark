#!/usr/bin/env python3
"""检测「写了但没在 mod 链挂载」的孤儿 .rs 文件。

以 rustc dep-info（cargo build --emit=dep-info）为基准，与 src/ 目录树 diff，
找出从未被任何 mod 链引用的孤儿文件。用编译器自身解析，零误报。

为兼容存量债务，支持 allowlist（基线）：CI 只对**新增**孤儿失败，存量孤儿计入
`tools/mod_reachability_allowlist.txt`，随各 crate 修复逐步删除。

用法：
    python3 tools/check_mod_reachability.py            # 全 workspace（CI 用，读 allowlist）
    python3 tools/check_mod_reachability.py --crate openlark-communication
    python3 tools/check_mod_reachability.py --update-allowlist  # 重生 allowlist 基线
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Iterable, Set

REPO_ROOT = Path(__file__).resolve().parents[1]
CRATES_DIR = REPO_ROOT / "crates"
TARGET_DEBUG = REPO_ROOT / "target" / "debug"
ALLOWLIST_PATH = REPO_ROOT / "tools" / "mod_reachability_allowlist.txt"


def parse_dep_info(dep_content: str, crate_src_dir: Path) -> Set[Path]:
    """从 .d 文件内容提取属于本 crate src/ 的 .rs 文件集合。

    .d 第 1 行格式：`<rlib>: <src1.rs> <src2.rs> ...`（空格分隔，可能 `\\` 换行续行）。
    只保留路径落在 crate_src_dir 下的 .rs 文件。
    """
    # 归一化为正斜杠前缀，用字符串前缀匹配（不要求路径存在，便于单测用虚拟路径）
    src_prefix = str(crate_src_dir).replace("\\", "/").rstrip("/") + "/"
    joined = dep_content.replace("\\\n", " ")  # 合并续行
    files: Set[Path] = set()
    for line in joined.splitlines():
        if ":" not in line:
            continue
        targets = line.split(":", 1)[1]  # 冒号前是产物 rlib 路径
        for tok in targets.split():
            tok = tok.strip().replace("\\", "/")
            if not tok.endswith(".rs"):
                continue
            if tok.startswith(src_prefix):
                files.add(Path(tok))
    return files


def diff_orphans(on_disk: Iterable[Path], compiled: Iterable[Path]) -> Set[Path]:
    """返回 on_disk 有、compiled 没有的孤儿文件集合。按字符串形式比较，规避绝对/相对路径差异。"""
    compiled_strs = {str(p) for p in compiled}
    return {p for p in on_disk if str(p) not in compiled_strs}


def list_src_files(crate_src_dir: Path) -> Set[Path]:
    """列出 crate src/ 下所有 .rs 文件。"""
    return set(crate_src_dir.rglob("*.rs"))


def crate_dep_file(crate_name: str) -> Path:
    """crate 名 -> dep 文件路径。openlark-communication -> libopenlark_communication.d"""
    suffix = crate_name.replace("-", "_")
    matches = sorted(TARGET_DEBUG.glob(f"lib{suffix}*.d"), key=lambda p: p.stat().st_mtime)
    return matches[-1] if matches else TARGET_DEBUG / f"lib{suffix}.d"


def orphan_key(crate: str, path: Path) -> str:
    """孤儿条目的稳定键（crate + 相对仓库根的路径），用于 allowlist 比对。"""
    try:
        rel = str(path.relative_to(REPO_ROOT))
    except ValueError:
        rel = str(path)
    return f"{crate}: {rel}"


def read_allowlist(path: Path) -> Set[str]:
    """读取 allowlist 文件，返回键集合。空文件/不存在 → 空集。"""
    if not path.is_file():
        return set()
    keys: Set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            keys.add(line)
    return keys


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crate", help="仅检查单个 crate（默认全 workspace）")
    parser.add_argument(
        "--allowlist",
        default=str(ALLOWLIST_PATH),
        help=f"allowlist 路径（默认 {ALLOWLIST_PATH.name}）",
    )
    parser.add_argument(
        "--update-allowlist",
        action="store_true",
        help="重生 allowlist 基线（写入当前全部孤儿，退出码 0）",
    )
    args = parser.parse_args()

    crate_dirs = sorted(d for d in CRATES_DIR.iterdir() if (d / "src" / "lib.rs").is_file())
    if args.crate:
        crate_dirs = [d for d in crate_dirs if d.name == args.crate]
        if not crate_dirs:
            print(f"❌ 未找到 crate：{args.crate}", file=sys.stderr)
            return 2

    # 生成 dep-info（全 workspace 或单 crate）
    if args.crate:
        cmd = ["cargo", "build", "--lib", "--all-features", "-p", args.crate]
    else:
        cmd = ["cargo", "build", "--workspace", "--all-features"]
    proc = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if proc.returncode != 0:
        print(f"❌ 编译失败（先修编译错误）：\n{proc.stderr}", file=sys.stderr)
        return 1

    all_orphans: list[tuple[str, Path]] = []
    for crate_dir in crate_dirs:
        crate_src = crate_dir / "src"
        dep = crate_dep_file(crate_dir.name)
        if not dep.is_file():
            print(f"⚠️  跳过 {crate_dir.name}：未找到 dep 文件 {dep.name}", file=sys.stderr)
            continue
        compiled = parse_dep_info(dep.read_text(encoding="utf-8"), crate_src)
        on_disk = list_src_files(crate_src)
        orphans = diff_orphans(on_disk, compiled)
        for o in sorted(orphans):
            all_orphans.append((crate_dir.name, o))

    all_keys = {orphan_key(c, p) for c, p in all_orphans}

    # --update-allowlist：写基线并退出
    if args.update_allowlist:
        allowlist_path = Path(args.allowlist)
        lines = sorted(all_keys)
        header = (
            "# mod 可达性守卫 allowlist（存量孤儿基线）\n"
            "# 由 `python3 tools/check_mod_reachability.py --update-allowlist` 生成。\n"
            "# CI 只对【新增】孤儿失败；各 crate 修复死代码后从此文件删除对应行，\n"
            "# 再跑 --update-allowlist 或直接编辑收敛。\n"
            f"# 当前存量孤儿：{len(lines)} 个\n\n"
        )
        allowlist_path.write_text(header + "\n".join(lines) + ("\n" if lines else ""), encoding="utf-8")
        print(f"✅ allowlist 已更新：{allowlist_path}（{len(lines)} 个存量孤儿）")
        return 0

    # 正常检查：only NEW orphans fail
    known = read_allowlist(Path(args.allowlist))
    new_orphans = [(c, p) for c, p in all_orphans if orphan_key(c, p) not in known]
    stale_known = known - all_keys  # allowlist 里有但已修复的（提示清理）

    if new_orphans:
        print(f"❌ 发现 {len(new_orphans)} 个【新增】孤儿文件（不在 allowlist 内）：\n")
        for crate, path in new_orphans:
            print(f"  {orphan_key(crate, path)}")
        print(
            "\n修复：在对应 crate 的 lib.rs / 上级 mod.rs 声明该模块，或删除该文件。\n"
            "若为已知存量，运行 `python3 tools/check_mod_reachability.py --update-allowlist` 更新基线。"
        )
        return 1

    msg = f"✅ 无新增孤儿（存量 {len(all_orphans)} 个已在 allowlist，CI 放行）。"
    if stale_known:
        msg += f"\n💡 allowlist 有 {len(stale_known)} 条已修复，可清理后跑 --update-allowlist 收敛。"
    print(msg)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
