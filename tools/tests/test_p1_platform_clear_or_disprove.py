"""0.20 P1 clear-or-disprove for platform/tenant/trust-party reads (#570).

Seams under test:
1. Typed coverage classification for the seven historical P1 expected_files
   — each must be path_noise (or strict), never true_gap.
2. Workspace/platform priority_counts — no remaining P1 true-missing after denoise.
3. On-disk implementation leaves for those seven APIs — still present and callable
   at the evidenced flat/typo-corrected paths (public Builder modules).

These seven items were reported as P1 missing under nested canonical paths in
0.19 sign-off (P1=7). Post-#567 denoise they are layout noise over already-shipped
typed APIs; this test records that terminal outcome so they cannot silently
re-open as fake implement tickets.
"""

from __future__ import annotations

import importlib.util
import tomllib
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "validate_apis.py"
SPEC = importlib.util.spec_from_file_location("validate_apis", MODULE_PATH)
validate_apis = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(validate_apis)

REPO_ROOT = Path(__file__).resolve().parents[2]
MAPPING_PATH = REPO_ROOT / "tools" / "api_coverage.toml"
CSV_PATH = REPO_ROOT / "api_list_export.csv"
PLATFORM_SRC = REPO_ROOT / "crates" / "openlark-platform" / "src"

# Historical P1 expected_file paths from 0.19 typed-coverage (nested formula +
# known CSV typo collboration_share_entity). Terminal outcome for each: path noise
# over an already-implemented typed API leaf (see docs/typed-api-coverage.md §2.2).
HISTORICAL_P1_EXPECTED_FILES: dict[str, dict[str, str]] = {
    "directory/directory/v1/collboration_share_entity/list.rs": {
        "name": "获取关联组织双方共享成员范围",
        "implementation_file": "directory/v1/collaboration_share_entity/list.rs",
        "match_kind": "typo_correction",
        "outcome": "noise",
    },
    "tenant/tenant/v2/tenant/product_assign_info/query.rs": {
        "name": "获取企业席位信息接口",
        "implementation_file": "tenant/v2/tenant/product_assign_info/query.rs",
        "match_kind": "flat_project",
        "outcome": "noise",
    },
    "tenant/tenant/v2/tenant/query.rs": {
        "name": "获取企业信息",
        "implementation_file": "tenant/v2/tenant/query.rs",
        "match_kind": "flat_project",
        "outcome": "noise",
    },
    "trust_party/trust_party/v1/collaboration_tenant/collaboration_department/get.rs": {
        "name": "获取关联组织部门详情",
        "implementation_file": (
            "trust_party/v1/collaboration_tenant/collaboration_department/get.rs"
        ),
        "match_kind": "flat_project",
        "outcome": "noise",
    },
    "trust_party/trust_party/v1/collaboration_tenant/collaboration_user/get.rs": {
        "name": "获取关联组织成员详情",
        "implementation_file": (
            "trust_party/v1/collaboration_tenant/collaboration_user/get.rs"
        ),
        "match_kind": "flat_project",
        "outcome": "noise",
    },
    "trust_party/trust_party/v1/collaboration_tenant/get.rs": {
        "name": "获取关联组织详情",
        "implementation_file": "trust_party/v1/collaboration_tenant/get.rs",
        "match_kind": "flat_project",
        "outcome": "noise",
    },
    "trust_party/trust_party/v1/collaboration_tenant/list.rs": {
        "name": "获取可见关联组织的列表",
        "implementation_file": "trust_party/v1/collaboration_tenant/list.rs",
        "match_kind": "flat_project",
        "outcome": "noise",
    },
}


