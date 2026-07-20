#!/usr/bin/env python3
"""
API 验证脚本

对比 CSV 文件中的 API 列表与实际代码实现，生成完成度报告，
并基于业务价值模型输出可排序的缺失 API 优先级清单。

命名规范：src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs
- meta.resource 中的 '.' 转换为 '/' 作为子目录
- meta.name 中的 '/' 转换为 '/' 作为子目录
- meta.name 中的 ':' 替换为 '_'
"""

import csv
import json
import os
import re
from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple

try:
    import tomllib  # py>=3.11
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None


PRIORITY_DIMENSIONS = (
    "business_value",
    "usage_frequency",
    "implementation_effort",
)


@dataclass
class APIInfo:
    """API 信息"""

    api_id: str
    name: str
    biz_tag: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name: str
    url: str
    doc_path: str
    expected_file: str = ""
    is_implemented: bool = False
    priority_level: str = ""
    priority_score: float = 0.0
    business_value: int = 0
    usage_frequency: int = 0
    implementation_effort: int = 0
    priority_reasons: List[str] = field(default_factory=list)
    priority_notes: List[str] = field(default_factory=list)

    @property
    def http_method(self) -> str:
        """从 URL 字段中提取 HTTP Method。"""
        method, _, _ = self.url.partition(":")
        return method.upper()

    @property
    def endpoint_path(self) -> str:
        """从 URL 字段中提取接口路径。"""
        _, _, path = self.url.partition(":")
        return path


@dataclass
class PriorityTier:
    """优先级档位定义。"""

    name: str
    min_score: float
    description: str = ""


@dataclass
class PriorityRule:
    """优先级规则。后面的规则可以覆盖前面的通用规则。"""

    name: str
    note: str = ""
    biz_tags: List[str] = field(default_factory=list)
    projects: List[str] = field(default_factory=list)
    versions: List[str] = field(default_factory=list)
    methods: List[str] = field(default_factory=list)
    resource_prefixes: List[str] = field(default_factory=list)
    name_prefixes: List[str] = field(default_factory=list)
    expected_file_prefixes: List[str] = field(default_factory=list)
    url_prefixes: List[str] = field(default_factory=list)
    business_value: Optional[int] = None
    usage_frequency: Optional[int] = None
    implementation_effort: Optional[int] = None

    @classmethod
    def from_dict(cls, payload: Dict[str, Any]) -> "PriorityRule":
        return cls(
            name=str(payload.get("name", "")).strip(),
            note=str(payload.get("note", "")).strip(),
            biz_tags=_as_string_list(payload.get("biz_tags")),
            projects=_as_string_list(payload.get("projects")),
            versions=_as_string_list(payload.get("versions")),
            methods=[item.upper() for item in _as_string_list(payload.get("methods"))],
            resource_prefixes=_as_string_list(payload.get("resource_prefixes")),
            name_prefixes=_as_string_list(payload.get("name_prefixes")),
            expected_file_prefixes=_as_string_list(payload.get("expected_file_prefixes")),
            url_prefixes=_as_string_list(payload.get("url_prefixes")),
            business_value=_optional_score(payload.get("business_value")),
            usage_frequency=_optional_score(payload.get("usage_frequency")),
            implementation_effort=_optional_score(payload.get("implementation_effort")),
        )

    def matches(self, api: APIInfo) -> bool:
        """判断规则是否命中当前 API。"""
        if self.biz_tags and api.biz_tag not in self.biz_tags:
            return False
        if self.projects and api.meta_project not in self.projects:
            return False
        if self.versions and api.meta_version not in self.versions:
            return False
        if self.methods and api.http_method not in self.methods:
            return False
        if self.resource_prefixes and not _matches_prefix(api.meta_resource, self.resource_prefixes):
            return False
        if self.name_prefixes and not _matches_prefix(api.meta_name, self.name_prefixes):
            return False
        if self.expected_file_prefixes and not _matches_prefix(api.expected_file, self.expected_file_prefixes):
            return False
        if self.url_prefixes and not _matches_prefix(api.endpoint_path, self.url_prefixes):
            return False
        return True


