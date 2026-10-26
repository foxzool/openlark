"""解释官方 Markdown 中可直接证明的契约，拒绝从示例或拍平表格推断层级。"""

from __future__ import annotations

import re
from dataclasses import dataclass
from urllib.parse import urlsplit


@dataclass(frozen=True)
class MarkdownField:
    name: str
    location: str
    required: bool | None
    field_type: str


@dataclass(frozen=True)
class MarkdownFields:
    complete: bool = False
    values: tuple[MarkdownField, ...] = ()


@dataclass(frozen=True)
class MarkdownContract:
    healthy: bool
    endpoint: tuple[str, str] | None = None
    tokens: tuple[str, ...] | None = None
    request_fields: MarkdownFields = MarkdownFields()
    response_fields: MarkdownFields = MarkdownFields()


def _without_examples(content: str) -> str:
    lines = []
    fence: str | None = None
    for line in content.splitlines():
        marker = re.match(r"^\s*(`{3,}|~{3,})", line)
        if marker:
            current = marker.group(1)
            if fence is None:
                fence = current
            elif current[0] == fence[0] and len(current) >= len(fence):
                fence = None
        elif fence is None:
            lines.append(line)
    return "\n".join(lines)


def _sections(content: str, level: int, title: str) -> tuple[str, ...]:
    headings = list(re.finditer(r"(?m)^(#{1,6})\s+(.+?)\s*#*\s*$", content))
    matches = [
        index for index, heading in enumerate(headings)
        if len(heading.group(1)) == level and heading.group(2) == title
    ]
    sections = []
    for index in matches:
        start = headings[index].end()
        end = next(
            (heading.start() for heading in headings[index + 1:]
             if len(heading.group(1)) <= level),
            len(content),
        )
        sections.append(content[start:end].strip())
    return tuple(sections)


def _section(content: str, level: int, title: str) -> str | None:
    sections = _sections(content, level, title)
    return sections[0] if len(sections) == 1 else None


def _cells(line: str) -> list[str]:
    return [cell.strip().strip("`") for cell in re.split(r"(?<!\\)\|", line.strip().strip("|"))]


def _table(content: str | None) -> tuple[list[str], list[list[str]]] | None:
    if content is None:
        return None
    lines = [line.strip() for line in content.splitlines()]
    starts = [
        index for index in range(len(lines) - 1)
        if "|" in lines[index]
        and all(re.fullmatch(r":?-{3,}:?", cell) for cell in _cells(lines[index + 1]))
    ]
    if len(starts) != 1:
        return None
    start = starts[0]
    header = _cells(lines[start])
    if len(set(header)) != len(header):
        return None
    rows = []
    for index in range(start + 2, len(lines)):
        line = lines[index]
        if "|" not in line:
            # 字段行可能因损坏而丢失所有分隔符，不能把非空尾部当成说明而丢弃。
            if any(remaining for remaining in lines[index:]):
                return None
            break
        row = _cells(line)
        if len(row) != len(header):
            return None
        rows.append(row)
    return header, rows


def _endpoint(request: str) -> tuple[str, str] | None:
    # 请求基础表位于第一个三级标题之前；后续字段表不能补充此维度。
    basic = re.split(r"(?m)^###\s+", request, maxsplit=1)[0]
    table = _table(basic)
    if table is None:
        return None
    _, rows = table
    urls = [row[1] for row in rows if len(row) == 2 and row[0] == "HTTP URL"]
    methods = [row[1] for row in rows if len(row) == 2 and row[0] == "HTTP Method"]
    if len(urls) != 1 or len(methods) != 1:
        return None
    url = urlsplit(urls[0])
    method = methods[0].upper()
    if (url.scheme != "https" or url.netloc != "open.feishu.cn"
            or not url.path.startswith("/open-apis/") or url.query or url.fragment
            or method not in {"GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"}):
        return None
    return method, url.path


def _tokens(request: str) -> tuple[str, ...] | None:
    table = _table(_section(request, 3, "请求头"))
    if table is None:
        return None
    header, rows = table
    if not {"名称", "必填", "描述"}.issubset(header):
        return None
    name_index, required_index, description_index = (
        header.index(column) for column in ("名称", "必填", "描述")
    )
    auth = [row for row in rows if row[name_index].lower() == "authorization"]
    if len(auth) != 1 or auth[0][required_index] != "是":
        return None
    tokens = set(re.findall(r"\b[a-z_]+_access_token\b", auth[0][description_index]))
    if not tokens or not tokens.issubset({"tenant_access_token", "user_access_token", "app_access_token"}):
        return None
    return tuple(sorted(tokens))


def _fields(section: str | None, location: str, *, request: bool) -> MarkdownFields:
    # 缺章节不是空契约；仅接受明确声明无参数的章节。
    if section is not None and section.strip() in {"无", "无。", "无请求体", "无查询参数", "无路径参数"}:
        return MarkdownFields(complete=True)
    table = _table(section)
    if table is None:
        return MarkdownFields()
    header, rows = table
    expected = {"名称", "类型", "必填"} if request else {"名称", "类型"}
    if not expected.issubset(header) or not rows:
        return MarkdownFields()
    names = [row[header.index("名称")] for row in rows]
    types = [row[header.index("类型")].replace("\\", "").lower() for row in rows]
    # 官方导出丢失 level；遇到对象、复合类型、重复名，整张表均不能证明字段路径。
    if (len(set(names)) != len(names)
            or any(not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", name) for name in names)
            or any(not re.fullmatch(r"(?:string|boolean|int|integer|float|double|number)(?:\[\])?", kind) for kind in types)):
        return MarkdownFields()
    values = []
    complete = True
    for row, name, kind in zip(rows, names, types):
        required = None
        if request:
            marker = row[header.index("必填")]
            required = {"是": True, "否": False}.get(marker)
            complete = complete and required is not None
        values.append(MarkdownField(name, location, required, kind))
    return MarkdownFields(complete, tuple(values))


def parse_markdown_contract(content: str) -> MarkdownContract:
    document = _without_examples(content)
    title = re.match(r"\s*#\s+([^\n]+)", document)
    request = _section(document, 2, "请求")
    response = _section(document, 2, "响应")
    if (title is None or (request is None and response is None)
            or re.search(r"404|not found|不存在|找不到", title.group(1), re.IGNORECASE)
            or re.search(r"<!doctype|<html\b", document, re.IGNORECASE)):
        return MarkdownContract(healthy=False)
    request_fields = MarkdownFields()
    if request is not None:
        sections = [("请求体", "request_body"), ("查询参数", "query"), ("路径参数", "path")]
        fields = []
        for title, location in sections:
            parts = _sections(request, 3, title)
            # 请求体必须明确存在；重复标题或存在但解析失败不能当成缺省章节跳过。
            if title == "请求体" or parts:
                fields.append(_fields(parts[0] if len(parts) == 1 else None, location, request=True))
        request_fields = MarkdownFields(
            complete=all(field.complete for field in fields),
            values=tuple(value for field in fields for value in field.values),
        )
    return MarkdownContract(
        healthy=True,
        endpoint=_endpoint(request) if request is not None else None,
        tokens=_tokens(request) if request is not None else None,
        request_fields=request_fields,
        response_fields=_fields(
            _section(response, 3, "响应体") if response is not None else None,
            "response_body",
            request=False,
        ),
    )
