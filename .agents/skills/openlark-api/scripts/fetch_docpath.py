#!/usr/bin/env python3
"""
抓取飞书开放平台 docPath 网页并提取“可能有用”的 API 定义信息（请求/响应/示例 JSON/字段表等）。

目标：
- 尽量用 stdlib（无需额外依赖）
- 输出可直接粘贴到上下文的 Markdown，辅助生成 Rust Request/Response 字段

用法：
  python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --format md --out /tmp/doc.md
  python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --format json

离线模式（无网络/受限环境）：
  python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --html-file /path/to/page.html --format md
  cat /path/to/page.html | python3 .agents/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --stdin --format md
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.request
from dataclasses import dataclass
from html.parser import HTMLParser


def _collapse_ws(s: str) -> str:
    s = re.sub(r"[ \t]+", " ", s)
    s = re.sub(r"\n{3,}", "\n\n", s)
    return s.strip()


@dataclass
class Extracted:
    url: str
    title: str | None
    headings: list[str]
    code_blocks: list[str]
    tables: list[list[list[str]]]
    text_sample: str | None


class _DocParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self._in_title = False
        self._in_heading = 0  # 1/2/3 for h1/h2/h3
        self._in_pre = False
        self._in_code = False
        self._in_script = False
        self._in_style = False
        self._in_table = False
        self._in_tr = False
        self._in_td = False
        self._in_th = False

        self.title: str | None = None
        self.headings: list[str] = []
        self.code_blocks: list[str] = []
        self.tables: list[list[list[str]]] = []  # tables -> rows -> cells
        self._current_code: list[str] = []
        self._text: list[str] = []
        self._current_heading: list[str] = []
        self._current_table: list[list[str]] = []
        self._current_row: list[str] = []
        self._current_cell: list[str] = []

    def handle_starttag(self, tag: str, attrs) -> None:
        tag = tag.lower()
        if tag == "script":
            self._in_script = True
        if tag == "style":
            self._in_style = True
        if self._in_script or self._in_style:
            return

        if tag == "title":
            self._in_title = True
        if tag in ("h1", "h2", "h3"):
            self._in_heading = int(tag[1])
            self._current_heading = []
        if tag == "pre":
            self._in_pre = True
            self._current_code = []
        if tag == "code":
            self._in_code = True
        if tag == "table":
            self._in_table = True
            self._current_table = []
        if tag == "tr" and self._in_table:
            self._in_tr = True
            self._current_row = []
        if tag in ("td", "th") and self._in_tr:
            self._in_td = tag == "td"
            self._in_th = tag == "th"
            self._current_cell = []

    def handle_endtag(self, tag: str) -> None:
        tag = tag.lower()
        if tag == "script":
            self._in_script = False
        if tag == "style":
            self._in_style = False
        if self._in_script or self._in_style:
            return

        if tag == "title":
            self._in_title = False
        if tag in ("h1", "h2", "h3") and self._in_heading:
            heading = _collapse_ws("".join(self._current_heading))
            if heading:
                self.headings.append(heading)
            self._in_heading = 0
            self._current_heading = []
        if tag == "code":
            self._in_code = False
        if tag == "pre":
            self._in_pre = False
            block = "".join(self._current_code).strip("\n")
            block = block.replace("\r\n", "\n")
            if block.strip():
                self.code_blocks.append(block)
            self._current_code = []
        if tag in ("td", "th") and (self._in_td or self._in_th):
            cell = _collapse_ws("".join(self._current_cell))
            if cell:
                self._current_row.append(cell)
            self._in_td = False
            self._in_th = False
            self._current_cell = []
        if tag == "tr" and self._in_tr:
            row = [c for c in self._current_row if c]
            if row:
                self._current_table.append(row)
            self._in_tr = False
            self._current_row = []
        if tag == "table" and self._in_table:
            if self._current_table:
                self.tables.append(self._current_table)
            self._in_table = False
            self._current_table = []

    def handle_data(self, data: str) -> None:
        if self._in_script or self._in_style:
            return
        if self._in_title:
            t = _collapse_ws(data)
            if t:
                self.title = t if self.title is None else f"{self.title} {t}"
            return
        if self._in_heading:
            self._current_heading.append(data)
        if self._in_pre:
            self._current_code.append(data)
        elif self._in_td or self._in_th:
            self._current_cell.append(data)
        else:
            # 只保留少量正文采样，避免输出爆炸
            if len(self._text) < 5000:
                self._text.append(data)

    def text_sample(self) -> str:
        return _collapse_ws("".join(self._text))


def fetch_html(url: str, timeout: int) -> str:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "open-lark(openlark-api-skill)/1.0",
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        },
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        raw = resp.read()
        charset = resp.headers.get_content_charset() or "utf-8"
    try:
        return raw.decode(charset, errors="replace")
    except LookupError:
        return raw.decode("utf-8", errors="replace")


def extract(url: str, html: str, max_code_blocks: int, max_text_chars: int) -> Extracted:
    parser = _DocParser()
    parser.feed(html)

    code_blocks = []
    for block in parser.code_blocks:
        if len(code_blocks) >= max_code_blocks:
            break
        block = block.strip()
        # 优先保留看起来像 JSON / curl / http 片段
        if block.startswith("{") or block.startswith("[") or "curl " in block or "/open-apis/" in block:
            code_blocks.append(block)

    if not code_blocks:
        code_blocks = parser.code_blocks[:max_code_blocks]

    text = parser.text_sample()
    if len(text) > max_text_chars:
        text = text[:max_text_chars] + "…"

    return Extracted(
        url=url,
        title=parser.title,
        headings=parser.headings[:50],
        code_blocks=code_blocks,
        tables=parser.tables[:6],
        text_sample=text if text else None,
    )


def _table_to_md(table: list[list[str]], max_rows: int = 40) -> str:
    if not table:
        return ""
    rows = table[:max_rows]
    # 取第一行作为 header（如果列数>=2 且有较短字段）
    header = rows[0]
    cols = max(len(r) for r in rows)
    header = (header + [""] * cols)[:cols]
    body = [(r + [""] * cols)[:cols] for r in rows[1:]]
    out = []
    out.append("| " + " | ".join(header) + " |")
    out.append("| " + " | ".join(["---"] * cols) + " |")
    for r in body:
        out.append("| " + " | ".join(r) + " |")
    if len(table) > max_rows:
        out.append(f"\n> 表格已截断，仅显示前 {max_rows} 行。")
    return "\n".join(out)


def to_markdown(data: Extracted) -> str:
    out: list[str] = []
    out.append(f"# docPath 抓取结果\n")
    out.append(f"- URL: {data.url}")
    if data.title:
        out.append(f"- Title: {data.title}")
    out.append("")

    if data.headings:
        out.append("## 目录/标题（采样）")
        for h in data.headings:
            out.append(f"- {h}")
        out.append("")

    if data.code_blocks:
        out.append("## 代码块/示例（优先保留 JSON/curl/open-apis）")
        for idx, block in enumerate(data.code_blocks, start=1):
            out.append(f"\n### block {idx}\n")
            out.append("```")
            out.append(block)
            out.append("```")
        out.append("")

    if data.tables:
        out.append("## 字段表/参数表（采样）")
        for idx, t in enumerate(data.tables, start=1):
            out.append(f"\n### table {idx}\n")
            out.append(_table_to_md(t))
        out.append("")

    if data.text_sample:
        out.append("## 正文文本（截断采样）")
        out.append("")
        out.append(data.text_sample)
        out.append("")

    out.append("## 使用方式（建议）")
    out.append("- 从上面的示例 JSON/字段表里整理出 Request/Response 字段（优先按官方字段命名 + serde rename）。")
    out.append("- 若页面信息不足：手动打开 docPath，复制“请求体/响应体/字段说明/示例”补充。")
    out.append("")
    return "\n".join(out)


def _parse_args(argv: list[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Fetch and extract useful parts from a docPath page.")
    p.add_argument("url", nargs="?", help="docPath URL（离线模式可省略）")
    p.add_argument("--timeout", type=int, default=20)
    p.add_argument("--max-code-blocks", type=int, default=12)
    p.add_argument("--max-text-chars", type=int, default=12000)
    p.add_argument("--format", choices=["md", "json"], default="md")
    p.add_argument("--out", help="Write output to file instead of stdout")
    p.add_argument("--html-file", help="Read HTML from a local file instead of fetching from network")
    p.add_argument("--stdin", action="store_true", help="Read HTML from stdin instead of fetching from network")
    return p.parse_args(argv)


def main(argv: list[str]) -> int:
    args = _parse_args(argv)
    if args.html_file and args.stdin:
        raise SystemExit("不能同时指定 --html-file 与 --stdin")

    if args.html_file:
        with open(args.html_file, "r", encoding="utf-8", errors="replace") as f:
            html = f.read()
        url = args.url or f"file:{args.html_file}"
    elif args.stdin:
        html = sys.stdin.read()
        url = args.url or "stdin"
    else:
        if not args.url:
            raise SystemExit("缺少 docPath URL：请提供 <docPath>，或使用 --html-file/--stdin 进入离线模式")
        html = fetch_html(args.url, timeout=args.timeout)
        url = args.url

    data = extract(
        url,
        html,
        max_code_blocks=args.max_code_blocks,
        max_text_chars=args.max_text_chars,
    )

    if args.format == "json":
        payload = json.dumps(
            {
                "url": data.url,
                "title": data.title,
                "headings": data.headings,
                "code_blocks": data.code_blocks,
                "tables": data.tables,
                "text_sample": data.text_sample,
            },
            ensure_ascii=False,
            indent=2,
        )
    else:
        payload = to_markdown(data)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as f:
            f.write(payload)
    else:
        sys.stdout.write(payload)
        if not payload.endswith("\n"):
            sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
