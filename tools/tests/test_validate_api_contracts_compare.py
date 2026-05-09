import csv
import sys
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.compare import compare_endpoint, compare_request_fields, compare_response_fields
from tools.api_contracts.models import ApiIdentity, OfficialField, RustApiContract, RustEndpointCall, RustField


class ContractCompareTests(unittest.TestCase):
    def _api(self, url: str = "POST:/open-apis/document_ai/v1/bank_card/recognize") -> ApiIdentity:
        return ApiIdentity(
            api_id="1",
            name="Bank card recognize",
            biz_tag="ai",
            meta_project="document_ai",
            meta_version="v1",
            meta_resource="bank_card",
            meta_name="recognize",
            url=url,
            doc_path="https://open.feishu.cn/document/mock",
            expected_file="ai/document_ai/v1/bank_card/recognize.rs",
        )

    def test_compare_endpoint_accepts_matching_method_and_path(self):
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            endpoint_calls=(
                RustEndpointCall(
                    method="POST",
                    argument="BANK_CARD",
                    line=10,
                    resolved_path="/open-apis/document_ai/v1/bank_card/recognize",
                ),
            ),
        )

        self.assertEqual(compare_endpoint(self._api(), rust), [])

    def test_compare_endpoint_detects_method_mismatch(self):
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            endpoint_calls=(
                RustEndpointCall(
                    method="GET",
                    argument="BANK_CARD",
                    line=10,
                    resolved_path="/open-apis/document_ai/v1/bank_card/recognize",
                ),
            ),
        )

        findings = compare_endpoint(self._api(), rust)

        self.assertEqual(findings[0].code, "E_ENDPOINT_METHOD_MISMATCH")
        self.assertEqual(findings[0].severity, "ERROR")

    def test_compare_endpoint_detects_path_mismatch(self):
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            endpoint_calls=(
                RustEndpointCall(
                    method="POST",
                    argument="BANK_CARD",
                    line=10,
                    resolved_path="/open-apis/document_ai/v1/business_card/recognize",
                ),
            ),
        )

        findings = compare_endpoint(self._api(), rust)

        self.assertEqual(findings[0].code, "E_ENDPOINT_PATH_MISMATCH")
        self.assertEqual(findings[0].severity, "ERROR")

    def test_compare_endpoint_accepts_colon_and_brace_params_as_equivalent(self):
        api = self._api("GET:/open-apis/contact/v3/users/:user_id")
        rust = RustApiContract(
            rel_path="contact/contact/v3/user/get.rs",
            endpoint_calls=(
                RustEndpointCall(
                    method="GET",
                    argument="CONTACT_V3_USER",
                    line=10,
                    resolved_path="/open-apis/contact/v3/users/{user_id}",
                ),
            ),
        )

        self.assertEqual(compare_endpoint(api, rust), [])

    def test_compare_request_fields_detects_missing_required_field(self):
        official_fields = (
            OfficialField(
                name="file",
                required=True,
                location="requestBody:multipart/form-data",
                field_type="string:binary",
            ),
        )
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            fields=(
                RustField(
                    struct_name="BankCardRecognizeBody",
                    field_name="file_token",
                    serialized_name="file_token",
                    type_name="String",
                    optional=False,
                    line=20,
                ),
            ),
        )

        findings = compare_request_fields(self._api(), official_fields, rust)

        self.assertEqual(findings[0].code, "E_REQUIRED_REQUEST_FIELD_MISSING")
        self.assertEqual(findings[0].severity, "ERROR")

    def test_compare_request_fields_warns_when_required_field_is_optional(self):
        official_fields = (
            OfficialField(
                name="file",
                required=True,
                location="requestBody:multipart/form-data",
            ),
        )
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            fields=(
                RustField(
                    struct_name="BankCardRecognizeBody",
                    field_name="file",
                    serialized_name="file",
                    type_name="Option<String>",
                    optional=True,
                    line=20,
                ),
            ),
        )

        findings = compare_request_fields(self._api(), official_fields, rust)

        self.assertEqual(findings[0].code, "W_REQUIRED_REQUEST_FIELD_OPTIONAL")

    def test_compare_response_fields_warns_for_missing_data_field(self):
        official_fields = (
            OfficialField(
                name="bank_card",
                required=False,
                location="responseBody:application/json.data",
            ),
        )
        rust = RustApiContract(
            rel_path="ai/document_ai/v1/bank_card/recognize.rs",
            response_fields=(
                RustField(
                    struct_name="BankCardRecognizeResult",
                    field_name="parsing_result",
                    serialized_name="parsing_result",
                    type_name="Option<ParsingResult>",
                    optional=True,
                    line=30,
                ),
            ),
        )

        findings = compare_response_fields(self._api(), official_fields, rust)

        self.assertEqual(findings[0].code, "W_RESPONSE_FIELD_MISSING")
        self.assertEqual(findings[0].severity, "WARN")