class PriorityModel:
    """缺失 API 的业务优先级模型。"""

    def __init__(
        self,
        source_path: str,
        defaults: Dict[str, int],
        weights: Dict[str, float],
        tiers: List[PriorityTier],
        rules: List[PriorityRule],
    ):
        self.source_path = source_path
        self.defaults = defaults
        self.weights = weights
        self.tiers = sorted(tiers, key=lambda item: item.min_score, reverse=True)
        self.rules = rules
        self.tier_rank = {tier.name: index for index, tier in enumerate(self.tiers)}

    @classmethod
    def from_path(cls, path: str) -> "PriorityModel":
        if tomllib is None:
            print("❌ 错误: 当前 Python 不支持 tomllib，请使用 Python 3.11+")
            raise SystemExit(1)
        config_path = Path(path)
        if not config_path.exists():
            print(f"❌ 错误: 优先级配置不存在: {config_path}")
            raise SystemExit(1)
        data = tomllib.loads(config_path.read_text(encoding="utf-8"))
        return cls.from_data(data, str(config_path))

    @classmethod
    def from_data(cls, data: Dict[str, Any], source_path: str) -> "PriorityModel":
        defaults_payload = data.get("defaults", {})
        defaults = {
            "business_value": _coerce_score(defaults_payload.get("business_value", 3)),
            "usage_frequency": _coerce_score(defaults_payload.get("usage_frequency", 3)),
            "implementation_effort": _coerce_score(defaults_payload.get("implementation_effort", 3)),
        }

        weights_payload = data.get("weights", {})
        raw_weights = {
            "business_value": _coerce_float(weights_payload.get("business_value", 0.5)),
            "usage_frequency": _coerce_float(weights_payload.get("usage_frequency", 0.3)),
            "implementation_effort": _coerce_float(weights_payload.get("implementation_effort", 0.2)),
        }
        total_weight = sum(raw_weights.values())
        if total_weight <= 0:
            raise ValueError("优先级权重之和必须大于 0")
        weights = {key: value / total_weight for key, value in raw_weights.items()}

        tiers_payload = data.get("priority_tiers") or data.get("tiers") or [
            {"name": "P0", "min_score": 4.4, "description": "核心业务闭环，优先补齐"},
            {"name": "P1", "min_score": 3.7, "description": "高价值能力，纳入近期计划"},
            {"name": "P2", "min_score": 3.0, "description": "中价值缺口，按容量推进"},
            {"name": "P3", "min_score": 0.0, "description": "低频或高成本项，排入尾部"},
        ]
        tiers = [
            PriorityTier(
                name=str(item["name"]).strip(),
                min_score=float(item["min_score"]),
                description=str(item.get("description", "")).strip(),
            )
            for item in tiers_payload
        ]
        if not tiers:
            raise ValueError("优先级档位不能为空")

        rules = [PriorityRule.from_dict(item) for item in data.get("rules", [])]
        invalid_rule_names = [rule.name for rule in rules if not rule.name]
        if invalid_rule_names:
            raise ValueError("存在缺少 name 的优先级规则")

        return cls(source_path=source_path, defaults=defaults, weights=weights, tiers=tiers, rules=rules)

    def evaluate(self, api: APIInfo) -> None:
        """为缺失 API 计算优先级。"""
        scores = dict(self.defaults)
        matched_rules: List[str] = []
        matched_notes: List[str] = []

        for rule in self.rules:
            if not rule.matches(api):
                continue
            matched_rules.append(rule.name)
            if rule.note:
                matched_notes.append(rule.note)
            for dimension in PRIORITY_DIMENSIONS:
                value = getattr(rule, dimension)
                if value is not None:
                    scores[dimension] = value

        api.business_value = scores["business_value"]
        api.usage_frequency = scores["usage_frequency"]
        api.implementation_effort = scores["implementation_effort"]
        api.priority_score = round(
            scores["business_value"] * self.weights["business_value"]
            + scores["usage_frequency"] * self.weights["usage_frequency"]
            + (6 - scores["implementation_effort"]) * self.weights["implementation_effort"],
            2,
        )
        api.priority_level = self._resolve_tier(api.priority_score).name
        api.priority_reasons = matched_rules or ["默认基线"]
        api.priority_notes = _dedupe_preserve_order(matched_notes)

    def sort_key(self, api: APIInfo) -> Tuple[int, float, str, str]:
        """生成稳定排序键。"""
        tier_rank = self.tier_rank.get(api.priority_level, len(self.tiers))
        return (tier_rank, -api.priority_score, api.biz_tag, api.expected_file)

    def priority_formula(self) -> str:
        """返回综合分公式，用于报告展示。"""
        return (
            f"业务价值×{self.weights['business_value']:.2f}"
            f" + 高频场景×{self.weights['usage_frequency']:.2f}"
            f" + (6-实现复杂度)×{self.weights['implementation_effort']:.2f}"
        )

    def _resolve_tier(self, score: float) -> PriorityTier:
        for tier in self.tiers:
            if score >= tier.min_score:
                return tier
        return self.tiers[-1]


