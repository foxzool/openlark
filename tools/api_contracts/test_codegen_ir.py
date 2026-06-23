"""codegen_ir parser 单元测试。fixture dict 覆盖各分支 + 真实 dump 样本集成测试。"""

from __future__ import annotations

import json
import unittest
from pathlib import Path

from tools.api_contracts.codegen_ir import (
    TypeArray,
    TypeOpaque,
    TypePrimitive,
    TypeStructRef,
    derive_endpoint_const,
    field_rust_name,
    iter_properties,
    parse_api_schema_to_ir,
    request_struct_name,
    to_pascal,
)
from tools.api_contracts.models import ApiIdentity

SAMPLES = Path(__file__).resolve().parent.parent / "schema_cache" / "samples"


def _api(
    api_id: str = "1",
    name: str = "发送消息",
    resource: str = "message",
    meta_name: str = "create",
) -> ApiIdentity:
    return ApiIdentity(
        api_id=api_id,
        name=name,
        biz_tag="im",
        meta_project="im",
        meta_version="v1",
        meta_resource=resource,
        meta_name=meta_name,
        url="POST:/open-apis/im/v1/messages",
        doc_path="doc",
        expected_file="im/im/v1/message/create.rs",
        full_path="/document/x",
    )


# --- POST body + query 枚举参数 + response.data --- #
SCHEMA_POST = {
    "httpMethod": "post",
    "path": "/open-apis/im/v1/messages",
    "parameters": [
        {
            "in": "query",
            "name": "receive_id_type",
            "required": True,
            "schema": {
                "type": "string",
                "format": "user_id_type",
                "default": "open_id",
                "options": [
                    {"name": "open_id", "value": "open_id", "description": "open"},
                    {"name": "union_id", "value": "union_id", "description": "union"},
                ],
            },
        }
    ],
    "requestBody": {
        "content": {
            "application/json": {
                "schema": {
                    "properties": [
                        {"name": "receive_id", "type": "string", "required": True, "description": "接收者"},
                        {"name": "msg_type", "type": "string", "required": True},
                        {"name": "uuid", "type": "string", "required": False},
                    ]
                }
            }
        }
    },
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
                                "objectName": "CreateMessageResp",
                                "properties": [
                                    {"name": "message_id", "type": "string", "description": "消息 ID"},
                                    {
                                        "name": "sender",
                                        "type": "object",
                                        "properties": [{"name": "id", "type": "string"}],
                                    },
                                ],
                            },
                        ]
                    }
                }
            }
        }
    },
    "security": {"supportedAccessToken": ["tenant_access_token", "user_access_token"]},
}

# --- GET + path param --- #
SCHEMA_GET_PATH = {
    "httpMethod": "get",
    "path": "/open-apis/im/v1/messages/:message_id",
    "parameters": [
        {"in": "path", "name": "message_id", "required": True, "schema": {"type": "string"}}
    ],
    "requestBody": {"content": {"application/json": {"schema": {"properties": []}}}},
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
                                "properties": [{"name": "message_id", "type": "string"}],
                            },
                        ]
                    }
                }
            }
        }
    },
}

# --- 各种类型分支 --- #
SCHEMA_TYPES = {
    "httpMethod": "post",
    "path": "/open-apis/x/v1/y",
    "requestBody": {
        "content": {
            "application/json": {
                "schema": {
                    "properties": [
                        {"name": "tags", "type": "array", "items": {"type": "string"}},
                        {"name": "ref_field", "$ref": "#/components/schemas/Z", "type": "object"},
                        {"name": "no_type", "description": "无 type"},
                        {"name": "count", "type": "integer", "format": "int64"},
                        {"name": "flag", "type": "boolean"},
                    ]
                }
            }
        }
    },
    "responses": {
        "200": {
            "content": {
                "application/json": {
                    "schema": {
                        "properties": [
                            {"name": "code", "type": "integer"},
                            {"name": "msg", "type": "string"},
                        ]
                    }
                }
            }
        }
    },
}


