import csv
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.official import (
    expected_file_path,
    extract_access_token_types_from_detail_payload,
    extract_endpoint_from_detail_payload,
    extract_request_fields_from_detail_payload,
    extract_response_fields_from_detail_payload,
    load_api_identities,
    normalize_endpoint_path,
    parse_access_token_types_from_markdown,
    split_method_path,
)


class OfficialContractTests(unittest.TestCase):
    def test_expected_file_path_matches_validate_apis_convention(self):
        row = {
            "bizTag": "base",
            "meta.Project": "bitable",
            "meta.Version": "v1",
            "meta.Resource": "app.table",
            "meta.Name": "record/create",
        }

        self.assertEqual(
            expected_file_path(row),
            "base/bitable/v1/app/table/record/create.rs",
        )

    def test_extract_endpoint_from_detail_payload_prefers_schema_api_schema(self):
        payload = {
            "data": {
                "schema": {
                    "apiSchema": {
                        "httpMethod": "post",
                        "path": "/open-apis/document_ai/v1/bank_card/recognize",
                    }
                }
            }
        }

        self.assertEqual(
            extract_endpoint_from_detail_payload(payload),
            ("POST", "/open-apis/document_ai/v1/bank_card/recognize"),
        )

    def test_extract_request_fields_from_detail_payload_reads_body_schema(self):
        payload = {
            "data": {
                "schema": {
                    "apiSchema": {
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {
                                        "properties": [
                                            {
                                                "name": "file",
                                                "type": "string",
                                                "format": "binary",
                                                "required": True,
                                            },
                                            {
                                                "name": "is_async",
                                                "type": "boolean",
                                            },
                                        ]
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        fields = extract_request_fields_from_detail_payload(payload)

        self.assertEqual([field.name for field in fields], ["file", "is_async"])
        self.assertTrue(fields[0].required)
        self.assertEqual(fields[0].field_type, "string:binary")
        self.assertFalse(fields[1].required)

    def test_extract_response_fields_from_detail_payload_unwraps_data_schema(self):
        payload = {
            "data": {
                "schema": {
                    "apiSchema": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "properties": [
                                                {"name": "code", "type": "integer"},
                                                {"name": "msg", "type": "string"},
                                                {
                                                    "name": "data",
                                                    "type": "object",
                                                    "properties": [
                                                        {"name": "bank_card", "type": "object"},
                                                    ],
                                                },
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        fields = extract_response_fields_from_detail_payload(payload)

        self.assertEqual([field.name for field in fields], ["bank_card"])
        self.assertEqual(fields[0].location, "responseBody:application/json.data")

    def test_split_and_normalize_endpoint_path(self):
        method, path = split_method_path("GET:/open-apis/contact/v3/users/:user_id")

        self.assertEqual(method, "GET")
        self.assertEqual(path, "/open-apis/contact/v3/users/:user_id")
        self.assertEqual(
            normalize_endpoint_path(path),
            normalize_endpoint_path("/open-apis/contact/v3/users/{user_id}"),
        )

    def test_load_api_identities_filters_by_biz_tag_and_old_version(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            path = Path(temp_dir) / "api.csv"
            header = [
                "id",
                "name",
                "bizTag",
                "meta.Project",
                "meta.Version",
                "meta.Resource",
                "meta.Name",
                "url",
                "docPath",
                "fullPath",
            ]
            with path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=header)
                writer.writeheader()
                writer.writerow(
                    {
                        "id": "1",
                        "name": "Current",
                        "bizTag": "ai",
                        "meta.Project": "document_ai",
                        "meta.Version": "v1",
                        "meta.Resource": "bank_card",
                        "meta.Name": "recognize",
                        "url": "POST:/open-apis/document_ai/v1/bank_card/recognize",
                        "docPath": "https://open.feishu.cn/document/mock",
                        "fullPath": "/document/uAjLw4CM/mock",
                    }
                )
                writer.writerow(
                    {
                        "id": "2",
                        "name": "Old",
                        "bizTag": "ai",
                        "meta.Project": "document_ai",
                        "meta.Version": "old",
                        "meta.Resource": "default",
                        "meta.Name": "legacy",
                        "url": "GET:/open-apis/legacy",
                        "docPath": "",
                        "fullPath": "/document/legacy",
                    }
                )

            identities = load_api_identities(path, filter_tags=["ai"])

        self.assertEqual(len(identities), 1)
        self.assertEqual(identities[0].expected_file, "ai/document_ai/v1/bank_card/recognize.rs")
        self.assertEqual(identities[0].full_path, "/document/uAjLw4CM/mock")


class AccessTokenDetailPayloadTests(unittest.TestCase):
    """token oracle：从 detail payload 解析 security.supportedAccessToken。"""

    @staticmethod
    def _detail_payload(tokens):
        security = {}
        if tokens is not None:
            security["supportedAccessToken"] = tokens
        return {"data": {"schema": {"apiSchema": {"security": security}}}}

    def test_returns_supported_access_token_list(self):
        tokens = extract_access_token_types_from_detail_payload(
            self._detail_payload(["tenant_access_token", "user_access_token"])
        )
        self.assertEqual(tokens, ("tenant_access_token", "user_access_token"))

    def test_single_tenant_token(self):
        self.assertEqual(
            extract_access_token_types_from_detail_payload(
                self._detail_payload(["tenant_access_token"])
            ),
            ("tenant_access_token",),
        )

    def test_missing_security_returns_empty(self):
        # 飞书未标注 supportedAccessToken（token 端点等）→ 空，调用方降级 UNVERIFIED
        self.assertEqual(extract_access_token_types_from_detail_payload({}), ())
        self.assertEqual(
            extract_access_token_types_from_detail_payload(
                {"data": {"schema": {"apiSchema": {}}}}
            ),
            (),
        )
        self.assertEqual(
            extract_access_token_types_from_detail_payload(
                {"data": {"schema": {"apiSchema": {"security": {}}}}}
            ),
            (),
        )

    def test_non_list_supported_access_token_returns_empty(self):
        self.assertEqual(
            extract_access_token_types_from_detail_payload(
                {"data": {"schema": {"apiSchema": {"security": {
                    "supportedAccessToken": "tenant_access_token",
                }}}}}
            ),
            (),
        )

    def test_filters_non_string_entries(self):
        tokens = extract_access_token_types_from_detail_payload(
            self._detail_payload(["tenant_access_token", 42, None, "user_access_token"])
        )
        self.assertEqual(tokens, ("tenant_access_token", "user_access_token"))


class AccessTokenMarkdownTests(unittest.TestCase):
    """token oracle 回退：从 .md 源 Authorization 行解析（server-docs 页 JSON 缺标注时）。"""

    def test_tenant_only_header_row(self):
        # acs user/get 的 .md 源 Authorization 行
        md = (
            "Authorization | string | 是 | `tenant_access_token`<br>"
            "**值格式**：\"Bearer `access_token`\"<br>"
            "**示例值**：\"Bearer t-7f1bcd13fc57d46bac21793a18e560\""
        )
        self.assertEqual(parse_access_token_types_from_markdown(md), ("tenant_access_token",))

    def test_multi_token_header_row(self):
        md = "Authorization | string | 是 | `tenant_access_token`、`user_access_token`"
        self.assertEqual(
            parse_access_token_types_from_markdown(md),
            ("tenant_access_token", "user_access_token"),
        )

    def test_user_only(self):
        md = "Authorization | string | 是 | `user_access_token`，示例值 \"Bearer u-xxx\""
        self.assertEqual(parse_access_token_types_from_markdown(md), ("user_access_token",))

    def test_no_authorization_row_returns_empty(self):
        self.assertEqual(parse_access_token_types_from_markdown("正文无表头\n其他内容"), ())
        self.assertEqual(parse_access_token_types_from_markdown(""), ())

    def test_ignores_prose_mentions(self):
        # 正文提到 tenant_access_token 但不在 Authorization 表头行 → 不采集
        md = "本接口需要调用方持有 tenant_access_token。\n其他说明 user_access_token。"
        self.assertEqual(parse_access_token_types_from_markdown(md), ())


if __name__ == "__main__":
    unittest.main()
