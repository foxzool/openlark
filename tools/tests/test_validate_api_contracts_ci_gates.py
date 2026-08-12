"""Pin the domain list of CI api-contracts strict gates.

These assertions are the seam for domain-by-domain contract expansion
(#566/#569/#586): adding a new strict domain must update **both**
`.github/workflows/ci.yml` and this test so the gate inventory cannot
drift silently (dual-edit rule).

Also pins the admission-policy docs that define how the *next*
field-strict domain is admitted — see
`docs/api-contract-validation.md` §1.5 / §1.6.
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CI_WORKFLOW_PATH = ROOT / ".github" / "workflows" / "ci.yml"
CONTRACT_POLICY_DOC = ROOT / "docs" / "api-contract-validation.md"

# 仅列入已证明 live Structured Tokens Evidence 为 Trusted 的 API。
TOKEN_STRICT_API_INVENTORY = {
    "openlark-security": "7321978105899122716",
    "openlark-auth": "7277403063290724380",
}

# Structured-only composition 尚无 request/response 双维度全 Trusted 的域。
FIELD_STRICT_DOMAINS = ()

# 原 strict 域继续完整采集并发布报告，但不得伪装为 strict pass。
FIELD_EVIDENCE_MONITOR_DOMAINS = (
    {
        "name": "attendance",
        "crate": "openlark-hr",
        "biz_tag": "attendance",
        "report_dir_fragment": "api_contract_fields/attendance",
    },
    {
        "name": "docs",
        "crate": "openlark-docs",
        "biz_tag": None,
        "report_dir_fragment": "api_contract_fields/docs",
    },
)


def _api_contracts_job(workflow: str) -> str:
    match = re.search(
        r"(?ms)^  api-contracts:\n(.*?)(?=^  [a-z0-9_-]+:|\Z)",
        workflow,
    )
    assert match is not None, "api-contracts job missing from ci.yml"
    return match.group(0)


class ApiContractsCiGatesTests(unittest.TestCase):
    def setUp(self) -> None:
        self.workflow = CI_WORKFLOW_PATH.read_text(encoding="utf-8")
        self.job = _api_contracts_job(self.workflow)

    def test_endpoint_strict_covers_all_crates_offline(self) -> None:
        strict_block = self._step_run_block_containing("--all-crates")
        self.assertIn("--strict endpoint", strict_block)
        self.assertNotIn("--live-endpoints", strict_block)

    def test_live_endpoint_monitor_uses_structured_only_cli_mode(self) -> None:
        live_block = self._step_run_block_containing("--live-endpoints")
        self.assertIn("--crate openlark-ai", live_block)
        self.assertNotIn("--strict endpoint", live_block)

    def test_token_strict_uses_explicit_trusted_api_inventory(self) -> None:
        strict_block = self._step_run_block_containing(
            "api_contract_tokens_strict"
        )
        self.assertEqual(strict_block.count("--strict tokens"), 2)
        for crate, api_id in TOKEN_STRICT_API_INVENTORY.items():
            self.assertIn(f"--crate {crate}", strict_block)
            self.assertIn(f"--api-id {api_id}", strict_block)

    def test_full_token_domains_remain_non_strict_monitors(self) -> None:
        monitor = self._step_run_block_containing(
            "api_contract_tokens/security"
        )
        self.assertIn("--crate openlark-security", monitor)
        self.assertIn("--crate openlark-auth", monitor)
        self.assertNotIn("--strict tokens", monitor)

    def test_attendance_field_monitor_is_not_strict(self) -> None:
        attendance_block = self._step_run_block_containing("--biz-tag attendance")
        self.assertIn("--crate openlark-hr", attendance_block)
        self.assertIn("--fields", attendance_block)
        self.assertIn("--live-fields", attendance_block)
        self.assertNotIn("--strict fields", attendance_block)
        self.assertIn("api_contract_fields/attendance", attendance_block)

    def test_docs_field_monitor_is_not_strict(self) -> None:
        docs_block = self._step_run_block_containing("--crate openlark-docs")
        self.assertIn("--fields", docs_block)
        self.assertIn("--live-fields", docs_block)
        self.assertNotIn("--strict fields", docs_block)
        self.assertIn("api_contract_fields/docs", docs_block)

    def test_field_strict_inventory_is_empty_until_trusted(self) -> None:
        strict_fields_steps = [
            block
            for block in self._step_run_blocks()
            if "--strict fields" in block
        ]
        self.assertEqual(len(strict_fields_steps), len(FIELD_STRICT_DOMAINS))

    def test_field_monitor_inventory_is_exactly_attendance_and_docs(self) -> None:
        for domain in FIELD_EVIDENCE_MONITOR_DOMAINS:
            block = self._step_run_block_containing(
                f"--crate {domain['crate']}"
            )
            self.assertIn(domain["report_dir_fragment"], block)
            self.assertNotIn("--strict fields", block)
            if domain["biz_tag"] is not None:
                self.assertIn(f"--biz-tag {domain['biz_tag']}", block)

    def test_monitors_do_not_masquerade_as_strict_or_hide_failures(self) -> None:
        self.assertNotIn("continue-on-error", self.job)
        self.assertNotIn("|| true", self.job)
        self.assertNotIn("playwright", self.job.lower())
        self.assertNotIn("recorded", self.job.lower())

    def test_no_monorepo_wide_field_strict(self) -> None:
        """Non-goal: never open monorepo-wide --strict fields in one shot (#586)."""
        for block in self._step_run_blocks():
            if "--strict fields" not in block:
                continue
            self.assertNotIn(
                "--all-crates",
                block,
                "field-strict must stay domain-scoped; do not pair "
                "--all-crates with --strict fields",
            )

    def test_admission_policy_doc_defines_next_domain_slot(self) -> None:
        """Docs pin admission criteria + dual-edit + non-goals for next domain."""
        self.assertTrue(
            CONTRACT_POLICY_DOC.is_file(),
            f"missing contract policy doc: {CONTRACT_POLICY_DOC}",
        )
        text = CONTRACT_POLICY_DOC.read_text(encoding="utf-8")

        # Next-domain admission criteria (slot + rules).
        self.assertRegex(
            text,
            r"next field-strict domain|下一[个个]? field-strict 域|下一域 admission",
            "docs must name the next field-strict domain admission slot",
        )
        self.assertRegex(
            text,
            r"0\s*ERROR|0 error|live baseline",
            "docs must require live baseline 0 ERROR before flipping strict",
        )
        self.assertRegex(
            text,
            r"dual-edit|双改|workflow.*inventory|ci\.yml.*inventory|gate inventory",
            "docs must state the dual-edit rule (workflow + inventory test)",
        )

        # Explicit non-goals restated.
        self.assertRegex(
            text,
            r"monorepo[- ]wide|--all-crates.*--strict fields|全仓.*strict fields",
            "docs must restate non-goal: no monorepo-wide field strict",
        )
        self.assertRegex(
            text,
            r"threshold|阈值",
            "docs must restate non-goal: no hard-gate threshold lowering",
        )

        # Current inventory still named so the slot is relative to known domains.
        self.assertIn("attendance", text)
        self.assertIn("openlark-docs", text)
        self.assertIn("test_validate_api_contracts_ci_gates", text)

    def test_gate_inventory_verification_section_present(self) -> None:
        """Written verification that 0.20 trust gates remain pinned (#586 AC)."""
        text = CONTRACT_POLICY_DOC.read_text(encoding="utf-8")
        self.assertRegex(
            text,
            r"gate inventory|门禁清单|trust gate",
            "docs must include a written gate-inventory verification section",
        )
        # Endpoint + token + field layers must all be listed.
        self.assertIn("--strict endpoint", text)
        self.assertIn("--strict tokens", text)
        self.assertIn("--strict fields", text)
        # Coverage locks referenced (not weakened).
        self.assertRegex(
            text,
            r"typed.coverage|typed_coverage_release|path_noise|true_gap",
            "docs must reference typed-coverage / path_noise locks",
        )

    def _step_run_blocks(self) -> list[str]:
        return re.split(r"(?m)^(?=      - (?:name|uses):)", self.job)

    def _step_run_block_containing(self, needle: str) -> str:
        # Split on step headers (`- name:` / `- uses:`) at the job step indent.
        for block in self._step_run_blocks():
            if needle in block:
                return block
        self.fail(f"no api-contracts step contains {needle!r}")


if __name__ == "__main__":
    unittest.main()
