import csv
import sys
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.compare import (
    compare_access_token_types,
    compare_endpoint,
    compare_request_fields,
    compare_response_fields,
)
from tools.api_contracts.models import (
    DEFAULT_ACCESS_TOKEN_TYPES,
    ApiIdentity,
    OfficialField,
    RustApiContract,
    RustEndpointCall,
    RustField,
)


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

    def test_compare_request_fields_skips_optional_missing_when_flatten_passthrough(self):
        # docx block patch：#[serde(flatten)] update: serde_json::Value 透传，
        # 官方 optional 字段（update_text_elements 等）应跳过缺失告警
        official_fields = (
            OfficialField(
                name="update_text_elements",
                required=False,
                location="requestBody:application/json",
            ),
        )
        rust = RustApiContract(
            rel_path="docx/v1/document/block/patch.rs",
            fields=(
                RustField(
                    struct_name="UpdateDocumentBlockParams",
                    field_name="update",
                    serialized_name="update",
                    type_name="serde_json::Value",
                    optional=False,
                    line=30,
                ),
            ),
            has_flatten_value_passthrough=True,
        )

        findings = compare_request_fields(self._api(), official_fields, rust)

        self.assertEqual(findings, [])

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

    def test_cli_validates_registered_legacy_implementation_path(self):
        import tools.validate_api_contracts as cli

        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            src = root / "crates" / "openlark-ai" / "src"
            legacy_dir = src / "legacy" / "bank_card"
            legacy_dir.mkdir(parents=True)
            (src / "endpoints.rs").write_text(
                'pub const BANK_CARD: &str = "/open-apis/document_ai/v1/bank_card/recognize";',
                encoding="utf-8",
            )
            (legacy_dir / "recognize.rs").write_text(
                "let req: ApiRequest<Response> = ApiRequest::post(BANK_CARD);",
                encoding="utf-8",
            )
            csv_path = root / "api_list_export.csv"
            self._write_csv(csv_path)
            mapping_path = root / "api_coverage.toml"
            mapping_path.write_text(
                '[crates.openlark-ai]\n'
                'src = "crates/openlark-ai/src"\n'
                'biz_tags = ["ai"]\n'
                'implementation_path_rewrites = [{ from = "ai/document_ai/v1/", to = "legacy/" }]\n',
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


class AccessTokenCompareTests(unittest.TestCase):
    """token 类型契约核对：compare_access_token_types 的判定规则。"""

    def _api(self, api_id: str = "acs/v1/user/get") -> ApiIdentity:
        return ApiIdentity(
            api_id=api_id,
            name=api_id,
            biz_tag="acs",
            meta_project="acs",
            meta_version="v1",
            meta_resource="user",
            meta_name="get",
            url="GET:/open-apis/acs/v1/users/:user_id",
            doc_path="",
            expected_file="acs/acs/v1/user/get.rs",
        )

    def _contract(self, types: tuple[str, ...]) -> RustApiContract:
        return RustApiContract(rel_path="acs/acs/v1/user/get.rs", access_token_types=types)

    def test_disjoint_returns_error(self):
        # Rust 声明 App，飞书只要 tenant → 必然鉴权失败
        findings = compare_access_token_types(
            self._api(), ("tenant_access_token",), self._contract(("app_access_token",))
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].severity, "ERROR")
        self.assertEqual(findings[0].code, "E_ACCESS_TOKEN_TYPE_MISMATCH")

    def test_overlap_returns_no_finding(self):
        findings = compare_access_token_types(
            self._api(),
            ("tenant_access_token", "user_access_token"),
            self._contract(("tenant_access_token",)),
        )
        self.assertEqual(findings, [])

    def test_default_tokens_against_tenant_only_is_ok(self):
        # 宽松默认 [User,Tenant] 对 tenant-only 接口不应误报（交集含 tenant）
        findings = compare_access_token_types(
            self._api(),
            ("tenant_access_token",),
            self._contract(DEFAULT_ACCESS_TOKEN_TYPES),
        )
        self.assertEqual(findings, [])

    def test_official_unannotated_returns_unverified(self):
        findings = compare_access_token_types(
            self._api(), (), self._contract(("app_access_token",))
        )
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].severity, "UNVERIFIED")
        self.assertEqual(findings[0].code, "U_ACCESS_TOKEN_UNANNOTATED")

    def test_missing_implementation_returns_warn(self):
        findings = compare_access_token_types(self._api(), ("tenant_access_token",), None)
        self.assertEqual(len(findings), 1)
        self.assertEqual(findings[0].severity, "WARN")
        self.assertEqual(findings[0].code, "W_IMPLEMENTATION_FILE_MISSING")