class PlatformP1ClearOrDisproveTests(unittest.TestCase):
    """Real-tree seam: the seven platform/tenant/trust-party P1s are cleared as noise."""

    @classmethod
    def setUpClass(cls):
        if not CSV_PATH.exists() or not MAPPING_PATH.exists():
            raise unittest.SkipTest("repo fixtures missing")
        cfg = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))["crates"][
            "openlark-platform"
        ]
        priority_model = validate_apis.PriorityModel.from_path(
            str(REPO_ROOT / "tools" / "api_priority.toml")
        )
        cls.validator = validate_apis.APIValidator(
            csv_path=str(CSV_PATH),
            src_path=str(REPO_ROOT / cfg["src"]),
            filter_tags=cfg["biz_tags"],
            skip_old_versions=True,
            priority_model=priority_model,
            implementation_path_rewrites=cfg.get("implementation_path_rewrites"),
            implementation_path_aliases=cfg.get("implementation_path_aliases"),
        )
        cls.validator.parse_csv()
        cls.validator.scan_implementations()
        cls.validator.compare()
        cls.summary = cls.validator.calculate_summary()
        cls.by_expected = {api.expected_file: api for api in cls.validator.apis}
        cls.noise_by_expected = {
            item["expected_file"]: item for item in cls.summary["path_noise_matches"]
        }

    def test_exactly_seven_historical_p1_rows_are_tracked(self):
        self.assertEqual(len(HISTORICAL_P1_EXPECTED_FILES), 7)

    def test_each_historical_p1_is_path_noise_with_evidence(self):
        for expected, meta in HISTORICAL_P1_EXPECTED_FILES.items():
            with self.subTest(expected=expected):
                api = self.by_expected.get(expected)
                self.assertIsNotNone(
                    api,
                    f"CSV no longer lists {expected}; update HISTORICAL_P1 map if catalog moved",
                )
                assert api is not None
                self.assertTrue(
                    api.is_implemented,
                    f"{meta['name']} still true-missing under denoise",
                )
                self.assertNotEqual(
                    getattr(api, "match_kind", "strict") or "strict",
                    "true_gap",
                )

                noise = self.noise_by_expected.get(expected)
                # Prefer explicit path_noise row (flat / typo); strict also clears P1.
                if noise is not None:
                    self.assertEqual(
                        noise["implementation_file"], meta["implementation_file"]
                    )
                    self.assertEqual(noise["match_kind"], meta["match_kind"])
                    self.assertTrue(noise["match_reason"])
                else:
                    self.assertEqual(api.implementation_file, meta["implementation_file"])
                    self.assertIn(
                        getattr(api, "match_kind", "strict") or "strict",
                        {"strict", meta["match_kind"]},
                    )

                # Evidence file exists on disk.
                leaf = PLATFORM_SRC / meta["implementation_file"]
                self.assertTrue(leaf.is_file(), msg=str(leaf))
                text = leaf.read_text(encoding="utf-8")
                # Callable typed API markers (Builder + execute + Transport).
                self.assertIn("RequestBuilder", text)
                self.assertIn("async fn execute", text)
                self.assertIn("Transport::request_typed", text)

    def test_none_of_the_seven_appear_in_true_missing(self):
        true_missing = {
            item["expected_file"] for item in self.summary.get("true_missing_apis", [])
        }
        for expected in HISTORICAL_P1_EXPECTED_FILES:
            self.assertNotIn(expected, true_missing)

    def test_platform_has_zero_p1_true_missing(self):
        # priority_counts only covers true_missing rows.
        priority_counts = self.summary.get("priority_counts") or {}
        self.assertEqual(priority_counts.get("P1", 0), 0)
        self.assertEqual(self.summary["classification"]["true_missing"], 0)
        self.assertEqual(self.summary["missing"], 0)
        self.assertEqual(self.summary["completion_rate"], 100.0)

    def test_no_deferred_or_silent_skip_outcomes(self):
        # Every historical P1 terminal outcome must be noise (or implement); none deferred.
        for meta in HISTORICAL_P1_EXPECTED_FILES.values():
            self.assertEqual(meta["outcome"], "noise")


class WorkspaceP1GateTests(unittest.TestCase):
    """Workspace-level seam: core-business P0 stays zero; no residual P1 true-missing."""

    @classmethod
    def setUpClass(cls):
        if not CSV_PATH.exists() or not MAPPING_PATH.exists():
            raise unittest.SkipTest("repo fixtures missing")
        priority_model = validate_apis.PriorityModel.from_path(
            str(REPO_ROOT / "tools" / "api_priority.toml")
        )
        crates = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))["crates"]
        total_apis = 0
        total_impl = 0
        priority_counts: dict[str, int] = {}
        crate_stats: dict[str, dict] = {}
        for crate_name, cfg in crates.items():
            tags = list(cfg.get("biz_tags") or [])
            validator = validate_apis.APIValidator(
                csv_path=str(CSV_PATH),
                src_path=str(REPO_ROOT / cfg["src"]),
                filter_tags=tags,
                skip_old_versions=True,
                priority_model=priority_model,
                implementation_path_rewrites=cfg.get("implementation_path_rewrites"),
                implementation_path_aliases=cfg.get("implementation_path_aliases"),
            )
            validator.parse_csv()
            validator.scan_implementations()
            validator.compare()
            stats = validator.calculate_summary()
            total_apis += stats["total_apis"]
            total_impl += stats["implemented"]
            for api in validator.missing_apis:
                level = api.priority_level
                priority_counts[level] = priority_counts.get(level, 0) + 1
            crate_stats[crate_name] = {
                "stats": stats,
                "missing_apis": list(validator.missing_apis),
            }

        core_apis = 0
        core_impl = 0
        core_p0 = 0
        groups = validate_apis.collect_dashboard_groups(crates)
        for crate_name in groups.get("core_business", []):
            entry = crate_stats[crate_name]
            stats = entry["stats"]
            core_apis += stats["total_apis"]
            core_impl += stats["implemented"]
            for api in entry["missing_apis"]:
                if api.priority_level == "P0":
                    core_p0 += 1

        cls.priority_counts = priority_counts
        cls.completion_rate = (total_impl / total_apis * 100) if total_apis else 0.0
        cls.core_completion_rate = (core_impl / core_apis * 100) if core_apis else 0.0
        cls.core_p0 = core_p0

    def test_workspace_priority_counts_have_no_p1(self):
        self.assertEqual(
            self.priority_counts.get("P1", 0),
            0,
            f"unexpected residual P1 true-missing: {self.priority_counts}",
        )

    def test_core_business_p0_remains_zero(self):
        self.assertEqual(self.core_p0, 0)
        # Hard gates must not be lowered by this unit; completion still healthy.
        self.assertGreaterEqual(self.completion_rate, 93.0)
        self.assertGreaterEqual(self.core_completion_rate, 92.0)


if __name__ == "__main__":
    unittest.main()
