from __future__ import annotations

import csv
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.official import (
    expected_file_path,
    load_api_identities,
    normalize_endpoint_path,
    split_method_path,
)


class OfficialCatalogTests(unittest.TestCase):
    def test_expected_file_path_matches_catalog_policy(self):
        row = {
            "bizTag": "base",
            "meta.Project": "bitable",
            "meta.Version": "v1",
            "meta.Resource": "app.table.record",
            "meta.Name": "batch_create",
        }
        self.assertEqual(
            expected_file_path(row),
            "base/bitable/v1/app/table/record/batch_create.rs",
        )

    def test_split_and_normalize_endpoint_path(self):
        method, path = split_method_path(
            "GET:/open-apis/contact/v3/users/:user_id"
        )
        self.assertEqual(method, "GET")
        self.assertEqual(
            normalize_endpoint_path(path),
            "/open-apis/contact/v3/users/{param}",
        )
        self.assertEqual(
            normalize_endpoint_path(
                "/open-apis/contact/v3/users/{open_id}?user_id_type=open_id"
            ),
            "/open-apis/contact/v3/users/{param}",
        )

    def test_load_api_identities_filters_and_preserves_catalog_provenance(self):
        fields = [
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
        with tempfile.TemporaryDirectory() as directory:
            csv_path = Path(directory) / "apis.csv"
            with csv_path.open("w", encoding="utf-8", newline="") as handle:
                writer = csv.DictWriter(handle, fieldnames=fields)
                writer.writeheader()
                writer.writerows(
                    [
                        {
                            "id": "1",
                            "name": "创建记录",
                            "bizTag": "base",
                            "meta.Project": "bitable",
                            "meta.Version": "v1",
                            "meta.Resource": "app.table.record",
                            "meta.Name": "create",
                            "url": "POST:/open-apis/bitable/v1/apps/{app_token}/records",
                            "docPath": "https://open.feishu.cn/document/mock",
                            "fullPath": "/document/uAjLw4CM/mock",
                        },
                        {
                            "id": "2",
                            "name": "旧接口",
                            "bizTag": "base",
                            "meta.Project": "bitable",
                            "meta.Version": "old",
                            "meta.Resource": "app",
                            "meta.Name": "get",
                            "url": "GET:/old",
                            "docPath": "",
                            "fullPath": "",
                        },
                        {
                            "id": "3",
                            "name": "其他领域",
                            "bizTag": "im",
                            "meta.Project": "im",
                            "meta.Version": "v1",
                            "meta.Resource": "message",
                            "meta.Name": "get",
                            "url": "GET:/im",
                            "docPath": "",
                            "fullPath": "",
                        },
                    ]
                )
            identities = load_api_identities(csv_path, filter_tags=["base"])

        self.assertEqual(len(identities), 1)
        identity = identities[0]
        self.assertEqual(identity.api_id, "1")
        self.assertEqual(
            identity.expected_file,
            "base/bitable/v1/app/table/record/create.rs",
        )
        self.assertEqual(identity.full_path, "/document/uAjLw4CM/mock")


if __name__ == "__main__":
    unittest.main()
