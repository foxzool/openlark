"""通过 CI 的真实执行入口验证 feature matrix 的退出状态与覆盖证据。"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/feature-matrix.yml"
STEPS = {
    "individual": "Test each feature individually",
    "core": "Test core feature combinations",
}


def workflow_run(step_name):
    """读取实际 run 块，回归测试不复制另一套执行命令。"""
    lines = WORKFLOW.read_text(encoding="utf-8").splitlines()
    start = lines.index(f"      - name: {step_name}")
    start = next(i for i in range(start, len(lines)) if lines[i] == "        run: |")
    body = []
    for line in lines[start + 1 :]:
        if line.strip() and not line.startswith("          "):
            break
        body.append(line[10:])
    return "\n".join(body)


def executed_arguments(result):
    return [
        json.loads(line[5:])
        for line in result.stdout.splitlines()
        if line.startswith("EXEC ")
    ]


class FeatureMatrixTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.report = self.directory / "report.json"
        cargo = self.directory / "cargo"
        cargo.write_text(
            textwrap.dedent("""\
                #!/usr/bin/env python3
                import os
                import json
                import sys
                import time

                if "--print-command-list" in sys.argv:
                    plan_mode = os.environ.get("FAKE_PLAN_MODE", "success")
                    if plan_mode == "failure":
                        sys.exit(23)
                    if plan_mode == "timeout":
                        time.sleep(2)
                    if plan_mode == "empty":
                        sys.exit(0)
                    for suffix in ("", " --features default", " --features docs"):
                        print("cargo test --lib --manifest-path Cargo.toml --no-default-features" + suffix)
                    if plan_mode == "invalid":
                        print("cargo test --manifest-path crates/openlark-auth/Cargo.toml")
                    sys.exit(0)
                print("EXEC " + json.dumps(sys.argv[1:]), flush=True)
                print("test result: ok (output is not the exit status)", flush=True)
                mode = os.environ.get("FAKE_CARGO_MODE", "failure")
                if mode == "timeout" and "--features" not in sys.argv:
                    time.sleep(2)
                if mode == "cumulative_timeout":
                    time.sleep(0.2)
                if mode == "signal":
                    os.kill(os.getpid(), 15)
                if mode == "success":
                    print("FAILED (diagnostic text is not the exit status)", flush=True)
                sys.exit(23 if mode == "failure" or (mode == "fail_docs" and "docs" in sys.argv) else 0)
                """),
            encoding="utf-8",
        )
        cargo.chmod(0o755)

    def run_phase(self, phase, **extra_env):
        self.report.unlink(missing_ok=True)
        env = {
            **os.environ,
            "PATH": f"{self.directory}{os.pathsep}{os.environ['PATH']}",
            "GITHUB_STEP_SUMMARY": str(self.directory / "summary.md"),
            "FEATURE_MATRIX_REPORT": str(self.report),
            "FEATURE_MATRIX_TIMEOUT_SECONDS": "10",
            **extra_env,
        }
        return subprocess.run(
            ["bash", "-e", "-c", workflow_run(STEPS[phase])],
            cwd=ROOT,
            env=env,
            text=True,
            capture_output=True,
            check=False,
            timeout=15,
        )

    def test_command_failure_reaches_both_workflow_steps(self):
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(phase)
                self.assertEqual(result.returncode, 23, result.stdout + result.stderr)

    def test_core_executes_every_approved_logical_combination(self):
        result = self.run_phase("core", FAKE_CARGO_MODE="success")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        report = json.loads(self.report.read_text())
        observed = executed_arguments(result)
        self.assertEqual(observed, [item["argv"][1:] for item in report["commands"]])
        combinations = set()
        for args in observed:
            self.assertIn("--no-default-features", args)
            self.assertEqual(args[args.index("--package") + 1], "openlark")
            self.assertIn("--lib", args)
            combinations.add(
                args[args.index("--features") + 1] if "--features" in args else ""
            )
        self.assertEqual(
            combinations,
            {
                "",
                "auth",
                "docs",
                "communication",
                "hr",
                "ai",
                "websocket",
                "auth,docs",
                "auth,communication",
                "auth,hr",
                "auth,ai",
                "auth,websocket",
                "docs,communication",
                "docs,hr",
                "docs,ai",
                "docs,websocket",
                "communication,hr",
                "communication,ai",
                "communication,websocket",
                "hr,ai",
                "hr,websocket",
                "ai,websocket",
            },
        )
        self.assertEqual(len(observed), 22)
        self.assertEqual(report["status"], "passed")
        self.assertEqual(report["passed"], 22)

    def test_individual_executes_the_cargo_hack_plan_without_rewriting_it(self):
        result = self.run_phase("individual", FAKE_CARGO_MODE="success")
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        report = json.loads(self.report.read_text())
        observed = executed_arguments(result)
        baseline = [
            "test",
            "--lib",
            "--manifest-path",
            "Cargo.toml",
            "--no-default-features",
        ]
        self.assertEqual(
            observed,
            [
                baseline,
                baseline + ["--features", "default"],
                baseline + ["--features", "docs"],
            ],
        )
        self.assertEqual(observed, [item["argv"][1:] for item in report["commands"]])
        self.assertEqual(report["status"], "passed")
        self.assertEqual(report["passed"], 3)

    def test_time_budget_exhaustion_fails_both_workflow_steps(self):
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(
                    phase,
                    FAKE_CARGO_MODE="timeout",
                    FEATURE_MATRIX_TIMEOUT_SECONDS="0.4",
                )
                self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
                report = json.loads(self.report.read_text())
                self.assertEqual(report["status"], "failed")
                self.assertEqual(report["commands"][0]["status"], "timed_out")
                self.assertEqual(report["passed"], 0)
                self.assertEqual(report["not_run"], report["planned"] - 1)

    def test_mid_plan_failure_preserves_unrun_items(self):
        for phase, planned in (("individual", 3), ("core", 22)):
            with self.subTest(phase=phase):
                result = self.run_phase(phase, FAKE_CARGO_MODE="fail_docs")
                self.assertEqual(result.returncode, 23, result.stdout + result.stderr)
                report = json.loads(self.report.read_text())
                self.assertEqual(
                    (
                        report["planned"],
                        report["completed"],
                        report["passed"],
                        report["failed"],
                        report["not_run"],
                    ),
                    (planned, 3, 2, 1, planned - 3),
                )
                self.assertEqual(report["status"], "failed")

    def test_invalid_or_empty_individual_plan_never_executes_a_partial_plan(self):
        for mode in ("invalid", "empty", "failure", "timeout"):
            with self.subTest(mode=mode):
                result = self.run_phase(
                    "individual",
                    FAKE_PLAN_MODE=mode,
                    FEATURE_MATRIX_TIMEOUT_SECONDS="0.4",
                )
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn("EXEC ", result.stdout)
                report = json.loads(self.report.read_text())
                self.assertEqual(report["status"], "failed")
                self.assertIn("error", report)

    def test_budget_is_shared_by_all_commands(self):
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(
                    phase,
                    FAKE_CARGO_MODE="cumulative_timeout",
                    FEATURE_MATRIX_TIMEOUT_SECONDS="0.5",
                )
                self.assertNotEqual(result.returncode, 0)
                report = json.loads(self.report.read_text())
                self.assertGreater(report["passed"], 0)
                self.assertEqual(report["status"], "failed")
                self.assertLess(report["passed"], report["planned"])

    def test_signal_termination_is_not_success(self):
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(phase, FAKE_CARGO_MODE="signal")
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(
                    json.loads(self.report.read_text())["status"], "failed"
                )

    def test_report_output_cannot_hide_test_failure(self):
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(phase, GITHUB_STEP_SUMMARY=str(self.directory))
                self.assertEqual(result.returncode, 23, result.stdout + result.stderr)

    def test_missing_executable_is_not_success(self):
        (self.directory / "cargo").unlink()
        (self.directory / "bash").symlink_to(shutil.which("bash"))
        (self.directory / "python3").symlink_to(sys.executable)
        for phase in STEPS:
            with self.subTest(phase=phase):
                result = self.run_phase(phase, PATH=str(self.directory))
                self.assertNotEqual(result.returncode, 0)
                report = json.loads(self.report.read_text())
                self.assertEqual(report["status"], "failed")
                self.assertIn("error", report)

    def test_workflow_runs_regressions_and_preserves_both_phase_reports(self):
        workflow = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn(
            "python3 -m unittest tools.tests.test_feature_matrix -v", workflow
        )
        self.assertIn("needs: feature-matrix-regression", workflow)
        self.assertIn("phase: [individual, core]", workflow)
        self.assertIn("name: feature-matrix-${{ matrix.phase }}", workflow)
        self.assertIn("if-no-files-found: error", workflow)


if __name__ == "__main__":
    unittest.main()
