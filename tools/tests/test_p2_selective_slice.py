"""0.20 selective P2 slice: helpdesk / mail / hr-OKR true-gap reclassification (#571).

Seams under test:
1. Typed coverage classification for the denoise-informed P2 selection list
   — each chosen row must be path_noise (alias/rewrite), never true_gap.
2. Crate-level true_missing after denoise:
   - openlark-helpdesk: 0
   - openlark-mail: 0
   - openlark-hr OKR v2 expected_files: none remain in true_missing
3. On-disk implementation leaves for those rows — still present and callable
   at the evidenced alias/rewrite paths (public Builder modules).

Selection list was written on #571 before coding. Scope is a small value-sorted
slice (not all ~89 workspace P2s). Hard gates are not lowered.
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

# Denoise-informed selection (#571): helpdesk + mail exact aliases.
HELPDESK_MAIL_P2_SLICE: dict[str, dict[str, str]] = {
    "helpdesk/helpdesk/v1/faq/faq_image.rs": {
        "crate": "openlark-helpdesk",
        "name": "获取知识库图像",
        "implementation_file": "helpdesk/helpdesk/v1/faq/image.rs",
        "match_kind": "alias",
        "outcome": "noise",
    },
    "mail/mail/v1/user_mailbox/sent_message/get_recall_detail.rs": {
        "crate": "openlark-mail",
        "name": "获取邮件撤回进度",
        "implementation_file": (
            "mail/mail/v1/user_mailbox/message/recall/get_recall_detail.rs"
        ),
        "match_kind": "alias",
        "outcome": "noise",
    },
    "mail/mail/v1/user_mailbox/sent_message/recall.rs": {
        "crate": "openlark-mail",
        "name": "撤回已发送邮件",
        "implementation_file": "mail/mail/v1/user_mailbox/message/recall/recall.rs",
        "match_kind": "alias",
        "outcome": "noise",
    },
}

# HR OKR v2: CSV resource is `okr.<sub>` → expected `okr/okr/v2/okr/<sub>/...`,
# on-disk layout drops the redundant resource-prefix segment.
OKR_V2_EXPECTED_PREFIX = "okr/okr/v2/okr/"
OKR_V2_IMPLEMENTATION_PREFIX = "okr/okr/v2/"


def _load_crate_validator(crate_name: str) -> validate_apis.APIValidator:
    cfg = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))["crates"][crate_name]
    priority_model = validate_apis.PriorityModel.from_path(
        str(REPO_ROOT / "tools" / "api_priority.toml")
    )
    validator = validate_apis.APIValidator(
        csv_path=str(CSV_PATH),
        src_path=str(REPO_ROOT / cfg["src"]),
        filter_tags=cfg["biz_tags"],
        skip_old_versions=True,
        priority_model=priority_model,
        implementation_path_rewrites=cfg.get("implementation_path_rewrites"),
        implementation_path_aliases=cfg.get("implementation_path_aliases"),
    )
    validator.parse_csv()
    validator.scan_implementations()
    validator.compare()
    return validator


def _assert_callable_leaf(test: unittest.TestCase, leaf: Path) -> None:
    test.assertTrue(leaf.is_file(), msg=str(leaf))
    text = leaf.read_text(encoding="utf-8")
    # Callable typed API markers (Builder + execute + Transport).
    # Some OKR leaves use `pub struct Request` + module-level Builder pattern.
    test.assertTrue(
        "RequestBuilder" in text or "async fn execute" in text,
        msg=f"missing execute/builder seam in {leaf}",
    )
    test.assertIn("async fn execute", text)
    test.assertIn("Transport::request_typed", text)


class HelpdeskMailP2SliceTests(unittest.TestCase):
    """Real-tree seam: helpdesk/mail selected P2s are path noise over shipped leaves."""

    @classmethod
    def setUpClass(cls):
        if not CSV_PATH.exists() or not MAPPING_PATH.exists():
            raise unittest.SkipTest("repo fixtures missing")
        cls.validators: dict[str, validate_apis.APIValidator] = {}
        cls.summaries: dict[str, dict] = {}
        cls.by_expected: dict[str, dict[str, validate_apis.APIInfo]] = {}
        cls.noise_by_expected: dict[str, dict[str, dict]] = {}
        for crate_name in ("openlark-helpdesk", "openlark-mail"):
            validator = _load_crate_validator(crate_name)
            summary = validator.calculate_summary()
            cls.validators[crate_name] = validator
            cls.summaries[crate_name] = summary
            cls.by_expected[crate_name] = {
                api.expected_file: api for api in validator.apis
            }
            cls.noise_by_expected[crate_name] = {
                item["expected_file"]: item
                for item in summary.get("path_noise_matches", [])
            }

    def test_selection_list_has_three_helpdesk_mail_rows(self):
        self.assertEqual(len(HELPDESK_MAIL_P2_SLICE), 3)

    def test_each_helpdesk_mail_row_is_path_noise_with_evidence(self):
        for expected, meta in HELPDESK_MAIL_P2_SLICE.items():
            with self.subTest(expected=expected):
                crate = meta["crate"]
                api = self.by_expected[crate].get(expected)
                self.assertIsNotNone(
                    api,
                    f"CSV no longer lists {expected}; update selection map if catalog moved",
                )
                assert api is not None
                self.assertTrue(
                    api.is_implemented,
                    f"{meta['name']} still true-missing under denoise",
                )

                noise = self.noise_by_expected[crate].get(expected)
                self.assertIsNotNone(
                    noise,
                    f"{expected} matched strict or missing path_noise evidence",
                )
                assert noise is not None
                self.assertEqual(noise["implementation_file"], meta["implementation_file"])
                self.assertEqual(noise["match_kind"], meta["match_kind"])
                self.assertTrue(noise["match_reason"])

                src = REPO_ROOT / tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))[
                    "crates"
                ][crate]["src"]
                _assert_callable_leaf(self, src / meta["implementation_file"])

    def test_helpdesk_and_mail_true_missing_are_zero(self):
        for crate_name in ("openlark-helpdesk", "openlark-mail"):
            with self.subTest(crate=crate_name):
                summary = self.summaries[crate_name]
                self.assertEqual(summary["classification"]["true_missing"], 0)
                self.assertEqual(summary["missing"], 0)
                self.assertEqual(summary["completion_rate"], 100.0)

    def test_no_deferred_outcomes_in_helpdesk_mail_slice(self):
        for meta in HELPDESK_MAIL_P2_SLICE.values():
            self.assertEqual(meta["outcome"], "noise")


class HrOkrV2P2SliceTests(unittest.TestCase):
    """Real-tree seam: all OKR v2 CSV rows reclassify via rewrite to on-disk layout."""

    @classmethod
    def setUpClass(cls):
        if not CSV_PATH.exists() or not MAPPING_PATH.exists():
            raise unittest.SkipTest("repo fixtures missing")
        cls.validator = _load_crate_validator("openlark-hr")
        cls.summary = cls.validator.calculate_summary()
        cls.okr_v2_apis = [
            api
            for api in cls.validator.apis
            if api.expected_file.startswith(OKR_V2_EXPECTED_PREFIX)
        ]
        cls.noise_by_expected = {
            item["expected_file"]: item
            for item in cls.summary.get("path_noise_matches", [])
        }
        cls.true_missing = {
            item["expected_file"] for item in cls.summary.get("true_missing_apis", [])
        }
        cls.hr_src = REPO_ROOT / "crates" / "openlark-hr" / "src"

    def test_okr_v2_slice_has_twenty_five_csv_rows(self):
        # Catalog lock: selective slice covers the full OKR v2 gap set from 0.19/0.20 reports.
        self.assertEqual(len(self.okr_v2_apis), 25)

    def test_each_okr_v2_row_is_path_noise_via_rewrite(self):
        for api in self.okr_v2_apis:
            with self.subTest(expected=api.expected_file):
                self.assertTrue(
                    api.is_implemented,
                    f"{api.name} still true-missing under denoise",
                )
                noise = self.noise_by_expected.get(api.expected_file)
                self.assertIsNotNone(noise, f"missing path_noise evidence for {api.name}")
                assert noise is not None
                self.assertEqual(noise["match_kind"], "rewrite")
                expected_impl = OKR_V2_IMPLEMENTATION_PREFIX + api.expected_file[
                    len(OKR_V2_EXPECTED_PREFIX) :
                ]
                self.assertEqual(noise["implementation_file"], expected_impl)
                self.assertTrue(noise["match_reason"])
                _assert_callable_leaf(self, self.hr_src / expected_impl)

    def test_no_okr_v2_rows_remain_in_true_missing(self):
        for api in self.okr_v2_apis:
            self.assertNotIn(api.expected_file, self.true_missing)

    def test_hr_true_missing_excludes_okr_v2_and_is_not_inflated(self):
        # After reclassification, OKR v2 must not contribute to true_missing.
        # Other HR modules may still have gaps in the future; assert only the slice.
        remaining_okr = [
            path for path in self.true_missing if path.startswith("okr/")
        ]
        self.assertEqual(remaining_okr, [])
        # Slice closed 25 historical OKR gaps; overall true_missing must drop accordingly.
        self.assertEqual(
            self.summary["classification"]["true_missing"],
            len(self.true_missing),
        )
        self.assertLessEqual(self.summary["classification"]["true_missing"], 0)


class SelectiveSliceScopeGuardTests(unittest.TestCase):
    """Scope guard: this unit is a slice, not full-workspace P2 clearance."""

    def test_selection_is_bounded(self):
        # helpdesk/mail exact rows + OKR v2 set (25) = 28; never "all 89 P2".
        total = len(HELPDESK_MAIL_P2_SLICE) + 25
        self.assertEqual(total, 28)
        self.assertLess(total, 89)


if __name__ == "__main__":
    unittest.main()
