"""#661 飞书 API 目录同步（2026-09-14）验收测试。

Seams（公开边界）:
1. 5 个已有 Catalog Entry 的 supportAppTypes 从 ["custom"] 扩为 ["isv", "custom"]
2. checked-in catalog 行数保持 1752
3. 对应 minutes/vc 身份（name/url/meta.Name）不变
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

ARTIFACTS_MINUTE_ID = "7621494177948142790"
SEARCH_MEETING_ID = "7621540721572694970"
GET_NOTE_ID = "7621600266278522080"
SUBSCRIBE_NOTE_ID = "7646314141168291027"
UNSUBSCRIBE_NOTE_ID = "7646314141168307411"

SUPPORT_APP_TYPES_ISV_CUSTOM = '["isv", "custom"]'

SUPPORT_APP_TYPES_EXPECTATIONS = {
    ARTIFACTS_MINUTE_ID: {
        "name": "获取妙记AI产物",
        "url": "GET:/open-apis/minutes/v1/minutes/:minute_token/artifacts",
        "meta.Name": "artifacts",
        "supportAppTypes": SUPPORT_APP_TYPES_ISV_CUSTOM,
    },
    SEARCH_MEETING_ID: {
        "name": "搜索会议记录",
        "url": "POST:/open-apis/vc/v1/meetings/search",
        "meta.Name": "search",
        "supportAppTypes": SUPPORT_APP_TYPES_ISV_CUSTOM,
    },
    GET_NOTE_ID: {
        "name": "获取纪要详情",
        "url": "GET:/open-apis/vc/v1/notes/:note_id",
        "meta.Name": "get",
        "supportAppTypes": SUPPORT_APP_TYPES_ISV_CUSTOM,
    },
    SUBSCRIBE_NOTE_ID: {
        "name": "订阅纪要变更事件",
        "url": "POST:/open-apis/vc/v1/notes/subscription",
        "meta.Name": "subscription",
        "supportAppTypes": SUPPORT_APP_TYPES_ISV_CUSTOM,
    },
    UNSUBSCRIBE_NOTE_ID: {
        "name": "取消订阅纪要变更事件",
        "url": "POST:/open-apis/vc/v1/notes/unsubscription",
        "meta.Name": "unsubscription",
        "supportAppTypes": SUPPORT_APP_TYPES_ISV_CUSTOM,
    },
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260914Tests(unittest.TestCase):
    def test_five_apis_gain_isv_support_app_types(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), 1752)
        for api_id, expected in SUPPORT_APP_TYPES_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")


if __name__ == "__main__":
    unittest.main()
