import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).resolve().parents[1] / "compare_api_catalogs.py"
SPEC = importlib.util.spec_from_file_location("compare_api_catalogs", MODULE_PATH)
compare_api_catalogs = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = compare_api_catalogs
SPEC.loader.exec_module(compare_api_catalogs)


BASE_ROW = {
    "id": "1",
    "name": "获取用户",
    "bizTag": "contact",
    "meta.Project": "contact",
    "meta.Version": "v3",
    "meta.Resource": "user",
    "meta.Name": "get",
    "detail": "旧描述",
    "chargingMethod": "none",
    "fullDose": "true",
    "fullPath": "/document/server-docs/contact/user/get",
    "url": "GET:/open-apis/contact/v3/users/:user_id",
    "orderMark": "1",
    "supportAppTypes": '["custom"]',
    "tags": "[]",
    "updateTime": "1700000000",
    "isCharge": "false",
    "meta.Type": "1",
    "docPath": "https://open.feishu.cn/document/server-docs/contact/user/get",
}


# 与 api_list_export.csv 中「搜索 Wiki」行对齐（url 为 v2，目录 apiPath 为过时的 v1）。
WIKI_SEARCH_ROW = {
    "id": "6927492937392324635",
    "name": "搜索 Wiki",
    "bizTag": "ccm",
    "meta.Project": "wiki",
    "meta.Version": "v1",
    "meta.Resource": "node",
    "meta.Name": "search",
    "detail": "搜索 Wiki，用户通过关键词查询 Wiki，只能查找自己可见的 wiki",
    "chargingMethod": "none",
    "fullDose": "true",
    "fullPath": "/document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki",
    "url": "POST:/open-apis/wiki/v2/nodes/search",
    "orderMark": "1",
    "supportAppTypes": '["isv", "custom"]',
    "tags": "[]",
    "updateTime": "1766400136",
    "isCharge": "false",
    "meta.Type": "1",
    "docPath": "https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki",
}


def write_csv(path, header, rows):
    import csv

    with path.open("w", encoding="utf-8", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=header)
        writer.writeheader()
        writer.writerows(rows)


class SuppressedFieldChangesTests(unittest.TestCase):
    """搜索 Wiki 的已知 v1/v2 误报及其他变更的回归测试。"""

    def test_wiki_search_url_v1_v2_is_suppressed_in_both_directions(self):
        v1_url = "POST:/open-apis/wiki/v1/nodes/search"
        v2_url = WIKI_SEARCH_ROW["url"]
        for old_url, new_url in ((v2_url, v1_url), (v1_url, v2_url)):
            with self.subTest(old_url=old_url, new_url=new_url):
                baseline = dict(WIKI_SEARCH_ROW, url=old_url)
                current = dict(WIKI_SEARCH_ROW, url=new_url)
                diff = compare_api_catalogs.compare_catalogs([baseline], [current])

                self.assertEqual(diff.changed, ())
                self.assertFalse(diff.has_changes)
                self.assertEqual(len(diff.suppressed), 1)
                self.assertEqual(diff.suppressed[0].key, "id:6927492937392324635")
                self.assertEqual(
                    diff.suppressed[0].field_change,
                    compare_api_catalogs.FieldChange("url", old_url, new_url),
                )

    def test_other_wiki_search_url_changes_are_reported(self):
        for url in (
            "POST:/open-apis/wiki/v3/nodes/search",
            "POST:/open-apis/wiki/v2/nodes/list",
            "GET:/open-apis/wiki/v2/nodes/search",
            "",
        ):
            other = dict(WIKI_SEARCH_ROW, url=url)
            for baseline, current in ((WIKI_SEARCH_ROW, other), (other, WIKI_SEARCH_ROW)):
                with self.subTest(old_url=baseline["url"], new_url=current["url"]):
                    diff = compare_api_catalogs.compare_catalogs([baseline], [current])

                    self.assertTrue(diff.has_changes)
                    self.assertEqual(len(diff.changed), 1)
                    expected = compare_api_catalogs.FieldChange(
                        "url", baseline["url"], current["url"]
                    )
                    self.assertEqual(diff.changed[0].field_changes, (expected,))
                    self.assertEqual(diff.suppressed, ())

    def test_other_api_url_change_is_still_reported(self):
        other = dict(BASE_ROW, id="42", name="其他 API")
        current = dict(other, url="GET:/open-apis/contact/v3/users")

        diff = compare_api_catalogs.compare_catalogs([other], [current])

        self.assertTrue(diff.has_changes)
        self.assertEqual(len(diff.changed), 1)
        self.assertEqual(
            [change.field for change in diff.changed[0].field_changes], ["url"]
        )
        self.assertEqual(diff.suppressed, ())

    def test_wiki_search_non_url_field_change_is_still_reported(self):
        current = dict(WIKI_SEARCH_ROW, name="搜索 Wiki（新名称）")

        diff = compare_api_catalogs.compare_catalogs([WIKI_SEARCH_ROW], [current])

        self.assertTrue(diff.has_changes)
        self.assertEqual(len(diff.changed), 1)
        self.assertEqual(
            [change.field for change in diff.changed[0].field_changes], ["name"]
        )
        self.assertEqual(diff.suppressed, ())

    def test_wiki_search_url_and_name_change_reports_only_name(self):
        current = dict(
            WIKI_SEARCH_ROW,
            url="POST:/open-apis/wiki/v1/nodes/search",
            name="搜索 Wiki（新名称）",
        )

        diff = compare_api_catalogs.compare_catalogs([WIKI_SEARCH_ROW], [current])

        self.assertTrue(diff.has_changes)
        self.assertEqual(len(diff.changed), 1)
        self.assertEqual(
            [change.field for change in diff.changed[0].field_changes], ["name"]
        )
        self.assertEqual(len(diff.suppressed), 1)
        self.assertEqual(diff.suppressed[0].field_change.field, "url")

    def test_main_report_notes_suppression_and_only_flags_real_changes(self):
        header = list(WIKI_SEARCH_ROW.keys())
        other = dict(WIKI_SEARCH_ROW, id="42", name="其他 API")
        wiki_current = dict(WIKI_SEARCH_ROW, url="POST:/open-apis/wiki/v1/nodes/search")
        other_current = dict(other, supportAppTypes='["custom"]')

        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            baseline_path = temp_path / "baseline.csv"
            current_path = temp_path / "current.csv"
            report_path = temp_path / "report.md"
            output_path = temp_path / "github_output.txt"
            write_csv(baseline_path, header, [WIKI_SEARCH_ROW, other])
            write_csv(current_path, header, [wiki_current, other_current])

            original_argv = sys.argv
            sys.argv = [
                "compare_api_catalogs.py",
                "--baseline",
                str(baseline_path),
                "--current",
                str(current_path),
                "--report",
                str(report_path),
                "--github-output",
                str(output_path),
            ]
            try:
                exit_code = compare_api_catalogs.main()
            finally:
                sys.argv = original_argv

            report = report_path.read_text(encoding="utf-8")
            output = output_path.read_text(encoding="utf-8")
            self.assertEqual(exit_code, 0)
            # 只有其他 API 的真实变化进入字段变化表；wiki 的 url 变化被抑制，不出现。
            self.assertIn("字段变化: 1", report)
            self.assertIn("| 42 |", report)
            self.assertNotIn("wiki/v1/nodes/search", report)
            self.assertIn("已知误报抑制", report)
            self.assertIn("has_changes=true", output)
            self.assertIn("changed_count=1", output)


