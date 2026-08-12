from __future__ import annotations

import json
import sys
import tempfile
import unittest
from dataclasses import replace
from pathlib import Path
from unittest import mock

from tools import verify_api_fields
from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official_evidence import (
    AcquisitionAttempt,
    DimensionEvidence,
    EvidenceDiagnostic,
    EvidenceDimension,
    EvidenceSource,
    EvidenceStatus,
    FieldObservation,
    FreshOfficialPolicy,
    OfficialDocumentEvidence,
    PreferSnapshotPolicy,
    RecordedOnlyPolicy,
    RecordedSnapshot,
    collect,
)

class _FieldCollector:
    def __init__(self, api: ApiIdentity) -> None:
        self.api = api
        self.requests = []

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        return None

    def collect(self, api, dimensions, policy):
        self.requests.append((api, tuple(dimensions), policy))
        return field_evidence(self.api)


def api_identity(*, method: str = "POST", full_path: str = "") -> ApiIdentity:
    return ApiIdentity(
        api_id="1",
        name="测试接口",
        biz_tag="approval",
        meta_project="approval",
        meta_version="v4",
        meta_resource="task",
        meta_name="pass",
        url=f"{method}:/open-apis/approval/v4/tasks/pass",
        doc_path="https://open.feishu.cn/document/mock",
        expected_file="approval/approval/v4/task/pass.rs",
        full_path=full_path,
    )


def field_evidence(
    api: ApiIdentity,
    request_status: EvidenceStatus = EvidenceStatus.TRUSTED,
    response_status: EvidenceStatus = EvidenceStatus.TRUSTED,
) -> OfficialDocumentEvidence:
    def dimension(
        kind: EvidenceDimension,
        status: EvidenceStatus,
        observations: tuple[FieldObservation, ...],
    ) -> DimensionEvidence:
        diagnostic = (
            ()
            if status is EvidenceStatus.TRUSTED
            else (EvidenceDiagnostic("structure_incomplete", "结构不完整"),)
        )
        return DimensionEvidence(
            dimension=kind,
            status=status,
            selected_source=EvidenceSource.STRUCTURED_DETAIL,
            observations=observations,
            snapshot_provenance=None,
            interpretation_provenance=None,
            diagnostics=diagnostic,
            acquisition_trail=(
                AcquisitionAttempt(
                    EvidenceSource.STRUCTURED_DETAIL,
                    status,
                    diagnostic,
                    None,
                ),
            ),
        )

    return OfficialDocumentEvidence(
        api,
        (
            dimension(
                EvidenceDimension.REQUEST_FIELDS,
                request_status,
                (
                    FieldObservation(
                        ("instance_code",),
                        "request_body",
                        True,
                        "string",
                        EvidenceSource.STRUCTURED_DETAIL,
                    ),
                    FieldObservation(
                        ("nested", "ignored"),
                        "request_body",
                        False,
                        "string",
                        EvidenceSource.STRUCTURED_DETAIL,
                    ),
                ),
            ),
            dimension(
                EvidenceDimension.RESPONSE_FIELDS,
                response_status,
                (
                    FieldObservation(
                        ("result",),
                        "response_body",
                        None,
                        "string",
                        EvidenceSource.STRUCTURED_DETAIL,
                    ),
                ),
            ),
        ),
    )


class ExtractStructFieldsTests(unittest.TestCase):
    def test_extracts_required_optional_vec_and_serde_names(self):
        source = '''
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
'''
        structs = verify_api_fields.extract_structs(source)
        self.assertEqual([item.name for item in structs], ["DemoBody", "DemoResponse"])
        fields = {item.name: item for item in structs[0].fields}
        self.assertEqual(fields["task_type"].effective_name, "type")
        self.assertTrue(fields["user_ids"].required)
        self.assertEqual(fields["user_ids"].type_name, "String")
        self.assertTrue(fields["user_ids"].is_array)
        self.assertFalse(fields["comment"].required)
        self.assertFalse(fields["comment"].is_array)

    def test_skips_absolute_skip_serializing_path_params(self):
        source = '''
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
'''
        structs = verify_api_fields.extract_structs(source)
        fields = {item.effective_name: item for item in structs[0].fields}
        self.assertNotIn("card_id", fields)
        self.assertIn("type", fields)
        self.assertIn("uuid", fields)
        self.assertFalse(fields["uuid"].required)
        self.assertIn("sequence", fields)


