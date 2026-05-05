#!/usr/bin/env python3
"""Validate OpenLark typed API endpoint contracts against the official API list."""

from __future__ import annotations

import argparse
import sys
from dataclasses import replace
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    tomllib = None

REPO_ROOT = Path(__file__).resolve().parents[1]
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.api_contracts.compare import (
    compare_endpoint,
    compare_request_fields,
    compare_response_fields,
    finding,
)
from tools.api_contracts.models import ContractReport
from tools.api_contracts.official import (
    extract_endpoint_from_detail_payload,
    extract_request_fields_from_detail_payload,
    extract_response_fields_from_detail_payload,
    fetch_detail_payload,
    load_api_identities,
)
from tools.api_contracts.report import write_report, write_summary
from tools.api_contracts.rust_source import load_endpoint_constants, scan_api_file


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate endpoint-level API contracts.")
    parser.add_argument("--csv", default="api_list_export.csv", help="Official API CSV path")
    parser.add_argument("--mapping", default="tools/api_coverage.toml", help="crate to bizTag mapping")
    parser.add_argument("--crate", dest="crate_name", help="Validate one mapped crate")
    parser.add_argument("--all-crates", action="store_true", help="Validate all mapped crates")
    parser.add_argument("--report-dir", default="reports/api_contracts", help="Report directory")
    parser.add_argument("--include-old", dest="skip_old", action="store_false", help="Include meta.Version=old APIs")
    parser.add_argument("--skip-old", dest="skip_old", action="store_true", default=True, help="Skip old APIs")
    parser.add_argument(
        "--live-endpoints",
        action="store_true",
        help="Fetch each official detail page and compare endpoints against the current official schema",
    )
    parser.add_argument(
        "--fields",
        action="store_true",
        help="Compare Rust request body fields against official request body fields",
    )
    parser.add_argument(
        "--live-fields",
        action="store_true",
        help="Fetch official detail pages for request body field validation",
    )
    parser.add_argument("--field-timeout", type=int, default=20, help="Official detail fetch timeout in seconds")
    parser.add_argument("--field-retries", type=int, default=1, help="Official detail fetch retries")
    parser.add_argument(
        "--max-field-apis",
        type=int,
        default=0,
        help="Limit live field validation to the first N APIs in each crate; 0 means no limit",
    )
    parser.add_argument(
        "--strict",
        default="",
        help="Comma-separated strict categories. Supported values: endpoint, fields",
    )
    return parser.parse_args()


def load_mapping(path: Path) -> dict[str, dict]:
    if tomllib is None:
        raise SystemExit("Python 3.11+ is required for tomllib")
    if not path.exists():
        raise SystemExit(f"Mapping file does not exist: {path}")
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    crates = data.get("crates", {})
    if not isinstance(crates, dict) or not crates:
        raise SystemExit(f"Mapping file lacks [crates.*] entries: {path}")
    return crates


def validate_crate(
    crate_name: str,
    crate_config: dict,
    csv_path: Path,
    report_dir: Path,
    skip_old: bool,
    live_endpoints: bool = False,
    fields: bool = False,
    field_timeout: int = 20,
    field_retries: int = 1,
    max_field_apis: int = 0,
) -> ContractReport:
    src_path = Path(crate_config["src"])
    biz_tags = list(crate_config.get("biz_tags") or [])
    apis = load_api_identities(csv_path, filter_tags=biz_tags, skip_old_versions=skip_old)
    constants = load_endpoint_constants(src_path)
    report = ContractReport(crate_name=crate_name, total_apis=len(apis))

    field_checks = 0
    for api in apis:
        rust_contract = scan_api_file(src_path, api.expected_file, constants)
        if rust_contract is not None:
            report.checked_apis += 1
        detail_payload = None
        endpoint_api = api
        should_check_fields = fields and (not max_field_apis or field_checks < max_field_apis)
        if live_endpoints or should_check_fields:
            try:
                detail_payload = fetch_detail_payload(api, timeout=field_timeout, retries=field_retries)
            except Exception as exc:  # noqa: BLE001 - report and keep validating the remaining APIs.
                report.add(
                    finding(
                        "UNVERIFIED",
                        "U_OFFICIAL_DETAIL_FETCH_FAILED",
                        "Official detail payload could not be fetched.",
                        api,
                        official=str(exc),
                    )
                )
            if live_endpoints and detail_payload is not None:
                method, path = extract_endpoint_from_detail_payload(detail_payload)
                if method and path:
                    endpoint_api = replace(api, url=f"{method}:{path}")
                else:
                    report.add(
                        finding(
                            "UNVERIFIED",
                            "U_LIVE_OFFICIAL_ENDPOINT_UNAVAILABLE",
                            "Current official schema did not expose httpMethod/path.",
                            api,
                        )
                    )

        for item in compare_endpoint(endpoint_api, rust_contract):
            report.add(item)

        if should_check_fields and detail_payload is not None:
            field_checks += 1
            request_fields = extract_request_fields_from_detail_payload(detail_payload)
            for item in compare_request_fields(api, request_fields, rust_contract):
                report.add(item)
            response_fields = extract_response_fields_from_detail_payload(detail_payload)
            for item in compare_response_fields(api, response_fields, rust_contract):
                report.add(item)

    write_report(
        report,
        report_dir / "crates" / f"{crate_name}.md",
        report_dir / "crates" / f"{crate_name}.json",
    )
    return report


def main() -> int:
    args = parse_args()
    if args.fields and not args.live_fields:
        print("Field validation requires --live-fields so official fields come from current docs.", file=sys.stderr)
        return 1
    csv_path = Path(args.csv)
    if not csv_path.exists():
        print(f"CSV file does not exist: {csv_path}", file=sys.stderr)
        return 1

    mapping = load_mapping(Path(args.mapping))
    if not args.all_crates and not args.crate_name:
        print("Specify --crate <name> or --all-crates", file=sys.stderr)
        return 1

    if args.crate_name:
        if args.crate_name not in mapping:
            print(f"Unknown crate in mapping: {args.crate_name}", file=sys.stderr)
            return 1
        crate_names = [args.crate_name]
    else:
        crate_names = sorted(mapping.keys())

    report_dir = Path(args.report_dir)
    reports = [
        validate_crate(
            crate_name,
            mapping[crate_name],
            csv_path,
            report_dir,
            args.skip_old,
            live_endpoints=args.live_endpoints,
            fields=args.fields,
            field_timeout=args.field_timeout,
            field_retries=args.field_retries,
            max_field_apis=args.max_field_apis,
        )
        for crate_name in crate_names
    ]
    write_summary(reports, report_dir / "summary.md", report_dir / "summary.json")

    total_errors = sum(report.error_count for report in reports)
    total_warnings = sum(report.warn_count for report in reports)
    print(
        "API contract validation complete: "
        f"{len(reports)} crate(s), {total_errors} error(s), {total_warnings} warning(s); "
        f"reports in {report_dir}"
    )

    strict_categories = {item.strip() for item in args.strict.split(",") if item.strip()}
    if ("endpoint" in strict_categories or "fields" in strict_categories) and total_errors:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
