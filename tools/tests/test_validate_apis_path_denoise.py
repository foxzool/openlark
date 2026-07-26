"""Coverage path denoise + classified missing reports (#567).

Seams under test:
1. APIValidator.compare / path candidates — flat vs nested layout matching
2. calculate_summary classification — true_gap / path_noise / extra_files
3. Real openlark-platform tree — before/after-style missing breakdown
"""

from __future__ import annotations

import importlib.util
import tempfile
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


def _api(
    *,
    expected_file: str,
    biz_tag: str = "admin",
    project: str = "admin",
    version: str = "v1",
    resource: str = "badge",
    name: str = "list",
    api_id: str = "1",
    url: str = "GET:/open-apis/admin/v1/badges",
    api_name: str = "列勋章",
) -> validate_apis.APIInfo:
    return validate_apis.APIInfo(
        api_id=api_id,
        name=api_name,
        biz_tag=biz_tag,
        meta_project=project,
        meta_version=version,
        meta_resource=resource,
        meta_name=name,
        url=url,
        doc_path="",
        expected_file=expected_file,
    )


class FlatLayoutPathMatchingTests(unittest.TestCase):
    def test_flat_layout_when_project_equals_biz_tag_counts_as_implemented(self):
        """platform-class layout: admin/admin/v1/... lives on disk as admin/v1/..."""
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            actual = source / "admin" / "v1" / "badge" / "list.rs"
            actual.parent.mkdir(parents=True)
            actual.write_text("// implemented", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(expected_file="admin/admin/v1/badge/list.rs"),
            ]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(validator.apis[0].implementation_file, "admin/v1/badge/list.rs")
            self.assertEqual(validator.apis[0].match_kind, "flat_project")
            self.assertEqual(validator.missing_apis, [])
            self.assertEqual(validator.extra_files, set())

    def test_nested_layout_still_preferred_when_both_exist(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            nested = source / "admin" / "admin" / "v1" / "badge" / "list.rs"
            flat = source / "admin" / "v1" / "badge" / "list.rs"
            nested.parent.mkdir(parents=True)
            flat.parent.mkdir(parents=True)
            nested.write_text("// nested", encoding="utf-8")
            flat.write_text("// flat", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(expected_file="admin/admin/v1/badge/list.rs"),
            ]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(validator.apis[0].implementation_file, "admin/admin/v1/badge/list.rs")
            self.assertEqual(validator.apis[0].match_kind, "strict")
            # flat file is extra (not claimed by any API)
            self.assertIn("admin/v1/badge/list.rs", validator.extra_files)

    def test_different_project_does_not_silently_drop_to_flat_biz_only(self):
        """app_engine/apaas/... must not match admin-style flat drop of project."""
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            # Only a wrong flat path exists — must remain a true gap.
            wrong = source / "app_engine" / "v1" / "workspace" / "list.rs"
            wrong.parent.mkdir(parents=True)
            wrong.write_text("// wrong layout", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(
                    expected_file="app_engine/apaas/v1/workspace/list.rs",
                    biz_tag="app_engine",
                    project="apaas",
                    resource="workspace",
                    name="list",
                ),
            ]
            validator.scan_implementations()
            validator.compare()

            self.assertFalse(validator.apis[0].is_implemented)
            self.assertEqual(len(validator.missing_apis), 1)
            self.assertEqual(validator.missing_apis[0].match_kind, "true_gap")


class RustKeywordPathMatchingTests(unittest.TestCase):
    def test_enum_directory_can_match_enum_mod_escape(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            actual = source / "app_engine" / "apaas" / "v1" / "workspace" / "enum_mod" / "list.rs"
            actual.parent.mkdir(parents=True)
            actual.write_text("// keyword escape", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(
                    expected_file="app_engine/apaas/v1/workspace/enum/list.rs",
                    biz_tag="app_engine",
                    project="apaas",
                    resource="workspace.enum",
                    name="list",
                ),
            ]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(
                validator.apis[0].implementation_file,
                "app_engine/apaas/v1/workspace/enum_mod/list.rs",
            )
            self.assertEqual(validator.apis[0].match_kind, "rust_keyword")


class KnownTypoPathMatchingTests(unittest.TestCase):
    def test_csv_collboration_typo_matches_collaboration_on_disk(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            actual = source / "directory" / "v1" / "collaboration_share_entity" / "list.rs"
            actual.parent.mkdir(parents=True)
            actual.write_text("// typo fix", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(
                    expected_file="directory/directory/v1/collboration_share_entity/list.rs",
                    biz_tag="directory",
                    project="directory",
                    resource="collboration_share_entity",
                    name="list",
                ),
            ]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(
                validator.apis[0].implementation_file,
                "directory/v1/collaboration_share_entity/list.rs",
            )
            self.assertIn(validator.apis[0].match_kind, {"typo_correction", "flat_project"})


class ClassificationReportTests(unittest.TestCase):
    def test_summary_separates_true_gap_path_noise_and_extra(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            # path noise (flat layout match)
            flat = source / "admin" / "v1" / "badge" / "list.rs"
            flat.parent.mkdir(parents=True)
            flat.write_text("// flat", encoding="utf-8")
            # true gap: no file
            # extra file unrelated
            extra = source / "admin" / "v1" / "users.rs"
            extra.write_text("// helper", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(
                    api_id="noise-1",
                    expected_file="admin/admin/v1/badge/list.rs",
                    api_name="勋章列表",
                ),
                _api(
                    api_id="gap-1",
                    expected_file="admin/admin/v1/password/reset.rs",
                    resource="password",
                    name="reset",
                    api_name="重置密码",
                    url="POST:/open-apis/admin/v1/password/reset",
                ),
            ]
            validator.scan_implementations()
            validator.compare()
            summary = validator.calculate_summary()

            classification = summary["classification"]
            self.assertEqual(classification["strict_matched"], 0)
            self.assertEqual(classification["path_noise_matched"], 1)
            self.assertEqual(classification["true_missing"], 1)
            self.assertEqual(classification["extra_files"], 1)
            self.assertEqual(summary["implemented"], 1)
            self.assertEqual(summary["missing"], 1)

            noise = summary["path_noise_matches"]
            self.assertEqual(len(noise), 1)
            self.assertEqual(noise[0]["expected_file"], "admin/admin/v1/badge/list.rs")
            self.assertEqual(noise[0]["implementation_file"], "admin/v1/badge/list.rs")
            self.assertEqual(noise[0]["match_kind"], "flat_project")
            self.assertTrue(noise[0]["match_reason"])

            gaps = summary["true_missing_apis"]
            self.assertEqual(len(gaps), 1)
            self.assertEqual(gaps[0]["api_id"], "gap-1")
            self.assertEqual(gaps[0]["classification"], "true_gap")

            self.assertEqual(summary["extra_file_list"], ["admin/v1/users.rs"])

    def test_markdown_report_includes_classification_sections(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            flat = source / "admin" / "v1" / "badge" / "list.rs"
            flat.parent.mkdir(parents=True)
            flat.write_text("// flat", encoding="utf-8")
            extra = source / "admin" / "v1" / "users.rs"
            extra.write_text("// helper", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
            )
            validator.apis = [
                _api(expected_file="admin/admin/v1/badge/list.rs"),
                _api(
                    api_id="gap-1",
                    expected_file="admin/admin/v1/password/reset.rs",
                    resource="password",
                    name="reset",
                    api_name="重置密码",
                ),
            ]
            validator.scan_implementations()
            validator.compare()

            report_path = Path(temp_dir) / "report.md"
            validator.generate_report(str(report_path))
            text = report_path.read_text(encoding="utf-8")

            self.assertIn("分类统计", text)
            self.assertIn("路径噪音匹配", text)
            self.assertIn("真缺口", text)
            self.assertIn("额外实现文件", text)
            self.assertIn("admin/v1/badge/list.rs", text)
            self.assertIn("flat_project", text)


class PlatformCrateDenoiseIntegrationTests(unittest.TestCase):
    """Real-tree seam: openlark-platform before/after denoise breakdown."""

    @classmethod
    def setUpClass(cls):
        if not CSV_PATH.exists() or not MAPPING_PATH.exists():
            raise unittest.SkipTest("repo fixtures missing")
        cfg = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))["crates"]["openlark-platform"]
        cls.validator = validate_apis.APIValidator(
            csv_path=str(CSV_PATH),
            src_path=str(REPO_ROOT / cfg["src"]),
            filter_tags=cfg["biz_tags"],
            skip_old_versions=True,
            implementation_path_rewrites=cfg.get("implementation_path_rewrites"),
            implementation_path_aliases=cfg.get("implementation_path_aliases"),
        )
        cls.validator.parse_csv()
        cls.validator.scan_implementations()
        cls.validator.compare()
        cls.summary = cls.validator.calculate_summary()

    def test_platform_completion_is_no_longer_dominated_by_path_noise(self):
        # Historical naive rate was ~39.5% (47/119). After denoise, path noise
        # should be reclassified as matched, leaving only a handful of true gaps.
        self.assertGreaterEqual(self.summary["completion_rate"], 95.0)
        classification = self.summary["classification"]
        self.assertGreaterEqual(classification["path_noise_matched"], 60)
        self.assertLessEqual(classification["true_missing"], 5)
        self.assertEqual(
            classification["true_missing"],
            self.summary["missing"],
            "missing count must equal true_gap only (no silent drop without evidence)",
        )

    def test_every_path_noise_match_is_evidenced(self):
        for item in self.summary["path_noise_matches"]:
            self.assertTrue(item["expected_file"])
            self.assertTrue(item["implementation_file"])
            self.assertNotEqual(item["expected_file"], item["implementation_file"])
            self.assertTrue(item["match_kind"])
            self.assertTrue(item["match_reason"])
            # evidence: matched file must exist on disk
            full = REPO_ROOT / "crates/openlark-platform/src" / item["implementation_file"]
            self.assertTrue(full.is_file(), msg=item["implementation_file"])

    def test_true_gaps_are_not_path_noise_rows(self):
        noise_expected = {item["expected_file"] for item in self.summary["path_noise_matches"]}
        for item in self.summary["true_missing_apis"]:
            self.assertNotIn(item["expected_file"], noise_expected)
            self.assertEqual(item["classification"], "true_gap")


if __name__ == "__main__":
    unittest.main()
