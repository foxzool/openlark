"""Pin typed-coverage release hard-gate floors (#586 institutionalization).

Thresholds may only rise (or stay) via an explicit policy PR — never lower
to force PASS. Floor pins below are the 0.20 institutionalized values.
"""

import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
POLICY_PATH = ROOT / "tools" / "typed_coverage_release.toml"
CRITERIA_DOC = ROOT / "docs" / "typed-coverage-release-criteria.md"

# 0.20 institutionalized floors — do not lower without an explicit policy PR.
SUMMARY_COMPLETION_FLOOR = 93.0
CORE_BUSINESS_COMPLETION_FLOOR = 92.0
CORE_CRATE_COMPLETION_FLOOR = 80.0


class TypedCoverageReleasePolicyTests(unittest.TestCase):
    def test_release_policy_toml_is_parseable_and_has_required_sections(self):
        payload = tomllib.loads(POLICY_PATH.read_text(encoding="utf-8"))

        self.assertEqual(payload["policy_name"], "typed-coverage-stable-release")
        self.assertIn("inputs", payload)
        self.assertIn("stable_release", payload)

        stable_release = payload["stable_release"]
        self.assertIn("hard_gates", stable_release)
        self.assertIn("waiver_gates", stable_release)
        self.assertIn("waiver", stable_release)
        self.assertIn("reporting", stable_release)

        hard_gates = stable_release["hard_gates"]
        self.assertGreaterEqual(
            hard_gates["summary_completion_rate_min"], SUMMARY_COMPLETION_FLOOR
        )
        self.assertGreaterEqual(
            hard_gates["core_business_completion_rate_min"],
            CORE_BUSINESS_COMPLETION_FLOOR,
        )
        self.assertGreaterEqual(
            hard_gates["core_crate_completion_rate_min"], CORE_CRATE_COMPLETION_FLOOR
        )

        waiver = stable_release["waiver"]
        self.assertIn("maintainer", waiver["required_approvers"])
        self.assertIn("domain-owner", waiver["required_approvers"])
        self.assertIn("target_release", waiver["required_fields"])

    def test_hard_gate_floors_are_not_lowered(self):
        """#586 non-goal: no hard-gate threshold lowering to game CI green."""
        payload = tomllib.loads(POLICY_PATH.read_text(encoding="utf-8"))
        hard_gates = payload["stable_release"]["hard_gates"]
        self.assertEqual(
            hard_gates["summary_completion_rate_min"], SUMMARY_COMPLETION_FLOOR
        )
        self.assertEqual(
            hard_gates["core_business_completion_rate_min"],
            CORE_BUSINESS_COMPLETION_FLOOR,
        )
        self.assertEqual(
            hard_gates["core_crate_completion_rate_min"], CORE_CRATE_COMPLETION_FLOOR
        )

    def test_criteria_doc_restates_no_threshold_lowering(self):
        text = CRITERIA_DOC.read_text(encoding="utf-8")
        self.assertRegex(
            text,
            r"不得下调|must not lower|不.*降低.*阈值|threshold.*not lower|no.*threshold lowering",
            "criteria doc must restate non-goal: no hard-gate threshold lowering",
        )


if __name__ == "__main__":
    unittest.main()
