"""#670 目录增量：unified_kms / block v2 / Search·Docx 分类钉在 CSV 上。

现场 `GET api_catalog`（2026-09-15）type=1 叶子 1753，仅线上多「三方快捷审批回调」
（无 HTTP，导出脚本会跳过）。CSV 仍 1752。`unified_kms` 与 `block` v2 entity/message
未进目录。Search/Docx 当前 CSV 行数如下，官方 lark-oapi 生成树多出的条目视为
codegen 超前，禁止手插 CSV。
"""

from __future__ import annotations

import csv
import unittest
from collections import Counter
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"


def _rows() -> list[dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return list(csv.DictReader(file))


class CatalogIssue670ClassificationTests(unittest.TestCase):
    def test_csv_has_no_unified_kms_or_block_v2_project(self) -> None:
        rows = _rows()
        self.assertEqual(len(rows), 1752)
        projects = {row.get("meta.Project", "") for row in rows}
        self.assertNotIn("unified_kms", projects)
        self.assertNotIn("block", projects)
        versions_by_project = Counter((row.get("meta.Project"), row.get("meta.Version")) for row in rows)
        self.assertEqual(versions_by_project.get(("block", "v2"), 0), 0)

    def test_search_and_docx_current_csv_counts(self) -> None:
        rows = _rows()
        by_project = Counter(row.get("meta.Project", "") for row in rows)
        self.assertEqual(by_project.get("search"), 15)
        self.assertEqual(by_project.get("docx"), 19)

    def test_no_hand_inserted_kms_url(self) -> None:
        rows = _rows()
        for row in rows:
            url = (row.get("url") or "") + (row.get("fullPath") or "")
            self.assertNotIn("unified_kms", url)
            self.assertNotIn("/open-apis/block/v2/", url)


if __name__ == "__main__":
    unittest.main()