class APIValidator:
    """API 验证器"""

    def __init__(
        self,
        csv_path: str,
        src_path: str,
        filter_tags: Optional[List[str]] = None,
        skip_old_versions: bool = True,
        with_timestamp: bool = False,
        priority_model: Optional[PriorityModel] = None,
        implementation_path_rewrites: Optional[List[Dict[str, str]]] = None,
        implementation_path_aliases: Optional[Dict[str, str]] = None,
    ):
        self.csv_path = csv_path
        self.src_path = Path(src_path)
        self.filter_tags = filter_tags
        self.skip_old_versions = skip_old_versions
        self.with_timestamp = with_timestamp
        self.priority_model = priority_model
        self.implementation_path_rewrites = implementation_path_rewrites or []
        self.implementation_path_aliases = implementation_path_aliases or {}
        self.apis: List[APIInfo] = []
        self.implemented_files: Set[str] = set()
        self.missing_apis: List[APIInfo] = []
        self.extra_files: Set[str] = set()
        self.skipped_old_count: int = 0

    @staticmethod
    def _camel_to_snake(name: str) -> str:
        """将 camelCase/PascalCase 转为 snake_case（尽量兼容缩写）。"""
        if not name:
            return name
        s1 = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
        s2 = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s1)
        return s2.lower()

    def _normalize_name_path(self, name_path: str) -> str:
        """
        将 meta.name 转为仓库当前实际使用的文件/目录命名风格：
        - 将 '#': `post#spreadsheets` -> `post_spreadsheets`
        - 将每个 path segment 做 snake_case：`dataValidation` -> `data_validation`
        - 对路径参数（已被 ':' 替换为 '_'）也做 snake_case：`_docToken` -> `_doc_token`
        """
        name_path = name_path.replace("#", "_")
        segments = [segment for segment in name_path.split("/") if segment]
        normalized: List[str] = []
        for segment in segments:
            if segment.startswith("_") and len(segment) > 1:
                normalized.append("_" + self._camel_to_snake(segment[1:]))
            else:
                normalized.append(self._camel_to_snake(segment))
        return "/".join(normalized)

    def parse_csv(self) -> None:
        """解析 CSV 文件"""
        print(f"📄 读取 CSV 文件: {self.csv_path}")

        if self.filter_tags:
            print(f"🏷️  过滤业务标签: {', '.join(self.filter_tags)}")

        if self.skip_old_versions:
            print("🔧 跳过旧版本 API: version='old'")

        with open(self.csv_path, "r", encoding="utf-8") as file:
            reader = csv.DictReader(file)
            for row in reader:
                if self.filter_tags and row["bizTag"] not in self.filter_tags:
                    continue

                if self.skip_old_versions and row["meta.Version"] == "old":
                    self.skipped_old_count += 1
                    continue

                api = APIInfo(
                    api_id=row["id"],
                    name=row["name"],
                    biz_tag=row["bizTag"],
                    meta_project=row["meta.Project"],
                    meta_version=row["meta.Version"],
                    meta_resource=row["meta.Resource"],
                    meta_name=row["meta.Name"],
                    url=row["url"],
                    doc_path=row["docPath"],
                )
                api.expected_file = self._generate_expected_file_path(api)
                self.apis.append(api)

        self._validate_csv_integrity()

        print(f"✅ 解析完成，共 {len(self.apis)} 个 API")
        if self.skip_old_versions and self.skipped_old_count > 0:
            print(f"   📋 已跳过 {self.skipped_old_count} 个旧版本 API")

    def _generate_expected_file_path(self, api: APIInfo) -> str:
        """根据 API 信息生成预期的文件路径。"""
        if api.biz_tag == "meeting_room" and api.meta_version == "old" and api.meta_resource == "default":
            name_path = api.meta_name.replace(":", "_")
            name_path = self._normalize_name_path(name_path)
            return f"meeting_room/{name_path}.rs"

        base = f"{api.biz_tag}/{api.meta_project}"
        version = api.meta_version
        resource_path = api.meta_resource.replace(".", "/")
        name_path = api.meta_name.replace(":", "_").rstrip("/")
        name_path = self._normalize_name_path(name_path)
        return f"{base}/{version}/{resource_path}/{name_path}.rs"

    def _validate_csv_integrity(self) -> None:
        """校验当前参与统计的 CSV 数据完整性。"""
        issues: List[str] = []

        unknown_apis = [
            api
            for api in self.apis
            if api.biz_tag.strip().lower() == "unknown" or api.meta_project.strip().lower() == "unknown"
        ]
        if unknown_apis:
            issues.append("unknown API rows detected:")
            for api in unknown_apis[:10]:
                issues.append(
                    f"  - id={api.api_id} bizTag={api.biz_tag} meta.Project={api.meta_project} "
                    f"expected_file={api.expected_file or '<empty>'}"
                )

        duplicates: Dict[str, List[APIInfo]] = defaultdict(list)
        for api in self.apis:
            if api.expected_file:
                duplicates[api.expected_file].append(api)

        duplicate_groups = [
            (expected_file, apis) for expected_file, apis in duplicates.items() if len(apis) > 1
        ]
        if duplicate_groups:
            issues.append("duplicate expected_file entries detected:")
            for expected_file, apis in duplicate_groups[:10]:
                issue_items = ", ".join(f"{api.api_id}:{api.name}" for api in apis)
                issues.append(f"  - {expected_file} <- {issue_items}")

        if issues:
            raise ValueError("CSV integrity validation failed:\n" + "\n".join(issues))

    def scan_implementations(self) -> None:
        """扫描实际实现的文件"""
        print(f"🔍 扫描代码实现目录: {self.src_path}")

        exclude_files = {"mod.rs", "models.rs", "macros.rs", "service.rs", "responses.rs"}
        for root, dirs, files in os.walk(self.src_path):
            dirs[:] = [directory for directory in dirs if not directory.startswith(".") and directory != "__pycache__"]
            for file_name in files:
                if not file_name.endswith(".rs") or file_name in exclude_files:
                    continue
                full_path = os.path.join(root, file_name)
                rel_path = os.path.relpath(full_path, self.src_path).replace("\\", "/")
                if not rel_path.startswith("lib.rs") and not rel_path.startswith("common/"):
                    self.implemented_files.add(rel_path)

        print(f"✅ 扫描完成，找到 {len(self.implemented_files)} 个实现文件")

    def compare(self) -> None:
        """对比 CSV 和实际实现"""
        print("🔬 开始对比分析...")

        matched_implementation_files: Set[str] = set()
        for api in self.apis:
            implementation_file = next(
                (
                    candidate
                    for candidate in self._implementation_path_candidates(api.expected_file)
                    if candidate in self.implemented_files
                ),
                None,
            )
            if implementation_file is not None:
                api.is_implemented = True
                matched_implementation_files.add(implementation_file)
                continue

            api.is_implemented = False
            if self.priority_model is not None:
                self.priority_model.evaluate(api)
            self.missing_apis.append(api)

        self.extra_files = self.implemented_files - matched_implementation_files
        if self.priority_model is not None:
            self.missing_apis = sorted(self.missing_apis, key=self.priority_model.sort_key)

        print("✅ 对比完成")
        print(f"   - 已实现: {len([api for api in self.apis if api.is_implemented])}")
        print(f"   - 未实现: {len(self.missing_apis)}")
        print(f"   - 额外文件: {len(self.extra_files)}")

    def _implementation_path_candidates(self, expected_file: str) -> List[str]:
        """返回 strict 路径及仓库已登记的 legacy 实现路径。"""
        if not expected_file:
            return []

        candidates = [expected_file]
        alias = self.implementation_path_aliases.get(expected_file)
        if alias:
            candidates.append(alias)

        for rewrite in self.implementation_path_rewrites:
            source_prefix = str(rewrite.get("from", ""))
            target_prefix = str(rewrite.get("to", ""))
            if source_prefix and expected_file.startswith(source_prefix):
                candidates.append(target_prefix + expected_file[len(source_prefix) :])

        return _dedupe_preserve_order(candidates)

    def generate_report(self, output_path: str) -> None:
        """生成报告"""
        print(f"📝 生成报告: {output_path}")

        with open(output_path, "w", encoding="utf-8") as file:
            file.write("# API 验证报告\n\n")
            if self.with_timestamp:
                file.write(f"**生成时间**: {self._get_timestamp()}\n")
            file.write(f"**CSV 文件**: {self.csv_path}\n")
            file.write(f"**源码目录**: {self.src_path}\n")
            file.write("**命名规范**: `src/bizTag/meta.project/meta.version/meta.resource/meta.name.rs`\n")
            if self.priority_model is not None:
                file.write(f"**优先级配置**: `{self.priority_model.source_path}`\n")
            file.write("\n")

            section_index = 1
            self._write_overall_section(file, section_index)
            section_index += 1
            self._write_module_section(file, section_index)
            section_index += 1

            if self.missing_apis:
                self._write_priority_section(file, section_index)
                section_index += 1
                self._write_missing_detail_section(file, section_index)
                section_index += 1

            if self.extra_files:
                self._write_extra_files_section(file, section_index)
                section_index += 1

            self._write_implemented_section(file, section_index)

        print("✅ 报告生成完成")

    def calculate_summary(self) -> Dict[str, Any]:
        """生成可序列化的统计摘要。"""
        total_apis = len(self.apis)
        implemented = len([api for api in self.apis if api.is_implemented])
        missing = len(self.missing_apis)
        completion_rate = (implemented / total_apis * 100) if total_apis > 0 else 0.0

        return {
            "total_apis": total_apis,
            "implemented": implemented,
            "missing": missing,
            "completion_rate": round(completion_rate, 1),
            "extra_files": len(self.extra_files),
            "skipped_old_versions": self.skipped_old_count if self.skip_old_versions else 0,
            "module_stats": self._calculate_module_stats(),
            "priority_counts": self._calculate_priority_counts(),
            "prioritized_missing_apis": [self._serialize_missing_api(api) for api in self.missing_apis],
        }

    def _write_overall_section(self, file: Any, section_index: int) -> None:
        total_apis = len(self.apis)
        implemented = len([api for api in self.apis if api.is_implemented])
        missing = len(self.missing_apis)
        completion_rate = (implemented / total_apis * 100) if total_apis > 0 else 0

        file.write(f"## {section_index}、总体统计\n\n")
        file.write("| 指标 | 数量 |\n")
        file.write("|------|------|\n")
        file.write(f"| **API 总数** | {total_apis} |\n")
        file.write(f"| **已实现** | {implemented} |\n")
        file.write(f"| **未实现** | {missing} |\n")
        file.write(f"| **完成率** | {completion_rate:.1f}% |\n")
        file.write(f"| **额外文件** | {len(self.extra_files)} |\n\n")

    def _write_module_section(self, file: Any, section_index: int) -> None:
        file.write(f"## {section_index}、模块统计\n\n")
        module_stats = self._calculate_module_stats()
        file.write("| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |\n")
        file.write("|------|---------|--------|--------|--------|\n")
        for module_name, stats in sorted(module_stats.items()):
            file.write(
                f"| {module_name} | {stats['total']} | {stats['implemented']} | "
                f"{stats['missing']} | {stats['rate']:.1f}% |\n"
            )
        file.write("\n")

    def _write_priority_section(self, file: Any, section_index: int) -> None:
        file.write(f"## {section_index}、缺失 API 优先级清单\n\n")
        if self.priority_model is not None:
            file.write("- 维度：业务价值 / 高频场景 / 实现复杂度，均为 `1-5` 分。\n")
            file.write("- 排序原则：综合分越高越靠前；实现复杂度越低，综合分越高。\n")
            file.write(f"- 综合分公式：`{self.priority_model.priority_formula()}`\n")
            file.write("- 规则应用方式：按配置文件顺序匹配，后面的更具体规则可覆盖前面的通用规则。\n\n")

        priority_counts = self._calculate_priority_counts()
        if priority_counts:
            file.write("| 优先级 | 数量 |\n")
            file.write("|--------|------|\n")
            for priority, count in sorted(priority_counts.items()):
                file.write(f"| {priority} | {count} |\n")
            file.write("\n")

        file.write("| 优先级 | 综合分 | 业务价值 | 高频场景 | 实现复杂度 | API | 预期文件 | 判定规则 |\n")
        file.write("|--------|--------|----------|----------|------------|-----|----------|----------|\n")
        for api in self.missing_apis:
            file.write(
                f"| {api.priority_level or '-'} | {api.priority_score:.2f} | "
                f"{api.business_value or '-'} | {api.usage_frequency or '-'} | "
                f"{api.implementation_effort or '-'} | {api.name} | "
                f"`{api.expected_file}` | {', '.join(api.priority_reasons)} |\n"
            )
        file.write("\n")

    def _write_missing_detail_section(self, file: Any, section_index: int) -> None:
        file.write(f"## {section_index}、未实现的 API（按模块）\n\n")
        missing_by_module: Dict[str, List[APIInfo]] = defaultdict(list)
        for api in self.missing_apis:
            missing_by_module[api.biz_tag.upper()].append(api)

        for module_name in sorted(missing_by_module.keys()):
            file.write(f"### {module_name} ({len(missing_by_module[module_name])} 个)\n\n")
            for api in missing_by_module[module_name]:
                file.write(f"#### {api.name}\n\n")
                file.write(f"- **API ID**: {api.api_id}\n")
                if api.priority_level:
                    file.write(f"- **优先级**: {api.priority_level} ({api.priority_score:.2f})\n")
                    file.write(
                        f"- **维度**: 业务价值={api.business_value}，"
                        f"高频场景={api.usage_frequency}，实现复杂度={api.implementation_effort}\n"
                    )
                    file.write(f"- **判定规则**: {', '.join(api.priority_reasons)}\n")
                    if api.priority_notes:
                        file.write(f"- **说明**: {'；'.join(api.priority_notes)}\n")
                file.write(f"- **预期文件**: `{api.expected_file}`\n")
                file.write(f"- **URL**: {api.url}\n")
                file.write(f"- **文档**: {api.doc_path}\n\n")

    def _write_extra_files_section(self, file: Any, section_index: int) -> None:
        file.write(f"## {section_index}、额外的实现文件\n\n")
        file.write("这些文件存在于代码中，但不在 CSV API 列表中：\n\n")
        for extra_file in sorted(self.extra_files):
            file.write(f"- `{extra_file}`\n")
        file.write("\n")

    def _write_implemented_section(self, file: Any, section_index: int) -> None:
        file.write(f"## {section_index}、已实现的 API\n\n")
        implemented_by_module: Dict[str, List[APIInfo]] = defaultdict(list)
        for api in self.apis:
            if api.is_implemented:
                implemented_by_module[api.biz_tag.upper()].append(api)

        for module_name in sorted(implemented_by_module.keys()):
            file.write(f"### {module_name} ({len(implemented_by_module[module_name])} 个)\n\n")
            for api in sorted(implemented_by_module[module_name], key=lambda item: item.name):
                file.write(f"- ✅ {api.name} (`{api.expected_file}`)\n")
            file.write("\n")

    def _calculate_module_stats(self) -> Dict[str, Dict[str, Any]]:
        """计算各模块的统计数据"""
        module_stats: Dict[str, Dict[str, Any]] = defaultdict(
            lambda: {"total": 0, "implemented": 0, "missing": 0, "rate": 0.0}
        )
        for api in self.apis:
            module_name = api.biz_tag.upper()
            module_stats[module_name]["total"] += 1
            if api.is_implemented:
                module_stats[module_name]["implemented"] += 1
            else:
                module_stats[module_name]["missing"] += 1

        for stats in module_stats.values():
            if stats["total"] > 0:
                stats["rate"] = (stats["implemented"] / stats["total"]) * 100

        return dict(module_stats)

    def _calculate_priority_counts(self) -> Dict[str, int]:
        counts: Dict[str, int] = defaultdict(int)
        for api in self.missing_apis:
            if api.priority_level:
                counts[api.priority_level] += 1
        return dict(sorted(counts.items()))

    @staticmethod
    def _serialize_missing_api(api: APIInfo) -> Dict[str, Any]:
        return {
            "api_id": api.api_id,
            "name": api.name,
            "biz_tag": api.biz_tag,
            "project": api.meta_project,
            "version": api.meta_version,
            "resource": api.meta_resource,
            "meta_name": api.meta_name,
            "url": api.url,
            "doc_path": api.doc_path,
            "expected_file": api.expected_file,
            "priority_level": api.priority_level,
            "priority_score": api.priority_score,
            "business_value": api.business_value,
            "usage_frequency": api.usage_frequency,
            "implementation_effort": api.implementation_effort,
            "priority_reasons": api.priority_reasons,
            "priority_notes": api.priority_notes,
        }

    @staticmethod
    def _get_timestamp() -> str:
        """获取当前时间戳"""
        from datetime import datetime

        return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def _as_string_list(value: Any) -> List[str]:
    if value is None:
        return []
    if isinstance(value, list):
        return [str(item).strip() for item in value if str(item).strip()]
    text = str(value).strip()
    return [text] if text else []


