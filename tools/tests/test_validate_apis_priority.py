import csv
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

MAPPING_PATH = Path(__file__).resolve().parents[1] / "api_coverage.toml"
CSV_PATH = Path(__file__).resolve().parents[2] / "api_list_export.csv"


class PriorityModelTests(unittest.TestCase):
    def _build_model(self):
        return validate_apis.PriorityModel.from_data(
            {
                "defaults": {
                    "business_value": 2,
                    "usage_frequency": 2,
                    "implementation_effort": 4,
                },
                "weights": {
                    "business_value": 0.5,
                    "usage_frequency": 0.3,
                    "implementation_effort": 0.2,
                },
                "priority_tiers": [
                    {"name": "P0", "min_score": 4.4},
                    {"name": "P1", "min_score": 3.7},
                    {"name": "P2", "min_score": 3.0},
                    {"name": "P3", "min_score": 0.0},
                ],
                "rules": [
                    {
                        "name": "task-baseline",
                        "biz_tags": ["task"],
                        "business_value": 4,
                        "usage_frequency": 4,
                    },
                    {
                        "name": "read-is-cheap",
                        "methods": ["GET"],
                        "implementation_effort": 2,
                    },
                    {
                        "name": "task-v2-core",
                        "expected_file_prefixes": ["task/task/v2/task/"],
                        "business_value": 5,
                    },
                ],
            },
            "<inline>",
        )

    def _build_api(self, expected_file: str, url: str, name: str = "列取任务列表"):
        return validate_apis.APIInfo(
            api_id="1",
            name=name,
            biz_tag="task",
            meta_project="task",
            meta_version="v2",
            meta_resource="task",
            meta_name="list",
            url=url,
            doc_path="",
            expected_file=expected_file,
        )

    def test_later_rule_overrides_generic_dimension(self):
        model = self._build_model()
        api = self._build_api("task/task/v2/task/list.rs", "GET:/open-apis/task/v2/tasks")

        model.evaluate(api)

        self.assertEqual(api.business_value, 5)
        self.assertEqual(api.usage_frequency, 4)
        self.assertEqual(api.implementation_effort, 2)
        self.assertEqual(api.priority_level, "P0")
        self.assertEqual(api.priority_reasons, ["task-baseline", "read-is-cheap", "task-v2-core"])

    def test_sort_key_prefers_higher_priority_and_score(self):
        model = self._build_model()
        high = self._build_api("task/task/v2/task/list.rs", "GET:/open-apis/task/v2/tasks", "高优先")
        low = self._build_api("task/task/v1/task/create.rs", "POST:/open-apis/task/v1/tasks", "低优先")
        low.expected_file = "task/task/v1/task/create.rs"
        low.meta_version = "v1"

        model.evaluate(high)
        model.evaluate(low)

        ordered = sorted([low, high], key=model.sort_key)
        self.assertEqual([api.name for api in ordered], ["高优先", "低优先"])

    def test_collect_dashboard_groups_uses_mapping_metadata(self):
        groups = validate_apis.collect_dashboard_groups(
            {
                "openlark-workflow": {"dashboard_groups": ["core_business"]},
                "openlark-user": {"dashboard_groups": ["secondary"]},
                "openlark-docs": {"dashboard_groups": ["core_business"]},
            }
        )

        self.assertEqual(
            groups,
            {
                "core_business": ["openlark-docs", "openlark-workflow"],
                "secondary": ["openlark-user"],
            },
        )

    def test_build_dashboard_payload_filters_to_group_crates(self):
        model = self._build_model()
        high = self._build_api("task/task/v2/task/list.rs", "GET:/open-apis/task/v2/tasks", "高优先")
        low = self._build_api("task/task/v1/task/create.rs", "POST:/open-apis/task/v1/tasks", "低优先")
        low.expected_file = "task/task/v1/task/create.rs"
        low.meta_version = "v1"
        ignored = self._build_api("task/task/v1/task/get.rs", "GET:/open-apis/task/v1/tasks/:task_id", "组外缺口")
        ignored.expected_file = "task/task/v1/task/get.rs"
        ignored.meta_version = "v1"

        model.evaluate(high)
        model.evaluate(low)
        model.evaluate(ignored)

        payload = validate_apis.build_dashboard_payload(
            "core_business",
            ["openlark-workflow", "openlark-security"],
            {
                "openlark-workflow": {
                    "biz_tags": ["task", "approval", "board"],
                    "total_apis": 112,
                    "implemented": 39,
                    "missing": 73,
                    "completion_rate": 34.8,
                    "extra_files": 82,
                    "priority_counts": {"P0": 48, "P1": 25},
                    "report": "crates/openlark-workflow.md",
                },
                "openlark-security": {
                    "biz_tags": ["acs", "security_and_compliance"],
                    "total_apis": 27,
                    "implemented": 22,
                    "missing": 5,
                    "completion_rate": 81.5,
                    "extra_files": 40,
                    "priority_counts": {"P1": 5},
                    "report": "crates/openlark-security.md",
                },
                "openlark-user": {
                    "biz_tags": ["personal_settings"],
                    "total_apis": 6,
                    "implemented": 6,
                    "missing": 0,
                    "completion_rate": 100.0,
                    "extra_files": 6,
                    "priority_counts": {},
                    "report": "crates/openlark-user.md",
                },
            },
            [
                ("openlark-workflow", high),
                ("openlark-security", low),
                ("openlark-user", ignored),
            ],
            model.priority_formula(),
        )

        self.assertEqual(payload["crates_total"], 2)
        self.assertEqual(payload["total_apis"], 139)
        self.assertEqual(payload["missing"], 78)
        self.assertEqual([item["name"] for item in payload["top_missing_apis"]], ["高优先", "低优先"])
        self.assertEqual(payload["crates"][0]["crate"], "openlark-workflow")
        self.assertEqual(payload["crates"][0]["top_missing_api"], "高优先")

    def test_write_dashboard_markdown_uses_parent_relative_report_links(self):
        payload = {
            "dashboard": "core_business",
            "crates_total": 1,
            "total_apis": 112,
            "implemented": 39,
            "missing": 73,
            "completion_rate": 34.8,
            "extra_files": 82,
            "priority_counts": {"P0": 48, "P1": 25},
            "priority_formula": "业务价值×0.50 + 高频场景×0.30 + (6-实现复杂度)×0.20",
            "crates": [
                {
                    "crate": "openlark-workflow",
                    "biz_tags": ["task", "approval", "board"],
                    "total_apis": 112,
                    "implemented": 39,
                    "missing": 73,
                    "completion_rate": 34.8,
                    "report": "crates/openlark-workflow.md",
                    "top_missing_api": "获取评论详情",
                    "top_missing_priority": "P0",
                }
            ],
            "top_missing_apis": [],
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            output = Path(temp_dir) / "dashboards" / "core_business.md"
            validate_apis.write_dashboard_markdown(output, payload)
            content = output.read_text(encoding="utf-8")

        self.assertIn("[report](../crates/openlark-workflow.md)", content)


class MappingConfigTests(unittest.TestCase):
    def test_openlark_auth_tracks_verification_information_biz_tag(self):
        config = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))

        self.assertIn(
            "verification_information",
            config["crates"]["openlark-auth"]["biz_tags"],
        )
        self.assertNotIn(
            "verification",
            config["crates"]["openlark-auth"]["biz_tags"],
        )

    def test_mapping_covers_all_csv_biz_tags(self):
        config = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))
        mapped_tags = set()
        for crate_config in config["crates"].values():
            mapped_tags.update(crate_config["biz_tags"])

        with CSV_PATH.open("r", encoding="utf-8-sig", newline="") as handle:
            csv_tags = {row["bizTag"] for row in csv.DictReader(handle)}

        self.assertEqual(sorted(csv_tags - mapped_tags), [])
        self.assertEqual(sorted(mapped_tags - csv_tags), [])

    def test_legacy_layouts_have_explicit_implementation_path_mappings(self):
        config = tomllib.loads(MAPPING_PATH.read_text(encoding="utf-8"))

        workflow = config["crates"]["openlark-workflow"]
        self.assertIn(
            {"from": "task/task/", "to": ""},
            workflow["implementation_path_rewrites"],
        )
        self.assertEqual(
            workflow["implementation_path_aliases"][
                "task/task/v2/comment/patch.rs"
            ],
            "v2/comment/update.rs",
        )

        security = config["crates"]["openlark-security"]
        self.assertIn(
            {
                "from": "security_and_compliance/security_and_compliance/v2/device_record/",
                "to": "security/security_and_compliance/v2/device_records/",
            },
            security["implementation_path_rewrites"],
        )