class BootstrapOracleTests(unittest.TestCase):
    """工具有效性验证：必须检出 #511 已核实的 4 个 token 误配。"""

    def _api(self, api_id: str) -> ApiIdentity:
        return ApiIdentity(
            api_id=api_id,
            name=api_id,
            biz_tag="acs",
            meta_project="",
            meta_version="",
            meta_resource="",
            meta_name="",
            url="",
            doc_path="",
            expected_file=api_id,
        )

    def _assert_error(self, api_id: str, official: list[str], rust: str):
        findings = compare_access_token_types(
            self._api(api_id),
            tuple(official),
            RustApiContract(rel_path=api_id, access_token_types=(rust,)),
        )
        errors = [f for f in findings if f.severity == "ERROR"]
        self.assertTrue(
            errors,
            f"期望检出 ERROR（{api_id}: Rust={rust} vs 飞书={official}），实际={findings}",
        )

    def test_acs_device_bind_should_be_user_not_app(self):
        self._assert_error(
            "acs/v1/rule_external/device_bind", ["user_access_token"], "app_access_token"
        )

    def test_acs_user_get_should_be_tenant_not_app(self):
        self._assert_error("acs/v1/user/get", ["tenant_access_token"], "app_access_token")

    def test_acs_user_list_should_be_tenant_not_app(self):
        self._assert_error("acs/v1/user/list", ["tenant_access_token"], "app_access_token")

    def test_security_user_migrations_get_should_be_tenant_or_user(self):
        self._assert_error(
            "security_and_compliance/v1/user_migrations/get",
            ["tenant_access_token", "user_access_token"],
            "app_access_token",
        )

    def test_after_fix_no_error(self):
        # 反向验证：修正后（device_bind→User）不再报错
        findings = compare_access_token_types(
            self._api("acs/v1/rule_external/device_bind"),
            ("user_access_token",),
            RustApiContract(rel_path="device_bind", access_token_types=("user_access_token",)),
        )
        self.assertEqual(findings, [])


class ManualAuthTokenReconciliationTests(unittest.TestCase):
    """声明 None 但手动注入 token（OIDC userinfo）的端点：按实际注入类型核对，不误报 ERROR（#515）。"""

    def _api(self):
        return ApiIdentity(
            api_id="authen/v1/user_info/get",
            name="获取用户信息",
            biz_tag="auth",
            meta_project="auth",
            meta_version="v1",
            meta_resource="user_info",
            meta_name="get",
            url="",
            doc_path="",
            expected_file="auth/authen/v1/user_info/get.rs",
        )

    def test_none_declared_with_manual_user_token_matches_user_doc(self):
        # 声明 None + 手动注入 user_access_token ↔ 文档 user_access_token → 无 finding
        contract = RustApiContract(
            rel_path="auth/authen/v1/user_info/get.rs",
            access_token_types=("none_access_token",),
            manual_auth_token="user_access_token",
        )
        self.assertEqual(
            compare_access_token_types(self._api(), ("user_access_token",), contract),
            [],
        )

    def test_none_declared_without_manual_injection_still_errors(self):
        # 真正无鉴权（None、无手动注入）对要求 user 的文档仍报 ERROR
        contract = RustApiContract(
            rel_path="x.rs",
            access_token_types=("none_access_token",),
        )
        findings = compare_access_token_types(self._api(), ("user_access_token",), contract)
        self.assertTrue(any(f.code == "E_ACCESS_TOKEN_TYPE_MISMATCH" for f in findings))


if __name__ == "__main__":
    unittest.main()
