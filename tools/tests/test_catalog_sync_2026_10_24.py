"""2026-10-24 飞书 API 目录同步验收测试。

Seams（公开边界）:
1. 官方下架 4 条（minutes 订阅/取消订阅、vc 纪要订阅/取消订阅），行数 1754 → 1750
2. 5 个 API 仅 detail 文案变更（结构不动）
3. 下架 API 的 Rust 实现保留（#679 先例：只删 CSV 行）
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path

from tools.tests.catalog_row_count import CURRENT_CATALOG_ROW_COUNT

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

DELISTED_IDS = {
    "7644841241634016203": "订阅妙记变更事件",
    "7644841241634032587": "取消订阅妙记变更事件",
    "7646314141168291027": "订阅纪要变更事件",
    "7646314141168307411": "取消订阅纪要变更事件",
}

# detail 文案变更（断言新文案特征子串，避免整段锁定）
DETAIL_UPDATED_SUBSTRINGS = {
    "6995779366223724548": "本接口已成为历史版本，不推荐使用",
    "7430737008881614850": "注册方式为预注册",
    "7621494177948142790": "妙记总结、章节、待办、关键词、逐字稿",
    "7657481714696588519": "调用前请根据鉴权身份完成准备",
    "7672664994767015159": "离开会议",
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20261024Tests(unittest.TestCase):
    def test_row_count_matches_constant(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)

    def test_delisted_apis_removed(self) -> None:
        rows = _load_csv_by_id()
        for api_id, name in DELISTED_IDS.items():
            with self.subTest(api_id=api_id):
                self.assertNotIn(api_id, rows, f"已下架 API 仍在 CSV: {name}")

    def test_detail_rewritten_apis_present_with_new_copy(self) -> None:
        rows = _load_csv_by_id()
        for api_id, needle in DETAIL_UPDATED_SUBSTRINGS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows)
                self.assertIn(
                    needle,
                    rows[api_id].get("detail", ""),
                    f"detail 未更新为 2026-10-24 文案: {rows[api_id].get('detail', '')[:60]}",
                )


if __name__ == "__main__":
    unittest.main()
