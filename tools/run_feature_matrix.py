#!/usr/bin/env python3
"""逐项执行 feature 测试计划，只将真实成功退出的命令计为通过。"""

import argparse
import itertools
import json
import math
import os
import shlex
import signal
import subprocess
import sys
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CORE_FEATURES = ("auth", "docs", "communication", "hr", "ai", "websocket")


def core_plan():
    commands = []
    for depth in range(3):
        for features in itertools.combinations(CORE_FEATURES, depth):
            command = [
                "cargo",
                "test",
                "--package",
                "openlark",
                "--lib",
                "--no-default-features",
            ]
            if features:
                command += ["--features", ",".join(features)]
            commands.append({"argv": command, "status": "pending"})
    return commands


def execute(command, deadline, capture=False):
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise subprocess.TimeoutExpired(command, 0)
    process = subprocess.Popen(
        command,
        cwd=ROOT,
        start_new_session=True,
        stdout=subprocess.PIPE if capture else None,
        text=True,
    )
    try:
        output, _ = process.communicate(timeout=remaining)
    except BaseException:
        # timeout/取消必须同时结束 cargo 启动的 rustc 等后代进程。
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.communicate()
        raise
    return process.returncode, output


def individual_plan(deadline):
    # 执行 cargo-hack 输出的同一份命令，避免另一次枚举改变覆盖集合。
    planner = [
        "cargo",
        "hack",
        "test",
        "--package",
        "openlark",
        "--lib",
        "--each-feature",
        "--exclude-features",
        "websocket",
        "--print-command-list",
    ]
    result, output = execute(planner, deadline, capture=True)
    if result:
        raise subprocess.CalledProcessError(result, planner, output=output)
    features = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))[
        "features"
    ]
    prefix = ["cargo", "test", "--lib", "--manifest-path", "Cargo.toml"]
    commands = []
    for line in output.splitlines():
        command = shlex.split(line)
        tail = command[len(prefix) :]
        valid = tail in (["--no-default-features"], ["--all-features"])
        if len(tail) == 3 and tail[:2] == ["--no-default-features", "--features"]:
            valid = tail[2] in features and tail[2] != "websocket"
        if command[: len(prefix)] != prefix or not valid:
            raise ValueError(f"无法识别 cargo-hack 计划命令：{line!r}")
        commands.append({"argv": command, "status": "pending"})
    if not commands:
        raise ValueError("cargo-hack 返回了空计划")
    return commands


def save_report(path, report):
    commands = report["commands"]
    report.update(
        planned=len(commands),
        passed=sum(item["status"] == "passed" for item in commands),
        failed=sum(
            item["status"] in ("failed", "timed_out", "cancelled") for item in commands
        ),
        not_run=sum(item["status"] == "pending" for item in commands),
        completed=sum("returncode" in item for item in commands),
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(
        json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    temporary.replace(path)


def write_summary(report):
    summary = (
        f"### Feature matrix: {report['phase']} — {report['status']}\n\n"
        f"Planned: {report['planned']}; completed: {report['completed']}; "
        f"passed: {report['passed']}; failed: {report['failed']}; not run: {report['not_run']}.\n\n"
        f"Elapsed: {report['elapsed_seconds']:.2f}s / {report['timeout_seconds']}s budget.\n"
    )
    print(summary, flush=True)
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with Path(os.environ["GITHUB_STEP_SUMMARY"]).open(
            "a", encoding="utf-8"
        ) as handle:
            handle.write(summary)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("phase", choices=("individual", "core"))
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--timeout-seconds", type=float, default=1200)
    args = parser.parse_args()
    if not math.isfinite(args.timeout_seconds) or args.timeout_seconds <= 0:
        parser.error("--timeout-seconds 必须是正的有限数值")
    started = time.monotonic()
    deadline = started + args.timeout_seconds
    report = {
        "phase": args.phase,
        "package": "openlark",
        "status": "running",
        "commands": [],
        "timeout_seconds": args.timeout_seconds,
        "revision": os.environ.get("GITHUB_SHA"),
        "run_id": os.environ.get("GITHUB_RUN_ID"),
    }
    result = 0
    current = None
    try:
        save_report(args.report, report)
        report["commands"] = (
            core_plan() if args.phase == "core" else individual_plan(deadline)
        )
        save_report(args.report, report)
        for item in report["commands"]:
            print("PLAN " + shlex.join(item["argv"]), flush=True)
        for index, current in enumerate(report["commands"], 1):
            print(
                f"START {index}/{report['planned']} " + shlex.join(current["argv"]),
                flush=True,
            )
            current.update(
                status="running", started_after_seconds=time.monotonic() - started
            )
            save_report(args.report, report)
            result, _ = execute(current["argv"], deadline)
            current.update(
                returncode=result, status="passed" if result == 0 else "failed"
            )
            save_report(args.report, report)
            print(
                f"{current['status']} {index}/{report['planned']} (exit {result})",
                flush=True,
            )
            if result:
                break
    except subprocess.TimeoutExpired:
        result = 124
        report["error"] = "feature matrix 总时间预算耗尽"
        if current:
            current.update(status="timed_out", returncode=result)
    except KeyboardInterrupt:
        result = 130
        report["error"] = "feature matrix 被取消"
        if current:
            current.update(status="cancelled", returncode=result)
    except subprocess.CalledProcessError as error:
        result = error.returncode
        report["error"] = f"cargo-hack 计划生成失败：{error}"
    except (OSError, ValueError) as error:
        result = 1
        report["error"] = str(error)
        if current:
            current.update(status="failed", returncode=result)
    report.update(
        status="passed" if result == 0 else "failed",
        elapsed_seconds=time.monotonic() - started,
    )
    try:
        save_report(args.report, report)
        write_summary(report)
    except OSError as error:
        print(f"无法写入 feature matrix 报告：{error}", file=sys.stderr)
        result = result or 1
    return 128 - result if result < 0 else result


def cancel(_signum, _frame):
    raise KeyboardInterrupt


if __name__ == "__main__":
    signal.signal(signal.SIGTERM, cancel)
    raise SystemExit(main())
