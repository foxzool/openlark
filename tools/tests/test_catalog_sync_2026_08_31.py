"""#648 飞书 API 目录同步（2026-08-31）验收测试。

Seams（公开边界）:
1. 3 个 Catalog Entry 的 id → url / fullPath / meta.*
2. checked-in catalog 行数从 1740 增至 1743
3. 现有 approval 定义 CRUD/订阅 与 minutes get/search/artifacts 身份不变
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

SEARCH_LAUNCHABLE_ID = "7678280659161042125"
CLIP_ID = "7676147106222902224"
UPLOAD_ID = "7673720420097412036"

CREATE_APPROVAL_ID = "7114621541589712899"
GET_APPROVAL_ID = "7114621541589860355"
SUBSCRIBE_APPROVAL_ID = "7117964632137105411"
UNSUBSCRIBE_APPROVAL_ID = "7117964632137220099"

GET_MINUTE_ID = "7181729161035628545"
SEARCH_MINUTE_ID = "7633638495471881156"
ARTIFACTS_MINUTE_ID = "7621494177948142790"

NEW_API_EXPECTATIONS = {
    SEARCH_LAUNCHABLE_ID: {
        "name": "搜索可发起的审批定义",
        "bizTag": "approval",
        "meta.Project": "approval",
        "meta.Version": "v4",
        "meta.Resource": "approval",
        "meta.Name": "search_launchable",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/approval/search_launchable",
        "url": "POST:/open-apis/approval/v4/approvals/search_launchable",
    },
    CLIP_ID: {
        "name": "创建妙记片段",
        "bizTag": "minutes",
        "meta.Project": "minutes",
        "meta.Version": "v1",
        "meta.Resource": "minute",
        "meta.Name": "clip",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/clip",
        "url": "POST:/open-apis/minutes/v1/minutes/:minute_token/clip",
    },
    UPLOAD_ID: {
        "name": "云空间文件生成妙记",
        "bizTag": "minutes",
        "meta.Project": "minutes",
        "meta.Version": "v1",
        "meta.Resource": "minute",
        "meta.Name": "upload",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/upload",
        "url": "POST:/open-apis/minutes/v1/minutes/upload",
    },
}

EXISTING_EXPECTATIONS = {
    CREATE_APPROVAL_ID: {
        "name": "创建审批定义",
        "url": "POST:/open-apis/approval/v4/approvals",
        "meta.Name": "create",
    },
    GET_APPROVAL_ID: {
        "name": "查看指定审批定义",
        "url": "GET:/open-apis/approval/v4/approvals/:approval_code",
        "meta.Name": "get",
    },
    SUBSCRIBE_APPROVAL_ID: {
        "name": "订阅审批事件",
        "url": "POST:/open-apis/approval/v4/approvals/:approval_code/subscribe",
        "meta.Name": "subscribe",
    },
    UNSUBSCRIBE_APPROVAL_ID: {
        "name": "取消订阅审批事件",
        "url": "POST:/open-apis/approval/v4/approvals/:approval_code/unsubscribe",
        "meta.Name": "unsubscribe",
    },
    GET_MINUTE_ID: {
        "name": "获取妙记信息",
        "url": "GET:/open-apis/minutes/v1/minutes/:minute_token",
        "meta.Name": "get",
    },
    SEARCH_MINUTE_ID: {
        "name": "搜索妙记",
        "url": "POST:/open-apis/minutes/v1/minutes/search",
        "meta.Name": "search",
    },
    ARTIFACTS_MINUTE_ID: {
        "name": "获取妙记AI产物",
        "url": "GET:/open-apis/minutes/v1/minutes/:minute_token/artifacts",
        "meta.Name": "artifacts",
    },
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260831Tests(unittest.TestCase):
    def test_checked_in_catalog_includes_three_new_apis(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), 1743)
        for api_id, expected in NEW_API_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_existing_approval_and_minutes_apis_are_unchanged(self) -> None:
        rows = _load_csv_by_id()
        for api_id, expected in EXISTING_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")


if __name__ == "__main__":
    unittest.main()