class SuspiciousPatternTests(unittest.TestCase):
    def test_user_level_extra_field_is_informational(self):
        api = api_identity(full_path="/document/x/reference/approval-v4/task/pass")
        structs = [
            verify_api_fields.StructFields(
                "PassBody",
                [verify_api_fields.FieldInfo("user_id", "String", True)],
            )
        ]
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, "")
        self.assertEqual([(item.severity, item.category) for item in issues], [("info", "user_level_extra_field")])

    def test_required_vec_without_nonempty_validation_warns(self):
        api = api_identity()
        structs = [
            verify_api_fields.StructFields(
                "PassBody",
                [verify_api_fields.FieldInfo("user_ids", "String", True)],
            )
        ]
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, "")
        self.assertIn("missing_list_validation", [item.category for item in issues])

    def test_manual_nonempty_validation_suppresses_warning(self):
        api = api_identity()
        structs = [
            verify_api_fields.StructFields(
                "PassBody",
                [verify_api_fields.FieldInfo("user_ids", "String", True)],
            )
        ]
        issues = verify_api_fields.detect_suspicious_patterns(
            api, structs, "if self.user_ids.is_empty() { return Err(error); }"
        )
        self.assertNotIn("missing_list_validation", [item.category for item in issues])

    def test_empty_get_response_is_informational(self):
        api = api_identity(method="GET")
        structs = [verify_api_fields.StructFields("GetResponse", [])]
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, "")
        self.assertEqual(issues[0].category, "empty_get_response")


