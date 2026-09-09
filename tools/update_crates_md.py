#!/usr/bin/env python3
"""
根据 tools/api_coverage.toml 与 api_list_export.csv 生成 crates.md。
"""

from __future__ import annotations

import argparse
import csv
from collections import Counter
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None


ROOT = Path(__file__).resolve().parents[1]
CSV_PATH = ROOT / "api_list_export.csv"
MAPPING_PATH = ROOT / "tools" / "api_coverage.toml"
OUTPUT_PATH = ROOT / "crates.md"

FEATURE_ROW_SPECS = [
    ("`auth`", "openlark-auth"),
    ("`communication`", "openlark-communication"),
    ("`cardkit`", "openlark-cardkit"),
    ("`docs` / `base` / `bitable`", "openlark-docs"),
    ("`hr`", "openlark-hr"),
    ("`security`", "openlark-security"),
    ("`meeting`", "openlark-meeting"),
    ("`workflow`", "openlark-workflow"),
    ("`platform`", "openlark-platform"),
    ("`analytics`", "openlark-analytics"),
    ("`user`", "openlark-user"),
    ("`helpdesk`", "openlark-helpdesk"),
    ("`mail`", "openlark-mail"),
    ("`application`", "openlark-application"),
    ("`pay`", "openlark-pay"),
]


def load_mapping() -> dict[str, dict]:
    if tomllib is None:
        raise RuntimeError("当前 Python 不支持 tomllib，请使用 Python 3.11+")
    return tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))["crates"]


def load_csv_rows() -> list[dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as handle:
        return list(csv.DictReader(handle))


def compute_biz_tag_stats() -> dict[str, tuple[int, int, int]]:
    rows = load_csv_rows()
    total_counts = Counter(row["bizTag"] for row in rows)
    old_counts = Counter(row["bizTag"] for row in rows if row["meta.Version"] == "old")
    non_old_counts = Counter(row["bizTag"] for row in rows if row["meta.Version"] != "old")

    return {
        tag: (non_old_counts[tag], total_counts[tag], old_counts[tag])
        for tag in sorted(total_counts.keys(), key=lambda item: (-non_old_counts[item], item))
    }


def render_document() -> str:
    mapping = load_mapping()
    stats = compute_biz_tag_stats()

    lines: list[str] = [
        "# Feature Crate → bizTag 对照表",
        "",
        "本项目的 API 代码按 **bizTag** 组织（对应 `api_list_export.csv` 的 `bizTag` 列，且决定 API 文件路径第一层目录）。",
        "",
        "为了让工具链能从「crate/feature」快速定位到对应的 bizTag（例如 API 覆盖率统计、`tools/validate_apis.py --crate` 自动补齐 `--src/--filter`），仓库维护了一份 **单一真相映射表**：",
        "",
        "- `tools/api_coverage.toml`",
        "",
        "## crate（feature-crate）→ bizTag",
        "",
        "> 下面表格与 `tools/api_coverage.toml` 保持一致，新增/调整映射请优先修改该 TOML。",
        "",
        "| crate（workspace package） | 负责的 bizTag（CSV 原值） |",
        "|---|---|",
    ]

    for crate_name, config in mapping.items():
        biz_tags = ", ".join(f"`{tag}`" for tag in config["biz_tags"])
        lines.append(f"| `{crate_name}` | {biz_tags} |")

    lines.extend(
        [
            "",
            "## bizTag API 数量（排除 meta.Version=old）",
            "",
            "> 数据来源：`api_list_export.csv`。统计口径：按 `bizTag` 分组计数，仅统计 `meta.Version != old` 的行（old 版本不计入“有效 API 数”）。",
            "",
            "| bizTag | API 数量（排除 old） | API 总数 | old 数量 |",
            "|---|---:|---:|---:|",
        ]
    )

    total_non_old = 0
    total_all = 0
    total_old = 0
    for tag, (non_old, total, old) in stats.items():
        lines.append(f"| `{tag}` | {non_old} | {total} | {old} |")
        total_non_old += non_old
        total_all += total
        total_old += old
    lines.append(f"| **合计** | {total_non_old} | {total_all} | {total_old} |")

    lines.extend(
        [
            "",
            "## 工具用法",
            "",
            "- 列出所有 crate 与 bizTag：`python3 tools/validate_apis.py --list-crates`",
            "- 以 crate 为入口验证实现：`python3 tools/validate_apis.py --crate openlark-docs`",
            "",
            "## open-lark（根 crate）feature → crate → bizTag",
            "",
            "当你通过 `open-lark` 这个统一包启用 feature 时，可以按下表理解它最终覆盖的 bizTag 范围：",
            "",
            "| `open-lark` feature | 依赖的 workspace crate | 对应 bizTag |",
            "|---|---|---|",
        ]
    )

    for feature_label, crate_name in FEATURE_ROW_SPECS:
        biz_tags = ", ".join(f"`{tag}`" for tag in mapping[crate_name]["biz_tags"])
        lines.append(f"| {feature_label} | `{crate_name}` | {biz_tags} |")

    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="根据映射与 CSV 生成 crates.md")
    parser.add_argument("--output", default=str(OUTPUT_PATH), help="输出路径（默认: crates.md）")
    parser.add_argument("--check", action="store_true", help="仅检查输出是否已同步，不写文件")
    args = parser.parse_args()

    rendered = render_document()
    output_path = Path(args.output)

    if args.check:
        existing = output_path.read_text(encoding="utf-8") if output_path.exists() else ""
        if existing != rendered:
            print(f"❌ {output_path} 未同步，请运行 `python3 tools/update_crates_md.py`")
            return 1
        print(f"✅ {output_path} 已同步")
        return 0

    output_path.write_text(rendered, encoding="utf-8")
    print(f"✅ 已生成 {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
