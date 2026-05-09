"""Shared data models for API contract validation."""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any


@dataclass(frozen=True)
class ApiIdentity:
    api_id: str
    name: str
    biz_tag: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name: str
    url: str
    doc_path: str
    expected_file: str
    full_path: str = ""

    @property
    def official_method(self) -> str:
        method, _, _ = self.url.partition(":")
        return method.upper()

    @property
    def official_path(self) -> str:
        _, _, path = self.url.partition(":")
        return path


@dataclass(frozen=True)
class RustEndpointCall:
    method: str
    argument: str
    line: int
    resolved_path: str = ""
    source: str = ""
    unresolved_reason: str = ""

    @property
    def is_resolved(self) -> bool:
        return bool(self.resolved_path)


@dataclass(frozen=True)
class RustApiContract:
    rel_path: str
    endpoint_calls: tuple[RustEndpointCall, ...] = ()
    fields: tuple["RustField", ...] = ()
    response_fields: tuple["RustField", ...] = ()


@dataclass(frozen=True)
class OfficialField:
    name: str
    required: bool
    location: str
    field_type: str = ""
    source: str = ""


@dataclass(frozen=True)
class RustField:
    struct_name: str
    field_name: str
    serialized_name: str
    type_name: str
    optional: bool
    line: int


@dataclass(frozen=True)
class ContractFinding:
    severity: str
    code: str
    message: str
    api_id: str
    api_name: str
    expected_file: str
    doc_path: str
    rust_line: int = 0
    official: str = ""
    rust: str = ""

    def sort_key(self) -> tuple[int, str, str, int, str]:
        severity_rank = {"ERROR": 0, "WARN": 1, "INFO": 2, "UNVERIFIED": 3}
        return (
            severity_rank.get(self.severity, 9),
            self.expected_file,
            self.code,
            self.rust_line,
            self.api_id,
        )


@dataclass
class ContractReport:
    crate_name: str
    total_apis: int = 0
    checked_apis: int = 0
    findings: list[ContractFinding] = field(default_factory=list)

    def add(self, finding: ContractFinding) -> None:
        self.findings.append(finding)

    @property
    def error_count(self) -> int:
        return sum(1 for finding in self.findings if finding.severity == "ERROR")

    @property
    def warn_count(self) -> int:
        return sum(1 for finding in self.findings if finding.severity == "WARN")

    @property
    def unresolved_count(self) -> int:
        return sum(1 for finding in self.findings if finding.severity == "UNVERIFIED")

    def to_jsonable(self) -> dict[str, Any]:
        return {
            "crate": self.crate_name,
            "total_apis": self.total_apis,
            "checked_apis": self.checked_apis,
            "error_count": self.error_count,
            "warn_count": self.warn_count,
            "unresolved_count": self.unresolved_count,
            "findings": [
                asdict(finding)
                for finding in sorted(self.findings, key=lambda item: item.sort_key())
            ],
        }
