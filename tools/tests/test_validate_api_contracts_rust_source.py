import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.rust_source import (
    EndpointResolver,
    extract_endpoint_calls,
    extract_rust_response_fields,
    extract_rust_fields,
    load_endpoint_constants,
    resolve_format_expression,
    scan_api_file,
)


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


if __name__ == "__main__":
    unittest.main()
