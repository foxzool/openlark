from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from tools.api_contracts.models import ApiIdentity
from tools.api_contracts.official import load_api_identities
from tools.api_contracts.rust_contract_resolution import (
    Ambiguous,
    Missing,
    ResolutionConfigurationError,
    Resolved,
    Unmapped,
    compose,
)
from tools.api_contracts.rust_source import RustSourceContractAdapter


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]


def _entry(*, biz_tag: str = "ai", expected_file: str = "ai/doc/v1/item/get.rs") -> ApiIdentity:
    return ApiIdentity(
        api_id="1",
        name="get item",
        biz_tag=biz_tag,
        meta_project="doc",
        meta_version="v1",
        meta_resource="item",
        meta_name="get",
        url="GET:/open-apis/doc/v1/items/:item_id",
        doc_path="https://open.feishu.cn/document/mock",
        expected_file=expected_file,
    )


class RustContractResolutionTests(unittest.TestCase):
    def _repository(self, mapping: str) -> tuple[tempfile.TemporaryDirectory[str], Path]:
        temporary = tempfile.TemporaryDirectory()
        root = Path(temporary.name)
        (root / "tools").mkdir()
        (root / "tools/api_coverage.toml").write_text(mapping, encoding="utf-8")
        (root / "crates/openlark-ai/src").mkdir(parents=True)
        return temporary, root

    def test_resolves_canonical_alias_and_rewrite_with_equal_weight(self) -> None:
        mapping = (
            "[crates.openlark-ai]\n"
            'src = "crates/openlark-ai/src"\n'
            'biz_tags = ["ai"]\n'
            'implementation_path_rewrites = [{ from = "ai/doc/v1/", to = "legacy/" }]\n'
            "[crates.openlark-ai.implementation_path_aliases]\n"
            '"ai/doc/v1/item/alias.rs" = "aliases/item.rs"\n'
        )
        temporary, root = self._repository(mapping)
        self.addCleanup(temporary.cleanup)
        canonical = root / "crates/openlark-ai/src/ai/doc/v1/item/get.rs"
        canonical.parent.mkdir(parents=True)
        canonical.write_text("// canonical", encoding="utf-8")
        alias = root / "crates/openlark-ai/src/aliases/item.rs"
        alias.parent.mkdir(parents=True)
        alias.write_text("// alias", encoding="utf-8")
        rewritten = root / "crates/openlark-ai/src/legacy/item/rewrite.rs"
        rewritten.parent.mkdir(parents=True)
        rewritten.write_text("// rewrite", encoding="utf-8")

        resolver = compose(repository_root=root)
        canonical_result = resolver.resolve(_entry())
        alias_result = resolver.resolve(
            _entry(expected_file="ai/doc/v1/item/alias.rs")
        )
        rewrite_result = resolver.resolve(
            _entry(expected_file="ai/doc/v1/item/rewrite.rs")
        )

        self.assertIsInstance(canonical_result, Resolved)
        self.assertEqual(
            canonical_result.target.repository_path,
            Path("crates/openlark-ai/src/ai/doc/v1/item/get.rs"),
        )
        self.assertIsInstance(alias_result, Resolved)
        self.assertEqual(
            alias_result.target.repository_path,
            Path("crates/openlark-ai/src/aliases/item.rs"),
        )
        self.assertIsInstance(rewrite_result, Resolved)

    def test_reports_unmapped_missing_and_ambiguous_without_guessing(self) -> None:
        mapping = (
            "[crates.openlark-ai]\n"
            'src = "crates/openlark-ai/src"\n'
            'biz_tags = ["ai"]\n'
            'implementation_path_rewrites = [{ from = "ai/doc/v1/", to = "legacy/" }]\n'
        )
        temporary, root = self._repository(mapping)
        self.addCleanup(temporary.cleanup)
        canonical = root / "crates/openlark-ai/src/ai/doc/v1/item/get.rs"
        legacy = root / "crates/openlark-ai/src/legacy/item/get.rs"
        canonical.parent.mkdir(parents=True)
        legacy.parent.mkdir(parents=True)
        canonical.write_text("// canonical", encoding="utf-8")
        legacy.write_text("// legacy", encoding="utf-8")

        resolver = compose(repository_root=root)
        self.assertIsInstance(resolver.resolve(_entry(biz_tag="unknown")), Unmapped)
        missing = resolver.resolve(_entry(expected_file="ai/doc/v1/item/missing.rs"))
        self.assertIsInstance(missing, Missing)
        self.assertEqual(len(missing.checked_candidates), 2)
        ambiguous = resolver.resolve(_entry())
        self.assertIsInstance(ambiguous, Ambiguous)
        self.assertEqual(len(ambiguous.candidates), 2)

    def test_snapshot_is_immutable_for_the_resolver_lifetime(self) -> None:
        temporary, root = self._repository(
            "[crates.openlark-ai]\n"
            'src = "crates/openlark-ai/src"\n'
            'biz_tags = ["ai"]\n'
        )
        self.addCleanup(temporary.cleanup)
        resolver = compose(repository_root=root)
        target = root / "crates/openlark-ai/src/ai/doc/v1/item/get.rs"
        target.parent.mkdir(parents=True)
        target.write_text("// created after composition", encoding="utf-8")

        self.assertIsInstance(resolver.resolve(_entry()), Missing)
        self.assertIsInstance(compose(repository_root=root).resolve(_entry()), Resolved)

    def test_composition_aggregates_mapping_diagnostics(self) -> None:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        (root / "tools").mkdir()
        (root / "tools/api_coverage.toml").write_text(
            "[crates.first]\n"
            'src = "../escape"\n'
            'biz_tags = ["ai"]\n'
            "[crates.second]\n"
            'src = "crates/missing/src"\n'
            'biz_tags = ["ai"]\n'
            'implementation_path_rewrites = [{ from = "bad", to = "target" }]\n',
            encoding="utf-8",
        )

        with self.assertRaises(ResolutionConfigurationError) as raised:
            compose(repository_root=root)

        codes = {item.code for item in raised.exception.diagnostics}
        self.assertIn("crate_source_malformed", codes)
        self.assertIn("crate_source_missing", codes)
        self.assertIn("duplicate_biz_tag", codes)
        self.assertIn("implementation_rewrite_malformed", codes)

    def test_invalid_catalog_path_is_an_exception_not_a_resolution(self) -> None:
        temporary, root = self._repository(
            "[crates.openlark-ai]\n"
            'src = "crates/openlark-ai/src"\n'
            'biz_tags = ["ai"]\n'
        )
        self.addCleanup(temporary.cleanup)
        resolver = compose(repository_root=root)

        with self.assertRaises(ValueError):
            resolver.resolve(_entry(expected_file="../outside.rs"))

    def test_source_adapter_consumes_target_and_caches_crate_indexes(self) -> None:
        temporary, root = self._repository(
            "[crates.openlark-ai]\n"
            'src = "crates/openlark-ai/src"\n'
            'biz_tags = ["ai"]\n'
        )
        self.addCleanup(temporary.cleanup)
        source = root / "crates/openlark-ai/src/ai/doc/v1/item/get.rs"
        source.parent.mkdir(parents=True)
        source.write_text(
            'let request = ApiRequest::get("/open-apis/doc/v1/items/:item_id");',
            encoding="utf-8",
        )
        resolution = compose(repository_root=root).resolve(_entry())
        self.assertIsInstance(resolution, Resolved)

        adapter = RustSourceContractAdapter(root)
        with (
            mock.patch(
                "tools.api_contracts.rust_source.load_endpoint_constants",
                return_value={},
            ) as constants,
            mock.patch(
                "tools.api_contracts.rust_source.load_enum_endpoints",
                return_value={},
            ),
            mock.patch(
                "tools.api_contracts.rust_source.load_enum_methods",
                return_value={},
            ),
        ):
            first = adapter.scan(resolution.target)
            second = adapter.scan(resolution.target)

        self.assertEqual(first.rel_path, "ai/doc/v1/item/get.rs")
        self.assertEqual(second.rel_path, first.rel_path)
        constants.assert_called_once()

    def test_repository_catalog_is_completely_and_safely_classified(self) -> None:
        resolver = compose(repository_root=REPOSITORY_ROOT)
        entries = load_api_identities(REPOSITORY_ROOT / "api_list_export.csv")
        resolutions = [resolver.resolve(entry) for entry in entries]

        self.assertEqual(len(resolutions), len(entries))
        self.assertTrue(any(isinstance(item, Resolved) for item in resolutions))
        self.assertFalse(any(isinstance(item, Unmapped) for item in resolutions))
        self.assertFalse(any(isinstance(item, Ambiguous) for item in resolutions))


if __name__ == "__main__":
    unittest.main()
