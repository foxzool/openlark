"""#771 飞书 API 目录同步（2026-11-02）验收测试。

Seams（公开边界）:
1. 3 个千人千面菜单 Catalog Entry 的 id → name / bizTag / meta.* / fullPath / url
2. checked-in catalog 行数增至 CURRENT_CATALOG_ROW_COUNT
3. 搜索 Wiki 的 url 仍是 v2，不跟随目录 apiPath 的 v1
4. 全表仍按 (bizTag, meta.Project, meta.Version, meta.Resource, meta.Name, id) 有序
5. 三个 Rust Contract Target 均 Resolved，且 HTTP method/path 与 catalog url 一致
"""

from __future__ import annotations

import csv
import unittest
from pathlib import Path

from tools.api_contracts.official import load_api_identities, normalize_endpoint_path
from tools.api_contracts.rust_contract_resolution import Resolved, compose
from tools.api_contracts.rust_source import RustSourceContractAdapter
from tools.tests.catalog_row_count import CURRENT_CATALOG_ROW_COUNT

REPO_ROOT = Path(__file__).resolve().parents[2]
CSV_PATH = REPO_ROOT / "api_list_export.csv"

DELETE_ID = "7570924844655607827"
GET_ID = "7570924844655591443"
SET_ID = "7570924844655640595"
WIKI_SEARCH_ID = "6927492937392324635"

NEW_API_EXPECTATIONS = {
    DELETE_ID: {
        "name": "删除千人千面菜单",
        "bizTag": "bot",
        "meta.Project": "bot",
        "meta.Version": "v3",
        "meta.Resource": "bot_menu",
        "meta.Name": "delete_bot_menu",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/delete_bot_menu",
        "url": "DELETE:/open-apis/bot/v3/bot_menu",
    },
    GET_ID: {
        "name": "获取千人千面菜单",
        "bizTag": "bot",
        "meta.Project": "bot",
        "meta.Version": "v3",
        "meta.Resource": "bot_menu",
        "meta.Name": "get_bot_menu",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/get_bot_menu",
        "url": "GET:/open-apis/bot/v3/bot_menu",
    },
    SET_ID: {
        "name": "创建千人千面菜单",
        "bizTag": "bot",
        "meta.Project": "bot",
        "meta.Version": "v3",
        "meta.Resource": "bot_menu",
        "meta.Name": "set_bot_menu",
        "fullPath": "/document/uAjLw4CM/ukTMukTMukTM/bot-v3/bot_menu/set_bot_menu",
        "url": "POST:/open-apis/bot/v3/bot_menu",
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


class CatalogSync20261102Tests(unittest.TestCase):
    def test_checked_in_catalog_includes_bot_menu_apis(self) -> None:
        rows = _load_csv_by_id()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)
        for api_id, expected in NEW_API_EXPECTATIONS.items():
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, rows, f"缺少 API id={api_id}")
                row = rows[api_id]
                for field, value in expected.items():
                    self.assertEqual(row.get(field), value, f"id={api_id} field={field}")

    def test_wiki_search_stays_on_v2(self) -> None:
        row = _load_csv_by_id()[WIKI_SEARCH_ID]
        self.assertEqual(row["name"], "搜索 Wiki")
        self.assertEqual(row["url"], "POST:/open-apis/wiki/v2/nodes/search")
        self.assertEqual(row["meta.Version"], "v1")
        self.assertEqual(row["meta.Name"], "search")

    def test_catalog_remains_sorted_by_meta_key(self) -> None:
        rows = _load_csv_rows()
        self.assertEqual(len(rows), CURRENT_CATALOG_ROW_COUNT)
        keys = [_sort_key(row) for row in rows]
        self.assertEqual(keys, sorted(keys))

    def test_bot_menu_contract_targets_are_resolved(self) -> None:
        identities = {
            entry.api_id: entry
            for entry in load_api_identities(REPO_ROOT / "api_list_export.csv")
        }
        resolver = compose(repository_root=REPO_ROOT)
        adapter = RustSourceContractAdapter(REPO_ROOT)

        for api_id in (DELETE_ID, GET_ID, SET_ID):
            with self.subTest(api_id=api_id):
                entry = identities[api_id]
                result = resolver.resolve(entry)
                self.assertIsInstance(result, Resolved, f"id={api_id} 未 Resolved: {result}")
                assert isinstance(result, Resolved)
                contract = adapter.scan(result.target)
                self.assertGreaterEqual(len(contract.endpoint_calls), 1, result.target)
                resolved = [call for call in contract.endpoint_calls if call.resolved_path]
                self.assertGreaterEqual(len(resolved), 1, contract.endpoint_calls)
                call = resolved[0]
                catalog_method, catalog_path = entry.url.split(":", 1)
                self.assertEqual(call.method, catalog_method)
                self.assertEqual(
                    normalize_endpoint_path(call.resolved_path),
                    normalize_endpoint_path(catalog_path),
                )


if __name__ == "__main__":
    unittest.main()
