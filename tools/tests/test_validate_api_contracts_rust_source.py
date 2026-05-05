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


if __name__ == "__main__":
    unittest.main()
