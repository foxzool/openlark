"""codegen_render 渲染器测试。结构化契约断言 + 真实样本渲染。"""

from __future__ import annotations

import json
import unittest
from pathlib import Path

from tools.api_contracts.codegen_ir import parse_api_schema_to_ir
from tools.api_contracts.codegen_render import (
    _emit_token_decl,
    render_api_file,
    render_endpoint_const_snippet,
)
from tools.api_contracts.models import ApiIdentity

SAMPLES = Path(__file__).resolve().parent.parent / "schema_cache" / "samples"


def _api(meta_name="create", resource="message", name="发送消息") -> ApiIdentity:
    return ApiIdentity(
        api_id="1",
        name=name,
        biz_tag="im",
        meta_project="im",
        meta_version="v1",
        meta_resource=resource,
        meta_name=meta_name,
        url="POST:/open-apis/im/v1/messages",
        doc_path="https://open.feishu.cn/document/server-docs/im-v1/message/create",
        expected_file="im/im/v1/message/create.rs",
        full_path="/document/x",
    )


SCHEMA_POST = {
    "httpMethod": "post",
    "path": "/open-apis/im/v1/messages",
    "parameters": [
        {"in": "query", "name": "receive_id_type", "required": True,
         "schema": {"type": "string", "options": [{"name": "open_id", "value": "open_id"}]}},
        {"in": "query", "name": "page_size", "required": False, "schema": {"type": "integer"}},
    ],
    "requestBody": {"content": {"application/json": {"schema": {"properties": [
        {"name": "receive_id", "type": "string", "required": True, "description": "接收者"},
        {"name": "msg_type", "type": "string", "required": True},
        {"name": "uuid", "type": "string", "required": False},
    ]}}}},
    "responses": {"200": {"content": {"application/json": {"schema": {"properties": [
        {"name": "code", "type": "integer"},
        {"name": "msg", "type": "string"},
        {"name": "data", "type": "object", "objectName": "CreateMessageResp",
         "properties": [{"name": "message_id", "type": "string"}]},
    ]}}}}},
}

SCHEMA_GET_PATH = {
    "httpMethod": "get",
    "path": "/open-apis/im/v1/messages/:message_id",
    "parameters": [
        {"in": "path", "name": "message_id", "required": True, "schema": {"type": "string"}}
    ],
    "responses": {"200": {"content": {"application/json": {"schema": {"properties": [
        {"name": "code", "type": "integer"},
        {"name": "msg", "type": "string"},
        {"name": "data", "type": "object", "properties": [{"name": "message_id", "type": "string"}]},
    ]}}}}},
}

# 无 body 无 path param 无 required query → 不该 import validate_required
SCHEMA_NO_VALIDATE = {
    "httpMethod": "get",
    "path": "/open-apis/im/v1/messages",
    "parameters": [
        {"in": "query", "name": "page_size", "required": False, "schema": {"type": "integer"}}
    ],
    "responses": {"200": {"content": {"application/json": {"schema": {"properties": [
        {"name": "code", "type": "integer"}, {"name": "msg", "type": "string"}]}}}}},
}


def _assert_core_contracts(testcase, code: str) -> None:
    """5 条核心契约 + issue41 范式断言。"""
    # 范式（过 E001/E002/E003）
    testcase.assertIn("pub async fn execute_with_options", code)
    testcase.assertIn("RequestOption::default()", code)
    testcase.assertIn("Transport::request(req, &self.config, Some(option))", code)
    # 取 data（typed: response.data.ok_or_else；value: extract_response_data）
    testcase.assertTrue(
        "response.data.ok_or_else" in code or "extract_response_data(resp," in code
    )
    # 端点用常量（契约 5）
    testcase.assertIn("endpoints::", code)
    # 契约 1：Config owned 非 Arc
    testcase.assertNotIn("Arc<Config>", code)
    # 契约 3：禁止 reqwest
    testcase.assertNotIn("reqwest::Client", code)
    # 契约 3：禁止 Transport::request(..., None)
    testcase.assertNotIn(", None).await", code)
    # codegen 标记
    testcase.assertIn("由 codegen 自动生成", code)


SCHEMA_TENANT_ONLY = {
    "httpMethod": "post",
    "path": "/open-apis/contact/v3/departments",
    "parameters": [],
    "requestBody": {"content": {"application/json": {"schema": {"properties": [
        {"name": "name", "type": "string", "required": True},
    ]}}}},
    "responses": {"200": {"content": {"application/json": {"schema": {"properties": [
        {"name": "code", "type": "integer"}, {"name": "msg", "type": "string"}]}}}}},
    "security": {"supportedAccessToken": ["tenant_access_token"]},
}


