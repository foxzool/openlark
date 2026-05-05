"""Report rendering for API contract validation."""

from __future__ import annotations

import json
from pathlib import Path

from .models import ContractFinding, ContractReport


def write_report(report: ContractReport, markdown_path: Path, json_path: Path) -> None:
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text(render_markdown(report), encoding="utf-8")
    json_path.write_text(
        json.dumps(report.to_jsonable(), ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def render_markdown(report: ContractReport) -> str:
    lines = [
        f"# API Contract Validation: {report.crate_name}",
        "",
        "## Summary",
        "",
        "| Metric | Count |",
        "|---|---:|",
        f"| API rows | {report.total_apis} |",
        f"| Checked API files | {report.checked_apis} |",
        f"| Errors | {report.error_count} |",
        f"| Warnings | {report.warn_count} |",
        f"| Unverified | {report.unresolved_count} |",
        "",
    ]

    findings = sorted(report.findings, key=lambda item: item.sort_key())
    if not findings:
        lines.extend(["No contract findings.", ""])
        return "\n".join(lines)

    lines.extend(
        [
            "## Findings",
            "",
            "| Severity | Code | API | File | Line | Official | Rust |",
            "|---|---|---|---|---:|---|---|",
        ]
    )
    for item in findings:
        lines.append(finding_row(item))
    lines.append("")
    return "\n".join(lines)

def finding_row(finding: ContractFinding) -> str:
    api = markdown_link(finding.api_name or finding.api_id, finding.doc_path)
    line = str(finding.rust_line) if finding.rust_line else ""
    return (
        f"| {escape(finding.severity)} | {escape(finding.code)} | {api} | "
        f"`{escape(finding.expected_file)}` | {line} | "
        f"`{escape(finding.official)}` | `{escape(finding.rust)}` |"
    )


def markdown_link(label: str, url: str) -> str:
    if url:
        return f"[{escape(label)}]({url})"
    return escape(label)


def escape(value: str) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ").strip()


def write_summary(reports: list[ContractReport], markdown_path: Path, json_path: Path) -> None:
    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.parent.mkdir(parents=True, exist_ok=True)
    markdown_path.write_text(render_summary_markdown(reports), encoding="utf-8")
    json_path.write_text(
        json.dumps(
            {
                "crates": [report.to_jsonable() for report in reports],
                "total_apis": sum(report.total_apis for report in reports),
                "checked_apis": sum(report.checked_apis for report in reports),
                "error_count": sum(report.error_count for report in reports),
                "warn_count": sum(report.warn_count for report in reports),
                "unresolved_count": sum(report.unresolved_count for report in reports),
            },
            ensure_ascii=False,
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )


def render_summary_markdown(reports: list[ContractReport]) -> str:
    lines = [
        "# API Contract Validation Summary",
        "",
        "| Crate | API rows | Checked | Errors | Warnings | Unverified |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for report in sorted(reports, key=lambda item: item.crate_name):
        report_link = f"crates/{report.crate_name}.md"
        lines.append(
            f"| [{escape(report.crate_name)}]({report_link}) | {report.total_apis} | "
            f"{report.checked_apis} | {report.error_count} | {report.warn_count} | {report.unresolved_count} |"
        )
    lines.append("")
    return "\n".join(lines)
