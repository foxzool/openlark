"""schema_cache.cache 单元测试。mock fetch_detail_payload，全程不触网。"""

from __future__ import annotations

import json
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from tools.api_contracts.models import ApiIdentity
from tools.schema_cache.cache import get_or_fetch, record_error


def _api(api_id: str = "123", full_path: str = "/document/x") -> ApiIdentity:
    return ApiIdentity(
        api_id=api_id,
        name="测试",
        biz_tag="im",
        meta_project="im",
        meta_version="v1",
        meta_resource="message",
        meta_name="create",
        url="POST:/open-apis/im/v1/messages",
        doc_path="",
        expected_file="im/im/v1/message/create.rs",
        full_path=full_path,
    )


def _payload(method: str = "post") -> dict:
    return {
        "data": {
            "schema": {
                "apiSchema": {"httpMethod": method, "path": "/open-apis/im/v1/messages"},
            }
        }
    }


class GetOrFetchTest(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp())

    def tearDown(self) -> None:
        shutil.rmtree(self.tmp, ignore_errors=True)

    def test_cache_miss_writes_file(self) -> None:
        with patch("tools.schema_cache.cache.fetch_detail_payload", return_value=_payload()) as mock:
            result = get_or_fetch(_api(), cache_dir=self.tmp)
        self.assertEqual(result.source, "fetch")
        self.assertEqual(result.api_schema["httpMethod"], "post")
        self.assertEqual(result.api_schema["path"], "/open-apis/im/v1/messages")
        mock.assert_called_once()
        self.assertTrue((self.tmp / "123.json").exists())

    def test_cache_hit_no_fetch(self) -> None:
        (self.tmp / "123.json").write_text(json.dumps(_payload()), encoding="utf-8")
        with patch("tools.schema_cache.cache.fetch_detail_payload") as mock:
            result = get_or_fetch(_api(), cache_dir=self.tmp)
        self.assertEqual(result.source, "cache")
        self.assertEqual(result.api_schema["httpMethod"], "post")
        mock.assert_not_called()

    def test_refresh_overwrites(self) -> None:
        (self.tmp / "123.json").write_text(json.dumps(_payload("post")), encoding="utf-8")
        with patch("tools.schema_cache.cache.fetch_detail_payload", return_value=_payload("get")):
            result = get_or_fetch(_api(), cache_dir=self.tmp, refresh=True)
        self.assertEqual(result.source, "fetch")
        self.assertEqual(result.api_schema["httpMethod"], "get")

    def test_corrupt_cache_refetches(self) -> None:
        (self.tmp / "123.json").write_text("{这不是合法 json", encoding="utf-8")
        with patch("tools.schema_cache.cache.fetch_detail_payload", return_value=_payload()):
            result = get_or_fetch(_api(), cache_dir=self.tmp)
        self.assertEqual(result.source, "fetch")
        # 文件已被修复为合法 JSON
        json.loads((self.tmp / "123.json").read_text(encoding="utf-8"))


class RecordErrorTest(unittest.TestCase):
    def test_appends_errors(self) -> None:
        d = Path(tempfile.mkdtemp())
        try:
            record_error(d, _api("1"), ValueError("boom1"))
            record_error(d, _api("2"), ValueError("boom2"))
            errors = json.loads((d / "errors.json").read_text(encoding="utf-8"))
            self.assertEqual(len(errors), 2)
            self.assertEqual(errors[0]["api_id"], "1")
            self.assertEqual(errors[1]["api_id"], "2")
            self.assertIn("boom2", errors[1]["error"])
        finally:
            shutil.rmtree(d, ignore_errors=True)

    def test_corrupt_errors_json_reset(self) -> None:
        d = Path(tempfile.mkdtemp())
        try:
            (d / "errors.json").write_text("[broken", encoding="utf-8")
            record_error(d, _api("1"), ValueError("boom"))
            errors = json.loads((d / "errors.json").read_text(encoding="utf-8"))
            self.assertEqual(len(errors), 1)
        finally:
            shutil.rmtree(d, ignore_errors=True)


if __name__ == "__main__":
    unittest.main()