def _matches_prefix(value: str, prefixes: List[str]) -> bool:
    return any(value.startswith(prefix) for prefix in prefixes)


def _coerce_score(value: Any) -> int:
    score = int(value)
    if not 1 <= score <= 5:
        raise ValueError(f"分值必须在 1-5 之间，实际为: {score}")
    return score


def _optional_score(value: Any) -> Optional[int]:
    if value is None:
        return None
    return _coerce_score(value)


def _coerce_float(value: Any) -> float:
    return float(value)


def _dedupe_preserve_order(values: List[str]) -> List[str]:
    seen: Set[str] = set()
    result: List[str] = []
    for value in values:
        if value in seen:
            continue
        seen.add(value)
        result.append(value)
    return result


def dashboard_slug(name: str) -> str:
    """将 dashboard 分组名转换为稳定的文件名。"""
    return re.sub(r"[^a-z0-9_]+", "_", name.strip().lower()).strip("_") or "dashboard"


def collect_dashboard_groups(crates_config: Dict[str, Dict[str, Any]]) -> Dict[str, List[str]]:
    """从 crate 映射配置中提取 dashboard 分组。"""
    groups: Dict[str, List[str]] = defaultdict(list)
    for crate_name in sorted(crates_config.keys()):
        config = crates_config[crate_name]
        for group_name in _dedupe_preserve_order(_as_string_list(config.get("dashboard_groups"))):
            groups[group_name].append(crate_name)
    return dict(sorted(groups.items()))


