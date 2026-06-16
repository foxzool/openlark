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


class TestDetectSuspiciousPatterns(unittest.TestCase):
    def test_user_level_with_user_id_field(self):
        """用户级接口的 Body 含 user_id -> 警告。"""
        api = verify_api_fields.ApiRecord(
            api_id="1", name="同意", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="task", meta_name="pass",
            url="POST:/open-apis/approval/v4/tasks/pass", doc_path="",
            full_path="/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass",
        )
        structs = [
            verify_api_fields.StructFields(
                name="PassTaskBodyV4",
                fields=[
                    verify_api_fields.FieldInfo("user_id", "String", True),
                    verify_api_fields.FieldInfo("instance_code", "String", True),
                ],
            )
        ]
        source = "pub fn execute() {}"  # 无 validate_required_list
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        # 应检测到 user_id 警告
        user_id_issues = [i for i in issues if "user_id" in i.detail]
        self.assertEqual(len(user_id_issues), 1)
        self.assertEqual(user_id_issues[0].severity, "warning")

    def test_vec_field_without_validate_required_list(self):
        """Body 有 Vec 字段但源码无 validate_required_list -> 警告。"""
        api = verify_api_fields.ApiRecord(
            api_id="2", name="抄送", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="instance", meta_name="add_cc",
            url="POST:/open-apis/approval/v4/instances/add_cc", doc_path="", full_path="",
        )
        structs = [
            verify_api_fields.StructFields(
                name="AddCcBody",
                fields=[verify_api_fields.FieldInfo("cc_user_ids", "String", True)],
            )
        ]
        source = "validate_required!(self.body.instance_code)"  # 无 _list
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        vec_issues = [i for i in issues if "validate_required_list" in i.detail]
        self.assertTrue(len(vec_issues) >= 1)

    def test_get_with_empty_response(self):
        """GET 查询接口 Response 无字段 -> 提示。"""
        api = verify_api_fields.ApiRecord(
            api_id="3", name="详情", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="instance", meta_name="detail",
            url="GET:/open-apis/approval/v4/instances/detail", doc_path="", full_path="",
        )
        structs = [verify_api_fields.StructFields(name="DetailResponse", fields=[])]
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, "")
        empty_resp = [i for i in issues if "Response" in i.detail or "响应" in i.detail]
        self.assertTrue(len(empty_resp) >= 1)
        self.assertEqual(empty_resp[0].severity, "info")


class TestQuickModeReport(unittest.TestCase):
    def test_run_quick_mode_on_temp_files(self):
        """用临时 CSV + 临时 .rs 文件跑快速模式，生成报告。"""
        import tempfile

        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)
            # 构造一个 CSV（单条用户级 API，含 user_id 红旗）
            csv_file = tmpdir / "apis.csv"
            csv_file.write_text(
                "id,name,bizTag,meta.Project,meta.Version,meta.Resource,meta.Name,"
                "detail,chargingMethod,fullDose,fullPath,url,orderMark,supportAppTypes,"
                "tags,updateTime,isCharge,meta.Type,docPath\n"
                '1,同意,approval,approval,v4,task,pass,x,none,true,'
                '/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass,'
                'POST:/open-apis/approval/v4/tasks/pass,1,"[]",[],0,false,1,\n',
                encoding="utf-8",
            )
            # 构造对应的 .rs 文件
            src_dir = tmpdir / "src" / "approval" / "approval" / "v4" / "task"
            src_dir.mkdir(parents=True)
            (src_dir / "pass.rs").write_text(
                "pub struct PassTaskBodyV4 {\n"
                "    pub user_id: String,\n"
                "    pub instance_code: String,\n"
                "}\n"
                "pub struct PassTaskResponseV4 {}\n",
                encoding="utf-8",
            )
            out_md = tmpdir / "report.md"
            out_json = tmpdir / "summary.json"

            report = verify_api_fields.run_quick_mode(
                csv_path=csv_file,
                src_root=tmpdir / "src",
                output_md=out_md,
                output_json=out_json,
            )

            # 报告应包含 user_id 警告
            self.assertIn("user_id", report)
            self.assertTrue(out_md.exists())
            self.assertTrue(out_json.exists())
            import json

            data = json.loads(out_json.read_text(encoding="utf-8"))
            self.assertEqual(data["total_apis"], 1)
            self.assertGreaterEqual(data["apis_with_issues"], 1)


class TestParseDocFields(unittest.TestCase):
    def test_parse_request_body_fields(self):
        """从文档文本提取 POST 请求体字段。"""
        doc_text = (
            "目录 Request Request body Request example Response\n"  # 导航（第1次）
            "Request body\n"  # 正文标题（第2次）
            "Parameter Type Required Description\n\n"
            "instance_code\n\nstring\n\nYes\n\n审批实例 Code\n\n"
            "task_id\n\nstring\n\nYes\n\n任务 ID\n\n"
            "Request example\n"
        )
        fields = verify_api_fields.parse_doc_request_fields(doc_text, method="POST")
        names = {f.name for f in fields}
        self.assertIn("instance_code", names)
        self.assertIn("task_id", names)
        required_map = {f.name: f.required for f in fields}
        self.assertTrue(required_map["instance_code"])

    def test_parse_response_fields_from_example(self):
        """从响应示例 JSON 提取响应字段名。"""
        doc_text = (
            'Response body example\n'
            '{\n'
            '    "code": 0,\n'
            '    "data": {\n'
            '        "definition_name": "请假",\n'
            '        "status": "PENDING",\n'
            '        "tasks": [{"id": "1"}]\n'
            '    }\n'
            '}\n'
            'Error code\n'
        )
        fields = verify_api_fields.parse_doc_response_fields(doc_text)
        self.assertIn("definition_name", fields)
        self.assertIn("status", fields)
        self.assertIn("tasks", fields)


class TestCompareFields(unittest.TestCase):
    def test_compare_finds_missing_and_extra(self):
        """对比代码字段与文档字段，找出缺失和多余。"""
        code_fields = [
            verify_api_fields.FieldInfo("instance_code", "String", True),
            verify_api_fields.FieldInfo("user_id", "String", True),  # 多余
        ]
        doc_fields = [
            verify_api_fields.FieldInfo("instance_code", "String", True),
            verify_api_fields.FieldInfo("task_id", "String", True),  # 代码缺失
        ]
        diff = verify_api_fields.compare_fields(code_fields, doc_fields)
        self.assertIn("task_id", diff.missing)  # 文档有代码无
        self.assertIn("user_id", diff.extra)  # 代码有文档无
        self.assertIn("instance_code", diff.matched)


if __name__ == "__main__":
    unittest.main()
