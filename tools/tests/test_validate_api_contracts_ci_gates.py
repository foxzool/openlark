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

# Canonical field-strict inventory for the institutionalization gate (#586).
# Expanding this list requires the dual-edit: ci.yml step + this constant +
# the inventory assertions below, in the *same* PR.
FIELD_STRICT_DOMAINS = (
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

    def test_endpoint_strict_covers_all_crates(self) -> None:
        self.assertIn("--all-crates", self.job)
        self.assertIn("--strict endpoint", self.job)
        self.assertIn("--live-endpoints", self.job)

    def test_token_strict_covers_security_and_auth(self) -> None:
        self.assertIn("--crate openlark-security", self.job)
        self.assertIn("--crate openlark-auth", self.job)
        self.assertGreaterEqual(self.job.count("--strict tokens"), 2)

    def test_attendance_field_strict_gate(self) -> None:
        self.assertIn("--crate openlark-hr", self.job)
        self.assertIn("--biz-tag attendance", self.job)
        self.assertIn("--strict fields", self.job)
        self.assertIn("api_contract_fields/attendance", self.job)
        attendance_block = self._step_run_block_containing("--biz-tag attendance")
        self.assertIn("--fields", attendance_block)
        self.assertIn("--live-fields", attendance_block)
        self.assertIn("--strict fields", attendance_block)

    def test_docs_field_strict_gate(self) -> None:
        """First 0.20 domain expansion beyond attendance (#569).

        openlark-docs (ccm/base/baike/minutes) must run live field validation
        under --strict fields so required-body drift fails CI.
        """
        self.assertIn("--crate openlark-docs", self.job)
        self.assertIn("api_contract_fields/docs", self.job)
        # Require the docs step to enable strict fields (not a non-strict monitor).
        docs_block = self._step_run_block_containing("--crate openlark-docs")
        self.assertIn("--fields", docs_block)
        self.assertIn("--live-fields", docs_block)
        self.assertIn("--strict fields", docs_block)

    def test_field_strict_inventory_is_exactly_attendance_and_docs(self) -> None:
        """Institutionalized inventory: only the admitted domains are field-strict.

        Adding a third domain is intentional expansion — update
        FIELD_STRICT_DOMAINS + ci.yml + admission docs together (#586 dual-edit).
        """
        strict_fields_steps = [
            block
            for block in self._step_run_blocks()
            if "--strict fields" in block
        ]
        self.assertEqual(
            len(strict_fields_steps),
            len(FIELD_STRICT_DOMAINS),
            "field-strict step count must match FIELD_STRICT_DOMAINS inventory",
        )
        for domain in FIELD_STRICT_DOMAINS:
            self.assertIn(f"--crate {domain['crate']}", self.job)
            self.assertIn(domain["report_dir_fragment"], self.job)
            if domain["biz_tag"] is not None:
                self.assertIn(f"--biz-tag {domain['biz_tag']}", self.job)

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
