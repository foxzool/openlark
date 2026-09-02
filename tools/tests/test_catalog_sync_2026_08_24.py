"""#645 飞书 API 目录同步（2026-08-24）验收测试。

Seams（公开边界）:
1. 3 个 VC bot Catalog Entry 的 id → url / fullPath / meta.*
2. checked-in catalog 行数从 1737 增至 1740
3. 现有 bot events / user_active_meeting 身份不变
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

JOIN_ID = "7672664994767015159"
LEAVE_ID = "7672664994766998775"
MESSAGE_ID = "7672664994766982391"
EVENTS_ID = "7657481714696588519"
USER_ACTIVE_MEETING_ID = "7657481714696604903"

NEW_API_EXPECTATIONS = {
    JOIN_ID: {
        "name": "加入会议",
        "bizTag": "vc",
        "meta.Project": "vc",
        "meta.Version": "v1",
        "meta.Resource": "bot",
        "meta.Name": "join",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/join",
        "url": "POST:/open-apis/vc/v1/bots/join",
    },
    LEAVE_ID: {
        "name": "离开会议",
        "bizTag": "vc",
        "meta.Project": "vc",
        "meta.Version": "v1",
        "meta.Resource": "bot",
        "meta.Name": "leave",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/leave",
        "url": "POST:/open-apis/vc/v1/bots/leave",
    },
    MESSAGE_ID: {
        "name": "发送会中消息",
        "bizTag": "vc",
        "meta.Project": "vc",
        "meta.Version": "v1",
        "meta.Resource": "bot",
        "meta.Name": "message",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/message",
        "url": "POST:/open-apis/vc/v1/bots/message",
    },
}

EXISTING_BOT_EXPECTATIONS = {
    EVENTS_ID: {
        "name": "获取会议事件列表",
        "url": "GET:/open-apis/vc/v1/bots/events",
        "meta.Name": "events",
    },
    USER_ACTIVE_MEETING_ID: {
        "name": "获取用户活跃会议列表",
        "url": "GET:/open-apis/vc/v1/bots/user_active_meeting",
        "meta.Name": "user_active_meeting",
    },
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260824Tests(unittest.TestCase):
    def test_checked_in_catalog_includes_three_vc_bot_write_apis(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), 1743)
        for api_id, expected in NEW_API_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_existing_bot_read_apis_are_unchanged(self) -> None:
        rows = _load_csv_by_id()
        for api_id, expected in EXISTING_BOT_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")


if __name__ == "__main__":
    unittest.main()
