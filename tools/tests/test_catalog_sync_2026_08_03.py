"""Issue #596 飞书 API 目录同步验收测试。

Seams（公开边界）:
1. 新增 Passport v1 重置登录密码 API 的完整目录身份
2. Issue 报告中的 19 条计费/应用类型元数据变化
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

PASSWORD_UPDATE_ID = "7537665430859382787"

APPROVAL_CHARGED_IDS = {
    "7663359183039794441",
    "7663359183039810825",
    "7663359183039761673",
    "7663359183039778057",
}

ISV_ENABLED_IDS = {
    "7642228089434950837",
    "7642228089434885301",
    "7642228089434918069",
    "7642228089434901685",
    "7642228089434934453",
    "7620858064161852623",
    "7642228089434868917",
    "7642228089434852533",
    "7620858064161869007",
    "7620858064161934543",
    "7620858064161885391",
    "7657481714696588519",
    "7657481714696604903",
    "7644841241633967051",
    "7644841241633983435",
}


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260803Tests(unittest.TestCase):
    def test_password_update_api_identity_is_checked_in(self):
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), 1737)
        row = rows.get(PASSWORD_UPDATE_ID)
        self.assertIsNotNone(row, f"缺少 API id={PASSWORD_UPDATE_ID}")
        assert row is not None
        expected = {
            "name": "重置登录密码",
            "bizTag": "passport",
            "meta.Project": "passport",
            "meta.Version": "v1",
            "meta.Resource": "password",
            "meta.Name": "update",
            "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/passport-v1/password/update",
            "url": "PUT:/open-apis/passport/v1/password",
            "supportAppTypes": '["custom"]',
        }
        for field, value in expected.items():
            with self.subTest(field=field):
                self.assertEqual(row.get(field), value)

    def test_approval_subscription_apis_are_marked_charged(self):
        rows = _load_csv_by_id()
        for api_id in APPROVAL_CHARGED_IDS:
            with self.subTest(api_id=api_id):
                self.assertEqual(rows[api_id]["chargingMethod"], "basic")
                self.assertEqual(rows[api_id]["isCharge"], "true")

    def test_spark_and_vc_apis_include_isv_app_support(self):
        rows = _load_csv_by_id()
        for api_id in ISV_ENABLED_IDS:
            with self.subTest(api_id=api_id):
                self.assertEqual(rows[api_id]["supportAppTypes"], '["isv", "custom"]')


if __name__ == "__main__":
    unittest.main()
