"""Assert the 0.20.0 pre-release sign-off record meets acceptance seams (#573).

Seams under test:
- in-repo GO sign-off document exists before tag
- explicit GO decision + gate PASS language
- pins full base SHA, policy paths, checklist walk
- does not lower thresholds (policy path present; no "lower threshold" language)
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SIGNOFF_PATH = ROOT / "docs" / "0.20.0_RELEASE_SIGNOFF.md"


class ReleaseSignoff020Tests(unittest.TestCase):
    def test_signoff_document_exists(self) -> None:
        self.assertTrue(
            SIGNOFF_PATH.is_file(),
            f"missing sign-off record: {SIGNOFF_PATH}",
        )

    def test_signoff_records_go_decision_and_gate_pass(self) -> None:
        text = SIGNOFF_PATH.read_text(encoding="utf-8")

        self.assertIn("0.20.0", text)
        self.assertRegex(
            text,
            r"\*\*GO\b|Decision\s*\|\s*\*\*GO",
            "sign-off must record an explicit GO decision for tagging",
        )
        self.assertIn("PASS", text)
        self.assertIn("allowed to tag", text)
        self.assertIn("#573", text)
        self.assertIn("tools/typed_coverage_release.toml", text)
        self.assertIn("docs/api-compatibility-release-checklist.md", text)
        self.assertIn("check-pre-release-compatibility", text)

        # Threshold discipline: policy present; no lowering claim.
        self.assertNotRegex(
            text,
            r"lower(ed|ing)?\s+threshold",
            re.IGNORECASE,
        )

        # Base tree SHA is a full 40-char hex so reviewers can pin the sign-off.
        self.assertRegex(
            text,
            r"\b[0-9a-f]{40}\b",
            "sign-off must pin a full base SHA",
        )

        # Checklist sections A–E must be walked (checked boxes present).
        checked = len(re.findall(r"- \[x\]", text, flags=re.IGNORECASE))
        self.assertGreaterEqual(
            checked,
            15,
            "API compatibility checklist should be walked with [x] marks",
        )

        # Typed coverage gate numbers must be frozen (any plausible completion %).
        self.assertRegex(text, r"completion[^\n%]*\d+\.\d+%", re.IGNORECASE)
        self.assertIn("Typed coverage", text)

        # Packaging unit scope: AC4 cut is deferred; do not claim ship complete.
        self.assertRegex(
            text,
            r"Packaging \+ pre-release GO only|Unit scope",
            "sign-off must declare packaging-only unit scope",
        )
        self.assertRegex(
            text,
            r"post-merge cut|Deferred",
            "sign-off must defer AC4 tag/Release/crates.io to post-merge cut",
        )
        self.assertRegex(
            text,
            re.compile(r"Refs #573|do not.*Fixes #573", re.IGNORECASE),
            "sign-off must forbid Fixes #573 until cut verifies",
        )


if __name__ == "__main__":
    unittest.main()
