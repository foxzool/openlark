"""Issue #41 / #585 leaf-paradigm guardrail seams.

Public seams under test:
1. ``scan_file`` — detects E001 / E002 / E003 (and clean golden)
2. CI inventory pin — deleting the hard gate step or critical flags fails tests

Enforcement ladder (see docs/issue-41-guardrail.md):
- Hard (CI): ERROR on DEFAULT_TARGET_CRATES, no --strict-warn
- Deferred: W001 + crates outside DEFAULT_TARGET_CRATES with historical E00x debt
"""

from __future__ import annotations

import importlib.util
import re
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MODULE_PATH = ROOT / "tools" / "issue41_guardrail.py"
CI_WORKFLOW_PATH = ROOT / ".github" / "workflows" / "ci.yml"
GUARDRAIL_DOC_PATH = ROOT / "docs" / "issue-41-guardrail.md"

SPEC = importlib.util.spec_from_file_location("issue41_guardrail", MODULE_PATH)
guardrail = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = guardrail
SPEC.loader.exec_module(guardrail)


# Minimal leaf snippets — independent source of truth for each rule code.
FIXTURE_E001_MISSING_EXECUTE_WITH_OPTIONS = """\
use openlark_core::req_option::RequestOption;

pub struct MissingEwoRequest {
    config: std::sync::Arc<openlark_core::config::Config>,
}

impl MissingEwoRequest {
    pub async fn execute(self) -> openlark_core::SDKResult<()> {
        // Intentionally no execute_with_options — E001
        Ok(())
    }
}
"""

FIXTURE_E002_EXECUTE_NOT_DELEGATING = """\
use openlark_core::req_option::RequestOption;

pub struct NoDelegateRequest {
    config: std::sync::Arc<openlark_core::config::Config>,
}

impl NoDelegateRequest {
    pub async fn execute(self) -> openlark_core::SDKResult<()> {
        // Has execute_with_options but does not delegate — E002
        Ok(())
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> openlark_core::SDKResult<()> {
        let _ = option;
        Ok(())
    }
}
"""

FIXTURE_E003_OPTION_IGNORED = """\
use openlark_core::{
    http::Transport,
    req_option::RequestOption,
};

pub struct OptionIgnoredRequest {
    config: std::sync::Arc<openlark_core::config::Config>,
}

impl OptionIgnoredRequest {
    pub async fn execute(self) -> openlark_core::SDKResult<()> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> openlark_core::SDKResult<()> {
        let _ = option;
        // Drops option — E003
        Transport::request((), &self.config, None).await
    }
}
"""

FIXTURE_CLEAN_GOLDEN = """\
use openlark_core::{
    http::Transport,
    req_option::RequestOption,
};

pub struct CleanLeafRequest {
    config: std::sync::Arc<openlark_core::config::Config>,
}

impl CleanLeafRequest {
    pub async fn execute(self) -> openlark_core::SDKResult<()> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> openlark_core::SDKResult<()> {
        Transport::request((), &self.config, Some(option)).await
    }
}
"""


def _write_fixture(tmp: Path, name: str, content: str) -> Path:
    path = tmp / name
    path.write_text(content, encoding="utf-8")
    return path


def _codes(findings) -> list[str]:
    return sorted({f.code for f in findings})


