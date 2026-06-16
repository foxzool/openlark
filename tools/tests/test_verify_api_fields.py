import importlib.util
import sys
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "verify_api_fields.py"
SPEC = importlib.util.spec_from_file_location("verify_api_fields", MODULE_PATH)
verify_api_fields = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = verify_api_fields
SPEC.loader.exec_module(verify_api_fields)


class TestCsvLoading(unittest.TestCase):
    def test_generate_expected_file_path(self):
        """验证路径推断：bizTag/project/version/resource/name.rs"""
        api = verify_api_fields.ApiRecord(
            api_id="1", name="同意", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="task", meta_name="pass",
            url="POST:/open-apis/approval/v4/tasks/pass", doc_path="", full_path="",
        )
        path = verify_api_fields.generate_expected_file_path(api)
        self.assertEqual(path, "approval/approval/v4/task/pass.rs")

    def test_generate_expected_file_path_with_dotted_resource(self):
        """resource 含 . 时转为 /（如 app.table.record）"""
        api = verify_api_fields.ApiRecord(
            api_id="2", name="创建记录", biz_tag="base", meta_project="bitable",
            meta_version="v1", meta_resource="app.table.record", meta_name="create",
            url="POST:/open-apis/bitable/v1/apps", doc_path="", full_path="",
        )
        path = verify_api_fields.generate_expected_file_path(api)
        self.assertEqual(path, "base/bitable/v1/app/table/record/create.rs")


if __name__ == "__main__":
    unittest.main()
