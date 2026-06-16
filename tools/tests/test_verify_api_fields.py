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



class TestExtractStructFields(unittest.TestCase):
    def test_extract_body_fields_basic(self):
        """提取必填 string 字段。"""
        source = '''
pub struct PassTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批任务 ID
    pub task_id: String,
}
'''
        structs = verify_api_fields.extract_structs(source)
        self.assertEqual(len(structs), 1)
        s = structs[0]
        self.assertEqual(s.name, "PassTaskBodyV4")
        self.assertEqual(len(s.fields), 2)
        self.assertEqual(s.fields[0].name, "instance_code")
        self.assertTrue(s.fields[0].required)
        self.assertEqual(s.fields[1].name, "task_id")

    def test_extract_optional_and_vec_fields(self):
        """Option 是选填，Vec 是必填数组。"""
        source = '''
pub struct DemoBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub user_ids: Vec<String>,
    pub count: i32,
}
'''
        structs = verify_api_fields.extract_structs(source)
        fields = {f.name: f for f in structs[0].fields}
        self.assertFalse(fields["comment"].required)  # Option -> 选填
        self.assertTrue(fields["user_ids"].required)  # Vec -> 必填
        self.assertTrue(fields["count"].required)
        self.assertEqual(fields["user_ids"].type_name, "String")  # Vec<String> -> String

    def test_extract_serde_rename(self):
        """serde rename 属性被记录。"""
        source = '''
pub struct DemoBody {
    #[serde(rename = "type")]
    pub task_type: String,
}
'''
        structs = verify_api_fields.extract_structs(source)
        f = structs[0].fields[0]
        self.assertEqual(f.name, "task_type")
        self.assertEqual(f.rename, "type")

    def test_extract_only_body_and_response(self):
        """只提取名字含 Body 或 Response 的 struct。"""
        source = '''
pub struct PassTaskRequestV4 {
    pub config: Config,
}
pub struct PassTaskBodyV4 {
    pub instance_code: String,
}
pub struct PassTaskResponseV4 {
    pub data: serde_json::Value,
}
'''
        structs = verify_api_fields.extract_structs(source)
        names = [s.name for s in structs]
        self.assertIn("PassTaskBodyV4", names)
        self.assertIn("PassTaskResponseV4", names)
        self.assertNotIn("PassTaskRequestV4", names)  # Request struct 不提取

if __name__ == "__main__":
    unittest.main()