class EndpointSnippetTest(unittest.TestCase):
    def test_snippet(self):
        ir = parse_api_schema_to_ir(_api(), SCHEMA_POST)
        snippet = render_endpoint_const_snippet(ir)
        self.assertEqual(snippet, 'pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";')


class RenderPostTest(unittest.TestCase):
    def setUp(self) -> None:
        self.ir = parse_api_schema_to_ir(_api(), SCHEMA_POST)
        self.code = render_api_file(self.ir)

    def test_core_contracts(self):
        _assert_core_contracts(self, self.code)

    def test_body_struct(self):
        self.assertIn("pub struct CreateMessageBody {", self.code)
        self.assertIn("pub receive_id: String,", self.code)
        self.assertIn("pub msg_type: String,", self.code)
        self.assertIn('    #[serde(skip_serializing_if = "Option::is_none")]', self.code)
        self.assertIn("pub uuid: Option<String>,", self.code)
        self.assertIn('#[serde(rename_all = "camelCase")]', self.code)

    def test_required_body_validation(self):
        self.assertIn('validate_required!(body.receive_id, "receive_id 不能为空");', self.code)
        self.assertIn('validate_required!(body.msg_type, "msg_type 不能为空");', self.code)

    def test_required_query_ok_or_else(self):
        # required query param 用 ok_or_else（Option<String> 无 Validatable）
        self.assertIn("let receive_id_type = self.receive_id_type.ok_or_else", self.code)
        self.assertIn('openlark_core::error::validation_error(', self.code)
        self.assertIn('.query("receive_id_type", receive_id_type)', self.code)

    def test_optional_query_opt(self):
        self.assertIn('.query_opt("page_size", self.page_size)', self.code)

    def test_endpoint_const_usage(self):
        self.assertIn("ApiRequest::post(IM_V1_MESSAGES)", self.code)

    def test_execute_delegates(self):
        self.assertIn("pub async fn execute(self, body: CreateMessageBody)", self.code)
        self.assertIn("self.execute_with_options(body,", self.code)
        self.assertIn("RequestOption::default())", self.code)
        self.assertIn(".await", self.code)

    def test_no_response_wrapper(self):
        # 契约 2：绝不生成 XxxResponse{data:..} 包装
        self.assertNotIn("struct CreateMessageResponse", self.code)
        self.assertNotIn("data: Option<", self.code)

    def test_tests_block(self):
        self.assertIn("#[cfg(test)]", self.code)
        self.assertIn("fn test_body_construct()", self.code)

    def test_typed_response(self):
        # typed response：response struct + ApiResponseTrait impl + execute 返回 typed
        self.assertIn("pub struct CreateMessageResp {", self.code)
        self.assertIn("impl ApiResponseTrait for CreateMessageResp {", self.code)
        self.assertIn(
            "pub async fn execute(self, body: CreateMessageBody) -> SDKResult<CreateMessageResp>",
            self.code,
        )
        self.assertIn("ApiRequest<CreateMessageResp>", self.code)
        self.assertIn("response.data.ok_or_else", self.code)


class RenderGetPathTest(unittest.TestCase):
    def setUp(self) -> None:
        self.ir = parse_api_schema_to_ir(_api(meta_name="get"), SCHEMA_GET_PATH)
        self.code = render_api_file(self.ir)

    def test_core_contracts(self):
        _assert_core_contracts(self, self.code)

    def test_no_body(self):
        # GET 无 body：execute(self) 无 body 参数，不调 serialize_params
        self.assertIn("pub async fn execute(self) -> SDKResult<GetMessageResponse>", self.code)
        self.assertNotIn("serialize_params", self.code)
        self.assertNotIn("Body", self.code)

    def test_path_format_url(self):
        self.assertIn('format!("{}/{}", IM_V1_MESSAGES, self.message_id)', self.code)
        self.assertIn("ApiRequest::get(", self.code)

    def test_path_param_validation(self):
        self.assertIn('validate_required!(self.message_id, "message_id 不能为空");', self.code)


class RenderNoValidateTest(unittest.TestCase):
    """无必填字段 → 不该 import validate_required（避免 unused import，clippy -Dwarnings）。"""

    def test_no_validate_required_import(self):
        ir = parse_api_schema_to_ir(_api(meta_name="list"), SCHEMA_NO_VALIDATE)
        code = render_api_file(ir)
        self.assertNotIn("validate_required", code)
        self.assertNotIn("Body", code)  # 无 body