class ComparisonTests(unittest.TestCase):
    def test_compare_fields_reports_missing_and_extra(self):
        code = [
            verify_api_fields.FieldInfo("instance_code", "String", True),
            verify_api_fields.FieldInfo("extra", "String", False),
        ]
        official = [
            verify_api_fields.FieldInfo("instance_code", "string", True),
            verify_api_fields.FieldInfo("task_id", "string", True),
        ]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(diff.matched, ["instance_code"])
        self.assertEqual(diff.missing, ["task_id"])
        self.assertEqual(diff.extra, ["extra"])
        self.assertEqual(diff.required_mismatches, [])
        self.assertEqual(diff.type_mismatches, [])

    def test_compare_fields_required_mismatch_doc_yes_code_option_is_error(self):
        code = [verify_api_fields.FieldInfo("instance_code", "String", False)]
        official = [verify_api_fields.FieldInfo("instance_code", "string", True)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(len(diff.required_mismatches), 1)
        self.assertEqual(diff.required_mismatches[0].severity, "error")
        self.assertEqual(diff.required_mismatches[0].category, "required_mismatch")

    def test_compare_fields_required_mismatch_doc_no_code_required_is_warning(self):
        code = [verify_api_fields.FieldInfo("comment", "String", True)]
        official = [verify_api_fields.FieldInfo("comment", "string", False)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(len(diff.required_mismatches), 1)
        self.assertEqual(diff.required_mismatches[0].severity, "warning")

    def test_compare_fields_type_mismatch_warns(self):
        code = [verify_api_fields.FieldInfo("add_sign_type", "String", True)]
        official = [verify_api_fields.FieldInfo("add_sign_type", "int", True)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(len(diff.type_mismatches), 1)
        self.assertEqual(diff.type_mismatches[0].category, "type_mismatch")
        self.assertEqual(diff.type_mismatches[0].severity, "warning")

    def test_compare_fields_array_type_mismatch_warns(self):
        code = [
            verify_api_fields.FieldInfo(
                "user_ids", "String", True, is_array=False
            )
        ]
        official = [
            verify_api_fields.FieldInfo(
                "user_ids", "string[]", True, is_array=True
            )
        ]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(len(diff.type_mismatches), 1)

    def test_compare_fields_skips_type_when_doc_type_empty(self):
        code = [verify_api_fields.FieldInfo("x", "String", True)]
        official = [verify_api_fields.FieldInfo("x", "", True)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(diff.type_mismatches, [])

    def test_compare_fields_skips_custom_enum_vs_doc_string(self):
        """serde 自定义枚举对文档 string 是合法建模，不得 type_mismatch。"""
        code = [
            verify_api_fields.FieldInfo(
                "recognition_model", "RecognitionModel", False
            )
        ]
        official = [
            verify_api_fields.FieldInfo("recognition_model", "string", False)
        ]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(diff.type_mismatches, [])

    def test_compare_fields_skips_custom_enum_vec_vs_doc_string_array(self):
        code = [
            verify_api_fields.FieldInfo(
                "modes", "RecognitionModel", True, is_array=True
            )
        ]
        official = [
            verify_api_fields.FieldInfo(
                "modes", "string[]", True, is_array=True
            )
        ]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(diff.type_mismatches, [])

    def test_compare_fields_still_warns_primitive_mismatch(self):
        """已知原始类型不一致仍应 warning（回归：自定义跳过不能吞掉真冲突）。"""
        code = [verify_api_fields.FieldInfo("count", "String", True)]
        official = [verify_api_fields.FieldInfo("count", "int", True)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(len(diff.type_mismatches), 1)

    def test_compare_fields_skips_required_when_doc_required_unknown(self):
        code = [verify_api_fields.FieldInfo("x", "String", True)]
        official = [verify_api_fields.FieldInfo("x", "string", None)]
        diff = verify_api_fields.compare_fields(code, official)
        self.assertEqual(diff.required_mismatches, [])

    def test_evidence_comparison_surfaces_required_and_type_mismatches(self):
        api = api_identity()
        evidence = OfficialDocumentEvidence(
            api,
            (
                DimensionEvidence(
                    dimension=EvidenceDimension.REQUEST_FIELDS,
                    status=EvidenceStatus.TRUSTED,
                    selected_source=EvidenceSource.STRUCTURED_DETAIL,
                    observations=(
                        FieldObservation(
                            ("add_sign_type",),
                            "request_body",
                            True,
                            "int",
                            EvidenceSource.STRUCTURED_DETAIL,
                        ),
                    ),
                    snapshot_provenance=None,
                    interpretation_provenance=None,
                    diagnostics=(),
                    acquisition_trail=(),
                ),
                DimensionEvidence(
                    dimension=EvidenceDimension.RESPONSE_FIELDS,
                    status=EvidenceStatus.TRUSTED,
                    selected_source=EvidenceSource.STRUCTURED_DETAIL,
                    observations=(),
                    snapshot_provenance=None,
                    interpretation_provenance=None,
                    diagnostics=(),
                    acquisition_trail=(),
                ),
            ),
        )
        issues = []
        verify_api_fields._compare_evidence_against_code(
            [
                verify_api_fields.StructFields(
                    "PassBody",
                    [
                        verify_api_fields.FieldInfo(
                            "add_sign_type", "String", False
                        )
                    ],
                )
            ],
            evidence,
            issues,
        )
        categories = {item.category for item in issues}
        self.assertIn("required_mismatch", categories)
        self.assertIn("type_mismatch", categories)

    def test_evidence_comparison_consumes_only_top_level_observations(self):
        api = api_identity()
        structs = [
            verify_api_fields.StructFields(
                "PassBody",
                [verify_api_fields.FieldInfo("instance_code", "String", True)],
            ),
            verify_api_fields.StructFields(
                "PassResponse",
                [verify_api_fields.FieldInfo("result", "String", True)],
            ),
        ]
        issues = []
        verify_api_fields._compare_evidence_against_code(
            structs, field_evidence(api), issues
        )
        self.assertEqual(issues, [])

    def test_trusted_empty_request_evidence_preserves_nonpassing_semantics(self):
        api = api_identity()
        evidence = field_evidence(api)
        evidence = OfficialDocumentEvidence(
            api,
            (
                replace(evidence.dimensions[0], observations=()),
                evidence.dimensions[1],
            ),
        )
        issues = []
        verify_api_fields._compare_evidence_against_code(
            [
                verify_api_fields.StructFields(
                    "PassBody",
                    [
                        verify_api_fields.FieldInfo(
                            "instance_code", "String", True
                        )
                    ],
                )
            ],
            evidence,
            issues,
        )
        self.assertEqual(issues[0].category, "doc_parse_empty")
        self.assertEqual(verify_api_fields._exit_code_for_issues(issues), 1)

    def test_incomplete_evidence_is_nonpassing_with_existing_code(self):
        issues = []
        verify_api_fields._compare_evidence_against_code(
            [],
            field_evidence(
                api_identity(), request_status=EvidenceStatus.INCOMPLETE
            ),
            issues,
        )
        self.assertEqual(issues[0].category, "doc_parse_empty")
        self.assertEqual(verify_api_fields._exit_code_for_issues(issues), 1)

    def test_unavailable_evidence_maps_to_fetch_failure(self):
        issues = []
        verify_api_fields._compare_evidence_against_code(
            [],
            field_evidence(
                api_identity(), response_status=EvidenceStatus.UNAVAILABLE
            ),
            issues,
        )
        self.assertEqual(issues[0].category, "doc_fetch_failed")
        self.assertEqual(issues[0].severity, "error")

    def test_info_does_not_fail_but_warning_and_error_do(self):
        issue = verify_api_fields.FieldIssue
        self.assertEqual(verify_api_fields._exit_code_for_issues([]), 0)
        self.assertEqual(verify_api_fields._exit_code_for_issues([issue("info", "x", "tip")]), 0)
        self.assertEqual(verify_api_fields._exit_code_for_issues([issue("warning", "x", "warn")]), 1)
        self.assertEqual(verify_api_fields._exit_code_for_issues([issue("error", "x", "error")]), 1)

class FieldCliEvidenceTests(unittest.TestCase):
    def test_single_full_cli_preserves_report_contract_and_adds_evidence(self):
        api = api_identity()
        collector = _FieldCollector(api)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "crates/openlark-workflow/src" / api.expected_file
            source.parent.mkdir(parents=True)
            source.write_text(
                "pub struct PassBody {\n"
                "    pub instance_code: String,\n"
                "}\n"
                "pub struct PassResponse {\n"
                "    pub result: String,\n"
                "}\n",
                encoding="utf-8",
            )
            output = root / "reports"
            argv = [
                "verify_api_fields.py",
                "--api-id",
                api.api_id,
                "--fetch-docs",
                "--output-dir",
                str(output),
            ]
            with (
                mock.patch.object(sys, "argv", argv),
                mock.patch.object(verify_api_fields, "REPO_ROOT", root),
                mock.patch.object(
                    verify_api_fields,
                    "load_api_identities",
                    return_value=[api],
                ),
                mock.patch.object(
                    verify_api_fields,
                    "compose_full",
                    return_value=collector,
                ),
            ):
                exit_code = verify_api_fields.main()

            self.assertEqual(exit_code, 0)
            self.assertEqual(
                collector.requests[0][1],
                (
                    EvidenceDimension.REQUEST_FIELDS,
                    EvidenceDimension.RESPONSE_FIELDS,
                ),
            )
            # 单 API 门禁默认 FreshOfficialPolicy（重抓）
            self.assertIsInstance(collector.requests[0][2], FreshOfficialPolicy)
            report = json.loads(
                (output / "summary.json").read_text(encoding="utf-8")
            )
            self.assertEqual(
                set(report),
                {"mode", "total_apis", "apis_with_issues", "apis"},
            )
            self.assertEqual(report["apis"][0]["issues"], [])
            markdown = (output / f"api-{api.api_id}.md").read_text(
                encoding="utf-8"
            )
            self.assertIn("Provenance", markdown)
            self.assertIn("Acquisition Trail", markdown)
            evidence = report["apis"][0]["evidence"][0]
            self.assertEqual(evidence["status"], "trusted")
            self.assertIn("provenance", evidence)
            self.assertIn("diagnostics", evidence)
            self.assertEqual(
                evidence["acquisition_trail"][0]["status"], "trusted"
            )

    def test_single_api_warns_on_multiple_crate_matches(self):
        api = api_identity()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for crate in ("openlark-workflow", "openlark-hr"):
                path = root / "crates" / crate / "src" / api.expected_file
                path.parent.mkdir(parents=True)
                path.write_text(
                    "pub struct PassBody {\n    pub instance_code: String,\n}\n",
                    encoding="utf-8",
                )
            output = root / "reports"
            argv = [
                "verify_api_fields.py",
                "--api-id",
                api.api_id,
                "--output-dir",
                str(output),
            ]
            with (
                mock.patch.object(sys, "argv", argv),
                mock.patch.object(verify_api_fields, "REPO_ROOT", root),
                mock.patch.object(
                    verify_api_fields,
                    "load_api_identities",
                    return_value=[api],
                ),
                mock.patch("builtins.print") as printed,
            ):
                exit_code = verify_api_fields.main()

            self.assertEqual(exit_code, 0)
            warning_text = "\n".join(
                str(call.args[0]) for call in printed.call_args_list if call.args
            )
            self.assertIn("多个 crate 中匹配", warning_text)
            self.assertIn("openlark-hr", warning_text)
            self.assertIn("openlark-workflow", warning_text)

    def test_resolve_evidence_policy_defaults(self):
        self.assertIsInstance(
            verify_api_fields._resolve_evidence_policy(
                force_refresh=False, max_age_days=30, single_api=True
            ),
            FreshOfficialPolicy,
        )
        self.assertIsInstance(
            verify_api_fields._resolve_evidence_policy(
                force_refresh=True, max_age_days=30, single_api=False
            ),
            FreshOfficialPolicy,
        )
        policy = verify_api_fields._resolve_evidence_policy(
            force_refresh=False, max_age_days=7, single_api=False
        )
        self.assertIsInstance(policy, PreferSnapshotPolicy)
        self.assertEqual(policy.max_age_days, 7)


class ResponseFieldDigitNameTests(unittest.TestCase):
    """#618: 响应示例字段名正则须保留含数字的名字（如 i18n_name）。"""

    def test_rendered_response_keeps_digit_containing_field_names(self):
        api = api_identity()
        padding = "Rendered Feishu API document content.\n" * 20
        content = (
            padding
            + "Response body example\n"
            + "Response body example\n"
            + '{"code": 0, "msg": "ok", "data": {'
            + '"i18n_name": "n", "md5": "x", "s3_key": "k", "plain_name": "p"'
            + "}}\n"
            + "Error code\n"
        )
        snapshot = RecordedSnapshot.rendered(
            version=1,
            catalog_entry=api,
            acquired_at="2026-08-11T00:00:00Z",
            source_uri="https://open.feishu.cn/document/mock",
            content=content,
        )
        result = collect(
            api,
            (EvidenceDimension.RESPONSE_FIELDS,),
            RecordedOnlyPolicy((snapshot,)),
        )
        names = {
            observation.path[0]
            for observation in result.for_dimension(
                EvidenceDimension.RESPONSE_FIELDS
            ).observations
        }
        self.assertIn("i18n_name", names)
        self.assertIn("md5", names)
        self.assertIn("s3_key", names)
        self.assertIn("plain_name", names)
        self.assertNotIn("code", names)
        self.assertNotIn("msg", names)
        self.assertNotIn("data", names)


if __name__ == "__main__":
    unittest.main()