class Issue41ScanFileFixturesTests(unittest.TestCase):
    """Golden / violation fixtures for the three hard ERROR codes."""

    def test_e001_missing_execute_with_options(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            path = _write_fixture(
                Path(d), "missing_ewo.rs", FIXTURE_E001_MISSING_EXECUTE_WITH_OPTIONS
            )
            findings = guardrail.scan_file(path, Path(d))
        self.assertIn("E001", _codes(findings))
        self.assertTrue(any(f.severity == "ERROR" and f.code == "E001" for f in findings))

    def test_e002_execute_not_delegating(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            path = _write_fixture(
                Path(d), "no_delegate.rs", FIXTURE_E002_EXECUTE_NOT_DELEGATING
            )
            findings = guardrail.scan_file(path, Path(d))
        self.assertIn("E002", _codes(findings))
        self.assertTrue(any(f.severity == "ERROR" and f.code == "E002" for f in findings))
        # execute_with_options exists, so E001 must not fire.
        self.assertNotIn("E001", _codes(findings))

    def test_e003_option_ignored_via_transport_none(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            path = _write_fixture(
                Path(d), "option_none.rs", FIXTURE_E003_OPTION_IGNORED
            )
            findings = guardrail.scan_file(path, Path(d))
        self.assertIn("E003", _codes(findings))
        self.assertTrue(any(f.severity == "ERROR" and f.code == "E003" for f in findings))

    def test_clean_golden_has_no_error(self) -> None:
        with tempfile.TemporaryDirectory() as d:
            path = _write_fixture(Path(d), "clean.rs", FIXTURE_CLEAN_GOLDEN)
            findings = guardrail.scan_file(path, Path(d))
        errors = [f for f in findings if f.severity == "ERROR"]
        self.assertEqual(errors, [], msg=f"unexpected ERROR on clean golden: {findings}")


class Issue41CiHardGateInventoryTests(unittest.TestCase):
    """Pin the CI hard gate so deleting the step or critical flags fails tests (#585)."""

    def setUp(self) -> None:
        self.workflow = CI_WORKFLOW_PATH.read_text(encoding="utf-8")
        self.doc = GUARDRAIL_DOC_PATH.read_text(encoding="utf-8")

    def _lint_job(self) -> str:
        match = re.search(
            r"(?ms)^  lint:\n(.*?)(?=^  [a-z0-9_-]+:|\Z)",
            self.workflow,
        )
        self.assertIsNotNone(match, "lint job missing from ci.yml")
        assert match is not None
        return match.group(0)

    def _issue41_step_block(self) -> str:
        job = self._lint_job()
        parts = re.split(r"(?m)^(?=      - (?:name|uses):)", job)
        for block in parts:
            if "issue41_guardrail.py" in block or "issue41-guardrail" in block:
                return block
        self.fail("no lint step invokes issue41_guardrail (hard gate missing)")

    def test_ci_runs_issue41_guardrail_as_hard_step(self) -> None:
        block = self._issue41_step_block()
        self.assertIn("python3 tools/issue41_guardrail.py", block)
        # Hard gate: ERROR fails; W001 must remain non-blocking for now.
        # Only the executable run lines count — comments may mention --strict-warn.
        run_lines = [
            line.strip()
            for line in block.splitlines()
            if line.strip().startswith("python3 ")
        ]
        self.assertTrue(
            any(line == "python3 tools/issue41_guardrail.py" for line in run_lines),
            f"expected bare guardrail invocation among {run_lines}",
        )
        for line in run_lines:
            self.assertNotIn(
                "--strict-warn",
                line,
                "CI hard gate must not enable --strict-warn until W001 debt is cleared",
            )

    def test_ci_runs_guardrail_unit_tests(self) -> None:
        job = self._lint_job()
        self.assertIn(
            "tools.tests.test_issue41_guardrail",
            job,
            "CI must run guardrail unit/inventory tests so the pin cannot be deleted alone",
        )

    def test_default_enforcement_crates_are_pinned(self) -> None:
        expected = {
            "openlark-docs",
            "openlark-meeting",
            "openlark-communication",
            "openlark-hr",
        }
        self.assertEqual(set(guardrail.DEFAULT_TARGET_CRATES), expected)

    def test_enforcement_ladder_documented(self) -> None:
        # Ladder sections must name hard vs deferred so agents cannot "forget" scope.
        self.assertRegex(self.doc, r"(?i)enforcement ladder|强制阶梯|执行阶梯")
        self.assertIn("E001", self.doc)
        self.assertIn("E002", self.doc)
        self.assertIn("E003", self.doc)
        self.assertIn("W001", self.doc)
        # Deferred historical debt crates (not hard-gated yet).
        for deferred in ("openlark-ai", "openlark-helpdesk", "openlark-platform"):
            self.assertIn(deferred, self.doc)


if __name__ == "__main__":
    unittest.main()
