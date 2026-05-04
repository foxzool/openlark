#!/usr/bin/env python3
"""对比两份飞书服务端 API CSV 清单，并生成适合 GitHub Issue 的 Markdown 报告。"""

from __future__ import annotations

import argparse
import csv
import datetime as dt
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterable, List, Sequence, Tuple

DEFAULT_COMPARE_FIELDS = [
    "name",
    "bizTag",
    "meta.Project",
    "meta.Version",
    "meta.Resource",
    "meta.Name",
    "chargingMethod",
    "fullDose",
    "fullPath",
    "url",
    "supportAppTypes",
    "tags",
    "isCharge",
    "meta.Type",
    "docPath",
]

# 噪声字段：不默认参与“接口是否变动”的判定。
NOISE_FIELDS = {"detail", "orderMark", "updateTime"}


@dataclass(frozen=True)
class FieldChange:
    field: str
    old: str
    new: str


@dataclass(frozen=True)
class ApiChange:
    key: str
    baseline: Dict[str, str]
    current: Dict[str, str]
    field_changes: Tuple[FieldChange, ...]


@dataclass(frozen=True)
class CatalogDiff:
    added: Tuple[Dict[str, str], ...]
    removed: Tuple[Dict[str, str], ...]
    changed: Tuple[ApiChange, ...]

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed or self.changed)


def read_csv_rows(path: Path) -> List[Dict[str, str]]:
    with path.open("r", encoding="utf-8-sig", newline="") as file:
        reader = csv.DictReader(file)
        return [{key: (value or "") for key, value in row.items()} for row in reader]


def row_identity(row: Dict[str, str]) -> str:
    api_id = row.get("id", "").strip()
    if api_id:
        return f"id:{api_id}"
    fallback = "|".join(
        [
            row.get("url", "").strip(),
            row.get("meta.Project", "").strip(),
            row.get("meta.Version", "").strip(),
            row.get("meta.Resource", "").strip(),
            row.get("meta.Name", "").strip(),
        ]
    )
    return f"api:{fallback}"


def index_rows(rows: Iterable[Dict[str, str]]) -> Dict[str, Dict[str, str]]:
    indexed: Dict[str, Dict[str, str]] = {}
    duplicate_counts: Dict[str, int] = {}
    for row in rows:
        key = row_identity(row)
        if key in indexed:
            duplicate_counts[key] = duplicate_counts.get(key, 1) + 1
            key = f"{key}#duplicate-{duplicate_counts[key]}"
        indexed[key] = row
    return indexed


def compare_catalogs(
    baseline_rows: Sequence[Dict[str, str]],
    current_rows: Sequence[Dict[str, str]],
    compare_fields: Sequence[str] = DEFAULT_COMPARE_FIELDS,
) -> CatalogDiff:
    baseline = index_rows(baseline_rows)
    current = index_rows(current_rows)

    baseline_keys = set(baseline)
    current_keys = set(current)

    added = tuple(current[key] for key in sorted(current_keys - baseline_keys, key=lambda k: sort_key(current[k])))
    removed = tuple(baseline[key] for key in sorted(baseline_keys - current_keys, key=lambda k: sort_key(baseline[k])))

    changed: List[ApiChange] = []
    for key in sorted(baseline_keys & current_keys, key=lambda k: sort_key(current[k])):
        old_row = baseline[key]
        new_row = current[key]
        field_changes = tuple(
            FieldChange(field, old_row.get(field, ""), new_row.get(field, ""))
            for field in compare_fields
            if old_row.get(field, "") != new_row.get(field, "")
        )
        if field_changes:
            changed.append(ApiChange(key, old_row, new_row, field_changes))

    return CatalogDiff(added=added, removed=removed, changed=tuple(changed))


def sort_key(row: Dict[str, str]) -> Tuple[str, str, str, str, str, str]:
    return (
        row.get("bizTag", ""),
        row.get("meta.Project", ""),
        row.get("meta.Version", ""),
        row.get("meta.Resource", ""),
        row.get("meta.Name", ""),
        row.get("id", ""),
    )


def markdown_escape(value: str) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ").strip()


def markdown_link(row: Dict[str, str]) -> str:
    doc_path = row.get("docPath", "").strip()
    name = row.get("name", "").strip() or row.get("id", "").strip() or "API"
    if doc_path:
        return f"[{markdown_escape(name)}]({doc_path})"
    return markdown_escape(name)


def row_brief(row: Dict[str, str]) -> str:
    meta_path = "/".join(
        part
        for part in [
            row.get("bizTag", ""),
            row.get("meta.Project", ""),
            row.get("meta.Version", ""),
            row.get("meta.Resource", ""),
            row.get("meta.Name", ""),
        ]
        if part
    )
    return meta_path or "-"


def render_rows_table(rows: Sequence[Dict[str, str]], empty_text: str, max_items: int) -> List[str]:
    if not rows:
        return [empty_text, ""]

    visible = rows[:max_items]
    lines = ["| ID | API | URL | Meta |", "|---|---|---|---|"]
    for row in visible:
        lines.append(
            "| {id} | {api} | `{url}` | `{meta}` |".format(
                id=markdown_escape(row.get("id", "")),
                api=markdown_link(row),
                url=markdown_escape(row.get("url", "")),
                meta=markdown_escape(row_brief(row)),
            )
        )
    if len(rows) > max_items:
        lines.append(f"\n> 仅展示前 {max_items} 条，另有 {len(rows) - max_items} 条未展开。")
    lines.append("")
    return lines


