import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "export_server_api_list.py"
SPEC = importlib.util.spec_from_file_location("export_server_api_list", MODULE_PATH)
export_server_api_list = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = export_server_api_list
SPEC.loader.exec_module(export_server_api_list)


class FinalizeRowsTests(unittest.TestCase):
    def _draft(self, **overrides):
        payload = dict(
            api_id="1",
            name="示例 API",
            meta_project="approval",
            meta_version="old",
            meta_resource="default",
            meta_name_base="spreadsheets/:spreadsheet_token/sheets_batch_update",
            http_method="POST",
            http_path="/open-apis/sheets/v2/spreadsheets/:spreadsheet_token/sheets_batch_update",
            detail="",
            charging_method="none",
            support_app_types=["isv", "custom"],
            update_time=0,
            doc_path="https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/example",
            full_path="/document/mock",
            catalog_index=0,
            catalog_order_mark="0",
        )
        payload.update(overrides)
        return export_server_api_list.ApiRowDraft(**payload)

    def test_finalize_rows_skips_non_callable_entries(self):
        rows, skipped = export_server_api_list.finalize_rows(
            [
                self._draft(
                    api_id="skip-me",
                    meta_project="unknown",
                    http_method="",
                    http_path="",
                    meta_name_base="",
                    doc_path="https://open.feishu.cn/document/server-docs/approval-v4/external_approval/quick-approval-callback",
                )
            ]
        )

        self.assertEqual(rows, [])
        self.assertEqual(len(skipped), 1)
        self.assertEqual(skipped[0].api_id, "skip-me")
        self.assertEqual(skipped[0].reason, "missing HTTP method/path")

    def test_finalize_rows_uses_doc_slug_to_disambiguate_same_method_old_entries(self):
        rows, skipped = export_server_api_list.finalize_rows(
            [
                self._draft(
                    api_id="a",
                    name="更新工作表属性",
                    doc_path="https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/update-sheet-properties",
                ),
                self._draft(
                    api_id="b",
                    name="操作工作表",
                    doc_path="https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet/operate-sheets",
                ),
            ]
        )

        self.assertEqual(len(rows), 2)
        self.assertEqual(skipped, [])
        self.assertEqual(
            {row["meta.Name"] for row in rows},
            {"post#update_sheet_properties", "post#operate_sheets"},
        )


class MainTests(unittest.TestCase):
    def test_main_refuses_partial_export_when_detail_fetch_fails(self):
        catalog = {
            "data": {
                "items": [
                    {
                        "id": "ok",
                        "name": "OK API",
                        "type": 1,
                        "orderMark": "1",
                        "apiSummary": {
                            "fullPath": "/ok",
                            "httpMethod": "GET",
                            "apiPath": "/open-apis/ok",
                            "supportAppTypes": ["custom"],
                            "apiIdentity": {
                                "project": "auth",
                                "version": "v1",
                                "resource": "ok",
                                "apiName": "get",
                            },
                        },
                    },
                    {
                        "id": "bad",
                        "name": "Bad API",
                        "type": 1,
                        "orderMark": "2",
                        "apiSummary": {
                            "fullPath": "/bad",
                            "httpMethod": "GET",
                            "apiPath": "/open-apis/bad",
                            "supportAppTypes": ["custom"],
                            "apiIdentity": {
                                "project": "auth",
                                "version": "v1",
                                "resource": "bad",
                                "apiName": "get",
                            },
                        },
                    },
                ]
            }
        }

        def fake_get_json(url, timeout, retries):
            if "api_catalog" in url:
                return catalog
            if "fullPath=%2Fbad" in url:
                raise RuntimeError("forced detail failure")
            return {
                "data": {
                    "schema": {
                        "description": "ok",
                        "apiSchema": {
                            "httpMethod": "GET",
                            "path": "/open-apis/ok",
                        },
                    },
                    "content": "",
                    "updateTime": 1000,
                    "redirectUri": "/ok",
                }
            }

        original_get_json = export_server_api_list.http_get_json
        original_argv = sys.argv
        with tempfile.TemporaryDirectory() as temp_dir:
            output_path = Path(temp_dir) / "api_list_export.csv"
            output_path.write_text("sentinel\n", encoding="utf-8")

            export_server_api_list.http_get_json = fake_get_json
            sys.argv = [
                "export_server_api_list.py",
                "--output",
                str(output_path),
                "--max-workers",
                "1",
                "--timeout",
                "1",
                "--retries",
                "0",
            ]
            try:
                exit_code = export_server_api_list.main()
            finally:
                export_server_api_list.http_get_json = original_get_json
                sys.argv = original_argv

            self.assertEqual(exit_code, 1)
            self.assertEqual(output_path.read_text(encoding="utf-8"), "sentinel\n")


if __name__ == "__main__":
    unittest.main()
