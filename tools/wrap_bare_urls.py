#!/usr/bin/env python3
"""把 crates/**/*.rs 的 //! / /// doc comment 中的裸 URL 包裹为 <URL>。

用于 enforce-bare-urls change（清零 rustdoc::bare_urls）。
设计：
  - 仅处理 //! / /// doc comment（bare_urls 是 rustdoc lint，普通 // 不触发）。
  - lookbehind (?<![<(]) 跳过已包裹的 <URL 和 markdown 链接 (url)，防重复包裹。
  - URL body [^\\s<>)]+ 遇空格/<>/) 结束，避免贪心吞行尾标点。
幂等：重复运行零改动。
"""
import re
import sys
import pathlib

URL = re.compile(r'(?<![<(])(https?://[^\s<>)]+)')


def wrap_file(path: pathlib.Path) -> int:
    """返回该文件被包裹的 URL 数（0 = 无改动）。"""
    text = path.read_text(encoding='utf-8')
    lines = text.splitlines(keepends=True)
    changed = 0
    for i, line in enumerate(lines):
        s = line.lstrip()
        if s.startswith('//!') or s.startswith('///'):
            new_line, n = URL.subn(r'<\1>', line)
            if n:
                lines[i] = new_line
                changed += n
    if changed:
        path.write_text(''.join(lines), encoding='utf-8')
    return changed


def main(root: str = 'crates') -> int:
    total = 0
    files = 0
    for f in pathlib.Path(root).rglob('*.rs'):
        n = wrap_file(f)
        if n:
            files += 1
            total += n
    print(f'wrapped {total} bare URLs across {files} files')
    return 0


if __name__ == '__main__':
    root = sys.argv[1] if len(sys.argv) > 1 else 'crates'
    sys.exit(main(root))