def render_changed_table(changes: Sequence[ApiChange], max_items: int, max_field_changes: int) -> List[str]:
    if not changes:
        return ["无字段变化。", ""]

    lines = ["| ID | API | URL | 变化字段 | 明细 |", "|---|---|---|---|---|"]
    for change in changes[:max_items]:
        row = change.current
        field_names = ", ".join(markdown_escape(fc.field) for fc in change.field_changes)
        details = "<br>".join(
            f"`{markdown_escape(fc.field)}`: `{markdown_escape(fc.old)}` → `{markdown_escape(fc.new)}`"
            for fc in change.field_changes[:max_field_changes]
        )
        if len(change.field_changes) > max_field_changes:
            details += f"<br>…另有 {len(change.field_changes) - max_field_changes} 个字段变化"
        lines.append(
            "| {id} | {api} | `{url}` | {fields} | {details} |".format(
                id=markdown_escape(row.get("id", "")),
                api=markdown_link(row),
                url=markdown_escape(row.get("url", "")),
                fields=field_names,
                details=details,
            )
        )
    if len(changes) > max_items:
        lines.append(f"\n> 仅展示前 {max_items} 条，另有 {len(changes) - max_items} 条未展开。")
    lines.append("")
    return lines


def render_report(
    diff: CatalogDiff,
    baseline_count: int,
    current_count: int,
    compare_fields: Sequence[str],
    max_items: int,
    max_field_changes: int,
) -> str:
    generated_at = dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%d %H:%M:%S UTC")
    lines = [
        "# 飞书开放平台 API 变动检测报告",
        "",
        f"**生成时间**: {generated_at}",
        "",
        "## 摘要",
        "",
        f"- 基准 API 数: {baseline_count}",
        f"- 当前 API 数: {current_count}",
        f"- 新增: {len(diff.added)}",
        f"- 删除: {len(diff.removed)}",
        f"- 字段变化: {len(diff.changed)}",
        "",
    ]

    if not diff.has_changes:
        lines.extend(["未检测到飞书服务端 API 清单变化。", ""])
        return "\n".join(lines)

    lines.extend(
        [
            "## 判定口径",
            "",
            "- 以 `id` 作为 API 身份；缺少 `id` 时退回到 `url + meta.*`。",
            "- 默认忽略 `detail`、`orderMark`、`updateTime`，避免文档描述、排序和更新时间噪声触发 Issue。",
            "- 比较字段: " + ", ".join(f"`{field}`" for field in compare_fields),
            "",
            "## 新增 API",
            "",
        ]
    )
    lines.extend(render_rows_table(diff.added, "无新增 API。", max_items))
    lines.extend(["## 删除 API", ""])
    lines.extend(render_rows_table(diff.removed, "无删除 API。", max_items))
    lines.extend(["## 字段变化", ""])
    lines.extend(render_changed_table(diff.changed, max_items, max_field_changes))
    lines.extend(
        [
            "---",
            "",
            "请根据上述变化评估是否需要更新 `api_list_export.csv`、API 覆盖率报告或补齐/调整对应 SDK 接口。",
            "",
        ]
    )
    return "\n".join(lines)


def write_github_output(path: str, diff: CatalogDiff) -> None:
    if not path:
        return
    with open(path, "a", encoding="utf-8") as file:
        file.write(f"has_changes={'true' if diff.has_changes else 'false'}\n")
        file.write(f"added_count={len(diff.added)}\n")
        file.write(f"removed_count={len(diff.removed)}\n")
        file.write(f"changed_count={len(diff.changed)}\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="对比飞书服务端 API CSV 清单，并生成 Markdown 变动报告")
    parser.add_argument("--baseline", default="api_list_export.csv", help="基准 CSV（默认 api_list_export.csv）")
    parser.add_argument("--current", required=True, help="当前刷新得到的 CSV")
    parser.add_argument("--report", required=True, help="Markdown 报告输出路径")
    parser.add_argument(
        "--compare-field",
        action="append",
        dest="compare_fields",
        help="指定参与比较的字段；可重复。默认比较接口身份/路径/计费/应用类型等稳定字段。",
    )
    parser.add_argument("--max-items", type=int, default=200, help="每个分区最多展开的条目数（默认 200）")
    parser.add_argument("--max-field-changes", type=int, default=8, help="每个变化 API 最多展开的字段数（默认 8）")
    parser.add_argument("--github-output", default=os.environ.get("GITHUB_OUTPUT", ""), help="GitHub Actions 输出文件路径")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    baseline_path = Path(args.baseline)
    current_path = Path(args.current)
    report_path = Path(args.report)

    compare_fields = tuple(args.compare_fields or DEFAULT_COMPARE_FIELDS)
    forbidden_fields = sorted(set(compare_fields) & NOISE_FIELDS)
    if forbidden_fields:
        print(f"⚠️  比较字段包含噪声字段: {', '.join(forbidden_fields)}")

    baseline_rows = read_csv_rows(baseline_path)
    current_rows = read_csv_rows(current_path)
    diff = compare_catalogs(baseline_rows, current_rows, compare_fields=compare_fields)

    report = render_report(
        diff,
        baseline_count=len(baseline_rows),
        current_count=len(current_rows),
        compare_fields=compare_fields,
        max_items=max(1, args.max_items),
        max_field_changes=max(1, args.max_field_changes),
    )
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(report, encoding="utf-8")
    write_github_output(args.github_output, diff)

    print(
        "API 变动检测完成："
        f"新增 {len(diff.added)}，删除 {len(diff.removed)}，字段变化 {len(diff.changed)}；"
        f"报告 {report_path}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
