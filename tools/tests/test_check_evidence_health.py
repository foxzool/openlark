"""check_evidence_health 绊线测试（防上游 schema 变更静默致盲）。"""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from tools.check_evidence_health import (
    collect_request_fields_status,
    evaluate,
    main,
)


def _write_crate_report(report_dir: Path, crate: str, statuses: list[str]) -> None:
    """写一个最小可解析的 crate 报告（evidence 嵌套 dimensions 形状）。"""
    crate_dir = report_dir / "crates"
    crate_dir.mkdir(parents=True, exist_ok=True)
    payload = {
        "crate": crate,
        "evidence": [
            {
                "api_id": f"{i}",
                "dimensions": [
                    {"dimension": "request_fields", "status": status},
                    {"dimension": "response_fields", "status": "trusted"},
                ],
            }
            for i, status in enumerate(statuses)
        ],
    }
    (crate_dir / f"{crate}.json").write_text(
        json.dumps(payload, ensure_ascii=False), encoding="utf-8"
    )


class EvidenceHealthTests(unittest.TestCase):
    def test_healthy_pool_passes(self) -> None:
        ok, message = evaluate(
            ["trusted"] * 40 + ["incomplete"] * 20,
            trusted_floor=0.40,
            min_evidence=30,
        )
        self.assertTrue(ok)
        self.assertIn("67%", message)

    def test_blinded_pool_fails_with_format_change_hint(self) -> None:
        ok, message = evaluate(
            ["incomplete"] * 59 + ["trusted"],
            trusted_floor=0.40,
            min_evidence=30,
        )
        self.assertFalse(ok)
        self.assertIn("致盲", message)
        self.assertIn("official_evidence", message)

    def test_insufficient_evidence_fails(self) -> None:
        ok, message = evaluate(["trusted"] * 5, trusted_floor=0.40, min_evidence=30)
        self.assertFalse(ok)
        self.assertIn("样本不足", message)

    def test_collect_reads_nested_dimensions_only_for_request_fields(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            report_dir = Path(tmp)
            _write_crate_report(report_dir, "openlark-ai", ["trusted", "incomplete"])
            statuses = collect_request_fields_status(report_dir)
        self.assertEqual(statuses, ["trusted", "incomplete"])

    def test_main_exit_codes(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            report_dir = Path(tmp)
            _write_crate_report(report_dir, "openlark-ai", ["trusted"] * 35)
            self.assertEqual(main(["--report-dir", str(report_dir)]), 0)
            _write_crate_report(report_dir, "openlark-platform", ["incomplete"] * 60)
            self.assertEqual(main(["--report-dir", str(report_dir)]), 1)


if __name__ == "__main__":
    unittest.main()