class ImplementationPathMappingTests(unittest.TestCase):
    def _api(self, expected_file):
        return validate_apis.APIInfo(
            api_id="1",
            name="测试接口",
            biz_tag="task",
            meta_project="task",
            meta_version="v2",
            meta_resource="task",
            meta_name="get",
            url="GET:/open-apis/task/v2/tasks/:task_id",
            doc_path="",
            expected_file=expected_file,
        )

    def test_prefix_rewrite_marks_legacy_file_as_implemented(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            actual = source / "v2" / "task" / "get.rs"
            actual.parent.mkdir(parents=True)
            actual.write_text("// implemented", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
                implementation_path_rewrites=[{"from": "task/task/", "to": ""}],
            )
            validator.apis = [self._api("task/task/v2/task/get.rs")]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(validator.extra_files, set())

    def test_exact_alias_marks_renamed_legacy_file_as_implemented(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            source = Path(temp_dir)
            actual = source / "v2" / "comment" / "update.rs"
            actual.parent.mkdir(parents=True)
            actual.write_text("// implemented", encoding="utf-8")

            validator = validate_apis.APIValidator(
                csv_path="unused.csv",
                src_path=str(source),
                implementation_path_aliases={
                    "task/task/v2/comment/patch.rs": "v2/comment/update.rs"
                },
            )
            validator.apis = [self._api("task/task/v2/comment/patch.rs")]
            validator.scan_implementations()
            validator.compare()

            self.assertTrue(validator.apis[0].is_implemented)
            self.assertEqual(validator.extra_files, set())


class CSVIntegrityValidationTests(unittest.TestCase):
    CSV_HEADER = [
        "id",
        "name",
        "bizTag",
        "meta.Project",
        "meta.Version",
        "meta.Resource",
        "meta.Name",
        "detail",
        "chargingMethod",
        "fullDose",
        "fullPath",
        "url",
        "orderMark",
        "supportAppTypes",
        "tags",
        "updateTime",
        "isCharge",
        "meta.Type",
        "docPath",
    ]

    def _write_csv(self, rows):
        temp_dir = tempfile.TemporaryDirectory()
        csv_path = Path(temp_dir.name) / "api_list_export.csv"
        with csv_path.open("w", encoding="utf-8", newline="") as handle:
            writer = csv.DictWriter(handle, fieldnames=self.CSV_HEADER)
            writer.writeheader()
            for row in rows:
                writer.writerow(row)
        return temp_dir, csv_path

    def _base_row(self, **overrides):
        row = {
            "id": "1",
            "name": "获取任务",
            "bizTag": "task",
            "meta.Project": "task",
            "meta.Version": "v2",
            "meta.Resource": "task",
            "meta.Name": "get",
            "detail": "",
            "chargingMethod": "none",
            "fullDose": "true",
            "fullPath": "/document/mock",
            "url": "GET:/open-apis/task/v2/tasks/:task_id",
            "orderMark": "1",
            "supportAppTypes": "[]",
            "tags": "[]",
            "updateTime": "0",
            "isCharge": "false",
            "meta.Type": "1",
            "docPath": "https://open.feishu.cn/document/mock",
        }
        row.update(overrides)
        return row

    def test_parse_csv_rejects_unknown_rows(self):
        temp_dir, csv_path = self._write_csv(
            [
                self._base_row(
                    id="unknown-1",
                    bizTag="unknown",
                    **{"meta.Project": "unknown"},
                )
            ]
        )
        with temp_dir:
            validator = validate_apis.APIValidator(csv_path=str(csv_path), src_path=temp_dir.name)

            with self.assertRaisesRegex(ValueError, "unknown"):
                validator.parse_csv()

    def test_parse_csv_rejects_duplicate_expected_files(self):
        temp_dir, csv_path = self._write_csv(
            [
                self._base_row(id="dup-1", name="获取任务"),
                self._base_row(id="dup-2", name="再次获取任务"),
            ]
        )
        with temp_dir:
            validator = validate_apis.APIValidator(csv_path=str(csv_path), src_path=temp_dir.name)

            with self.assertRaisesRegex(ValueError, "duplicate expected_file"):
                validator.parse_csv()


if __name__ == "__main__":
    unittest.main()
