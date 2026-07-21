#!/usr/bin/env python3
"""#486 CI guard：禁止业务 crate leaf 重新出现手写 response 抽取（防 #470 楔子迁移后 drift）。

leaf 必须走 canonical 入口 `Transport::request_typed`（或 facade 用 `Response::decode`）。
本脚本扫业务 crate src，标记以下 drift 模式：
  - `.data.ok_or_else(`     —— 应走 request_typed
  - `.into_result()`        —— Response::into_result，应走 request_typed / decode
  - `.data.unwrap_or[`      —— 应走 request_typed + empty_success 声明空成功

openlark-core 豁免（定义 Response::decode/into_result 等）。每个文件的 `#[cfg(test)]`
mod 块剥离（测试不计）。命中即 exit 1。

用法：python3 tools/check_no_handwritten_extraction.py
"""
from __future__ import annotations

import glob
import re
import sys

PATTERNS = [
    (re.compile(r"\.data\s*\.ok_or_else"),
     ".data.ok_or_else — 用 Transport::request_typed"),
    (re.compile(r"\.into_result\(\)"),
     ".into_result() — 用 Transport::request_typed（facade 用 Response::decode）"),
    (re.compile(r"\.data\s*\.unwrap_or"),
     ".data.unwrap_or — 用 request_typed + 给 Response 类型声明 empty_success"),
]

ALLOW_CRATES = {"openlark-core"}  # core 定义这些；非业务 leaf


def strip_test_mod(src: str) -> str:
    """剥离 `#[cfg(test)] mod IDENT { ... }`（含 `#[cfg(all(test, ...))]` 等）test mod 块。

    用花括号配对精准剥整个 test mod，而非朴素截到首个 `#[cfg(test)]`——后者对早期
    内联 cfg(test) 文件会误删其后全部生产代码（drift 漏抓）。fn 级 `#[cfg(test)]`
    不剥（罕见，且测试 fn 内 pattern 非 drift）。
    """
    mod_open = re.compile(r"#\[cfg\([^[]*test[^[]*\)\]\s*mod\s+\w+\s*\{")
    out: list[str] = []
    i, n = 0, len(src)
    while i < n:
        m = mod_open.match(src, i)
        if m:
            # 花括号配对找到 mod 闭合 }
            depth, j = 1, m.end()  # m.end() 指向 '{' 之后
            while j < n and depth > 0:
                ch = src[j]
                if ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                j += 1
            i = j
            # 吃掉块后一个换行（避免留空行）
            if i < n and src[i] == "\n":
                i += 1
        else:
            out.append(src[i])
            i += 1
    return "".join(out)


def main() -> int:
    violations: list[str] = []
    for path in sorted(glob.glob("crates/openlark-*/src/**/*.rs", recursive=True)):
        crate = path.split("/")[1]
        if crate in ALLOW_CRATES:
            continue
        src = open(path, encoding="utf-8").read()
        for ln, line in enumerate(strip_test_mod(src).splitlines(), 1):
            for pat, msg in PATTERNS:
                if pat.search(line):
                    violations.append(f"{path}:{ln}: {msg}\n    {line.strip()}")
    if violations:
        print(f"❌ 发现 {len(violations)} 处手写 response 抽取（#470 canonical 路径 drift）：\n")
        for v in violations:
            print(f"  {v}\n")
        print("迁到 Transport::request_typed（facade 持有 Response<T> 的用 Response::decode）。见 #470/#486。")
        return 1
    print("✅ 业务 crate 无手写 response 抽取（全走 canonical 入口）")
    return 0


if __name__ == "__main__":
    sys.exit(main())