class DeriveEndpointConstTest(unittest.TestCase):
    def test_no_params(self):
        name, val, tmpl, fmt, params = derive_endpoint_const("/open-apis/im/v1/messages")
        self.assertEqual(name, "IM_V1_MESSAGES")
        self.assertEqual(val, "/open-apis/im/v1/messages")
        self.assertEqual(tmpl, "")
        self.assertFalse(fmt)
        self.assertEqual(params, [])

    def test_colon_param(self):
        name, val, tmpl, fmt, params = derive_endpoint_const(
            "/open-apis/im/v1/messages/:message_id"
        )
        self.assertEqual(name, "IM_V1_MESSAGES")  # 复用 base 常量
        self.assertEqual(val, "/open-apis/im/v1/messages")
        self.assertEqual(tmpl, "{}/{}")
        self.assertTrue(fmt)
        self.assertEqual(params, ["message_id"])

    def test_brace_param(self):
        name, val, tmpl, fmt, params = derive_endpoint_const(
            "/open-apis/im/v1/messages/{message_id}"
        )
        self.assertEqual(tmpl, "{}/{}")
        self.assertTrue(fmt)
        self.assertEqual(params, ["message_id"])

    def test_middle_param_reuses_base_const(self):
        # /messages/:id/reply → base /messages → IM_V1_MESSAGES（复用），tmpl 含 /reply
        name, val, tmpl, fmt, params = derive_endpoint_const(
            "/open-apis/im/v1/messages/:message_id/reply"
        )
        self.assertEqual(name, "IM_V1_MESSAGES")
        self.assertEqual(tmpl, "{}/{}/reply")
        self.assertEqual(params, ["message_id"])

    def test_contact_department_children(self):
        name, val, tmpl, fmt, params = derive_endpoint_const(
            "/open-apis/contact/v3/departments/:department_id/children"
        )
        self.assertEqual(name, "CONTACT_V3_DEPARTMENTS")  # base 到第一个参数前
        self.assertEqual(tmpl, "{}/{}/children")
        self.assertEqual(params, ["department_id"])


class IterPropertiesTest(unittest.TestCase):
    def test_dict_form(self):
        result = iter_properties({"a": {"type": "string"}, "b": {"type": "integer"}})
        self.assertEqual([n for n, _ in result], ["a", "b"])

    def test_list_form(self):
        result = iter_properties([{"name": "a", "type": "string"}])
        self.assertEqual([n for n, _ in result], ["a"])

    def test_empty(self):
        self.assertEqual(iter_properties(None), [])
        self.assertEqual(iter_properties([]), [])


class NamingTest(unittest.TestCase):
    def test_to_pascal(self):
        self.assertEqual(to_pascal("create"), "Create")
        self.assertEqual(to_pascal("receive_id_type"), "ReceiveIdType")
        self.assertEqual(to_pascal("update_department_id"), "UpdateDepartmentId")

    def test_field_rust_name_keyword(self):
        self.assertEqual(field_rust_name("type"), "type_")
        self.assertEqual(field_rust_name("self"), "self_")
        self.assertEqual(field_rust_name("normal"), "normal")

    def test_request_struct_name(self):
        self.assertEqual(request_struct_name(_api()), "CreateMessageRequest")


class ParsePostTest(unittest.TestCase):
    def setUp(self) -> None:
        self.ir = parse_api_schema_to_ir(_api(), SCHEMA_POST)

    def test_endpoint(self):
        self.assertEqual(self.ir.method, "POST")
        self.assertEqual(self.ir.endpoint_const_name, "IM_V1_MESSAGES")
        self.assertFalse(self.ir.needs_format)

    def test_query_param_with_options(self):
        self.assertEqual(len(self.ir.query_params), 1)
        qp = self.ir.query_params[0]
        self.assertEqual(qp.name, "receive_id_type")
        self.assertTrue(qp.required)
        self.assertEqual(len(qp.options), 2)
        self.assertEqual(qp.options[0], ("open_id", "open"))

    def test_body_struct(self):
        b = self.ir.body_struct
        self.assertIsNotNone(b)
        self.assertEqual(b.rust_name, "CreateMessageBody")
        names = {f.name: f for f in b.fields}
        self.assertTrue(names["receive_id"].required)
        self.assertTrue(names["msg_type"].required)
        self.assertFalse(names["uuid"].required)

    def test_response_struct_from_object_name(self):
        r = self.ir.response_struct
        self.assertIsNotNone(r)
        self.assertEqual(r.rust_name, "CreateMessageResp")
        names = {f.name for f in r.fields}
        self.assertIn("message_id", names)
        self.assertIn("sender", names)

    def test_nested_struct_registered(self):
        # sender 是嵌套 object → TypeStructRef + 注册到 ir.structs
        sender_field = next(f for f in self.ir.response_struct.fields if f.name == "sender")
        self.assertIsInstance(sender_field.type_expr, TypeStructRef)
        self.assertIn(sender_field.type_expr.struct_key, self.ir.structs)

    def test_supported_access_tokens(self):
        self.assertEqual(
            self.ir.supported_access_tokens,
            ("tenant_access_token", "user_access_token"),
        )