class ContractCliTests(unittest.TestCase):
    def test_cli_requires_live_fields_for_field_validation(self):
        import tools.validate_api_contracts as cli

        original_argv = sys.argv
        sys.argv = ["validate_api_contracts.py", "--fields"]
        try:
            self.assertEqual(cli.main(), 1)
        finally:
            sys.argv = original_argv

    def test_cli_writes_reports_for_fixture_crate(self):
        import tools.validate_api_contracts as cli

        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            src = root / "crates" / "openlark-ai" / "src"
            api_dir = src / "ai" / "document_ai" / "v1" / "bank_card"
            api_dir.mkdir(parents=True)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/document_ai/v1/bank_card/recognize";',
                encoding="utf-8",
            )
            (api_dir / "recognize.rs").write_text(
                "let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);",
                encoding="utf-8",
            )
            csv_path = root / "api_list_export.csv"
            self._write_csv(csv_path)
            mapping_path = root / "api_coverage.toml"
            mapping_path.write_text(
                '[crates.openlark-ai]\n'
                'src = "crates/openlark-ai/src"\n'
                'biz_tags = ["ai"]\n',
                encoding="utf-8",
            )
            report_dir = root / "reports"

            original_argv = sys.argv
            original_cwd = Path.cwd()
            sys.argv = [
                "validate_api_contracts.py",
                "--crate",
                "openlark-ai",
                "--csv",
                str(csv_path),
                "--mapping",
                str(mapping_path),
                "--report-dir",
                str(report_dir),
                "--strict",
                "endpoint",
            ]
            try:
                import os

                os.chdir(root)
                exit_code = cli.main()
                self.assertEqual(exit_code, 0)
                self.assertTrue((report_dir / "summary.md").exists())
                self.assertIn(
                    "Errors | 0",
                    (report_dir / "crates" / "openlark-ai.md").read_text(encoding="utf-8"),
                )
            finally:
                os.chdir(original_cwd)
                sys.argv = original_argv

        self.assertEqual(exit_code, 0)

    def test_cli_field_validation_report_only_returns_zero_with_findings(self):
        import tools.validate_api_contracts as cli

        def fake_fetch_detail_payload(api, timeout, retries):
            return {
                "data": {
                    "schema": {
                        "apiSchema": {
                            "requestBody": {
                                "content": {
                                    "multipart/form-data": {
                                        "schema": {
                                            "properties": [
                                                {
                                                    "name": "file",
                                                    "type": "string",
                                                    "format": "binary",
                                                    "required": True,
                                                }
                                            ]
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            src = root / "crates" / "openlark-ai" / "src"
            api_dir = src / "ai" / "document_ai" / "v1" / "bank_card"
            api_dir.mkdir(parents=True)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/document_ai/v1/bank_card/recognize";',
                encoding="utf-8",
            )
            (api_dir / "recognize.rs").write_text(
                "\n".join(
                    [
                        "use openlark_core::api::ApiRequest;",
                        "pub struct BankCardRecognizeBody {",
                        "    pub file_token: String,",
                        "}",
                        "let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);",
                    ]
                ),
                encoding="utf-8",
            )
            csv_path = root / "api_list_export.csv"
            self._write_csv(csv_path)
            mapping_path = root / "api_coverage.toml"
            mapping_path.write_text(
                '[crates.openlark-ai]\n'
                'src = "crates/openlark-ai/src"\n'
                'biz_tags = ["ai"]\n',
                encoding="utf-8",
            )
            report_dir = root / "reports"

            original_argv = sys.argv
            original_cwd = Path.cwd()
            original_fetch = cli.fetch_detail_payload
            sys.argv = [
                "validate_api_contracts.py",
                "--crate",
                "openlark-ai",
                "--csv",
                str(csv_path),
                "--mapping",
                str(mapping_path),
                "--report-dir",
                str(report_dir),
                "--fields",
                "--live-fields",
                "--max-field-apis",
                "1",
            ]
            try:
                import os

                cli.fetch_detail_payload = fake_fetch_detail_payload
                os.chdir(root)
                exit_code = cli.main()
                self.assertEqual(exit_code, 0)
                report_text = (report_dir / "crates" / "openlark-ai.md").read_text(encoding="utf-8")
                self.assertIn("E_REQUIRED_REQUEST_FIELD_MISSING", report_text)
            finally:
                cli.fetch_detail_payload = original_fetch
                os.chdir(original_cwd)
                sys.argv = original_argv

    def _write_csv(self, path: Path) -> None:
        header = [
            "id",
            "name",
            "bizTag",
            "meta.Project",
            "meta.Version",
            "meta.Resource",
            "meta.Name",
            "url",
            "docPath",
            "fullPath",
        ]
        with path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=header)
            writer.writeheader()
            writer.writerow(
                {
                    "id": "1",
                    "name": "Bank card recognize",
                    "bizTag": "ai",
                    "meta.Project": "document_ai",
                    "meta.Version": "v1",
                    "meta.Resource": "bank_card",
                    "meta.Name": "recognize",
                    "url": "POST:/open-apis/document_ai/v1/bank_card/recognize",
                    "docPath": "https://open.feishu.cn/document/mock",
                    "fullPath": "/document/mock",
                }
            )


if __name__ == "__main__":
    unittest.main()
