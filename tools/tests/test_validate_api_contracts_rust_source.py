import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.rust_source import (
    EndpointResolver,
    MULTIPART_FORM_STRUCT_NAME,
    extract_access_token_types,
    extract_endpoint_calls,
    extract_manual_auth_token,
    extract_rust_response_fields,
    extract_rust_fields,
    extract_structs,
    load_endpoint_constants,
    load_enum_endpoints,
    load_enum_methods,
    resolve_format_expression,
    scan_api_file,
)

REPO_ROOT = Path(__file__).resolve().parents[2]


class RustSourceContractTests(unittest.TestCase):
    def test_load_endpoint_constants_resolves_aliases(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            src = Path(temp_dir)
            (src / "endpoints").mkdir()
            (src / "endpoints" / "mod.rs").write_text(
                '\n'.join(
                    [
                        'pub const BANK_CARD: &str = "/open-apis/document_ai/v1/bank_card/recognize";',
                        "pub const BANK_CARD_ALIAS: &str = BANK_CARD;",
                    ]
                ),
                encoding="utf-8",
            )

            constants = load_endpoint_constants(src)

        self.assertEqual(
            constants["BANK_CARD_ALIAS"],
            "/open-apis/document_ai/v1/bank_card/recognize",
        )

    def test_extract_endpoint_calls_resolves_direct_constant(self):
        text = """
        let req: ApiRequest<Response> =
            ApiRequest::post(DOCUMENT_AI_BANK_CARD_RECOGNIZE)
                .body(body);
        """
        resolver = EndpointResolver(
            {"DOCUMENT_AI_BANK_CARD_RECOGNIZE": "/open-apis/document_ai/v1/bank_card/recognize"}
        )

        calls = extract_endpoint_calls(text, resolver)

        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0].method, "POST")
        self.assertEqual(calls[0].resolved_path, "/open-apis/document_ai/v1/bank_card/recognize")

    def test_extract_endpoint_calls_marks_to_url_unresolved(self):
        text = "let req: ApiRequest<Response> = ApiRequest::get(&api_endpoint.to_url());"

        calls = extract_endpoint_calls(text, EndpointResolver({}))

        self.assertEqual(len(calls), 1)
        self.assertFalse(calls[0].is_resolved)
        self.assertIn("to_url", calls[0].unresolved_reason)

    def test_resolve_format_expression_with_constant_and_parameter(self):
        resolved = resolve_format_expression(
            'format!("{}/{}", IM_V1_CHATS, self.chat_id)',
            {"IM_V1_CHATS": "/open-apis/im/v1/chats"},
        )

        self.assertEqual(resolved, "/open-apis/im/v1/chats/{param}")

    def test_resolve_format_expression_with_captured_constant(self):
        resolved = resolve_format_expression(
            'format!("{IM_V1_CHATS}/search")',
            {"IM_V1_CHATS": "/open-apis/im/v1/chats"},
        )

        self.assertEqual(resolved, "/open-apis/im/v1/chats/search")

    def test_scan_api_file_extracts_endpoint_contract(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            src = Path(temp_dir)
            (src / "ai" / "document_ai" / "v1" / "bank_card").mkdir(parents=True)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/document_ai/v1/bank_card/recognize";',
                encoding="utf-8",
            )
            (src / "ai" / "document_ai" / "v1" / "bank_card" / "recognize.rs").write_text(
                "let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);",
                encoding="utf-8",
            )

            contract = scan_api_file(src, "ai/document_ai/v1/bank_card/recognize.rs")

        self.assertIsNotNone(contract)
        assert contract is not None
        self.assertEqual(contract.endpoint_calls[0].resolved_path, "/open-apis/document_ai/v1/bank_card/recognize")

    def test_extract_rust_fields_uses_serde_rename_and_optional_type(self):
        text = """
        #[derive(Debug, Clone, Serialize)]
        pub struct BankCardRecognizeBody {
            #[serde(rename = "file")]
            pub file_token: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub is_async: Option<bool>,
        }

        pub struct BankCardRecognizeRequest {
            pub config: Config,
        }
        """

        fields = extract_rust_fields(text)

        self.assertEqual([field.serialized_name for field in fields], ["file", "is_async"])
        self.assertFalse(fields[0].optional)
        self.assertTrue(fields[1].optional)

    def test_extract_rust_fields_applies_camel_case_rename_all(self):
        text = """
        #[serde(rename_all = "camelCase")]
        pub struct ListQuery {
            pub page_size: Option<i32>,
        }
        """

        fields = extract_rust_fields(text)

        self.assertEqual(fields[0].serialized_name, "pageSize")

    def test_extract_rust_fields_maps_file_content_to_multipart_file_field(self):
        text = """
        pub struct BankCardRecognizeBody {
            #[serde(skip_serializing)]
            pub file: Vec<u8>,
        }

        let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD)
            .body(body)
            .file_content(body.file.clone());
        """

        fields = extract_rust_fields(text)

        self.assertEqual([field.serialized_name for field in fields], ["file"])
        self.assertEqual(fields[0].struct_name, "MultipartFile")
        self.assertFalse(fields[0].optional)

    def test_extract_rust_response_fields_reads_response_and_result_structs(self):
        text = """
        pub struct BankCardRecognizeResponse {
            pub data: Option<BankCardRecognizeResult>,
        }

        pub struct BankCardRecognizeResult {
            pub parsing_result: Option<ParsingResult>,
        }

        pub struct BankCardRecognizeBody {
            pub file_token: String,
        }
        """

        fields = extract_rust_response_fields(text)

        self.assertEqual([field.serialized_name for field in fields], ["data", "parsing_result"])

    def test_extract_rust_response_fields_reads_resp_suffix_struct(self):
        # baike 的 MatchEntityResp 等用 Resp 后缀命名响应 struct
        text = """
        pub struct MatchEntityResp {
            #[serde(default)]
            pub results: Vec<MatchEntityResult>,
        }
        """

        fields = extract_rust_response_fields(text)

        self.assertEqual([field.serialized_name for field in fields], ["results"])

    def test_extract_rust_fields_collects_multipart_meta_struct_fields(self):
        # drive 上传：局部 UploadMeta 结构体组织 multipart 表单字段
        text = """
        pub struct UploadAllResponse {
            pub file_token: String,
        }

        pub async fn execute(self) -> SDKResult<UploadAllResponse> {
            #[derive(Serialize)]
            struct UploadMeta {
                file_name: String,
                parent_type: String,
                parent_node: String,
                size: usize,
                #[serde(skip_serializing_if = "Option::is_none")]
                checksum: Option<String>,
            }

            let request = ApiRequest::<UploadAllResponse>::post(&api_endpoint.to_url())
                .json_body(&meta)
                .file_content(self.file);
        }
        """

        fields = extract_rust_fields(text)

        names = {field.serialized_name for field in fields}
        self.assertIn("file", names)
        self.assertIn("file_name", names)
        self.assertIn("parent_type", names)
        self.assertIn("parent_node", names)
        self.assertIn("size", names)
        self.assertIn("checksum", names)

    def test_extract_rust_fields_collects_json_literal_multipart_keys(self):
        # baike 上传：serde_json::json!({"name": ..., "__file_name": ...})
        text = """
        pub struct UploadFileResponse {
            pub file_token: String,
        }

        let body = serde_json::json!({
            "name": name,
            "__file_name": name,
        });

        let api_request: ApiRequest<UploadFileResponse> =
            ApiRequest::post(&BaikeApiV1::FileUpload.to_url())
                .body(body)
                .file_content(self.file);
        """

        fields = extract_rust_fields(text)

        names = {field.serialized_name for field in fields}
        self.assertIn("file", names)
        self.assertIn("name", names)
        # 内部字段（下划线前缀）不应作为表单字段
        self.assertNotIn("__file_name", names)

    def test_scan_api_file_detects_flatten_value_passthrough(self):
        # docx block patch：#[serde(flatten)] update: serde_json::Value（透传写法）
        text = """
        pub struct UpdateDocumentBlockParams {
            #[serde(skip_serializing)]
            pub document_id: String,
            #[serde(flatten)]
            pub update: serde_json::Value,
        }

        let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);
        """

        with tempfile.TemporaryDirectory() as temp_dir:
            src = Path(temp_dir)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/x/v1/y";',
                encoding="utf-8",
            )
            (src / "docx").mkdir(parents=True)
            (src / "docx" / "patch.rs").write_text(text, encoding="utf-8")

            contract = scan_api_file(src, "docx/patch.rs")

        self.assertIsNotNone(contract)
        assert contract is not None
        self.assertTrue(contract.has_flatten_value_passthrough)

    def test_scan_api_file_detects_flatten_typed_enum_passthrough(self):
        # docx block patch：#[serde(flatten)] update: BlockUpdateOperation（typed 枚举写法）
        text = """
        pub struct UpdateDocumentBlockParams {
            #[serde(skip_serializing)]
            pub document_id: String,
            #[serde(flatten)]
            pub update: BlockUpdateOperation,
        }

        let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);
        """

        with tempfile.TemporaryDirectory() as temp_dir:
            src = Path(temp_dir)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/x/v1/y";',
                encoding="utf-8",
            )
            (src / "docx").mkdir(parents=True)
            (src / "docx" / "patch.rs").write_text(text, encoding="utf-8")

            contract = scan_api_file(src, "docx/patch.rs")

        self.assertIsNotNone(contract)
        assert contract is not None
        self.assertTrue(contract.has_flatten_value_passthrough)