class ParseGetPathTest(unittest.TestCase):
    def test_path_param(self):
        ir = parse_api_schema_to_ir(_api(meta_name="get"), SCHEMA_GET_PATH)
        self.assertEqual(ir.method, "GET")
        self.assertEqual(ir.endpoint_const_name, "IM_V1_MESSAGES")
        self.assertTrue(ir.needs_format)
        self.assertEqual(len(ir.path_params), 1)
        self.assertEqual(ir.path_params[0].name, "message_id")
        self.assertEqual(ir.path_params[0].type_expr.rust, "String")


class ParseTypesTest(unittest.TestCase):
    def setUp(self) -> None:
        self.ir = parse_api_schema_to_ir(_api(), SCHEMA_TYPES)

    def _field(self, name: str):
        return next(f for f in self.ir.body_struct.fields if f.name == name)

    def test_array_items(self):
        self.assertIsInstance(self._field("tags").type_expr, TypeArray)
        item = self._field("tags").type_expr.item
        self.assertEqual(item.rust, "String")

    def test_ref_degrades(self):
        f = self._field("ref_field")
        self.assertIsInstance(f.type_expr, TypeOpaque)
        self.assertEqual(f.type_expr.reason, "unsupported_ref")
        self.assertTrue(any("$ref" in n for n in self.ir.notes))

    def test_missing_type_degrades(self):
        f = self._field("no_type")
        self.assertIsInstance(f.type_expr, TypeOpaque)
        self.assertEqual(f.type_expr.reason, "unknown_type")

    def test_int64(self):
        self.assertEqual(self._field("count").type_expr.rust, "i64")

    def test_bool(self):
        self.assertEqual(self._field("flag").type_expr.rust, "bool")

    def test_no_data_response(self):
        # response 只有 code/msg，无 data → response_struct None
        self.assertIsNone(self.ir.response_struct)


class RealSampleTest(unittest.TestCase):
    """用 Step 0 真实 dump 的 im/message/create 样本验证 parser。"""

    def test_im_message_create(self):
        sample = SAMPLES / "6946222931479527425.json"
        if not sample.exists():
            self.skipTest("Step 0 dump 样本不存在（先跑 dump_samples.py）")
        api_schema = (
            json.loads(sample.read_text(encoding="utf-8"))
            .get("data", {})
            .get("schema", {})
            .get("apiSchema")
        )
        api = _api(api_id="6946222931479527425")
        ir = parse_api_schema_to_ir(api, api_schema)

        self.assertEqual(ir.method, "POST")
        self.assertEqual(ir.endpoint_const_name, "IM_V1_MESSAGES")
        self.assertFalse(ir.needs_format)
        # query: receive_id_type
        qp_names = {p.name for p in ir.query_params}
        self.assertIn("receive_id_type", qp_names)
        # body: receive_id/msg_type/content required
        b = ir.body_struct
        self.assertEqual(b.rust_name, "CreateMessageBody")
        body_names = {f.name: f for f in b.fields}
        for required_field in ("receive_id", "msg_type", "content"):
            self.assertIn(required_field, body_names)
            self.assertTrue(body_names[required_field].required)
        # response data 含 message_id
        self.assertIsNotNone(ir.response_struct)
        resp_names = {f.name for f in ir.response_struct.fields}
        self.assertIn("message_id", resp_names)
        # Token 数据源
        self.assertTrue(len(ir.supported_access_tokens) > 0)


if __name__ == "__main__":
    unittest.main()