def build_dashboard_payload(
    dashboard_name: str,
    crate_names: List[str],
    crate_summaries: Dict[str, Any],
    missing_apis: List[Tuple[str, APIInfo]],
    priority_formula: str,
) -> Dict[str, Any]:
    """为指定 crate 分组构建专题 dashboard 载荷。"""
    scoped_crates = [crate_name for crate_name in crate_names if crate_name in crate_summaries]
    scoped_crate_set = set(scoped_crates)
    scoped_missing = [(crate_name, api) for crate_name, api in missing_apis if crate_name in scoped_crate_set]

    total_apis = sum(crate_summaries[crate_name]["total_apis"] for crate_name in scoped_crates)
    implemented = sum(crate_summaries[crate_name]["implemented"] for crate_name in scoped_crates)
    missing = sum(crate_summaries[crate_name]["missing"] for crate_name in scoped_crates)
    extra_files = sum(crate_summaries[crate_name]["extra_files"] for crate_name in scoped_crates)
    completion_rate = (implemented / total_apis * 100) if total_apis > 0 else 0.0

    priority_counts: Dict[str, int] = defaultdict(int)
    for _, api in scoped_missing:
        priority_counts[api.priority_level] += 1
    top_gap_by_crate: Dict[str, APIInfo] = {}
    for crate_name, api in scoped_missing:
        top_gap_by_crate.setdefault(crate_name, api)

    crate_rows: List[Dict[str, Any]] = []
    for crate_name in scoped_crates:
        stats = crate_summaries[crate_name]
        top_gap = top_gap_by_crate.get(crate_name)
        crate_rows.append(
            {
                "crate": crate_name,
                "biz_tags": stats["biz_tags"],
                "total_apis": stats["total_apis"],
                "implemented": stats["implemented"],
                "missing": stats["missing"],
                "completion_rate": stats["completion_rate"],
                "extra_files": stats["extra_files"],
                "priority_counts": stats["priority_counts"],
                "report": stats["report"],
                "top_missing_api": top_gap.name if top_gap is not None else "",
                "top_missing_priority": top_gap.priority_level if top_gap is not None else "",
            }
        )

    crate_rows.sort(key=lambda item: (-item["missing"], item["completion_rate"], item["crate"]))

    return {
        "dashboard": dashboard_name,
        "priority_formula": priority_formula,
        "crates_total": len(scoped_crates),
        "total_apis": total_apis,
        "implemented": implemented,
        "missing": missing,
        "completion_rate": round(completion_rate, 1),
        "extra_files": extra_files,
        "priority_counts": dict(sorted(priority_counts.items())),
        "crates": crate_rows,
        "top_missing_apis": [
            {"crate": crate_name, **APIValidator._serialize_missing_api(api)}
            for crate_name, api in scoped_missing[:20]
        ],
    }