class DocsCatalogEndpointResolverTests(unittest.TestCase):
    """#568：docs 域 CatalogEndpoint / .to_request() 解析盲区。"""

    def test_load_enum_endpoints_reads_api_endpoints_submodules(self):
        src = REPO_ROOT / "crates" / "openlark-docs" / "src"
        endpoints = load_enum_endpoints(src, load_endpoint_constants(src))
        self.assertIn("LingoApiV1::RepoList", endpoints)
        self.assertEqual(endpoints["LingoApiV1::RepoList"], "/open-apis/lingo/v1/repos")
        self.assertIn("BaseApiV2::RoleCreate", endpoints)
        self.assertEqual(
            endpoints["BaseApiV2::RoleCreate"],
            "/open-apis/base/v2/apps/{param}/roles",
        )

    def test_load_enum_endpoints_keeps_baike_and_lingo_path_prefixes_distinct(self):
        src = REPO_ROOT / "crates" / "openlark-docs" / "src"
        endpoints = load_enum_endpoints(src, load_endpoint_constants(src))
        self.assertEqual(
            endpoints["BaikeApiV1::DraftUpdate"],
            "/open-apis/baike/v1/drafts/{param}",
        )
        self.assertEqual(
            endpoints["LingoApiV1::DraftUpdate"],
            "/open-apis/lingo/v1/drafts/{param}",
        )
        self.assertEqual(
            endpoints["LingoApiV1::EntityMatch"],
            "/open-apis/lingo/v1/entities/match",
        )
        self.assertEqual(
            endpoints["BaikeApiV1::EntityMatch"],
            "/open-apis/baike/v1/entities/match",
        )

    def test_load_enum_methods_from_catalog_endpoint_impl(self):
        src = REPO_ROOT / "crates" / "openlark-docs" / "src"
        methods = load_enum_methods(src)
        self.assertEqual(methods.get("LingoApiV1::RepoList"), "GET")
        self.assertEqual(methods.get("LingoApiV1::DraftUpdate"), "PUT")
        self.assertEqual(methods.get("LingoApiV1::EntityDelete"), "DELETE")
        self.assertEqual(methods.get("BaseApiV2::RoleCreate"), "POST")
        self.assertEqual(methods.get("BaikeApiV1::DraftUpdate"), "PUT")
        self.assertEqual(methods.get("MinutesExtraApiV1::Search"), "POST")

    def test_extract_endpoint_calls_resolves_direct_to_request(self):
        text = """
        let api_request: ApiRequest<ListRepoResp> = LingoApiV1::RepoList.to_request();
        """
        resolver = EndpointResolver(
            constants={},
            enum_endpoints={"LingoApiV1::RepoList": "/open-apis/lingo/v1/repos"},
            enum_methods={"LingoApiV1::RepoList": "GET"},
        )
        calls = extract_endpoint_calls(text, resolver)
        self.assertEqual(len(calls), 1)
        self.assertTrue(calls[0].is_resolved)
        self.assertEqual(calls[0].method, "GET")
        self.assertEqual(calls[0].resolved_path, "/open-apis/lingo/v1/repos")

    def test_extract_endpoint_calls_resolves_to_request_with_variant_args(self):
        text = """
        let mut api_request: ApiRequest<UpdateDraftResp> = BaikeApiV1::DraftUpdate(self.draft_id)
            .to_request()
            .body(serde_json::to_value(&self.req)?);
        """
        resolver = EndpointResolver(
            constants={},
            enum_endpoints={"BaikeApiV1::DraftUpdate": "/open-apis/baike/v1/drafts/{param}"},
            enum_methods={"BaikeApiV1::DraftUpdate": "PUT"},
        )
        calls = extract_endpoint_calls(text, resolver)
        self.assertEqual(len(calls), 1)
        self.assertTrue(calls[0].is_resolved)
        self.assertEqual(calls[0].method, "PUT")
        self.assertEqual(calls[0].resolved_path, "/open-apis/baike/v1/drafts/{param}")

    def test_extract_endpoint_calls_resolves_variable_to_request(self):
        text = """
        let api_endpoint = SheetsApiV3::GetFilter(spreadsheet_token.to_string(), sheet_id.to_string());
        let api_request: ApiRequest<GetFilterResponse> = api_endpoint.to_request();
        """
        resolver = EndpointResolver(
            constants={},
            enum_endpoints={
                "SheetsApiV3::GetFilter": (
                    "/open-apis/sheets/v3/spreadsheets/{param}/sheets/{param}/filter"
                )
            },
            enum_methods={"SheetsApiV3::GetFilter": "GET"},
        )
        calls = extract_endpoint_calls(text, resolver)
        self.assertEqual(len(calls), 1)
        self.assertTrue(calls[0].is_resolved)
        self.assertEqual(calls[0].method, "GET")

    def test_scan_api_file_resolves_docs_catalog_to_request(self):
        src = REPO_ROOT / "crates" / "openlark-docs" / "src"
        contract = scan_api_file(src, "baike/lingo/v1/repo/list.rs")
        self.assertIsNotNone(contract)
        assert contract is not None
        self.assertTrue(contract.endpoint_calls)
        self.assertTrue(contract.endpoint_calls[0].is_resolved)
        self.assertEqual(contract.endpoint_calls[0].method, "GET")
        self.assertEqual(
            contract.endpoint_calls[0].resolved_path,
            "/open-apis/lingo/v1/repos",
        )


