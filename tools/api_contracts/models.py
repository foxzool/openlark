"""Shared data models for API contract validation."""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any


_HTTP_METHODS = {"GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "TRACE"}

# ApiRequest 未显式声明 supported_access_token_types 时的默认取值
# （见 crates/openlark-core/src/api/mod.rs: supported_access_token_types() 的默认分支）。
# 使用飞书凭证名，以便与官方文档 security.supportedAccessToken 直接比对。
DEFAULT_ACCESS_TOKEN_TYPES: tuple[str, ...] = ("user_access_token", "tenant_access_token")


def _split_official_url(value: str) -> tuple[str, str]:
    method, _, path = value.partition(":")
    method = method.strip().upper()
    if method in _HTTP_METHODS and path:
        return method, path.strip()
    return "", value.strip()


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
        method, path = _split_official_url(self.url)
        if method:
            return method
        method, _ = _split_official_url(self.full_path)
        return method

    @property
    def official_path(self) -> str:
        _, path = _split_official_url(self.url)
        if path:
            return path
        _, path = _split_official_url(self.full_path)
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
    # request struct 含 #[serde(flatten)] 字段时，视为该 API 透传所有官方 optional
    # request 字段（透传 Value 或 typed 枚举，非 correctness bug）。
    has_flatten_value_passthrough: bool = False
    # Rust 侧声明的有效 token 类型（飞书凭证名）。未显式调用
    # .with_supported_access_token_types(...) 时取 DEFAULT_ACCESS_TOKEN_TYPES。
    access_token_types: tuple[str, ...] = DEFAULT_ACCESS_TOKEN_TYPES
    # 声明 None（自行管理鉴权，bypass token cache）但手动注入
    # ``Authorization: Bearer <self.token_field>`` 时，实际注入的 token 凭证名
    # （如 OIDC userinfo 注入 user_access_token）。compare 据此把 none_access_token
    # 替换为该类型核对，避免误报 disjoint ERROR。未检测到注入时为空串。
    manual_auth_token: str = ""


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
