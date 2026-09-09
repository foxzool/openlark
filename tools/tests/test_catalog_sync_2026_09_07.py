"""#652 飞书 API 目录同步（2026-09-07）验收测试。

Seams（公开边界）:
1. 9 个 Catalog Entry 的 id → name / bizTag / meta.* / fullPath / url
2. checked-in catalog 行数从 1743 增至 1752
3. 相邻既有 API（pay 前后、spark app 家族、vc bot 家族）身份字段不变
4. 全表仍按 (bizTag, meta.Project, meta.Version, meta.Resource, meta.Name, id) 有序
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

PAY_ORDER_GET_ID = "6907569742384037890"
PAY_ORDER_LIST_ID = "6907569742384988162"
PAY_CHECK_USER_ID = "6907569742384087042"
SPARK_ANALYTICS_OVERVIEW_ID = "7679858182357994438"
SPARK_CREDIT_USAGE_ID = "7679858182357978054"
SPARK_QUERY_ANALYTICS_ID = "7679858182358010822"
SPARK_GET_AVAILABLE_SCOPE_ID = "7679858182358027206"
SPARK_UPDATE_AVAILABLE_SCOPE_ID = "7679858182358043590"
VC_COUNTDOWN_ID = "7680486345160821966"

PASSPORT_SESSION_QUERY_ID = "7096084771490578433"
PAYROLL_ACCT_ITEM_LIST_ID = "7387360801748402177"
SPARK_APP_LIST_ID = "7642228089434901685"
SPARK_APP_PATCH_ID = "7642228089434934453"
SPARK_SQL_COMMANDS_ID = "7620858064161852623"
SPARK_UPLOAD_HTML_ID = "7642228089434852533"
SPARK_ENUM_DETAIL_ID = "7620858064161918159"
VC_ALERT_LIST_ID = "7146108826851770396"
VC_BOT_EVENTS_ID = "7657481714696588519"
VC_BOT_JOIN_ID = "7672664994767015159"

NEW_API_EXPECTATIONS = {
    PAY_ORDER_GET_ID: {
        "name": "查询订单详情",
        "bizTag": "pay",
        "meta.Project": "pay",
        "meta.Version": "old",
        "meta.Resource": "default",
        "meta.Name": "v1/order/get",
        "fullPath": "/document/ukTMukTMukTM/uITNwUjLyUDM14iM1ATN",
        "url": "GET:/open-apis/pay/v1/order/get",
    },
    PAY_ORDER_LIST_ID: {
        "name": "查询租户购买的付费方案",
        "bizTag": "pay",
        "meta.Project": "pay",
        "meta.Version": "old",
        "meta.Resource": "default",
        "meta.Name": "v1/order/list",
        "fullPath": "/document/ukTMukTMukTM/uETNwUjLxUDM14SM1ATN",
        "url": "GET:/open-apis/pay/v1/order/list",
    },
    PAY_CHECK_USER_ID: {
        "name": "查询用户是否在应用开通范围",
        "bizTag": "pay",
        "meta.Project": "pay",
        "meta.Version": "old",
        "meta.Resource": "default",
        "meta.Name": "v1/paid_scope/check_user",
        "fullPath": "/document/ukTMukTMukTM/uATNwUjLwUDM14CM1ATN",
        "url": "GET:/open-apis/pay/v1/paid_scope/check_user",
    },
    SPARK_ANALYTICS_OVERVIEW_ID: {
        "name": "获取妙搭应用运营数据总览",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app",
        "meta.Name": "open_api_analytics_overview",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/open_api_analytics_overview",
        "url": "GET:/open-apis/spark/v1/apps/:app_id/analytics/overview",
    },
    SPARK_CREDIT_USAGE_ID: {
        "name": "获取妙搭应用消耗 AI 额度",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app",
        "meta.Name": "open_api_credit_usage",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/open_api_credit_usage",
        "url": "GET:/open-apis/spark/v1/apps/:app_id/credit_usage",
    },
    SPARK_QUERY_ANALYTICS_ID: {
        "name": "获取妙搭应用运营数据趋势",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app",
        "meta.Name": "query_analytics_data",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app/query_analytics_data",
        "url": "POST:/open-apis/spark/v1/apps/:app_id/query_analytics_data",
    },
    SPARK_GET_AVAILABLE_SCOPE_ID: {
        "name": "获取妙搭产品使用权限",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app.available_scope",
        "meta.Name": "open_api_get_miaoda_available_scope",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-available_scope/open_api_get_miaoda_available_scope",
        "url": "GET:/open-apis/spark/v1/available_scope",
    },
    SPARK_UPDATE_AVAILABLE_SCOPE_ID: {
        "name": "修改妙搭产品使用权限",
        "bizTag": "spark",
        "meta.Project": "spark",
        "meta.Version": "v1",
        "meta.Resource": "app.available_scope",
        "meta.Name": "open_api_update_miaoda_available_scope",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/spark-v1/app-available_scope/open_api_update_miaoda_available_scope",
        "url": "PUT:/open-apis/spark/v1/available_scope",
    },
    VC_COUNTDOWN_ID: {
        "name": "会中倒计时",
        "bizTag": "vc",
        "meta.Project": "vc",
        "meta.Version": "v1",
        "meta.Resource": "bot",
        "meta.Name": "countdown",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/vc-v1/bot/countdown",
        "url": "POST:/open-apis/vc/v1/bots/countdown",
    },
}

EXISTING_EXPECTATIONS = {
    PASSPORT_SESSION_QUERY_ID: {
        "name": "批量获取脱敏的用户登录信息",
        "url": "POST:/open-apis/passport/v1/sessions/query",
        "meta.Name": "query",
    },
    PAYROLL_ACCT_ITEM_LIST_ID: {
        "name": "批量查询算薪项",
        "url": "GET:/open-apis/payroll/v1/acct_items",
        "meta.Name": "list",
    },
    SPARK_APP_LIST_ID: {
        "name": "批量获取妙搭应用",
        "url": "GET:/open-apis/spark/v1/apps",
        "meta.Name": "list",
    },
    SPARK_APP_PATCH_ID: {
        "name": "更新妙搭应用信息",
        "url": "PATCH:/open-apis/spark/v1/apps/:app_id",
        "meta.Name": "patch",
    },
    SPARK_SQL_COMMANDS_ID: {
        "name": "执行 SQL",
        "url": "POST:/open-apis/spark/v1/apps/:app_id/sql_commands",
        "meta.Name": "sql_commands",
    },
    SPARK_UPLOAD_HTML_ID: {
        "name": "上传 HTML 代码并发布",
        "url": "POST:/open-apis/spark/v1/apps/:app_id/upload_and_release_html_code",
        "meta.Name": "upload_html_code_and_release",
    },
    SPARK_ENUM_DETAIL_ID: {
        "name": "获取自定义枚举详细信息",
        "url": "GET:/open-apis/spark/v1/apps/:app_id/enums/:enum_name",
        "meta.Name": "get_enum_detail",
    },
    VC_ALERT_LIST_ID: {
        "name": "获取告警记录",
        "url": "GET:/open-apis/vc/v1/alerts",
        "meta.Name": "list",
    },
    VC_BOT_EVENTS_ID: {
        "name": "获取会议事件列表",
        "url": "GET:/open-apis/vc/v1/bots/events",
        "meta.Name": "events",
    },
    VC_BOT_JOIN_ID: {
        "name": "加入会议",
        "url": "POST:/open-apis/vc/v1/bots/join",
        "meta.Name": "join",
    },
}

SORT_KEY_FIELDS = (
    "bizTag",
    "meta.Project",
    "meta.Version",
    "meta.Resource",
    "meta.Name",
    "id",
)


def _load_csv_rows() -> list[dict[str, str]]:
    with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as file:
        return [row for row in csv.DictReader(file) if row.get("id")]


def _load_csv_by_id() -> dict[str, dict[str, str]]:
    return {row["id"]: row for row in _load_csv_rows()}


def _sort_key(row: dict[str, str]) -> tuple[str, ...]:
    return tuple(row.get(field, "") for field in SORT_KEY_FIELDS)


class CatalogSync20260907Tests(unittest.TestCase):
    def test_checked_in_catalog_includes_nine_new_apis(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), 1752)
        for api_id, expected in NEW_API_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_adjacent_existing_apis_are_unchanged(self) -> None:
        rows = _load_csv_by_id()
        for api_id, expected in EXISTING_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_catalog_remains_sorted_by_meta_key(self) -> None:
        rows = _load_csv_rows()
        self.assertEqual(len(rows), 1752)
        keys = [_sort_key(row) for row in rows]
        self.assertEqual(keys, sorted(keys))


if __name__ == "__main__":
    unittest.main()