class ExtractAccessTokensTests(unittest.TestCase):
    """token 契约：解析 .with_supported_access_token_types 声明（未声明回落默认）。"""

    def test_explicit_single_app(self):
        source = (
            "let req = ApiRequest::get(&path)"
            ".with_supported_access_token_types(vec![AccessTokenType::App]);"
        )
        self.assertEqual(extract_access_token_types(source), ("app_access_token",))

    def test_explicit_none(self):
        source = ".with_supported_access_token_types(vec![AccessTokenType::None]);"
        self.assertEqual(extract_access_token_types(source), ("none_access_token",))

    def test_explicit_multiple_variants(self):
        source = (
            ".with_supported_access_token_types("
            "vec![AccessTokenType::User, AccessTokenType::Tenant]);"
        )
        self.assertEqual(
            extract_access_token_types(source),
            ("user_access_token", "tenant_access_token"),
        )

    def test_multiline_vec_literal(self):
        source = (
            ".with_supported_access_token_types(vec![\n"
            "    AccessTokenType::User,\n"
            "    AccessTokenType::Tenant,\n"
            "]);"
        )
        self.assertEqual(
            extract_access_token_types(source),
            ("user_access_token", "tenant_access_token"),
        )

    def test_no_call_returns_default(self):
        # ApiRequest 默认 supported_access_token_types = [User, Tenant]（见 api/mod.rs）
        self.assertEqual(
            extract_access_token_types("let req = ApiRequest::get(&path);"),
            ("user_access_token", "tenant_access_token"),
        )

    def test_real_auth_token_endpoint_declares_none(self):
        # 锁定提取器对真实 .rs 源码的解析能力。选用 #512 明确不动（保持 App/None）
        # 的 auth/v3 token 端点，避免被 #515 的 acs/security 修正连带改坏。
        source = (
            REPO_ROOT
            / "crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token_internal.rs"
        ).read_text(encoding="utf-8")
        self.assertEqual(extract_access_token_types(source), ("none_access_token",))


