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
        import csv

        with path.open("w", encoding="utf-8", newline="") as file:
            writer = csv.DictWriter(file, fieldnames=header)
            writer.writeheader()
            writer.writerows(rows)


if __name__ == "__main__":
    unittest.main()