class RealSampleTest(unittest.TestCase):
    def test_render_im_message_create(self):
        sample = SAMPLES / "6946222931479527425.json"
        if not sample.exists():
            self.skipTest("Step 0 dump 样本不存在")
        api_schema = (
            json.loads(sample.read_text(encoding="utf-8"))
            .get("data", {}).get("schema", {}).get("apiSchema")
        )
        ir = parse_api_schema_to_ir(
            ApiIdentity(
                api_id="6946222931479527425", name="发送消息", biz_tag="im",
                meta_project="im", meta_version="v1", meta_resource="message", meta_name="create",
                url="POST:/open-apis/im/v1/messages",
                doc_path="https://open.feishu.cn/document/server-docs/im-v1/message/create",
                expected_file="im/im/v1/message/create.rs",
                full_path="/document/x",
            ),
            api_schema,
        )
        code = render_api_file(ir)
        _assert_core_contracts(self, code)
        self.assertIn("pub struct CreateMessageBody {", code)
        # 真实 schema 的必填字段
        self.assertIn("pub receive_id: String,", code)
        self.assertIn("pub content: String,", code)
        # 把渲染产物落一份到 /tmp 便于人工 review（测试不依赖它）
        Path("/tmp/codegen_create.rs").write_text(code, encoding="utf-8")


class TokenDeclTest(unittest.TestCase):
    """_emit_token_decl 映射逻辑：默认/单值/空/未知/集合顺序。"""

    def test_empty(self):
        self.assertIsNone(_emit_token_decl(()))

    def test_default_set_unordered(self):
        # 默认 {User, Tenant} → None，无论 dump 顺序
        self.assertIsNone(_emit_token_decl(("tenant_access_token", "user_access_token")))
        self.assertIsNone(_emit_token_decl(("user_access_token", "tenant_access_token")))

    def test_tenant_only(self):
        self.assertEqual(
            _emit_token_decl(("tenant_access_token",)),
            "vec![AccessTokenType::Tenant]",
        )

    def test_user_only(self):
        self.assertEqual(
            _emit_token_decl(("user_access_token",)),
            "vec![AccessTokenType::User]",
        )

    def test_app_only(self):
        self.assertEqual(
            _emit_token_decl(("app_access_token",)),
            "vec![AccessTokenType::App]",
        )

    def test_unknown_degrades_to_default(self):
        self.assertIsNone(_emit_token_decl(("lambda_access_token",)))

    def test_unknown_skipped_keeps_known(self):
        # 混合：已知 tenant + 未知 → 只输出 tenant（非默认 → 生成）
        self.assertEqual(
            _emit_token_decl(("tenant_access_token", "lambda_access_token")),
            "vec![AccessTokenType::Tenant]",
        )


class RenderTokenTest(unittest.TestCase):
    """G5 渲染集成：[tenant] 单值 → 生成 token 声明 + import；默认 → 不生成。"""

    def test_tenant_only_emits_decl_and_import(self):
        ir = parse_api_schema_to_ir(_api(resource="department"), SCHEMA_TENANT_ONLY)
        code = render_api_file(ir)
        self.assertIn(
            ".with_supported_access_token_types(vec![AccessTokenType::Tenant])",
            code,
        )
        self.assertIn("constants::AccessTokenType", code)  # 条件 import

    def test_default_no_token_decl(self):
        # SCHEMA_POST 无 security → supported_access_tokens=() → 不生成、不 import
        ir = parse_api_schema_to_ir(_api(), SCHEMA_POST)
        code = render_api_file(ir)
        self.assertNotIn("with_supported_access_token_types", code)
        self.assertNotIn("AccessTokenType", code)


class RenderNestedArrayStructTest(unittest.TestCase):
    """_reachable 递归 array 内层 struct（array of object 不漏渲染）。"""

    def test_array_of_object_struct_rendered(self):
        schema = {
            "httpMethod": "post",
            "path": "/open-apis/im/v1/messages",
            "requestBody": {"content": {"application/json": {"schema": {"properties": [
                {"name": "tags", "type": "array", "required": True,
                 "items": {"type": "object", "properties": [{"name": "key", "type": "string"}]}},
            ]}}}},
            "responses": {"200": {"content": {"application/json": {"schema": {"properties": [
                {"name": "code", "type": "integer"}]}}}}},
        }
        ir = parse_api_schema_to_ir(_api(), schema)
        code = render_api_file(ir)
        # 嵌套 struct（tags 的 item）应被渲染，不漏
        self.assertIn("pub struct CreateMessageBodyTagsItem {", code)
        self.assertIn("Vec<CreateMessageBodyTagsItem>", code)


if __name__ == "__main__":
    unittest.main()