class ExtractManualAuthTokenTests(unittest.TestCase):
    """token 契约：识别声明 None 但手动注入 ``Authorization: Bearer`` 的端点（OIDC userinfo）。

    声明 ``AccessTokenType::None`` 表示自行管理鉴权（bypass token cache）。validator 据此
    把 ``none_access_token`` 替换为实际注入的 token 类型，避免误报 disjoint ERROR（#515）。
    """

    def test_detects_manual_user_token_bearer_injection(self):
        source = (
            "ApiRequest::get(&path)"
            '.header("Authorization", format!("Bearer {}", self.user_access_token))'
            ".with_supported_access_token_types(vec![AccessTokenType::None]);"
        )
        self.assertEqual(extract_manual_auth_token(source), "user_access_token")

    def test_detects_multiline_header_injection(self):
        # 真实 userinfo 写法：header 调用跨多行
        source = (
            "ApiRequest::get(api_endpoint.path())\n"
            "    .header(\n"
            '        "Authorization",\n'
            '        format!("Bearer {}", self.user_access_token),\n'
            "    )\n"
            "    .with_supported_access_token_types(vec![AccessTokenType::None]);"
        )
        self.assertEqual(extract_manual_auth_token(source), "user_access_token")

    def test_no_bearer_injection_returns_empty(self):
        # 真正无鉴权的 token 签发端点（tenant_access_token_internal）：声明 None 但不注入 Bearer
        source = (
            "ApiRequest::post(path)"
            ".with_supported_access_token_types(vec![AccessTokenType::None]);"
        )
        self.assertEqual(extract_manual_auth_token(source), "")

    def test_real_userinfo_source_detected(self):
        source = (
            REPO_ROOT / "crates/openlark-auth/src/auth/authen/v1/user_info/get.rs"
        ).read_text(encoding="utf-8")
        self.assertEqual(extract_manual_auth_token(source), "user_access_token")


