import importlib.util
import sys
import unittest
from pathlib import Path


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

        # 与 checked-in api_list_export.csv 对齐（#648 catalog sync 后：1743 总行，
        # 其中 meta.Version=old 不计入 non-old；spark 妙搭域当前为 24 条）。
        self.assertEqual(stats["spark"], (24, 24, 0))
        self.assertEqual(stats["vc"], (67, 67, 0))
        self.assertEqual(stats["approval"], (47, 70, 23))
        self.assertEqual(stats["minutes"], (10, 10, 0))
        self.assertEqual(
            sum(non_old for non_old, _, _ in stats.values()),
            1634,
        )
        self.assertEqual(
            sum(total for _, total, _ in stats.values()),
            1743,
        )


if __name__ == "__main__":
    unittest.main()
