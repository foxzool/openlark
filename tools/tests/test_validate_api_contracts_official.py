import csv
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.official import (
    expected_file_path,
    extract_endpoint_from_detail_payload,
    extract_request_fields_from_detail_payload,
    extract_response_fields_from_detail_payload,
    load_api_identities,
    normalize_endpoint_path,
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


if __name__ == "__main__":
    unittest.main()
