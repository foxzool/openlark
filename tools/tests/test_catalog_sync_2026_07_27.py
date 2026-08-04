"""#581 飞书 API 目录同步（2026-07-27）验收测试。

Seams（公开边界）:
1. 仓库 checked-in `api_list_export.csv` 与 live 目录对齐的比较字段
2. message_cot 公开 SDK leaf 源文件仍在（catalog 可删行，API 默认冻结保留）
3. approval instance/task subscription 的 wire URL 不变
"""

from __future__ import annotations

import csv
import importlib.util
import sys
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"
COMM_SRC = REPO_ROOT / "crates" / "openlark-communication" / "src"
WORKFLOW_SRC = REPO_ROOT / "crates" / "openlark-workflow" / "src"

COMPARE_MODULE = REPO_ROOT / "tools" / "compare_api_catalogs.py"
SPEC = importlib.util.spec_from_file_location("compare_api_catalogs", COMPARE_MODULE)
compare_api_catalogs = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = compare_api_catalogs
SPEC.loader.exec_module(compare_api_catalogs)

# Live 复核确认已从目录列表移除、但官方文档仍 HTTP 200 的 message_cot 三 leaf。
MESSAGE_COT_IDS = {
    "7657479549125069790",  # create
    "7657479549125102558",  # complete
    "7657479549125118942",  # update
}

MESSAGE_COT_SDK_FILES = [
    COMM_SRC / "im" / "im" / "v1" / "message_cot" / "create.rs",
    COMM_SRC / "im" / "im" / "v1" / "message_cot" / "complete.rs",
    COMM_SRC / "im" / "im" / "v1" / "message_cot" / "update.rs",
    COMM_SRC / "im" / "im" / "v1" / "message_cot" / "mod.rs",
]

# #581 确认的 7 条稳定目录身份（后续 issue 改变的动态元数据由对应测试维护）。
FIELD_CHANGE_EXPECTATIONS = {
    # approval instance/task subscription：fullPath 重命名；docPath live 为空
    "7663359183039794441": {
        "url": "POST:/open-apis/approval/v4/instances/subscription",
        "fullDose": "true",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/subscription",
        "docPath": "",
    },
    "7663359183039810825": {
        "url": "DELETE:/open-apis/approval/v4/instances/subscription",
        "fullDose": "true",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/unsubscription",
        "docPath": "",
    },
    "7663359183039761673": {
        "url": "POST:/open-apis/approval/v4/tasks/subscription",
        "fullDose": "true",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/subscription",
        "docPath": "",
    },
    "7663359183039778057": {
        "url": "DELETE:/open-apis/approval/v4/tasks/subscription",
        "fullDose": "true",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/unsubscription",
        "docPath": "",
    },
    # IM 搜索消息：计费元数据
    "7649057980096580572": {
        "url": "POST:/open-apis/im/v1/messages/search",
        "chargingMethod": "basic",
        "isCharge": "true",
    },
    # VC bot 读接口：中文 name + 计费
    "7657481714696588519": {
        "url": "GET:/open-apis/vc/v1/bots/events",
        "name": "获取会议事件列表",
        "chargingMethod": "basic",
        "isCharge": "true",
    },
    "7657481714696604903": {
        "url": "GET:/open-apis/vc/v1/bots/user_active_meeting",
        "name": "获取用户活跃会议列表",
        "chargingMethod": "basic",
        "isCharge": "true",
    },
}

# wire 端点常量：本单禁止因 fullPath 文档 slug 变更而改 path/method
APPROVAL_SUBSCRIPTION_WIRE_SNIPPETS = {
    WORKFLOW_SRC
    / "approval"
    / "approval"
    / "v4"
    / "instance"
    / "subscription.rs": "/open-apis/approval/v4/instances/subscription",
    WORKFLOW_SRC
    / "approval"
    / "approval"
    / "v4"
    / "instance"
    / "unsubscription.rs": "/open-apis/approval/v4/instances/subscription",
    WORKFLOW_SRC
    / "approval"
    / "approval"
    / "v4"
    / "task"
    / "subscription.rs": "/open-apis/approval/v4/tasks/subscription",
    WORKFLOW_SRC
    / "approval"
    / "approval"
    / "v4"
    / "task"
    / "unsubscription.rs": "/open-apis/approval/v4/tasks/subscription",
}


def _load_csv_by_id(path: Path) -> dict[str, dict[str, str]]:
    with path.open("r", encoding="utf-8-sig", newline="") as file:
        return {row["id"]: row for row in csv.DictReader(file) if row.get("id")}


class CatalogSync20260727Tests(unittest.TestCase):
    def test_checked_in_catalog_drops_message_cot_rows(self):
        """CSV 与 live 对齐：三行 message_cot 应从 checked-in catalog 移除。"""
        rows = _load_csv_by_id(CSV_PATH)
        present = sorted(api_id for api_id in MESSAGE_COT_IDS if api_id in rows)
        self.assertEqual(
            present,
            [],
            f"message_cot 仍在 catalog 中（应移除并对齐 live）: {present}",
        )

    def test_seven_catalog_identities_remain_stable(self):
        """#581 的稳定目录身份仍保留，且 url 列无错误改写。"""
        rows = _load_csv_by_id(CSV_PATH)
        for api_id, expected in FIELD_CHANGE_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(
                        row.get(field, ""),
                        value,
                        f"id={api_id} field={field}",
                    )

    def test_message_cot_public_sdk_leaves_remain(self):
        """公开 API 冻结：catalog 可删行，SDK leaf 源码必须仍可被依赖方引用。"""
        for path in MESSAGE_COT_SDK_FILES:
            with self.subTest(path=str(path.relative_to(REPO_ROOT))):
                self.assertTrue(path.is_file(), f"missing SDK leaf: {path}")
                text = path.read_text(encoding="utf-8")
                # 至少保留 Request 类型名约定，避免静默删公开表面
                if path.name != "mod.rs":
                    self.assertRegex(
                        text,
                        r"pub struct \w+MessageCotRequest",
                        f"{path.name} 缺少公开 Request 类型",
                    )

    def test_approval_subscription_wire_urls_unchanged(self):
        """禁止为对齐 fullPath 文档 slug 而改 wire method/path。"""
        for path, snippet in APPROVAL_SUBSCRIPTION_WIRE_SNIPPETS.items():
            with self.subTest(path=str(path.relative_to(REPO_ROOT))):
                self.assertTrue(path.is_file(), f"missing leaf: {path}")
                text = path.read_text(encoding="utf-8")
                self.assertIn(snippet, text)

    def test_compare_fields_stable_against_self(self):
        """sanity：checked-in catalog 与自身比较无变化（工具链未破）。"""
        rows = list(_load_csv_by_id(CSV_PATH).values())
        diff = compare_api_catalogs.compare_catalogs(rows, rows)
        self.assertFalse(diff.has_changes)
        self.assertEqual(diff.added, ())
        self.assertEqual(diff.removed, ())
        self.assertEqual(diff.changed, ())


if __name__ == "__main__":
    unittest.main()
