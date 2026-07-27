"""Pin the domain list of CI api-contracts strict gates.

These assertions are the seam for domain-by-domain contract expansion (#566/#569):
adding a new strict domain must update both `.github/workflows/ci.yml` and this
test so the gate inventory cannot drift silently.
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CI_WORKFLOW_PATH = ROOT / ".github" / "workflows" / "ci.yml"


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

    def test_token_strict_covers_security_and_auth(self) -> None:
        self.assertIn("--crate openlark-security", self.job)
        self.assertIn("--crate openlark-auth", self.job)
        self.assertGreaterEqual(self.job.count("--strict tokens"), 2)

    def test_attendance_field_strict_gate(self) -> None:
        self.assertIn("--crate openlark-hr", self.job)
        self.assertIn("--biz-tag attendance", self.job)
        self.assertIn("--strict fields", self.job)
        self.assertIn("api_contract_fields/attendance", self.job)

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

    def _step_run_block_containing(self, needle: str) -> str:
        # Split on step headers (`- name:` / `- uses:`) at the job step indent.
        parts = re.split(r"(?m)^(?=      - (?:name|uses):)", self.job)
        for block in parts:
            if needle in block:
                return block
        self.fail(f"no api-contracts step contains {needle!r}")


if __name__ == "__main__":
    unittest.main()
