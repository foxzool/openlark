"""Assert the 0.19.0 pre-release sign-off record meets acceptance seams (#559)."""

from __future__ import annotations

import re
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SIGNOFF_PATH = ROOT / "docs" / "0.19.0_RELEASE_SIGNOFF.md"


class ReleaseSignoff019Tests(unittest.TestCase):
    def test_signoff_document_exists(self) -> None:
        self.assertTrue(
            SIGNOFF_PATH.is_file(),
            f"missing sign-off record: {SIGNOFF_PATH}",
        )

    def test_signoff_records_go_decision_and_gate_pass(self) -> None:
        text = SIGNOFF_PATH.read_text(encoding="utf-8")

        self.assertIn("0.19.0", text)
        self.assertRegex(
            text,
            r"\*\*GO\b|Decision\s*\|\s*\*\*GO",
            "sign-off must record an explicit GO decision for tagging",
        )
        self.assertIn("PASS", text)
        self.assertIn("allowed to tag", text)
        self.assertIn("#559", text)
        self.assertIn("tools/typed_coverage_release.toml", text)
        self.assertIn("docs/api-compatibility-release-checklist.md", text)

        # Frozen metrics from the gate on the 0.19 tree (do not lower thresholds).
        self.assertRegex(text, r"93\.9%")
        self.assertRegex(text, r"97\.8%")

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


if __name__ == "__main__":
    unittest.main()
