"""Pin field-verify CI to fixed coverage iteration and merged reports (#638).

Rust Contract Resolution prevents empty global path scans. Weekly/full workflows
still invoke `--crate` per coverage crate to preserve artifact layout and failure
isolation. Dual-edit: changing the workflow invocation must update this test.
"""

from __future__ import annotations

import json
import io
import os
import stat
import sys
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from types import SimpleNamespace
from unittest import mock

from tools import run_field_verify_ci as ci_entry


ROOT = Path(__file__).resolve().parents[2]
WEEKLY_WORKFLOW = (
    ROOT / ".github" / "workflows" / "api-field-verify-weekly.yml"
)
FULL_WORKFLOW = ROOT / ".github" / "workflows" / "api-field-verify-full.yml"


def _direct_verify_lines(workflow: str) -> list[str]:
    lines = []
    for raw in workflow.splitlines():
        line = raw.strip()
        if "verify_api_fields.py" not in line:
            continue
        if line.startswith("echo ") or line.startswith("#"):
            continue
        lines.append(line)
    return lines


class WeeklyFieldVerifyGateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.workflow = WEEKLY_WORKFLOW.read_text(encoding="utf-8")

    def test_weekly_scan_goes_through_ci_entry(self) -> None:
        self.assertIn("run_field_verify_ci.py", self.workflow)
        self.assertEqual(
            _direct_verify_lines(self.workflow),
            [],
            "weekly must not invoke verify_api_fields.py directly",
        )

    def test_weekly_job_runs_this_module(self) -> None:
        self.assertIn(
            "tools.tests.test_verify_api_fields_ci_gates",
            self.workflow,
        )


class FullFieldVerifyGateTests(unittest.TestCase):
    def test_full_scan_goes_through_ci_entry(self) -> None:
        workflow = FULL_WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("run_field_verify_ci.py", workflow)
        self.assertEqual(
            _direct_verify_lines(workflow),
            [],
            "full must not invoke verify_api_fields.py directly",
        )


class FieldVerifyCiEntrySourceTests(unittest.TestCase):
    def test_entry_reads_coverage_table_and_merges_summaries(self) -> None:
        source = CI_ENTRY.read_text(encoding="utf-8")
        self.assertIn("api_coverage.toml", source)
        self.assertIn('glob("*/summary.json")', source)


class CiRunsFieldVerifyGatesTests(unittest.TestCase):
    def test_pr_ci_runs_this_module(self) -> None:
        ci = (ROOT / ".github" / "workflows" / "ci.yml").read_text(
            encoding="utf-8"
        )
        self.assertIn("tools.tests.test_verify_api_fields_ci_gates", ci)


CI_ENTRY = ROOT / "tools" / "run_field_verify_ci.py"


def _write_stub_verify(path: Path) -> None:
    """Mimic verify_api_fields --crate output: <dir>/summary.json + <dir>/<crate>.md."""
    path.write_text(
        """#!/usr/bin/env python3
import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("--crate", required=True)
parser.add_argument("--output-dir", required=True)
parser.add_argument("--fetch-docs", action="store_true")
parser.add_argument("--max-age")
parser.add_argument("--force-refresh", action="store_true")
args = parser.parse_args()
out = Path(args.output_dir)
out.mkdir(parents=True, exist_ok=True)
(out / "summary.json").write_text(
    '{"mode":"quick","total_apis":1,"apis_with_issues":0,'
    f'"apis":[{{"id":"1","file_exists":true,"issues":[],"crate":"{args.crate}"}}]}}',
    encoding="utf-8",
)
(out / f"{args.crate}.md").write_text(f"# {args.crate}\\n", encoding="utf-8")
""",
        encoding="utf-8",
    )
    path.chmod(path.stat().st_mode | stat.S_IEXEC)


class FieldVerifyCiEntryTests(unittest.TestCase):
    def _run(
        self,
        *args: str,
        repository_root: Path,
        env: dict[str, str] | None = None,
    ) -> SimpleNamespace:
        stdout = io.StringIO()
        stderr = io.StringIO()
        with (
            mock.patch.dict(os.environ, env or {}, clear=False),
            redirect_stdout(stdout),
            redirect_stderr(stderr),
        ):
            try:
                returncode = ci_entry.main(list(args), repository_root)
            except SystemExit as exc:
                returncode = int(exc.code or 0)
        return SimpleNamespace(
            returncode=returncode,
            stdout=stdout.getvalue(),
            stderr=stderr.getvalue(),
        )

    def test_empty_coverage_list_exits_nonzero(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            mapping = root / "tools/api_coverage.toml"
            mapping.parent.mkdir()
            mapping.write_text("# no crates\n", encoding="utf-8")
            result = self._run(
                "full", "--output-dir", tmp, repository_root=root
            )
            self.assertNotEqual(result.returncode, 0, result.stdout)
            combined = f"{result.stdout}\n{result.stderr}".lower()
            self.assertRegex(
                combined,
                r"empty|no crates|crate list",
                "empty coverage must fail closed with an explicit reason, "
                f"not a missing-file accident: {combined!r}",
            )

    def test_explicit_crate_keeps_root_artifact_layout(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            mapping = root / "tools/api_coverage.toml"
            mapping.parent.mkdir()
            mapping.write_text(
                '[crates.openlark-workflow]\nsrc = "crates/openlark-workflow/src"\n',
                encoding="utf-8",
            )
            stub = root / "stub_verify.py"
            _write_stub_verify(stub)
            out = root / "reports"
            result = self._run(
                "full",
                "--output-dir",
                str(out),
                "--crate",
                "openlark-workflow",
                repository_root=root,
                env={"FIELD_VERIFY_BIN": str(stub)},
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue(
                (out / "summary.json").is_file(),
                "explicit crate must write summary.json at output-dir root",
            )
            self.assertTrue(
                (out / "openlark-workflow.md").is_file(),
                "explicit crate must write <crate>.md at output-dir root",
            )
            self.assertFalse(
                (out / "openlark-workflow" / "summary.json").exists(),
                "explicit crate must not nest artifacts under <crate>/",
            )
            summary = json.loads((out / "summary.json").read_text(encoding="utf-8"))
            self.assertTrue(summary["apis"][0]["file_exists"])

    def test_full_all_crates_writes_root_summary_with_full_mode(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            mapping = root / "tools/api_coverage.toml"
            mapping.parent.mkdir()
            mapping.write_text(
                "[crates.openlark-workflow]\n"
                'src = "crates/openlark-workflow/src"\n'
                "[crates.openlark-docs]\n"
                'src = "crates/openlark-docs/src"\n',
                encoding="utf-8",
            )
            stub = root / "stub_verify.py"
            _write_stub_verify(stub)
            out = root / "reports"
            result = self._run(
                "full",
                "--output-dir",
                str(out),
                repository_root=root,
                env={"FIELD_VERIFY_BIN": str(stub)},
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            root_summary = out / "summary.json"
            self.assertTrue(
                root_summary.is_file(),
                "full all-crates must write merged summary.json at output-dir root",
            )
            summary = json.loads(root_summary.read_text(encoding="utf-8"))
            self.assertEqual(summary["mode"], "full")
            self.assertEqual(summary["total_apis"], 2)
            crates = {api["crate"] for api in summary["apis"]}
            self.assertEqual(crates, {"openlark-docs", "openlark-workflow"})
            self.assertTrue((out / "openlark-workflow" / "summary.json").is_file())
            self.assertTrue((out / "openlark-docs" / "summary.json").is_file())


if __name__ == "__main__":
    unittest.main()
