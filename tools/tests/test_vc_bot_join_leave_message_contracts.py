"""#645 VC bot join/leave/message Rust Contract Target 验收。

Seams（公开边界）:
1. 三个 POST Catalog Entry 均 Resolved
2. HTTP method/path 与 catalog url 一致
"""

from __future__ import annotations

import unittest
from pathlib import Path

from tools.api_contracts.official import load_api_identities
from tools.api_contracts.rust_contract_resolution import Resolved, compose
from tools.api_contracts.rust_source import RustSourceContractAdapter


REPO_ROOT = Path(__file__).resolve().parents[2]

API_IDS = (
    "7672664994767015159",  # join
    "7672664994766998775",  # leave
    "7672664994766982391",  # message
)


class VcBotJoinLeaveMessageContractTests(unittest.TestCase):
    def test_three_post_contract_targets_are_resolved(self) -> None:
        identities = {
            entry.api_id: entry
            for entry in load_api_identities(REPO_ROOT / "api_list_export.csv")
        }
        resolver = compose(repository_root=REPO_ROOT)
        adapter = RustSourceContractAdapter(REPO_ROOT)

        for api_id in API_IDS:
            with self.subTest(api_id=api_id):
                self.assertIn(api_id, identities, f"catalog 缺少 id={api_id}")
                entry = identities[api_id]
                result = resolver.resolve(entry)
                self.assertIsInstance(result, Resolved, f"id={api_id} 未 Resolved: {result}")
                assert isinstance(result, Resolved)
                contract = adapter.scan(result.target)
                self.assertGreaterEqual(len(contract.endpoint_calls), 1)
                call = contract.endpoint_calls[0]
                catalog_method, catalog_path = entry.url.split(":", 1)
                self.assertEqual(call.method, catalog_method)
                self.assertEqual(call.resolved_path, catalog_path)


if __name__ == "__main__":
    unittest.main()
