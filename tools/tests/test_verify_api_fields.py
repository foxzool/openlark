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
        """用户级接口的 Body 含 user_id -> info 提示（弱启发式）。"""
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
        source = "pub fn execute() {}"
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        user_id_issues = [i for i in issues if "user_id" in i.detail]
        self.assertEqual(len(user_id_issues), 1)
        self.assertEqual(user_id_issues[0].severity, "info")

    def test_vec_field_without_any_validation(self):
        """必填 Vec 字段无任何非空校验 -> 警告。"""
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
        source = "validate_required!(self.body.instance_code)"  # 无 _list 也无 is_empty
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        vec_issues = [i for i in issues if "cc_user_ids" in i.detail and "校验" in i.detail]
        self.assertTrue(len(vec_issues) >= 1)
        self.assertEqual(vec_issues[0].severity, "warning")

    def test_vec_field_with_manual_is_empty_check_not_flagged(self):
        """必填 Vec 字段用手写 is_empty() 校验 -> 不报。"""
        api = verify_api_fields.ApiRecord(
            api_id="4", name="创建用户", biz_tag="contact", meta_project="contact",
            meta_version="v3", meta_resource="user", meta_name="create",
            url="POST:/open-apis/contact/v3/users", doc_path="", full_path="",
        )
        structs = [
            verify_api_fields.StructFields(
                name="CreateUserBody",
                fields=[verify_api_fields.FieldInfo("department_ids", "String", True)],
            )
        ]
        # 手写校验
        source = "if body.department_ids.is_empty() { return Err(...); }"
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        vec_issues = [i for i in issues if "department_ids" in i.detail and "校验" in i.detail]
        self.assertEqual(len(vec_issues), 0)  # 手写校验不应报

    def test_optional_vec_field_not_flagged(self):
        """Option<Vec> 选填字段 -> 不报（选填本不该校验非空）。"""
        api = verify_api_fields.ApiRecord(
            api_id="5", name="恢复用户", biz_tag="contact", meta_project="contact",
            meta_version="v3", meta_resource="user", meta_name="resurrect",
            url="POST:/open-apis/contact/v3/users/x/resurrect", doc_path="", full_path="",
        )
        structs = [
            verify_api_fields.StructFields(
                name="ResurrectBody",
                fields=[
                    verify_api_fields.FieldInfo("subscription_ids", "String", required=False),
                ],
            )
        ]
        source = "validate_required!(body.user_id)"  # 无 subscription 校验
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        vec_issues = [i for i in issues if "subscription_ids" in i.detail]
        self.assertEqual(len(vec_issues), 0)  # Option 字段不应报

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

    def test_parse_put_request_body_fields(self):
        """PUT 接口与 POST 一样从 Request body 正文解析字段。"""
        doc_text = (
            "目录 Request Request body Request example Response\n"
            "Request body\n"
            "Parameter Type Required Description\n\n"
            "user_id\n\nstring\n\nYes\n\n目标用户\n\n"
            "password\n\nstring\n\nNo\n\n新密码\n\n"
            "require_reset\n\nboolean\n\nNo\n\n下次登录重置\n\n"
            "Request example\n"
        )

        fields = verify_api_fields.parse_doc_request_fields(doc_text, method="PUT")

        self.assertEqual(
            [(field.name, field.required) for field in fields],
            [("user_id", True), ("password", False), ("require_reset", False)],
        )

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

    def test_parse_param_table_skips_banned_words(self):
        """_parse_param_table 跳过 parameter/type/required 等表头词。"""
        section = (
            "parameter\n\ntype\n\nrequired\n\n"  # 表头（应跳过）
            "instance_code\n\nstring\n\nYes\n\n实例 Code\n\n"
            "comment\n\nstring\n\nNo\n\n意见\n"
        )
        fields = verify_api_fields._parse_param_table(section)
        names = {f.name for f in fields}
        self.assertEqual(names, {"instance_code", "comment"})
        # 确认表头词没被当成字段
        self.assertNotIn("parameter", names)
        self.assertNotIn("type", names)
        self.assertNotIn("required", names)
        # 必填性正确
        req_map = {f.name: f.required for f in fields}
        self.assertTrue(req_map["instance_code"])
        self.assertFalse(req_map["comment"])

    def test_parse_param_table_sparse_blank_lines_like_feishu_spa(self):
        """飞书 SPA 在字段名与 Yes/No 之间插入多行空行时仍能解析，且不把 string 当字段。"""
        section = (
            "Parameter\n\nType\n\nRequired\n\nDescription\n\n\n\n"
            "instance_code\n\n\n\nstring\n\n\n\nYes\n\n\n\n"
            "Approval instance Code\n\n"
            "Example value: \"81D31358\"\n\n\n\n\n"
            "task_id\n\n\n\nstring\n\n\n\nYes\n\n\n\n"
            "The approval task ID\n\n"
            "form\n\n\n\nstring\n\n\n\nNo\n\n\n\n"
            "Form data\n\n"
            "comment\n\n\n\nstring\n\n\n\nNo\n\n\n\n"
            "approval comment\n"
        )
        fields = verify_api_fields._parse_param_table(section)
        names = [f.name for f in fields]
        self.assertEqual(names, ["instance_code", "task_id", "form", "comment"])
        self.assertNotIn("string", names)
        req = {f.name: f.required for f in fields}
        self.assertTrue(req["instance_code"])
        self.assertTrue(req["task_id"])
        self.assertFalse(req["form"])
        self.assertFalse(req["comment"])


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