class ExtractStructsTypedTests(unittest.TestCase):
    """分组类型语义视图（verify_api_fields 消费；#636 收口后唯一提取实现）。

    由 tools/tests/test_verify_api_fields.py 的 ExtractStructFieldsTests 迁入并按
    新语义改写（multipart 元字段用例改为 dunder 序列名版本，skip_serializing 的
    file 字段用例并入 path params 用例与 MultipartForm 分组用例），另补齐收口
    引入的新语义正测：rename_all / 括号配平 / 逗号泛型 / std::option / 版本尾缀。
    """

    def test_groups_body_and_response_with_type_semantics(self):
        source = """
pub struct DemoBody {
    #[serde(rename = "type")]
    pub task_type: String,
    pub user_ids: Vec<String>,
    pub comment: Option<String>,
}
pub struct DemoResponse {
    pub result: String,
}
pub struct DemoRequest {
    pub config: Config,
}
"""
        structs = extract_structs(source)
        self.assertEqual([item.name for item in structs], ["DemoBody", "DemoResponse"])
        fields = {item.name: item for item in structs[0].fields}
        self.assertEqual(fields["task_type"].effective_name, "type")
        self.assertTrue(fields["user_ids"].required)
        self.assertEqual(fields["user_ids"].type_name, "String")
        self.assertTrue(fields["user_ids"].is_array)
        self.assertFalse(fields["comment"].required)
        self.assertFalse(fields["comment"].is_array)

    def test_skips_absolute_skip_serializing_path_params(self):
        source = """
pub struct UpdateBody {
    /// path param
    #[serde(skip_serializing)]
    pub card_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub sequence: i32,
}
"""
        structs = extract_structs(source)
        fields = {item.effective_name: item for item in structs[0].fields}
        self.assertNotIn("card_id", fields)
        self.assertIn("type", fields)
        self.assertIn("uuid", fields)
        self.assertFalse(fields["uuid"].required)
        self.assertIn("sequence", fields)

    def test_skips_dunder_serialized_fields(self):
        source = """
pub struct UploadBody {
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    pub pdf_page_limit: i32,
}
"""
        structs = extract_structs(source)
        fields = {item.effective_name: item for item in structs[0].fields}
        self.assertNotIn("__file_name", fields)
        self.assertNotIn("file_name", fields)
        self.assertIn("pdf_page_limit", fields)

    def test_rename_all_camel_case_feeds_effective_name(self):
        source = """
#[serde(rename_all = "camelCase")]
pub struct DemoBody {
    pub page_size: Option<i32>,
}
"""
        structs = extract_structs(source)
        fields = structs[0].fields
        self.assertEqual(fields[0].effective_name, "pageSize")
        self.assertFalse(fields[0].required)

    def test_rename_survives_doc_comment_between_attr_and_field(self):
        # 现实写法（whiteboard node create）：#[serde(rename)] 与字段之间夹 doc 注释
        source = """
pub struct DemoBody {
    #[serde(rename = "type")]
    /// 节点类型。
    pub node_type: String,
}
"""
        structs = extract_structs(source)
        self.assertEqual(structs[0].fields[0].effective_name, "type")

    def test_balanced_braces_in_doc_comment_do_not_truncate_fields(self):
        # 旧 verify 正则 ([^}]*) 停在 doc 注释里的第一个 }（含平衡花括号），截断后续字段；
        # 括号配平后平衡的 JSON 示例不再截断。不平衡的孤立 } 仍会截断（注释感知不在范围）。
        source = '''
pub struct DemoBody {
    /// 示例 payload：{"a": 1} 与 {"b": 2} 的花括号会截断旧实现
    pub keep_me: String,
    pub also_kept: i32,
}
'''
        structs = extract_structs(source)
        self.assertEqual(
            [item.effective_name for item in structs[0].fields],
            ["keep_me", "also_kept"],
        )

    def test_comma_generic_type_stays_whole(self):
        source = """
pub struct DemoBody {
    pub extra: HashMap<String, String>,
}
"""
        flat = extract_rust_fields(source)
        self.assertEqual(flat[0].type_name, "HashMap<String, String>")
        typed = extract_structs(source)[0].fields
        # 核心类型取首个泛型参数（HashMap<K, V> -> K），与旧 _parse_type 语义一致
        self.assertEqual(typed[0].type_name, "String")
        self.assertTrue(typed[0].required)

    def test_std_option_prefix_is_optional(self):
        source = """
pub struct DemoBody {
    pub flag: std::option::Option<bool>,
}
"""
        structs = extract_structs(source)
        self.assertFalse(structs[0].fields[0].required)

    def test_suffix_match_excludes_substring_only_names(self):
        # endswith 后缀匹配：BodyWrapper 含 "Body" 子串但不尾缀，不进入提取
        source = """
pub struct DemoBodyWrapper {
    pub x: String,
}
pub struct DemoQuery {
    pub page: Option<i32>,
}
"""
        structs = extract_structs(source)
        self.assertEqual([item.name for item in structs], ["DemoQuery"])

    def test_versioned_suffix_structs_match(self):
        # approval v4 用户级 / workflow v1 的版本尾缀命名（BodyV4/ResponseV1）
        source = """
pub struct AddCcInstanceBodyV4 {
    pub instance_code: String,
}
pub struct UpdateTaskResponseV1 {
    pub task_id: String,
}
"""
        structs = extract_structs(source)
        self.assertEqual(
            [item.name for item in structs],
            ["AddCcInstanceBodyV4", "UpdateTaskResponseV1"],
        )
        flat = extract_rust_fields(source)
        self.assertEqual([item.serialized_name for item in flat], ["instance_code"])

    def test_last_field_without_trailing_comma_is_captured(self):
        source = """
pub struct DemoBody {
    pub a: String,
    pub b: i32
}
"""
        structs = extract_structs(source)
        self.assertEqual(
            [item.name for item in structs[0].fields], ["a", "b"]
        )

    def test_multipart_channels_join_as_synthetic_group(self):
        source = """
pub struct UploadAllResponse {
    pub file_token: String,
}

pub async fn execute(self) -> SDKResult<UploadAllResponse> {
    #[derive(Serialize)]
    struct UploadMeta {
        file_name: String,
        parent_node: String,
    }

    let request = ApiRequest::<UploadAllResponse>::post(&api_endpoint.to_url())
        .json_body(&meta)
        .file_content(self.file);
}
"""
        structs = extract_structs(source)
        by_name = {item.name: item for item in structs}
        self.assertIn(MULTIPART_FORM_STRUCT_NAME, by_name)
        form_fields = {
            item.effective_name for item in by_name[MULTIPART_FORM_STRUCT_NAME].fields
        }
        # 二进制 file 通道不进合成分组（官方 request_body 字段表不列它）
        self.assertNotIn("file", form_fields)
        self.assertIn("file_name", form_fields)
        self.assertIn("parent_node", form_fields)
        # 响应 struct 与合成分组并存，互不混入
        self.assertEqual(
            [item.effective_name for item in by_name["UploadAllResponse"].fields],
            ["file_token"],
        )


if __name__ == "__main__":
    unittest.main()
