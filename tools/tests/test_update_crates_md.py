import importlib.util
import sys
import unittest
from pathlib import Path

from tools.tests.catalog_row_count import (
    CURRENT_CATALOG_NON_OLD_ROW_COUNT,
    CURRENT_CATALOG_ROW_COUNT,
)

MODULE_PATH = Path(__file__).resolve().parents[1] / "update_crates_md.py"
SPEC = importlib.util.spec_from_file_location("update_crates_md", MODULE_PATH)
update_crates_md = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = update_crates_md
SPEC.loader.exec_module(update_crates_md)


class UpdateCratesMdTests(unittest.TestCase):
    def test_rendered_document_matches_repository_file(self):
        rendered = update_crates_md.render_document()
        actual = Path("crates.md").read_text(encoding="utf-8")
        self.assertEqual(rendered, actual)

    def test_stats_include_spark_and_expected_totals(self):
        stats = update_crates_md.compute_biz_tag_stats()

        # meta.Version=old 不计入 non-old；spark 妙搭域 29 条；pay 3 条均为 old。
        self.assertEqual(stats["spark"], (29, 29, 0))
        self.assertEqual(stats["vc"], (68, 68, 0))
        self.assertEqual(stats["pay"], (0, 3, 3))
        self.assertEqual(stats["approval"], (47, 70, 23))
        self.assertEqual(stats["minutes"], (10, 10, 0))
        self.assertEqual(
            sum(non_old for non_old, _, _ in stats.values()),
            CURRENT_CATALOG_NON_OLD_ROW_COUNT,
        )
        self.assertEqual(
            sum(total for _, total, _ in stats.values()),
            CURRENT_CATALOG_ROW_COUNT,
        )


if __name__ == "__main__":
    unittest.main()
