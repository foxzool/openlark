"""从版本化 Recorded Snapshot 收集官方文档证据。"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from typing import Any, Iterable

from ..models import ApiIdentity


__all__ = [
    "AcquisitionAttempt",
    "DimensionEvidence",
    "EndpointObservation",
    "EvidenceDiagnostic",
    "EvidenceDimension",
    "EvidenceError",
    "EvidenceSource",
    "EvidenceStatus",
    "FieldObservation",
    "InterpretationProvenance",
    "InterpreterError",
    "InvalidEvidenceRequest",
    "OfficialDocumentEvidence",
    "RecordedOnlyPolicy",
    "RecordedSnapshot",
    "SnapshotInvariantError",
    "SnapshotProvenance",
    "TokenObservation",
    "collect",
]

_INTERPRETER_REVISION = "official-evidence/1"


class EvidenceDimension(str, Enum):
    ENDPOINT = "endpoint"
    REQUEST_FIELDS = "request_fields"
    RESPONSE_FIELDS = "response_fields"
    TOKENS = "tokens"


class EvidenceSource(str, Enum):
    STRUCTURED_DETAIL = "structured_detail"
    RENDERED_DOCUMENT = "rendered_document"


class EvidenceStatus(str, Enum):
    TRUSTED = "trusted"
    INCOMPLETE = "incomplete"
    UNAVAILABLE = "unavailable"
    REJECTED = "rejected"


class EvidenceError(RuntimeError):
    """官方文档证据收集不可降级的错误。"""


class InvalidEvidenceRequest(EvidenceError):
    """Evidence Request 不满足 collect interface。"""


class SnapshotInvariantError(EvidenceError):
    """Recorded Snapshot 违反不可变性或 provenance invariant。"""


class InterpreterError(EvidenceError):
    """解释器发生非预期缺陷。"""


@dataclass(frozen=True)
class EvidenceDiagnostic:
    code: str
    message: str


@dataclass(frozen=True)
class SnapshotProvenance:
    catalog_entry: ApiIdentity
    source: EvidenceSource
    acquired_at: str
    source_uri: str
    content_digest: str
    snapshot_version: int


@dataclass(frozen=True)
class InterpretationProvenance:
    snapshot_digest: str
    interpreter_revision: str
    dimension: EvidenceDimension


@dataclass(frozen=True)
class EndpointObservation:
    method: str
    path: str
    source: EvidenceSource


@dataclass(frozen=True)
class FieldObservation:
    path: tuple[str, ...]
    location: str | None
    required: bool | None
    field_type: str | None
    source: EvidenceSource

    @property
    def canonical_path(self) -> str:
        return ".".join(self.path)


@dataclass(frozen=True)
class TokenObservation:
    token: str
    source: EvidenceSource


EvidenceObservation = EndpointObservation | FieldObservation | TokenObservation


@dataclass(frozen=True)
class AcquisitionAttempt:
    source: EvidenceSource
    status: EvidenceStatus
    diagnostics: tuple[EvidenceDiagnostic, ...]
    snapshot_provenance: SnapshotProvenance | None


@dataclass(frozen=True)
class DimensionEvidence:
    dimension: EvidenceDimension
    status: EvidenceStatus
    selected_source: EvidenceSource | None
    observations: tuple[EvidenceObservation, ...]
    snapshot_provenance: SnapshotProvenance | None
    interpretation_provenance: InterpretationProvenance | None
    diagnostics: tuple[EvidenceDiagnostic, ...]
    acquisition_trail: tuple[AcquisitionAttempt, ...]


@dataclass(frozen=True)
class OfficialDocumentEvidence:
    catalog_entry: ApiIdentity
    dimensions: tuple[DimensionEvidence, ...]

    def for_dimension(self, dimension: EvidenceDimension) -> DimensionEvidence:
        for evidence in self.dimensions:
            if evidence.dimension is dimension:
                return evidence
        raise KeyError(dimension)


@dataclass(frozen=True)
class RecordedSnapshot:
    version: int
    source_kind: EvidenceSource
    catalog_entry: ApiIdentity
    acquired_at: str
    source_uri: str
    content_digest: str
    raw_representation: str

    @classmethod
    def structured(
        cls,
        *,
        version: int,
        catalog_entry: ApiIdentity,
        acquired_at: str,
        source_uri: str,
        payload: dict[str, Any],
    ) -> RecordedSnapshot:
        raw = json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
        return cls._create(
            version=version,
            source_kind=EvidenceSource.STRUCTURED_DETAIL,
            catalog_entry=catalog_entry,
            acquired_at=acquired_at,
            source_uri=source_uri,
            raw_representation=raw,
        )

    @classmethod
    def rendered(
        cls,
        *,
        version: int,
        catalog_entry: ApiIdentity,
        acquired_at: str,
        source_uri: str,
        content: str,
    ) -> RecordedSnapshot:
        return cls._create(
            version=version,
            source_kind=EvidenceSource.RENDERED_DOCUMENT,
            catalog_entry=catalog_entry,
            acquired_at=acquired_at,
            source_uri=source_uri,
            raw_representation=content,
        )

    @classmethod
    def _create(
        cls,
        *,
        version: int,
        source_kind: EvidenceSource,
        catalog_entry: ApiIdentity,
        acquired_at: str,
        source_uri: str,
        raw_representation: str,
    ) -> RecordedSnapshot:
        digest = hashlib.sha256(raw_representation.encode("utf-8")).hexdigest()
        return cls(
            version=version,
            source_kind=source_kind,
            catalog_entry=catalog_entry,
            acquired_at=acquired_at,
            source_uri=source_uri,
            content_digest=digest,
            raw_representation=raw_representation,
        )

    @property
    def provenance(self) -> SnapshotProvenance:
        return SnapshotProvenance(
            catalog_entry=self.catalog_entry,
            source=self.source_kind,
            acquired_at=self.acquired_at,
            source_uri=self.source_uri,
            content_digest=self.content_digest,
            snapshot_version=self.version,
        )


@dataclass(frozen=True)
class RecordedOnlyPolicy:
    snapshots: tuple[RecordedSnapshot, ...]


@dataclass(frozen=True)
class _Candidate:
    status: EvidenceStatus
    source: EvidenceSource
    observations: tuple[EvidenceObservation, ...]
    diagnostics: tuple[EvidenceDiagnostic, ...]
    snapshot: RecordedSnapshot | None


def collect(
    catalog_entry: ApiIdentity,
    dimensions: Iterable[EvidenceDimension],
    policy: RecordedOnlyPolicy,
) -> OfficialDocumentEvidence:
    """只通过 Recorded Snapshots 为一个 Catalog Entry 收集所需维度。"""
    try:
        requested = tuple(dimensions)
    except TypeError as error:
        raise InvalidEvidenceRequest("Evidence Dimensions 必须可迭代") from error
    _validate_request(catalog_entry, requested, policy)
    snapshots = {snapshot.source_kind: snapshot for snapshot in policy.snapshots}
    results = tuple(
        _collect_dimension(dimension, snapshots)
        for dimension in requested
    )
    return OfficialDocumentEvidence(catalog_entry=catalog_entry, dimensions=results)


def _validate_request(
    catalog_entry: ApiIdentity,
    dimensions: tuple[EvidenceDimension, ...],
    policy: RecordedOnlyPolicy,
) -> None:
    if not isinstance(catalog_entry, ApiIdentity):
        raise InvalidEvidenceRequest("catalog_entry 必须是 ApiIdentity")
    if not isinstance(policy, RecordedOnlyPolicy):
        raise InvalidEvidenceRequest("仅支持 recorded-only acquisition policy")
    if not dimensions or any(not isinstance(item, EvidenceDimension) for item in dimensions):
        raise InvalidEvidenceRequest("至少请求一个有效 Evidence Dimension")
    if len(set(dimensions)) != len(dimensions):
        raise InvalidEvidenceRequest("Evidence Dimensions 不得重复")
    if not isinstance(policy.snapshots, tuple):
        raise SnapshotInvariantError("Recorded-only policy 必须保存不可变 snapshot 序列")
    if any(not isinstance(snapshot, RecordedSnapshot) for snapshot in policy.snapshots):
        raise SnapshotInvariantError("Recorded-only policy 包含无效 snapshot")
    if any(snapshot.catalog_entry != catalog_entry for snapshot in policy.snapshots):
        raise InvalidEvidenceRequest("Recorded Snapshot 与 Catalog Entry 不匹配")
    sources = [snapshot.source_kind for snapshot in policy.snapshots]
    if any(not isinstance(source, EvidenceSource) for source in sources):
        raise SnapshotInvariantError("Recorded Snapshot source kind 无效")
    if len(set(sources)) != len(sources):
        raise InvalidEvidenceRequest("每种 Official Evidence Source 最多一个 Recorded Snapshot")
    for snapshot in policy.snapshots:
        if snapshot.version != 1:
            raise SnapshotInvariantError("不支持的 Recorded Snapshot version")
        if not isinstance(snapshot.raw_representation, str):
            raise SnapshotInvariantError("Recorded Snapshot 原始表示必须是字符串")
        if not isinstance(snapshot.acquired_at, str):
            raise SnapshotInvariantError("Recorded Snapshot acquisition time 无效")
        try:
            acquired_at = datetime.fromisoformat(snapshot.acquired_at.replace("Z", "+00:00"))
        except ValueError as error:
            raise SnapshotInvariantError("Recorded Snapshot acquisition time 无效") from error
        if acquired_at.tzinfo is None:
            raise SnapshotInvariantError("Recorded Snapshot acquisition time 必须包含时区")
        if not isinstance(snapshot.source_uri, str) or not snapshot.source_uri.strip():
            raise SnapshotInvariantError("Recorded Snapshot source URI 不能为空")
        digest = hashlib.sha256(snapshot.raw_representation.encode("utf-8")).hexdigest()
        if digest != snapshot.content_digest:
            raise SnapshotInvariantError("Recorded Snapshot content digest 不匹配")


def _collect_dimension(
    dimension: EvidenceDimension,
    snapshots: dict[EvidenceSource, RecordedSnapshot],
) -> DimensionEvidence:
    structured_snapshot = snapshots.get(EvidenceSource.STRUCTURED_DETAIL)
    structured = (
        _interpret_snapshot(structured_snapshot, dimension)
        if structured_snapshot is not None
        else _unavailable(EvidenceSource.STRUCTURED_DETAIL)
    )
    candidates = [structured]
    if structured.status is not EvidenceStatus.TRUSTED:
        rendered_snapshot = snapshots.get(EvidenceSource.RENDERED_DOCUMENT)
        rendered = (
            _interpret_snapshot(rendered_snapshot, dimension)
            if rendered_snapshot is not None
            else _unavailable(EvidenceSource.RENDERED_DOCUMENT)
        )
        candidates.append(rendered)
    selected = max(candidates, key=_selection_key)
    trail = tuple(_attempt(candidate) for candidate in candidates)
    return _to_evidence(dimension, selected, trail)


def _interpret_snapshot(
    snapshot: RecordedSnapshot,
    dimension: EvidenceDimension,
) -> _Candidate:
    try:
        if snapshot.source_kind is EvidenceSource.STRUCTURED_DETAIL:
            return _interpret_structured(snapshot, dimension)
        if snapshot.source_kind is EvidenceSource.RENDERED_DOCUMENT:
            return _interpret_rendered(snapshot, dimension)
        raise SnapshotInvariantError("Recorded Snapshot source kind 无效")
    except EvidenceError:
        raise
    except Exception as error:
        raise InterpreterError(
            f"{dimension.value} Evidence 解释器发生非预期错误"
        ) from error


def _selection_key(candidate: _Candidate) -> tuple[int, int]:
    status_rank = {
        EvidenceStatus.UNAVAILABLE: 0,
        EvidenceStatus.REJECTED: 1,
        EvidenceStatus.INCOMPLETE: 2,
        EvidenceStatus.TRUSTED: 3,
    }
    source_rank = int(candidate.source is EvidenceSource.STRUCTURED_DETAIL)
    return status_rank[candidate.status], source_rank


def _interpret_structured(
    snapshot: RecordedSnapshot,
    dimension: EvidenceDimension,
) -> _Candidate:
    try:
        payload = json.loads(snapshot.raw_representation)
    except json.JSONDecodeError:
        return _rejected(snapshot, "document_unhealthy")
    if not isinstance(payload, dict):
        return _rejected(snapshot, "document_unhealthy")
    data = payload.get("data")
    schema = data.get("schema") if isinstance(data, dict) else None
    api_schema = schema.get("apiSchema") if isinstance(schema, dict) else None
    if not isinstance(api_schema, dict):
        return _rejected(snapshot, "document_unhealthy")
    if dimension is EvidenceDimension.ENDPOINT:
        return _interpret_structured_endpoint(snapshot, api_schema)
    if dimension is EvidenceDimension.REQUEST_FIELDS:
        return _interpret_structured_fields(
            snapshot,
            api_schema,
            dimension,
        )
    if dimension is EvidenceDimension.RESPONSE_FIELDS:
        return _interpret_structured_fields(
            snapshot,
            api_schema,
            dimension,
        )
    if dimension is EvidenceDimension.TOKENS:
        return _interpret_structured_tokens(snapshot, api_schema)
    raise InterpreterError(f"未处理的 Evidence Dimension: {dimension}")


def _interpret_structured_endpoint(
    snapshot: RecordedSnapshot,
    api_schema: dict[str, Any],
) -> _Candidate:
    method = api_schema.get("httpMethod")
    path = api_schema.get("path")
    if not isinstance(method, str) or not isinstance(path, str) or not method or not path:
        return _incomplete(snapshot, "structure_incomplete")
    return _trusted(
        snapshot,
        (
            EndpointObservation(
                method=method.strip().upper(),
                path=_normalize_endpoint(path),
                source=snapshot.source_kind,
            ),
        ),
    )


def _interpret_structured_fields(
    snapshot: RecordedSnapshot,
    api_schema: dict[str, Any],
    dimension: EvidenceDimension,
) -> _Candidate:
    schemas: list[dict[str, Any]] = []
    complete = True
    if dimension is EvidenceDimension.REQUEST_FIELDS:
        body = api_schema.get("requestBody")
        if not isinstance(body, dict):
            return _incomplete(snapshot, "structure_incomplete")
        content = body.get("content")
    else:
        responses = api_schema.get("responses")
        if not isinstance(responses, dict):
            return _incomplete(snapshot, "structure_incomplete")
        response = responses.get("200", responses.get(200))
        if not isinstance(response, dict):
            return _incomplete(snapshot, "structure_incomplete")
        content = response.get("content")
    if not isinstance(content, dict):
        return _incomplete(snapshot, "structure_incomplete")
    for representation in content.values():
        if not isinstance(representation, dict):
            complete = False
            continue
        field_schema = representation.get("schema")
        if not isinstance(field_schema, dict):
            complete = False
            continue
        schemas.append(field_schema)
    if not schemas:
        return _incomplete(snapshot, "structure_incomplete")
    observations: dict[tuple[str, ...], FieldObservation] = {}
    location = (
        "request_body"
        if dimension is EvidenceDimension.REQUEST_FIELDS
        else "response_body"
    )
    for field_schema in schemas:
        parsed, branch_complete = _walk_schema_properties(
            field_schema,
            location=location,
            source=snapshot.source_kind,
        )
        complete = complete and branch_complete
        for observation in parsed:
            observations.setdefault(observation.path, observation)
    values = tuple(observations.values())
    if complete:
        return _trusted(snapshot, values)
    return _incomplete(snapshot, "structure_incomplete", values)


def _walk_schema_properties(
    schema: dict[str, Any],
    *,
    location: str,
    source: EvidenceSource,
    prefix: tuple[str, ...] = (),
) -> tuple[tuple[FieldObservation, ...], bool]:
    if "properties" not in schema:
        return (), False
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        return (), False
    required_value = schema.get("required")
    required_names = (
        {str(item) for item in required_value}
        if isinstance(required_value, list)
        else None
    )
    observations: list[FieldObservation] = []
    complete = True
    for raw_name, definition in properties.items():
        if not isinstance(raw_name, str) or not raw_name or not isinstance(definition, dict):
            complete = False
            continue
        path = (*prefix, raw_name)
        explicit_required = definition.get("required")
        if isinstance(explicit_required, bool):
            required = explicit_required
        elif required_names is not None:
            required = raw_name in required_names
        else:
            required = None
        raw_type = definition.get("type")
        field_type = raw_type if isinstance(raw_type, str) and raw_type else None
        raw_format = definition.get("format")
        if isinstance(raw_format, str) and raw_format:
            field_type = f"{field_type}:{raw_format}" if field_type else raw_format
        observations.append(
            FieldObservation(
                path=path,
                location=location,
                required=required,
                field_type=field_type,
                source=source,
            )
        )
        if "properties" in definition:
            nested, nested_complete = _walk_schema_properties(
                definition,
                location=location,
                source=source,
                prefix=path,
            )
            observations.extend(nested)
            complete = complete and nested_complete
    return tuple(observations), complete


def _interpret_structured_tokens(
    snapshot: RecordedSnapshot,
    api_schema: dict[str, Any],
) -> _Candidate:
    security = api_schema.get("security")
    if not isinstance(security, dict):
        return _incomplete(snapshot, "structure_incomplete")
    tokens = security.get("supportedAccessToken")
    if not isinstance(tokens, list):
        return _incomplete(snapshot, "structure_incomplete")
    observations = tuple(
        TokenObservation(token=token, source=snapshot.source_kind)
        for token in tokens
        if isinstance(token, str) and token
    )
    if len(observations) != len(tokens):
        return _incomplete(snapshot, "structure_incomplete", observations)
    return _trusted(snapshot, observations)


def _interpret_rendered(
    snapshot: RecordedSnapshot,
    dimension: EvidenceDimension,
) -> _Candidate:
    content = snapshot.raw_representation.strip()
    lowered = content.casefold()
    if not content or "documentation could not be found" in lowered or "404 not found" in lowered:
        return _rejected(snapshot, "document_unhealthy")
    heading = {
        EvidenceDimension.ENDPOINT: "Endpoint",
        EvidenceDimension.REQUEST_FIELDS: "Request Fields",
        EvidenceDimension.RESPONSE_FIELDS: "Response Fields",
        EvidenceDimension.TOKENS: "Tokens",
    }[dimension]
    section = _rendered_section(content, heading)
    if section is None:
        return _incomplete(snapshot, "structure_incomplete")
    if dimension is EvidenceDimension.ENDPOINT:
        match = re.search(
            r"(?im)^\s*(GET|POST|PATCH|PUT|DELETE|HEAD|OPTIONS)\s+(\S+)\s*$",
            section,
        )
        if match is None:
            return _incomplete(snapshot, "structure_incomplete")
        return _trusted(
            snapshot,
            (
                EndpointObservation(
                    method=match.group(1).upper(),
                    path=_normalize_endpoint(match.group(2)),
                    source=snapshot.source_kind,
                ),
            ),
        )
    if dimension is EvidenceDimension.TOKENS:
        stripped = section.strip()
        if stripped.casefold() in {"none", "none."}:
            return _trusted(snapshot, ())
        tokens = tuple(
            TokenObservation(token=match.group(1), source=snapshot.source_kind)
            for match in re.finditer(r"(?m)^\s*[-*]\s+([A-Za-z][A-Za-z0-9_]*)\s*$", section)
        )
        if not tokens:
            return _incomplete(snapshot, "structure_incomplete")
        return _trusted(snapshot, tokens)
    return _interpret_rendered_fields(snapshot, section, dimension)


def _rendered_section(content: str, heading: str) -> str | None:
    match = re.search(
        rf"(?ims)^##[ \t]+{re.escape(heading)}[ \t]*\r?\n(.*?)(?=^##[ \t]+|\Z)",
        content,
    )
    return match.group(1) if match else None


def _interpret_rendered_fields(
    snapshot: RecordedSnapshot,
    section: str,
    dimension: EvidenceDimension,
) -> _Candidate:
    if section.strip().casefold() in {"none", "none.", "no fields", "no fields."}:
        return _trusted(snapshot, ())
    observations: list[FieldObservation] = []
    complete = True
    expected_location = (
        "request_body"
        if dimension is EvidenceDimension.REQUEST_FIELDS
        else "response_body"
    )
    for line in section.splitlines():
        stripped = line.strip()
        if not stripped.startswith("|"):
            if stripped:
                complete = False
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if not cells or cells[0].casefold() == "path" or all(set(cell) <= {"-", ":"} for cell in cells):
            continue
        if len(cells) != 4 or not cells[0]:
            complete = False
            continue
        path = _canonical_field_path(cells[0])
        if path is None:
            complete = False
            continue
        location = None if cells[1].casefold() in {"", "unknown"} else cells[1]
        if location is not None and location != expected_location:
            complete = False
        required_text = cells[2].casefold()
        if required_text in {"yes", "true"}:
            required = True
        elif required_text in {"no", "false"}:
            required = False
        elif required_text in {"", "unknown"}:
            required = None
        else:
            complete = False
            required = None
        field_type = None if cells[3].casefold() in {"", "unknown"} else cells[3]
        observations.append(
            FieldObservation(
                path=path,
                location=location,
                required=required,
                field_type=field_type,
                source=snapshot.source_kind,
            )
        )
    values = tuple(observations)
    if not values:
        return _incomplete(snapshot, "structure_incomplete")
    if complete:
        return _trusted(snapshot, values)
    return _incomplete(snapshot, "structure_incomplete", values)


def _canonical_field_path(value: str) -> tuple[str, ...] | None:
    parts = tuple(part.strip() for part in value.split("."))
    if not parts or any(not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", part) for part in parts):
        return None
    return parts


def _normalize_endpoint(path: str) -> str:
    normalized = re.sub(r":([A-Za-z_][A-Za-z0-9_]*)", r"{\1}", path.strip())
    return normalized.rstrip("/") or "/"


def _diagnostic(code: str) -> EvidenceDiagnostic:
    messages = {
        "snapshot_unavailable": "未提供可用的 Recorded Snapshot",
        "document_unhealthy": "原始官方文档表示未通过健康检查",
        "structure_incomplete": "相关官方文档结构无法完整解释",
        "structure_unsupported": "相关官方文档结构暂不受解释器支持",
    }
    return EvidenceDiagnostic(code=code, message=messages[code])


def _unavailable(source: EvidenceSource) -> _Candidate:
    return _Candidate(
        status=EvidenceStatus.UNAVAILABLE,
        source=source,
        observations=(),
        diagnostics=(_diagnostic("snapshot_unavailable"),),
        snapshot=None,
    )


def _trusted(
    snapshot: RecordedSnapshot,
    observations: tuple[EvidenceObservation, ...],
) -> _Candidate:
    return _Candidate(
        status=EvidenceStatus.TRUSTED,
        source=snapshot.source_kind,
        observations=observations,
        diagnostics=(),
        snapshot=snapshot,
    )


def _rejected(snapshot: RecordedSnapshot, code: str) -> _Candidate:
    return _Candidate(
        status=EvidenceStatus.REJECTED,
        source=snapshot.source_kind,
        observations=(),
        diagnostics=(_diagnostic(code),),
        snapshot=snapshot,
    )


def _incomplete(
    snapshot: RecordedSnapshot,
    code: str,
    observations: tuple[EvidenceObservation, ...] = (),
) -> _Candidate:
    return _Candidate(
        status=EvidenceStatus.INCOMPLETE,
        source=snapshot.source_kind,
        observations=observations,
        diagnostics=(_diagnostic(code),),
        snapshot=snapshot,
    )


def _attempt(candidate: _Candidate) -> AcquisitionAttempt:
    return AcquisitionAttempt(
        source=candidate.source,
        status=candidate.status,
        diagnostics=candidate.diagnostics,
        snapshot_provenance=(candidate.snapshot.provenance if candidate.snapshot else None),
    )


def _to_evidence(
    dimension: EvidenceDimension,
    candidate: _Candidate,
    trail: tuple[AcquisitionAttempt, ...],
) -> DimensionEvidence:
    snapshot_provenance = candidate.snapshot.provenance if candidate.snapshot else None
    interpretation_provenance = None
    if candidate.snapshot is not None:
        interpretation_provenance = InterpretationProvenance(
            snapshot_digest=candidate.snapshot.content_digest,
            interpreter_revision=_INTERPRETER_REVISION,
            dimension=dimension,
        )
    return DimensionEvidence(
        dimension=dimension,
        status=candidate.status,
        selected_source=candidate.source,
        observations=candidate.observations,
        snapshot_provenance=snapshot_provenance,
        interpretation_provenance=interpretation_provenance,
        diagnostics=candidate.diagnostics,
        acquisition_trail=trail,
    )
