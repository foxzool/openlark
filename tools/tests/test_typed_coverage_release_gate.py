import importlib.util
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "check_typed_coverage_release.py"
SPEC = importlib.util.spec_from_file_location("check_typed_coverage_release", MODULE_PATH)
check_typed_coverage_release = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(check_typed_coverage_release)


class TypedCoverageReleaseGateTests(unittest.TestCase):
    POLICY = {
        "stable_release": {
            "hard_gates": {
                "summary_completion_rate_min": 93.0,
                "core_business_completion_rate_min": 92.0,
                "core_crate_completion_rate_min": 80.0,
            },
            "waiver_gates": {"core_business_p0_missing": "waiver_required"},
        }
    }

    def test_passes_when_all_hard_and_waiver_gates_pass(self):
        summary = {"completion_rate": 95.0}
        dashboard = {
            "completion_rate": 96.0,
            "priority_counts": {},
            "crates": [
                {"crate": "openlark-workflow", "completion_rate": 94.0},
                {"crate": "openlark-security", "completion_rate": 91.0},
            ],
        }

        result = check_typed_coverage_release.evaluate_release_gate(
            self.POLICY, summary, dashboard
        )

        self.assertEqual(result.status, "PASS")
        self.assertEqual(result.failures, [])

    def test_blocks_when_any_hard_gate_fails(self):
        summary = {"completion_rate": 88.5}
        dashboard = {
            "completion_rate": 90.1,
            "priority_counts": {"P0": 52},
            "crates": [
                {"crate": "openlark-workflow", "completion_rate": 43.6},
                {"crate": "openlark-security", "completion_rate": 51.9},
            ],
        }

        result = check_typed_coverage_release.evaluate_release_gate(
            self.POLICY, summary, dashboard
        )

        self.assertEqual(result.status, "BLOCKED")
        self.assertEqual(len(result.failures), 4)
        self.assertIn("openlark-workflow", "\n".join(result.failures))

    def test_requires_waiver_when_only_p0_gap_remains(self):
        summary = {"completion_rate": 95.0}
        dashboard = {
            "completion_rate": 96.0,
            "priority_counts": {"P0": 1},
            "crates": [{"crate": "openlark-workflow", "completion_rate": 94.0}],
        }

        result = check_typed_coverage_release.evaluate_release_gate(
            self.POLICY, summary, dashboard
        )

        self.assertEqual(result.status, "WAIVER REQUIRED")
        self.assertEqual(result.failures, [])


if __name__ == "__main__":
    unittest.main()