class TestDocFetchGate(unittest.TestCase):
    """完整模式不得在抓取失败时假绿放行。"""

    def test_validate_doc_text_rejects_thin_or_404(self):
        self.assertIsNotNone(verify_api_fields._validate_doc_text(""))
        self.assertIsNotNone(verify_api_fields._validate_doc_text("x" * 100))
        self.assertIsNotNone(
            verify_api_fields._validate_doc_text(
                "The documentation could not be found.\n" + ("a" * 600)
            )
        )
        self.assertIsNone(verify_api_fields._validate_doc_text("正文内容" * 200))

    def test_exit_code_error_and_warning_fail(self):
        self.assertEqual(verify_api_fields._exit_code_for_issues([]), 0)
        self.assertEqual(
            verify_api_fields._exit_code_for_issues(
                [verify_api_fields.FieldIssue("info", "x", "tip")]
            ),
            0,
        )
        self.assertEqual(
            verify_api_fields._exit_code_for_issues(
                [verify_api_fields.FieldIssue("warning", "x", "warn")]
            ),
            1,
        )
        self.assertEqual(
            verify_api_fields._exit_code_for_issues(
                [verify_api_fields.FieldIssue("error", "doc_fetch_failed", "fail")]
            ),
            1,
        )

    def test_fetch_docs_failure_recorded_as_error_exit_1(self):
        """--fetch-docs 时抓取失败必须记 error 且返回非 0（不再假绿）。"""
        import tempfile
        from unittest import mock

        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)
            src_root = tmpdir / "crates"
            api_dir = (
                src_root / "openlark-workflow" / "src" / "approval" / "approval" / "v4" / "task"
            )
            api_dir.mkdir(parents=True)
            (api_dir / "pass.rs").write_text(
                "pub struct PassTaskBodyV4 {\n"
                "    pub instance_code: String,\n"
                "    pub task_id: String,\n"
                "}\n"
                "pub struct PassTaskResponseV4 {}\n",
                encoding="utf-8",
            )
            out_dir = tmpdir / "out"
            api = verify_api_fields.ApiRecord(
                api_id="999",
                name="同意",
                biz_tag="approval",
                meta_project="approval",
                meta_version="v4",
                meta_resource="task",
                meta_name="pass",
                url="POST:/open-apis/approval/v4/tasks/pass",
                doc_path="",
                full_path="/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass",
            )
            fake_fail = verify_api_fields.DocFetchResult(error="playwright missing")
            with mock.patch.object(
                verify_api_fields, "_fetch_single_doc", return_value=fake_fail
            ):
                code = verify_api_fields._run_single_api(
                    "999", [api], src_root, out_dir, "api-999", fetch_docs=True
                )
            self.assertEqual(code, 1)
            report = (out_dir / "api-999.md").read_text(encoding="utf-8")
            self.assertIn("文档抓取失败", report)
            self.assertIn("有问题 | 1", report)

    def test_fetch_docs_success_compares_fields(self):
        """抓取成功时对比字段，多余字段记 warning 并非 0 退出。"""
        import tempfile
        from unittest import mock

        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)
            src_root = tmpdir / "crates"
            api_dir = (
                src_root / "openlark-workflow" / "src" / "approval" / "approval" / "v4" / "task"
            )
            api_dir.mkdir(parents=True)
            (api_dir / "pass.rs").write_text(
                "pub struct PassTaskBodyV4 {\n"
                "    pub instance_code: String,\n"
                "    pub task_id: String,\n"
                "    pub user_id: String,\n"  # 文档没有 → extra
                "}\n"
                "pub struct PassTaskResponseV4 {}\n",
                encoding="utf-8",
            )
            out_dir = tmpdir / "out"
            api = verify_api_fields.ApiRecord(
                api_id="998",
                name="同意",
                biz_tag="approval",
                meta_project="approval",
                meta_version="v4",
                meta_resource="task",
                meta_name="pass",
                url="POST:/open-apis/approval/v4/tasks/pass",
                doc_path="",
                full_path="/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass",
            )
            # 伪造足够长的文档正文（含 Request body 段）
            pad = "x" * 400
            doc_text = (
                f"{pad}\n目录 Request Request body Request example Response\n"
                "Request body\n"
                "instance_code\nstring\nYes\n实例\n"
                "task_id\nstring\nYes\n任务\n"
                "Request example\n"
                f"{pad}\n"
            )
            ok = verify_api_fields.DocFetchResult(text=doc_text)
            with mock.patch.object(verify_api_fields, "_fetch_single_doc", return_value=ok):
                code = verify_api_fields._run_single_api(
                    "998", [api], src_root, out_dir, "api-998", fetch_docs=True
                )
            self.assertEqual(code, 1)  # extra_field warning
            report = (out_dir / "api-998.md").read_text(encoding="utf-8")
            self.assertIn("多余字段", report)

    # --- issue #595：404 子串无锚点误伤 + 空解析假绿 ---

    def test_validate_notfound_phrase_in_body_not_flagged(self):
        """合法文档正文深处含 404 短语不应误判（issue #595 问题1）。

        真实样本：错误码表/排障说明里合法出现「文档不存在」/
        "the documentation could not be found"，在正文深处（数百行后）。
        当前全文无锚点子串匹配会误判 -> _fetch_single_doc unlink 缓存 ->
        每次 resume 重抓同一份有效文档又重判红（thrash）。
        """
        nav = "\n".join(f"nav {i}" for i in range(40))
        body = "正常正文内容" * 200  # > MIN_DOC_CHARS
        # 404 短语埋在正文深处（远超头部窗口）
        doc = nav + "\n" + body + "\n若 文档不存在，请联系管理员。\n"
        self.assertIsNone(verify_api_fields._validate_doc_text(doc))

    def test_validate_notfound_phrase_in_head_still_flagged(self):
        """404 短语出现在页面头部仍应判无效（真实 404 提示在导航后第 17 行）。"""
        head = "\n".join([
            "Customer Stories", "Documentation", "API Explorer", "CardKit",
            "The documentation could not be found.",
        ])
        # 补足 >MIN_DOC_CHARS（模拟渲染较多导航的 404），短语仍在头部窗口内
        doc = head + "\n" + "nav tail\n" * 60
        self.assertIsNotNone(verify_api_fields._validate_doc_text(doc))

    def test_compare_empty_doc_parse_records_warning(self):
        """文档过 validate 但请求/响应字段均解析为空 -> 记 warning，禁止假绿（issue #595 问题2）。"""
        api = verify_api_fields.ApiRecord(
            api_id="1", name="查询", biz_tag="x", meta_project="x",
            meta_version="v1", meta_resource="r", meta_name="q",
            url="GET:/open-apis/x/v1/r/q", doc_path="",
            full_path="/document/x/reference/x-v1/r/q",
        )
        structs = [verify_api_fields.StructFields(name="QResponse", fields=[])]
        # >MIN_DOC_CHARS、无头部 404 短语、但无 Request body / Response body example 段
        doc_text = "导航项内容\n" * 80
        issues = []
        verify_api_fields._compare_doc_against_code(api, structs, doc_text, issues)
        empty = [i for i in issues if i.category == "doc_parse_empty"]
        self.assertEqual(len(empty), 1)
        self.assertEqual(empty[0].severity, "warning")

    def test_compare_body_present_but_no_req_fields_single_warning(self):
        """有 Body 实现但文档未解析出请求字段 -> 仅一条 doc_parse_empty（不与空解析检查重复）。"""
        api = verify_api_fields.ApiRecord(
            api_id="2", name="创建", biz_tag="x", meta_project="x",
            meta_version="v1", meta_resource="r", meta_name="c",
            url="POST:/open-apis/x/v1/r/c", doc_path="",
            full_path="/document/x/reference/x-v1/r/c",
        )
        structs = [verify_api_fields.StructFields(
            name="CBody", fields=[verify_api_fields.FieldInfo("foo", "String", True)],
        )]
        # 无 Request body 段 -> doc_req 空；无 Response body example -> doc_resp 空
        doc_text = "导航项内容\n" * 80
        issues = []
        verify_api_fields._compare_doc_against_code(api, structs, doc_text, issues)
        empty = [i for i in issues if i.category == "doc_parse_empty"]
        self.assertEqual(len(empty), 1)  # 不重复

    # --- issue #599：空解析 warning 对合法无字段 action API 假阳性 ---

    def test_compare_fieldless_api_with_sections_not_failing(self):
        """合法无字段 action API（doc 含标准段标题但段内无字段）-> 降为 info，不报 failing（issue #599）。

        PR #598 的空解析 warning 对 envelope-only 响应的合法 API 假阳性（exit 1）。
        修复：doc 含标准段标题（已渲染/结构正常）但字段空 -> 合法无字段 -> info（不阻断）。
        """
        api = verify_api_fields.ApiRecord(
            api_id="3", name="操作", biz_tag="x", meta_project="x",
            meta_version="v1", meta_resource="r", meta_name="act",
            url="POST:/open-apis/x/v1/r/act", doc_path="",
            full_path="/document/x/reference/x-v1/r/act",
        )
        # 无 Body struct；Response 无字段（操作型 / envelope-only API）
        structs = [verify_api_fields.StructFields(name="ActResponse", fields=[])]
        # doc 渲染了真实段：TOC + section 标题各一次（Response body example 共 2 次），
        # 但 data:{} 无字段；不含 "Request body"（无请求体段）
        doc_text = (
            "The contents of this article\n"
            "Response body example\n"  # TOC 导航项（第 1 次）
            + "API intro padding content. " * 30
            + "\nResponse body example\n"  # 真实 section 标题（第 2 次）
            + '{\n  "code": 0,\n  "msg": "ok",\n  "data": {}\n}\n'
            + "Error code\n"
        )
        issues = []
        verify_api_fields._compare_doc_against_code(api, structs, doc_text, issues)
        failing = [i for i in issues if i.severity in ("warning", "error")]
        self.assertEqual(failing, [])  # 合法无字段 → 不阻断（最多一条 info）

    def test_compare_unrendered_doc_without_sections_still_warning(self):
        """缺标准段标题的未渲染文档 → 仍 warning，文案点明「缺标准段标题」（issue #599 反向分支）。"""
        api = verify_api_fields.ApiRecord(
            api_id="4", name="查询", biz_tag="x", meta_project="x",
            meta_version="v1", meta_resource="r", meta_name="q",
            url="GET:/open-apis/x/v1/r/q", doc_path="",
            full_path="/document/x/reference/x-v1/r/q",
        )
        structs = [verify_api_fields.StructFields(name="QResponse", fields=[])]
        # 未渲染 SPA 外壳：无任一标准段标题，且无字段
        doc_text = "导航壳占位内容\n" * 80
        issues = []
        verify_api_fields._compare_doc_against_code(api, structs, doc_text, issues)
        warns = [
            i for i in issues
            if i.category == "doc_parse_empty" and i.severity == "warning"
        ]
        self.assertEqual(len(warns), 1)
        self.assertIn("缺标准段标题", warns[0].detail)  # 钉 #599 warning 分支文案

    def test_compare_toc_only_shell_still_warning_not_false_green(self):
        """部分渲染 shell（TOC 含段标题子串但 section bodies 未渲染）-> 仍 warning，不假绿。

        回归保护：_doc_has_standard_sections 用子串时被 TOC 导航项误导（对抗验证发现），
        未渲染 shell 误判 info -> exit 0 假绿（违背 #595）。改用 count>=2（TOC + 真实段
        各一次）后修复：真实文档 Response body example 恒 2 次，TOC-only shell 仅 1 次。
        """
        api = verify_api_fields.ApiRecord(
            api_id="5", name="操作", biz_tag="x", meta_project="x",
            meta_version="v1", meta_resource="r", meta_name="act",
            url="POST:/open-apis/x/v1/r/act", doc_path="",
            full_path="/document/x/reference/x-v1/r/act",
        )
        structs = [verify_api_fields.StructFields(name="ActResponse", fields=[])]
        # TOC-only shell：段标题仅在 TOC 出现 1 次，无 section bodies
        doc_text = (
            "The contents of this article\n"
            "Request body\nQuery parameters\nResponse body example\n"  # TOC 导航项（各 1 次）
            + "nav padding content line\n" * 80  # 凑长度，无真实 section body
        )
        issues = []
        verify_api_fields._compare_doc_against_code(api, structs, doc_text, issues)
        warns = [
            i for i in issues
            if i.category == "doc_parse_empty" and i.severity == "warning"
        ]
        self.assertEqual(len(warns), 1)  # TOC-only shell -> warning，不假绿


if __name__ == "__main__":
    unittest.main()
