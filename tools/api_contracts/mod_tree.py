"""模块树增量维护：从 expected_file 推多层 mod.rs，增量补 pub mod（不重写现有）。

G4：让 codegen 生成的 .rs 自动挂到模块树。安全原则——只 append 缺失的
`pub mod <seg>;`，绝不重写或删除现有内容（pub use / re-export / doc 注释全保留）。

仅处理 expected_file 第一段之后的层（假设 crate lib.rs 已声明顶层 biz 模块，
如 `pub mod im;`——lib.rs 有 feature gate，codegen 不触碰）。
"""

from __future__ import annotations

import re
from pathlib import Path


def ensure_mod_chain(
    crate_src: Path, expected_file: str, *, dry_run: bool = False
) -> list[str]:
    """从 expected_file 推多层模块路径，每层 mod.rs 增量补 `pub mod <下一段>;`。

    expected_file 形如 "im/im/v1/message/create.rs"（相对 crate_src）。逐层：
      src/im/mod.rs          += pub mod im;
      src/im/im/mod.rs       += pub mod v1;
      src/im/im/v1/mod.rs    += pub mod message;
      src/im/im/v1/message/mod.rs += pub mod create;

    已有则跳过；mod.rs 不存在则创建（含最小 module doc）；存在则 append。
    返回操作描述列表；dry_run 时只报告不写盘。
    """
    segments = expected_file.removesuffix(".rs").split("/")
    actions: list[str] = []
    for i in range(len(segments) - 1):
        dir_path = crate_src.joinpath(*segments[: i + 1])
        next_seg = segments[i + 1]
        mod_rs = dir_path / "mod.rs"
        actions.extend(_ensure_pub_mod(mod_rs, next_seg, dry_run=dry_run))
    return actions


def _ensure_pub_mod(
    mod_rs: Path, segment: str, *, dry_run: bool
) -> list[str]:
    line = f"pub mod {segment};"
    if mod_rs.exists():
        content = mod_rs.read_text(encoding="utf-8")
        if _has_pub_mod(content, segment):
            return []  # 已声明，跳过
        if dry_run:
            return [f"[MOD] {mod_rs}: +{line}"]
        # append（保留全部现有内容）
        new_content = content.rstrip() + "\n" + line + "\n"
        mod_rs.write_text(new_content, encoding="utf-8")
        return [f"[MOD] {mod_rs}: +{line}"]
    # mod.rs 不存在 → 创建（最小骨架）
    if dry_run:
        return [f"[CREATE] {mod_rs}"]
    mod_rs.parent.mkdir(parents=True, exist_ok=True)
    mod_rs.write_text(
        f"//! {segment} 模块（由 codegen 自动创建）。\n\n{line}\n", encoding="utf-8"
    )
    return [f"[CREATE] {mod_rs}"]


def _has_pub_mod(content: str, segment: str) -> bool:
    """mod.rs 是否已声明 `pub mod <segment>;`（容忍缩进/空格）。"""
    return bool(
        re.search(rf"^\s*pub\s+mod\s+{re.escape(segment)}\s*;", content, re.MULTILINE)
    )