def write_dashboard_markdown(output_path: Path, payload: Dict[str, Any]) -> None:
    """输出专题 dashboard 的 Markdown 视图。"""
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with output_path.open("w", encoding="utf-8") as file:
        file.write(f"# Typed API 覆盖率专题看板：{payload['dashboard']}\n\n")
        file.write("## 总览\n\n")
        file.write("| 指标 | 数量 |\n")
        file.write("|------|------|\n")
        file.write(f"| crate 数量 | {payload['crates_total']} |\n")
        file.write(f"| API 总数 | {payload['total_apis']} |\n")
        file.write(f"| 已实现 | {payload['implemented']} |\n")
        file.write(f"| 未实现 | {payload['missing']} |\n")
        file.write(f"| 完成率 | {payload['completion_rate']:.1f}% |\n")
        file.write(f"| 额外文件 | {payload['extra_files']} |\n\n")

        file.write("## 核心 crate 状态\n\n")
        file.write("| crate | bizTag | 总数 | 已实现 | 未实现 | 完成率 | 重点缺口 | 报告 |\n")
        file.write("|-------|--------|------|--------|--------|--------|----------|------|\n")
        for row in payload["crates"]:
            tags_text = ", ".join(row["biz_tags"])
            focus_gap = row["top_missing_api"]
            report_link = (Path("..") / row["report"]).as_posix()
            if row["top_missing_priority"] and focus_gap:
                focus_gap = f"{row['top_missing_priority']} · {focus_gap}"
            elif not focus_gap:
                focus_gap = "-"
            file.write(
                f"| {row['crate']} | `{tags_text}` | {row['total_apis']} | {row['implemented']} | "
                f"{row['missing']} | {row['completion_rate']:.1f}% | {focus_gap} | "
                f"[report]({report_link}) |\n"
            )
        file.write("\n")

        if payload["priority_counts"]:
            file.write("## 缺口优先级分布\n\n")
            file.write("| 优先级 | 数量 |\n")
            file.write("|--------|------|\n")
            for priority, count in payload["priority_counts"].items():
                file.write(f"| {priority} | {count} |\n")
            file.write("\n")

        if payload["top_missing_apis"]:
            file.write("## 重点缺口 Backlog\n\n")
            file.write(f"- 综合分公式：`{payload['priority_formula']}`\n\n")
            file.write("| 优先级 | 综合分 | crate | API | 预期文件 | 判定规则 |\n")
            file.write("|--------|--------|-------|-----|----------|----------|\n")
            for item in payload["top_missing_apis"]:
                file.write(
                    f"| {item['priority_level']} | {item['priority_score']:.2f} | {item['crate']} | "
                    f"{item['name']} | `{item['expected_file']}` | {', '.join(item['priority_reasons'])} |\n"
                )
            file.write("\n")