class CompareCatalogsTests(unittest.TestCase):
    def test_noise_fields_do_not_trigger_change(self):
        current = dict(BASE_ROW, detail="新描述", orderMark="2", updateTime="1800000000")

        diff = compare_api_catalogs.compare_catalogs([BASE_ROW], [current])

        self.assertFalse(diff.has_changes)
        self.assertEqual(diff.changed, ())

    def test_detects_added_removed_and_stable_field_changes(self):
        removed = dict(BASE_ROW, id="2", name="删除 API", url="DELETE:/open-apis/removed")
        changed = dict(BASE_ROW, supportAppTypes='["isv", "custom"]')
        added = dict(BASE_ROW, id="3", name="新增 API", url="POST:/open-apis/new")

        diff = compare_api_catalogs.compare_catalogs([BASE_ROW, removed], [changed, added])

        self.assertTrue(diff.has_changes)
        self.assertEqual([row["id"] for row in diff.added], ["3"])
        self.assertEqual([row["id"] for row in diff.removed], ["2"])
        self.assertEqual(len(diff.changed), 1)
        self.assertEqual(diff.changed[0].field_changes[0].field, "supportAppTypes")

    def test_main_writes_report_and_github_output(self):
        header = list(BASE_ROW.keys())
        current = dict(BASE_ROW, supportAppTypes='["isv", "custom"]')

        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)
            baseline_path = temp_path / "baseline.csv"
            current_path = temp_path / "current.csv"
            report_path = temp_path / "report.md"
            output_path = temp_path / "github_output.txt"
            self._write_csv(baseline_path, header, [BASE_ROW])
            self._write_csv(current_path, header, [current])

            original_argv = sys.argv
            sys.argv = [
                "compare_api_catalogs.py",
                "--baseline",
                str(baseline_path),
                "--current",
                str(current_path),
                "--report",
                str(report_path),
                "--github-output",
                str(output_path),
            ]
            try:
                exit_code = compare_api_catalogs.main()
            finally:
                sys.argv = original_argv

            self.assertEqual(exit_code, 0)
            self.assertIn("字段变化: 1", report_path.read_text(encoding="utf-8"))
            output = output_path.read_text(encoding="utf-8")
            self.assertIn("has_changes=true", output)
            self.assertIn("changed_count=1", output)

    def _write_csv(self, path, header, rows):
        write_csv(path, header, rows)


if __name__ == "__main__":
    unittest.main()