def main() -> int:
    """主函数"""
    import argparse

    parser = argparse.ArgumentParser(description="API 验证脚本（基于 strict 命名规范）")
    parser.add_argument("--csv", default="api_list_export.csv", help="CSV 文件路径 (默认: api_list_export.csv)")
    parser.add_argument(
        "--src",
        default=None,
        help="源码目录路径（默认: crates/openlark-meeting/src；也可用 --crate 自动设置）",
    )
    parser.add_argument(
        "--output",
        default=None,
        help="报告输出路径（默认: API_VALIDATION_REPORT.md；--crate 时默认: reports/api_validation/<crate>.md）",
    )
    parser.add_argument("--filter", nargs="+", help="过滤业务标签 (例如: --filter calendar vc meeting_room)")
    parser.add_argument("--crate", help="按 crate 自动设置 --src/--filter（来源: tools/api_coverage.toml）")
    parser.add_argument(
        "--mapping",
        default="tools/api_coverage.toml",
        help="crate→bizTag 映射文件路径 (默认: tools/api_coverage.toml)",
    )
    parser.add_argument(
        "--priority-config",
        default="tools/api_priority.toml",
        help="缺失 API 优先级配置文件 (默认: tools/api_priority.toml)",
    )
    parser.add_argument("--list-crates", action="store_true", help="列出映射文件中的 crate 与 bizTag，然后退出")
    parser.add_argument("--all-crates", action="store_true", help="按映射文件批量验证所有 crate，并生成汇总报告")
    parser.add_argument("--report-dir", default="reports/api_validation", help="批量模式报告目录 (默认: reports/api_validation)")
    parser.add_argument("--skip-old", dest="skip_old", action="store_true", default=True, help="跳过旧版本 API (version=old，默认启用)")
    parser.add_argument("--include-old", dest="skip_old", action="store_false", help="包含旧版本 API (version=old)")
    parser.add_argument("--with-timestamp", action="store_true", help="在报告中写入生成时间（默认关闭，以支持稳定复现）")

    args = parser.parse_args()

    print("=" * 60)
    print("🚀 API 验证工具（Strict 命名规范）")
    print("=" * 60)
    print()

    def _load_mapping(path: str) -> dict:
        if tomllib is None:
            print("❌ 错误: 当前 Python 不支持 tomllib，请使用 Python 3.11+")
            raise SystemExit(1)
        mapping_path = Path(path)
        if not mapping_path.exists():
            print(f"❌ 错误: 映射文件不存在: {mapping_path}")
            raise SystemExit(1)
        data = tomllib.loads(mapping_path.read_text(encoding="utf-8"))
        crates = data.get("crates", {})
        if not isinstance(crates, dict) or not crates:
            print(f"❌ 错误: 映射文件缺少 [crates.*] 配置: {mapping_path}")
            raise SystemExit(1)
        return crates

    if args.list_crates:
        crates = _load_mapping(args.mapping)
        print(f"📄 映射文件: {args.mapping}\n")
        for crate_name in sorted(crates.keys()):
            cfg = crates.get(crate_name, {})
            src = cfg.get("src", "")
            tags = cfg.get("biz_tags", [])
            tags_text = ", ".join(tags) if isinstance(tags, list) else str(tags)
            print(f"- {crate_name}: src={src} biz_tags=[{tags_text}]")
        return 0

    try:
        priority_model = PriorityModel.from_path(args.priority_config)
    except ValueError as exc:
        print(f"❌ 错误: 优先级配置无效: {exc}")
        return 1

    def _run_validator(
        csv_path: str,
        src_path: str,
        filter_tags: Optional[List[str]],
        skip_old: bool,
        with_timestamp: bool,
        implementation_path_rewrites: Optional[List[Dict[str, str]]] = None,
        implementation_path_aliases: Optional[Dict[str, str]] = None,
    ) -> APIValidator:
        validator = APIValidator(
            csv_path,
            src_path,
            filter_tags,
            skip_old,
            with_timestamp,
            priority_model=priority_model,
            implementation_path_rewrites=implementation_path_rewrites,
            implementation_path_aliases=implementation_path_aliases,
        )
        try:
            validator.parse_csv()
        except ValueError as exc:
            print(f"❌ 错误: CSV 校验失败\n{exc}")
            raise SystemExit(1) from exc
        validator.scan_implementations()
        validator.compare()
        return validator

    def _write_summary_markdown(
        output_path: Path,
        crate_rows: List[Tuple[str, Dict[str, Any], str, List[str]]],
        skip_old: bool,
        priority_source_path: str,
        top_missing_apis: List[Dict[str, Any]],
        dashboards: Dict[str, Dict[str, Any]],
    ) -> None:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with output_path.open("w", encoding="utf-8") as file:
            file.write("# Typed API 覆盖率汇总报告（按 crate）\n\n")
            file.write("## 统计口径\n\n")
            if skip_old:
                file.write("- 默认排除 `meta.Version=old`。\n")
            else:
                file.write("- 包含 `meta.Version=old`。\n")
            file.write("- 数据来源：`api_list_export.csv` 对比 crate 源码目录。\n")
            file.write(f"- 缺失 API 优先级配置：`{priority_source_path}`。\n")
            file.write(f"- 综合分公式：`{priority_model.priority_formula()}`。\n\n")

            total_apis = sum(row[1]["total_apis"] for row in crate_rows)
            total_impl = sum(row[1]["implemented"] for row in crate_rows)
            total_missing = sum(row[1]["missing"] for row in crate_rows)
            total_extra = sum(row[1]["extra_files"] for row in crate_rows)
            total_rate = (total_impl / total_apis * 100) if total_apis > 0 else 0.0

            file.write("## 总览\n\n")
            file.write("| 指标 | 数量 |\n")
            file.write("|------|------|\n")
            file.write(f"| crate 数量 | {len(crate_rows)} |\n")
            file.write(f"| API 总数 | {total_apis} |\n")
            file.write(f"| 已实现 | {total_impl} |\n")
            file.write(f"| 未实现 | {total_missing} |\n")
            file.write(f"| 完成率 | {total_rate:.1f}% |\n")
            file.write(f"| 额外文件 | {total_extra} |\n\n")

            file.write("## 各 crate 覆盖率\n\n")
            file.write("| crate | bizTag | 总数 | 已实现 | 未实现 | 完成率 | 额外文件 | 报告 |\n")
            file.write("|-------|--------|------|--------|--------|--------|----------|------|\n")
            for crate_name, stats, report_rel, tags in sorted(crate_rows, key=lambda item: item[0]):
                tags_text = ", ".join(tags)
                file.write(
                    f"| {crate_name} | `{tags_text}` | {stats['total_apis']} | "
                    f"{stats['implemented']} | {stats['missing']} | {stats['completion_rate']:.1f}% | "
                    f"{stats['extra_files']} | [{crate_name}]({report_rel}) |\n"
                )
            file.write("\n")

            if top_missing_apis:
                file.write("## 高价值缺失 API Backlog\n\n")
                file.write("| 优先级 | 综合分 | crate | bizTag | API | 预期文件 | 判定规则 |\n")
                file.write("|--------|--------|-------|--------|-----|----------|----------|\n")
                for item in top_missing_apis:
                    file.write(
                        f"| {item['priority_level']} | {item['priority_score']:.2f} | "
                        f"{item['crate']} | {item['biz_tag']} | {item['name']} | "
                        f"`{item['expected_file']}` | {', '.join(item['priority_reasons'])} |\n"
                    )
                file.write("\n")

            if dashboards:
                file.write("## 专题 Dashboard\n\n")
                file.write("| 分组 | crate 数量 | 未实现 | 完成率 | Markdown | JSON |\n")
                file.write("|------|-----------|--------|--------|----------|------|\n")
                for dashboard_name, payload in sorted(dashboards.items()):
                    file.write(
                        f"| {dashboard_name} | {payload['crates_total']} | {payload['missing']} | "
                        f"{payload['completion_rate']:.1f}% | "
                        f"[md]({payload['markdown_report']}) | [json]({payload['json_report']}) |\n"
                    )
                file.write("\n")

    def _write_summary_json(output_path: Path, payload: Dict[str, Any]) -> None:
        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if args.all_crates:
        if not os.path.exists(args.csv):
            print(f"❌ 错误: CSV 文件不存在: {args.csv}")
            return 1

        crates = _load_mapping(args.mapping)
        invalid_crates: List[Tuple[str, Any]] = []
        for crate_name, cfg in sorted(crates.items()):
            src = cfg.get("src")
            if not src or not os.path.exists(src):
                invalid_crates.append((crate_name, src))
        if invalid_crates:
            print("❌ 错误: 发现映射中的 crate 源码目录不存在，批量验证终止。")
            for crate_name, src in invalid_crates:
                print(f"   - {crate_name}: {src}")
            return 1

        report_dir = Path(args.report_dir)
        crate_dir = report_dir / "crates"
        crate_rows: List[Tuple[str, Dict[str, Any], str, List[str]]] = []
        crate_summaries: Dict[str, Any] = {}
        all_missing_apis: List[Tuple[str, APIInfo]] = []

        for crate_name in sorted(crates.keys()):
            cfg = crates[crate_name]
            src = cfg.get("src")
            tags = cfg.get("biz_tags", [])
            report_path = crate_dir / f"{crate_name}.md"
            print()
            print(f"📦 处理 {crate_name}")
            validator = _run_validator(
                args.csv,
                src,
                tags,
                args.skip_old,
                args.with_timestamp,
                cfg.get("implementation_path_rewrites"),
                cfg.get("implementation_path_aliases"),
            )
            report_path.parent.mkdir(parents=True, exist_ok=True)
            validator.generate_report(str(report_path))
            stats = validator.calculate_summary()
            report_rel = report_path.relative_to(report_dir).as_posix()
            crate_rows.append((crate_name, stats, report_rel, tags))
            crate_summaries[crate_name] = {
                "source_dir": src,
                "biz_tags": tags,
                "report": report_rel,
                **stats,
            }
            for api in validator.missing_apis:
                all_missing_apis.append((crate_name, api))

        all_missing_apis.sort(key=lambda item: priority_model.sort_key(item[1]))
        top_missing_apis = [
            {"crate": crate_name, **APIValidator._serialize_missing_api(api)}
            for crate_name, api in all_missing_apis[:30]
        ]

        dashboard_payloads: Dict[str, Dict[str, Any]] = {}
        for dashboard_name, crate_names in collect_dashboard_groups(crates).items():
            payload = build_dashboard_payload(
                dashboard_name,
                crate_names,
                crate_summaries,
                all_missing_apis,
                priority_model.priority_formula(),
            )
            slug = dashboard_slug(dashboard_name)
            markdown_path = report_dir / "dashboards" / f"{slug}.md"
            json_path = report_dir / "dashboards" / f"{slug}.json"
            write_dashboard_markdown(markdown_path, payload)
            _write_summary_json(json_path, payload)
            dashboard_payloads[dashboard_name] = {
                **payload,
                "markdown_report": markdown_path.relative_to(report_dir).as_posix(),
                "json_report": json_path.relative_to(report_dir).as_posix(),
            }

        summary_md = report_dir / "summary.md"
        summary_json = report_dir / "summary.json"
        _write_summary_markdown(
            summary_md,
            crate_rows,
            args.skip_old,
            priority_model.source_path,
            top_missing_apis,
            dashboard_payloads,
        )

        total_apis = sum(item["total_apis"] for item in crate_summaries.values())
        total_impl = sum(item["implemented"] for item in crate_summaries.values())
        total_missing = sum(item["missing"] for item in crate_summaries.values())
        total_extra = sum(item["extra_files"] for item in crate_summaries.values())
        total_rate = (total_impl / total_apis * 100) if total_apis > 0 else 0.0

        priority_counts: Dict[str, int] = defaultdict(int)
        for _, api in all_missing_apis:
            priority_counts[api.priority_level] += 1

        summary_payload = {
            "csv_path": args.csv,
            "mapping_path": args.mapping,
            "priority_config_path": priority_model.source_path,
            "priority_formula": priority_model.priority_formula(),
            "skip_old_versions": args.skip_old,
            "crates_total": len(crate_summaries),
            "total_apis": total_apis,
            "implemented": total_impl,
            "missing": total_missing,
            "completion_rate": round(total_rate, 1),
            "extra_files": total_extra,
            "priority_counts": dict(sorted(priority_counts.items())),
            "top_missing_apis": top_missing_apis,
            "dashboards": dashboard_payloads,
            "crates": crate_summaries,
        }
        _write_summary_json(summary_json, summary_payload)

        print()
        print("=" * 60)
        print("✅ 批量验证完成！")
        print(f"📄 汇总报告: {summary_md}")
        print(f"📄 机器可读: {summary_json}")
        print(f"📁 各 crate 报告目录: {crate_dir}")
        print("=" * 60)
        return 0

    if args.crate:
        crates = _load_mapping(args.mapping)
        if args.crate not in crates:
            print(f"❌ 错误: 映射文件中不存在 crate: {args.crate}")
            print("   提示：运行 `python3 tools/validate_apis.py --list-crates` 查看可用项")
            return 1
        cfg = crates[args.crate]
        if args.src is None:
            args.src = cfg.get("src")
        if args.filter is None:
            args.filter = cfg.get("biz_tags")
        implementation_path_rewrites = cfg.get("implementation_path_rewrites")
        implementation_path_aliases = cfg.get("implementation_path_aliases")
    else:
        implementation_path_rewrites = None
        implementation_path_aliases = None

    if args.output is None:
        if args.crate:
            args.output = f"reports/api_validation/{args.crate}.md"
        else:
            args.output = "API_VALIDATION_REPORT.md"

    if args.src is None:
        args.src = "crates/openlark-meeting/src"

    if not os.path.exists(args.csv):
        print(f"❌ 错误: CSV 文件不存在: {args.csv}")
        return 1

    if not os.path.exists(args.src):
        print(f"❌ 错误: 源码目录不存在: {args.src}")
        return 1

    validator = _run_validator(
        args.csv,
        args.src,
        args.filter,
        args.skip_old,
        args.with_timestamp,
        implementation_path_rewrites,
        implementation_path_aliases,
    )
    Path(args.output).parent.mkdir(parents=True, exist_ok=True)
    validator.generate_report(args.output)

    print()
    print("=" * 60)
    print("✅ 验证完成！")
    print(f"📄 报告已保存到: {args.output}")
    print("=" * 60)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
