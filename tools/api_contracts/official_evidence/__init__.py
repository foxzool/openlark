"""收集并解释逐维度的 Official Document Evidence。"""

from __future__ import annotations

import hashlib
import http.client
import json
import os
import re
import select
import socket
import subprocess
import tempfile
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from contextlib import AbstractContextManager
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from enum import Enum
from pathlib import Path
from typing import Any, Iterable

from ..models import ApiIdentity


__all__ = [
    "AcquisitionAttempt",
    "AdapterContractError",
    "DimensionEvidence",
    "EndpointObservation",
    "EvidenceDiagnostic",
    "EvidenceDimension",
    "EvidenceError",
    "EvidenceSource",
    "EvidenceStatus",
    "FieldObservation",
    "FreshOfficialPolicy",
    "InterpretationProvenance",
    "InterpreterError",
    "InvalidEvidenceRequest",
    "OfficialDocumentEvidence",
    "PreferSnapshotPolicy",
    "RecordedOnlyPolicy",
    "RecordedSnapshot",
    "SnapshotInvariantError",
    "SnapshotProvenance",
    "SnapshotStoreError",
    "TokenObservation",
    "collect",
    "compose",
    "compose_full",
]

_INTERPRETER_REVISION = "official-evidence/1"

_STRUCTURED_DETAIL_URL = (
    "https://open.feishu.cn/document_portal/v1/document/get_detail"
)
_RENDERED_WORKER_COMMAND = (
    "node",
    str(Path(__file__).with_name("rendered_document_worker.js")),
)
_RENDERED_MIN_DOC_CHARS = 500
_RENDERED_NOT_FOUND_HEAD_LINES = 30
_RENDERED_REQUEST_BODY = "Request body"
_RENDERED_QUERY_PARAMETERS = "Query parameters"
_RENDERED_REQUEST_EXAMPLE = "Request example"
_RENDERED_RESPONSE_EXAMPLE = "Response body example"
_RENDERED_ERROR_CODE = "Error code"
_RENDERED_STANDARD_SECTIONS = (
    _RENDERED_REQUEST_BODY,
    _RENDERED_QUERY_PARAMETERS,
    _RENDERED_RESPONSE_EXAMPLE,
)


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


class SnapshotStoreError(EvidenceError):
    """Snapshot store 无法维持持久化契约。"""


class AdapterContractError(EvidenceError):
    """Live source adapter 返回了违反契约的结果。"""


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
class FreshOfficialPolicy:
    """必须从当前 Official Source 获取新快照。"""


@dataclass(frozen=True)
class PreferSnapshotPolicy:
    """优先复用 provenance 匹配的持久快照，否则获取新快照。"""


@dataclass(frozen=True)
class _Candidate:
    status: EvidenceStatus
    source: EvidenceSource
    observations: tuple[EvidenceObservation, ...]
    diagnostics: tuple[EvidenceDiagnostic, ...]
    snapshot: RecordedSnapshot | None


@dataclass(frozen=True)
class _AcquisitionResult:
    snapshot: RecordedSnapshot | None
    failure: _Candidate | None


class _StructuredDetailAdapter:
    def __init__(self, base_url: str, timeout_seconds: float, retries: int) -> None:
        self._base_url = base_url
        self._timeout_seconds = timeout_seconds
        self._retries = retries

    def acquire(self, catalog_entry: ApiIdentity) -> _AcquisitionResult:
        full_path = _detail_full_path(catalog_entry)
        if not full_path:
            raise InvalidEvidenceRequest("Catalog Entry 缺少 Structured Detail fullPath")
        separator = "&" if "?" in self._base_url else "?"
        url = (
            self._base_url
            + separator
            + urllib.parse.urlencode({"fullPath": full_path})
        )
        last_error: Exception | None = None
        for attempt in range(self._retries + 1):
            try:
                request = urllib.request.Request(
                    url,
                    headers={"User-Agent": "openlark-official-evidence/1.0"},
                )
                with urllib.request.urlopen(
                    request,
                    timeout=self._timeout_seconds,
                ) as response:
                    raw_bytes = response.read()
                    source_uri = response.geturl()
                raw = raw_bytes.decode("utf-8", "replace")
                return _AcquisitionResult(
                    snapshot=_raw_structured_snapshot(
                        catalog_entry,
                        source_uri,
                        raw,
                    ),
                    failure=None,
                )
            except urllib.error.HTTPError as error:
                status = error.code
                error.close()
                if status == 404:
                    return _AcquisitionResult(
                        snapshot=None,
                        failure=_unavailable(
                            EvidenceSource.STRUCTURED_DETAIL,
                            "document_not_found",
                        ),
                    )
                last_error = error
            except (TimeoutError, socket.timeout) as error:
                last_error = error
            except urllib.error.URLError as error:
                last_error = error
            except (OSError, http.client.HTTPException) as error:
                last_error = error
            if attempt < self._retries:
                time.sleep(min(2**attempt, 8))
        code = (
            "acquisition_timeout"
            if _is_timeout(last_error)
            else "acquisition_failed"
        )
        return _AcquisitionResult(
            snapshot=None,
            failure=_unavailable(EvidenceSource.STRUCTURED_DETAIL, code),
        )


class _UnavailableRenderedDocumentAdapter:
    def acquire(self, catalog_entry: ApiIdentity) -> _AcquisitionResult:
        return _AcquisitionResult(
            snapshot=None,
            failure=_unavailable(
                EvidenceSource.RENDERED_DOCUMENT,
                "adapter_unavailable",
            ),
        )

    def close(self) -> None:
        pass


class _RenderedDocumentAdapter:
    def __init__(self, timeout_seconds: float) -> None:
        self._timeout_seconds = timeout_seconds
        self._process: subprocess.Popen[str] | None = None
        self._lock = threading.Lock()
        self._request_id = 0

    def acquire(self, catalog_entry: ApiIdentity) -> _AcquisitionResult:
        with self._lock:
            return self._acquire(catalog_entry)

    def _acquire(self, catalog_entry: ApiIdentity) -> _AcquisitionResult:
        url = _rendered_url(catalog_entry)
        process = self._ensure_process()
        if process is None:
            return _AcquisitionResult(
                snapshot=None,
                failure=_unavailable(
                    EvidenceSource.RENDERED_DOCUMENT,
                    "adapter_unavailable",
                ),
            )
        self._request_id += 1
        request_id = self._request_id
        request = json.dumps(
            {
                "id": request_id,
                "url": url,
                "timeout_ms": max(1, int(self._timeout_seconds * 1000)),
            },
            separators=(",", ":"),
        )
        if process.stdin is None:
            raise AdapterContractError(
                "Rendered Document adapter 缺少 stdin pipe"
            )
        try:
            process.stdin.write(request + "\n")
            process.stdin.flush()
        except (BrokenPipeError, OSError):
            self._stop_process()
            return _AcquisitionResult(
                snapshot=None,
                failure=_unavailable(
                    EvidenceSource.RENDERED_DOCUMENT,
                    "acquisition_failed",
                ),
            )

        if process.stdout is None:
            raise AdapterContractError(
                "Rendered Document adapter 缺少 stdout pipe"
            )
        readable, _, _ = select.select(
            (process.stdout,),
            (),
            (),
            self._timeout_seconds,
        )
        if not readable:
            self._stop_process()
            return _AcquisitionResult(
                snapshot=None,
                failure=_unavailable(
                    EvidenceSource.RENDERED_DOCUMENT,
                    "acquisition_timeout",
                ),
            )
        raw_response = process.stdout.readline()
        if not raw_response:
            self._stop_process()
            return _AcquisitionResult(
                snapshot=None,
                failure=_unavailable(
                    EvidenceSource.RENDERED_DOCUMENT,
                    "acquisition_failed",
                ),
            )
        try:
            response = json.loads(raw_response)
        except json.JSONDecodeError as error:
            raise AdapterContractError(
                "Rendered Document adapter 返回了无效 JSON"
            ) from error
        if not isinstance(response, dict) or response.get("id") != request_id:
            raise AdapterContractError(
                "Rendered Document adapter response 与请求不匹配"
            )
        status = response.get("status")
        if status == "unavailable":
            code = response.get("code")
            if code not in {
                "adapter_unavailable",
                "acquisition_failed",
                "acquisition_timeout",
                "document_not_found",
            }:
                raise AdapterContractError(
                    "Rendered Document adapter 返回了无效 failure"
                )
            return _AcquisitionResult(
                snapshot=None,
                failure=_unavailable(EvidenceSource.RENDERED_DOCUMENT, code),
            )
        if status != "ok":
            raise AdapterContractError(
                "Rendered Document adapter 返回了未知状态"
            )
        source_uri = response.get("source_uri")
        content = response.get("content")
        if (
            not isinstance(source_uri, str)
            or re.fullmatch(r"https?://\S+", source_uri) is None
            or not isinstance(content, str)
        ):
            raise AdapterContractError(
                "Rendered Document adapter 成功响应缺少内容或来源"
            )
        return _AcquisitionResult(
            snapshot=RecordedSnapshot._create(
                version=1,
                source_kind=EvidenceSource.RENDERED_DOCUMENT,
                catalog_entry=catalog_entry,
                acquired_at=_now_iso8601(),
                source_uri=source_uri,
                raw_representation=content,
            ),
            failure=None,
        )

    def close(self) -> None:
        with self._lock:
            self._close()

    def _close(self) -> None:
        process = self._process
        if process is None:
            return
        if process.poll() is None and process.stdin is not None:
            try:
                process.stdin.write('{"type":"shutdown"}\n')
                process.stdin.flush()
                process.wait(timeout=1)
            except (BrokenPipeError, OSError, subprocess.TimeoutExpired):
                self._stop_process()
                return
        self._close_pipes(process)
        self._process = None

    def _ensure_process(self) -> subprocess.Popen[str] | None:
        if self._process is not None and self._process.poll() is None:
            return self._process
        self._stop_process()
        try:
            self._process = subprocess.Popen(
                _RENDERED_WORKER_COMMAND,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                text=True,
                bufsize=1,
            )
        except OSError:
            return None
        return self._process

    def _stop_process(self) -> None:
        process = self._process
        if process is None:
            return
        if process.poll() is None:
            process.terminate()
            try:
                process.wait(timeout=1)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
        self._close_pipes(process)
        self._process = None

    @staticmethod
    def _close_pipes(process: subprocess.Popen[str]) -> None:
        if process.stdin is not None:
            process.stdin.close()
        if process.stdout is not None:
            process.stdout.close()


class _SnapshotStore:
    def __init__(self, directory: Path) -> None:
        self._directory = directory

    def load(
        self,
        catalog_entry: ApiIdentity,
        source: EvidenceSource,
    ) -> RecordedSnapshot | None:
        source_directory = self._source_directory(catalog_entry, source)
        try:
            if not source_directory.exists():
                return None
            snapshots = tuple(
                self._read(path)
                for path in source_directory.iterdir()
                if path.is_file() and path.suffix == ".json"
            )
        except SnapshotStoreError:
            raise
        except OSError as error:
            raise SnapshotStoreError("读取 Official Snapshot store 失败") from error
        matching = tuple(
            snapshot
            for snapshot in snapshots
            if snapshot.catalog_entry == catalog_entry
            and snapshot.source_kind is source
        )
        if len(matching) != len(snapshots):
            raise SnapshotStoreError("Snapshot store provenance 与存储路径不匹配")
        return max(matching, key=_snapshot_time) if matching else None

    def save(self, snapshot: RecordedSnapshot) -> None:
        _validate_snapshot(snapshot)
        source_directory = self._source_directory(
            snapshot.catalog_entry,
            snapshot.source_kind,
        )
        path = source_directory / f"{_snapshot_key(snapshot)}.json"
        record = {
            "version": snapshot.version,
            "source_kind": snapshot.source_kind.value,
            "catalog_entry": asdict(snapshot.catalog_entry),
            "acquired_at": snapshot.acquired_at,
            "source_uri": snapshot.source_uri,
            "content_digest": snapshot.content_digest,
            "raw_representation": snapshot.raw_representation,
        }
        serialized = json.dumps(
            record,
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        try:
            source_directory.mkdir(parents=True, exist_ok=True)
        except OSError as error:
            raise SnapshotStoreError(
                "创建 Official Snapshot store 目录失败"
            ) from error
        temporary_path: Path | None = None
        try:
            descriptor, temporary_name = tempfile.mkstemp(
                dir=source_directory,
                prefix=".snapshot-",
            )
            temporary_path = Path(temporary_name)
            with os.fdopen(descriptor, "w", encoding="utf-8") as handle:
                handle.write(serialized)
                handle.flush()
                os.fsync(handle.fileno())
            try:
                os.link(temporary_path, path)
            except FileExistsError:
                if path.read_text(encoding="utf-8") != serialized:
                    raise SnapshotStoreError(
                        "不可变 Official Snapshot 写入发生键冲突"
                    )
        except SnapshotStoreError:
            raise
        except OSError as error:
            raise SnapshotStoreError(
                "写入 Official Snapshot store 失败"
            ) from error
        finally:
            if temporary_path is not None:
                try:
                    temporary_path.unlink(missing_ok=True)
                except OSError as error:
                    raise SnapshotStoreError(
                        "清理 Official Snapshot 临时文件失败"
                    ) from error

    def remove(self, snapshot: RecordedSnapshot) -> None:
        path = self._source_directory(
            snapshot.catalog_entry,
            snapshot.source_kind,
        ) / f"{_snapshot_key(snapshot)}.json"
        try:
            path.unlink(missing_ok=True)
        except OSError as error:
            raise SnapshotStoreError("淘汰 Rejected Official Snapshot 失败") from error

    def _source_directory(
        self,
        catalog_entry: ApiIdentity,
        source: EvidenceSource,
    ) -> Path:
        identity = json.dumps(
            asdict(catalog_entry),
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        identity_key = hashlib.sha256(identity.encode("utf-8")).hexdigest()
        return self._directory / identity_key / source.value

    @staticmethod
    def _read(path: Path) -> RecordedSnapshot:
        try:
            record = json.loads(path.read_text(encoding="utf-8"))
            if not isinstance(record, dict):
                raise ValueError("snapshot record is not an object")
            snapshot = RecordedSnapshot(
                version=record["version"],
                source_kind=EvidenceSource(record["source_kind"]),
                catalog_entry=ApiIdentity(**record["catalog_entry"]),
                acquired_at=record["acquired_at"],
                source_uri=record["source_uri"],
                content_digest=record["content_digest"],
                raw_representation=record["raw_representation"],
            )
            _validate_snapshot(snapshot)
            return snapshot
        except (KeyError, TypeError, ValueError, json.JSONDecodeError) as error:
            raise SnapshotStoreError("Official Snapshot store 记录损坏") from error
        except OSError as error:
            raise SnapshotStoreError("读取 Official Snapshot store 失败") from error


class _OfficialEvidenceCollector(AbstractContextManager["_OfficialEvidenceCollector"]):
    """绑定 live adapters 与 snapshot store 的 Evidence composition。"""

    def __init__(
        self,
        structured_detail: _StructuredDetailAdapter,
        rendered_document: _RenderedDocumentAdapter
        | _UnavailableRenderedDocumentAdapter,
        snapshot_store: _SnapshotStore,
    ) -> None:
        self._structured_detail = structured_detail
        self._rendered_document = rendered_document
        self._snapshot_store = snapshot_store

    def __enter__(self) -> _OfficialEvidenceCollector:
        return self

    def __exit__(self, exc_type, exc_value, traceback) -> None:
        self.close()

    def close(self) -> None:
        self._rendered_document.close()

    def collect(
        self,
        catalog_entry: ApiIdentity,
        dimensions: Iterable[EvidenceDimension],
        policy: FreshOfficialPolicy | PreferSnapshotPolicy | RecordedOnlyPolicy,
    ) -> OfficialDocumentEvidence:
        requested = _requested_dimensions(catalog_entry, dimensions)
        if isinstance(policy, RecordedOnlyPolicy):
            _validate_recorded_policy(catalog_entry, policy)
            return _collect_from_snapshots(
                catalog_entry,
                requested,
                policy.snapshots,
            )
        if not isinstance(policy, (FreshOfficialPolicy, PreferSnapshotPolicy)):
            raise InvalidEvidenceRequest("不支持的 Evidence Acquisition Policy")

        structured = self._obtain_candidates(
            catalog_entry,
            requested,
            EvidenceSource.STRUCTURED_DETAIL,
            self._structured_detail,
            policy,
        )
        fallback_dimensions = tuple(
            dimension
            for dimension in requested
            if structured[dimension].status is not EvidenceStatus.TRUSTED
        )
        rendered: dict[EvidenceDimension, _Candidate] = {}
        if fallback_dimensions:
            rendered = self._obtain_candidates(
                catalog_entry,
                fallback_dimensions,
                EvidenceSource.RENDERED_DOCUMENT,
                self._rendered_document,
                policy,
            )

        results = []
        for dimension in requested:
            candidates = [structured[dimension]]
            if dimension in rendered:
                candidates.append(rendered[dimension])
            selected = max(candidates, key=_selection_key)
            trail = tuple(_attempt(candidate) for candidate in candidates)
            results.append(_to_evidence(dimension, selected, trail))
        return OfficialDocumentEvidence(
            catalog_entry=catalog_entry,
            dimensions=tuple(results),
        )

    def _obtain_candidates(
        self,
        catalog_entry: ApiIdentity,
        dimensions: tuple[EvidenceDimension, ...],
        source: EvidenceSource,
        adapter: _StructuredDetailAdapter
        | _RenderedDocumentAdapter
        | _UnavailableRenderedDocumentAdapter,
        policy: FreshOfficialPolicy | PreferSnapshotPolicy,
    ) -> dict[EvidenceDimension, _Candidate]:
        if isinstance(policy, PreferSnapshotPolicy):
            cached = self._snapshot_store.load(catalog_entry, source)
            if cached is not None:
                cached_candidates = {
                    dimension: _interpret_snapshot(cached, dimension)
                    for dimension in dimensions
                }
                if not any(
                    candidate.status is EvidenceStatus.REJECTED
                    for candidate in cached_candidates.values()
                ):
                    return cached_candidates
                self._snapshot_store.remove(cached)

        acquisition = adapter.acquire(catalog_entry)
        snapshot, failure = _validate_acquisition(
            acquisition,
            catalog_entry,
            source,
        )
        if failure is not None:
            return {dimension: failure for dimension in dimensions}
        if snapshot is None:
            raise AdapterContractError("adapter acquisition invariant 违例")
        self._snapshot_store.save(snapshot)
        candidates = {
            dimension: _interpret_snapshot(snapshot, dimension)
            for dimension in dimensions
        }
        if any(
            candidate.status is EvidenceStatus.REJECTED
            for candidate in candidates.values()
        ):
            self._snapshot_store.remove(snapshot)
        return candidates


def compose(
    *,
    snapshot_directory: Path,
    timeout_seconds: float,
    retries: int,
    structured_detail_url: str = _STRUCTURED_DETAIL_URL,
) -> _OfficialEvidenceCollector:
    """配置不依赖 Playwright 的 CI live composition。"""
    return _compose(
        snapshot_directory=snapshot_directory,
        timeout_seconds=timeout_seconds,
        retries=retries,
        structured_detail_url=structured_detail_url,
        rendered_document=_UnavailableRenderedDocumentAdapter(),
    )


def compose_full(
    *,
    snapshot_directory: Path,
    timeout_seconds: float,
    retries: int,
    structured_detail_url: str = _STRUCTURED_DETAIL_URL,
) -> _OfficialEvidenceCollector:
    """配置可复用 Playwright 生命周期的人工/full live composition。"""
    return _compose(
        snapshot_directory=snapshot_directory,
        timeout_seconds=timeout_seconds,
        retries=retries,
        structured_detail_url=structured_detail_url,
        rendered_document=_RenderedDocumentAdapter(float(timeout_seconds)),
    )


def _compose(
    *,
    snapshot_directory: Path,
    timeout_seconds: float,
    retries: int,
    structured_detail_url: str,
    rendered_document: _RenderedDocumentAdapter
    | _UnavailableRenderedDocumentAdapter,
) -> _OfficialEvidenceCollector:
    if not isinstance(snapshot_directory, Path):
        raise TypeError("snapshot_directory 必须是 pathlib.Path")
    if not isinstance(timeout_seconds, (int, float)) or timeout_seconds <= 0:
        raise ValueError("timeout_seconds 必须大于零")
    if not isinstance(retries, int) or isinstance(retries, bool) or retries < 0:
        raise ValueError("retries 必须是非负整数")
    if not isinstance(structured_detail_url, str) or not structured_detail_url:
        raise ValueError("structured_detail_url 不能为空")
    return _OfficialEvidenceCollector(
        _StructuredDetailAdapter(
            structured_detail_url,
            float(timeout_seconds),
            retries,
        ),
        rendered_document,
        _SnapshotStore(snapshot_directory),
    )


def collect(
    catalog_entry: ApiIdentity,
    dimensions: Iterable[EvidenceDimension],
    policy: RecordedOnlyPolicy,
) -> OfficialDocumentEvidence:
    """通过统一 collect seam 解释版本化 Recorded Snapshots。"""
    requested = _requested_dimensions(catalog_entry, dimensions)
    if not isinstance(policy, RecordedOnlyPolicy):
        raise InvalidEvidenceRequest(
            "live acquisition policy 必须通过 compose() 配置"
        )
    _validate_recorded_policy(catalog_entry, policy)
    return _collect_from_snapshots(
        catalog_entry,
        requested,
        policy.snapshots,
    )


def _requested_dimensions(
    catalog_entry: ApiIdentity,
    dimensions: Iterable[EvidenceDimension],
) -> tuple[EvidenceDimension, ...]:
    if not isinstance(catalog_entry, ApiIdentity):
        raise InvalidEvidenceRequest("catalog_entry 必须是 ApiIdentity")
    try:
        requested = tuple(dimensions)
    except TypeError as error:
        raise InvalidEvidenceRequest("Evidence Dimensions 必须可迭代") from error
    if not requested or any(
        not isinstance(item, EvidenceDimension)
        for item in requested
    ):
        raise InvalidEvidenceRequest("至少请求一个有效 Evidence Dimension")
    if len(set(requested)) != len(requested):
        raise InvalidEvidenceRequest("Evidence Dimensions 不得重复")
    return requested


def _validate_recorded_policy(
    catalog_entry: ApiIdentity,
    policy: RecordedOnlyPolicy,
) -> None:
    if not isinstance(policy.snapshots, tuple):
        raise SnapshotInvariantError(
            "Recorded-only policy 必须保存不可变 snapshot 序列"
        )
    if any(
        not isinstance(snapshot, RecordedSnapshot)
        for snapshot in policy.snapshots
    ):
        raise SnapshotInvariantError("Recorded-only policy 包含无效 snapshot")
    if any(
        snapshot.catalog_entry != catalog_entry
        for snapshot in policy.snapshots
    ):
        raise InvalidEvidenceRequest(
            "Recorded Snapshot 与 Catalog Entry 不匹配"
        )
    sources = [snapshot.source_kind for snapshot in policy.snapshots]
    if len(set(sources)) != len(sources):
        raise InvalidEvidenceRequest(
            "每种 Official Evidence Source 最多一个 Recorded Snapshot"
        )
    for snapshot in policy.snapshots:
        _validate_snapshot(snapshot)


def _validate_snapshot(snapshot: RecordedSnapshot) -> None:
    if not isinstance(snapshot.catalog_entry, ApiIdentity):
        raise SnapshotInvariantError("Official Snapshot Catalog Entry 无效")
    if not isinstance(snapshot.source_kind, EvidenceSource):
        raise SnapshotInvariantError("Official Snapshot source kind 无效")
    if snapshot.version != 1:
        raise SnapshotInvariantError("不支持的 Official Snapshot version")
    if not isinstance(snapshot.raw_representation, str):
        raise SnapshotInvariantError("Official Snapshot 原始表示必须是字符串")
    if not isinstance(snapshot.acquired_at, str):
        raise SnapshotInvariantError("Official Snapshot acquisition time 无效")
    try:
        acquired_at = datetime.fromisoformat(
            snapshot.acquired_at.replace("Z", "+00:00")
        )
    except ValueError as error:
        raise SnapshotInvariantError(
            "Official Snapshot acquisition time 无效"
        ) from error
    if acquired_at.tzinfo is None:
        raise SnapshotInvariantError(
            "Official Snapshot acquisition time 必须包含时区"
        )
    if (
        not isinstance(snapshot.source_uri, str)
        or not snapshot.source_uri.strip()
    ):
        raise SnapshotInvariantError("Official Snapshot source URI 不能为空")
    digest = hashlib.sha256(
        snapshot.raw_representation.encode("utf-8")
    ).hexdigest()
    if digest != snapshot.content_digest:
        raise SnapshotInvariantError(
            "Official Snapshot content digest 不匹配"
        )


def _validate_acquisition(
    acquisition: object,
    catalog_entry: ApiIdentity,
    source: EvidenceSource,
) -> tuple[RecordedSnapshot | None, _Candidate | None]:
    source_name = (
        "Structured Detail"
        if source is EvidenceSource.STRUCTURED_DETAIL
        else "Rendered Document"
    )
    if not isinstance(acquisition, _AcquisitionResult):
        raise AdapterContractError(
            f"{source_name} adapter 返回了无效 acquisition result"
        )
    if acquisition.snapshot is None:
        if (
            acquisition.failure is None
            or acquisition.failure.source is not source
            or acquisition.failure.status is not EvidenceStatus.UNAVAILABLE
            or acquisition.failure.snapshot is not None
        ):
            raise AdapterContractError(
                f"{source_name} adapter 未返回有效 snapshot 或 failure"
            )
        return None, acquisition.failure
    if acquisition.failure is not None:
        raise AdapterContractError(
            f"{source_name} adapter 同时返回 snapshot 与 failure"
        )
    snapshot = acquisition.snapshot
    if (
        snapshot.catalog_entry != catalog_entry
        or snapshot.source_kind is not source
    ):
        raise AdapterContractError(
            f"{source_name} snapshot provenance 与请求不匹配"
        )
    _validate_snapshot(snapshot)
    return snapshot, None


def _collect_from_snapshots(
    catalog_entry: ApiIdentity,
    dimensions: tuple[EvidenceDimension, ...],
    snapshots: tuple[RecordedSnapshot, ...],
) -> OfficialDocumentEvidence:
    by_source = {snapshot.source_kind: snapshot for snapshot in snapshots}
    results = tuple(
        _collect_dimension(dimension, by_source)
        for dimension in dimensions
    )
    return OfficialDocumentEvidence(
        catalog_entry=catalog_entry,
        dimensions=results,
    )

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
    if not _rendered_content_is_healthy(content):
        return _rejected(snapshot, "document_unhealthy")
    heading = {
        EvidenceDimension.ENDPOINT: "Endpoint",
        EvidenceDimension.REQUEST_FIELDS: "Request Fields",
        EvidenceDimension.RESPONSE_FIELDS: "Response Fields",
        EvidenceDimension.TOKENS: "Tokens",
    }[dimension]
    section = _rendered_section(content, heading)
    if section is None:
        if dimension in {
            EvidenceDimension.REQUEST_FIELDS,
            EvidenceDimension.RESPONSE_FIELDS,
        } and not _is_recorded_rendered_format(content):
            return _interpret_rendered_inner_text_fields(
                snapshot,
                content,
                dimension,
            )
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


def _is_recorded_rendered_format(content: str) -> bool:
    return re.search(r"(?m)^##[ \t]+", content) is not None


def _rendered_content_is_healthy(content: str) -> bool:
    if not content:
        return False
    head = "\n".join(
        content.splitlines()[:_RENDERED_NOT_FOUND_HEAD_LINES]
    ).casefold()
    if (
        "the documentation could not be found" in head
        or "文档不存在" in head
        or "404 not found" in head
    ):
        return False
    if _is_recorded_rendered_format(content):
        return True
    return (
        len(content) >= _RENDERED_MIN_DOC_CHARS
        and any(
            content.count(section) >= 2
            for section in _RENDERED_STANDARD_SECTIONS
        )
    )


def _interpret_rendered_inner_text_fields(
    snapshot: RecordedSnapshot,
    content: str,
    dimension: EvidenceDimension,
) -> _Candidate:
    if dimension is EvidenceDimension.RESPONSE_FIELDS:
        section = _extract_rendered_inner_text_section(
            content,
            _RENDERED_RESPONSE_EXAMPLE,
            _RENDERED_ERROR_CODE,
            occurrence=1,
        )
        names = tuple(
            name
            for name in re.findall(r'"([a-z_]+)"\s*:', section)
            if name not in {"code", "msg", "data"}
        )
        observations = tuple(
            FieldObservation(
                path=(name,),
                location="response_body",
                required=None,
                field_type=None,
                source=snapshot.source_kind,
            )
            for name in dict.fromkeys(names)
        )
        return _trusted(snapshot, observations)

    method = snapshot.catalog_entry.url.partition(":")[0].upper()
    if method in {"POST", "PUT", "PATCH"}:
        start = _RENDERED_REQUEST_BODY
        occurrence = 2
        location = "request_body"
    else:
        start = _RENDERED_QUERY_PARAMETERS
        occurrence = 1
        location = "query"
    section = _extract_rendered_inner_text_section(
        content,
        start,
        _RENDERED_REQUEST_EXAMPLE,
        occurrence=occurrence,
    )
    observations = tuple(
        FieldObservation(
            path=(name,),
            location=location,
            required=required,
            field_type=field_type or None,
            source=snapshot.source_kind,
        )
        for name, field_type, required in _parse_rendered_parameter_table(section)
    )
    return _trusted(snapshot, observations)


def _extract_rendered_inner_text_section(
    content: str,
    start: str,
    end: str,
    *,
    occurrence: int,
) -> str:
    parts = content.split(start)
    if len(parts) <= occurrence:
        return ""
    section = start.join(parts[occurrence:])
    end_index = section.find(end)
    return section if end_index < 0 else section[:end_index]


def _parse_rendered_parameter_table(
    section: str,
) -> tuple[tuple[str, str, bool], ...]:
    lines = [line.strip() for line in section.splitlines()]
    banned = {
        "parameter",
        "type",
        "required",
        "description",
        "authorization",
        "content",
        "value",
        "example",
        "facts",
        "scopes",
        "header",
        "string",
        "int",
        "integer",
        "boolean",
        "bool",
        "number",
        "float",
        "double",
        "object",
        "array",
        "file",
        "binary",
        "map",
        "null",
        "string[]",
        "int[]",
        "integer[]",
        "boolean[]",
        "number[]",
        "object[]",
    }
    type_line = re.compile(
        r"^(string|int|integer|boolean|bool|number|float|double|object|array|"
        r"file|binary|map|null)(\[\])?$",
        re.IGNORECASE,
    )
    observations: list[tuple[str, str, bool]] = []
    index = 0
    while index < len(lines):
        name = lines[index]
        if (
            re.fullmatch(r"[a-z][a-z0-9_]*", name)
            and name not in banned
            and type_line.fullmatch(name) is None
            and len(name) >= 2
        ):
            for required_index in range(
                index + 1,
                min(index + 16, len(lines)),
            ):
                if lines[required_index] not in {"Yes", "No"}:
                    continue
                field_type = next(
                    (
                        lines[type_index]
                        for type_index in range(index + 1, required_index)
                        if type_line.fullmatch(lines[type_index])
                    ),
                    "",
                )
                observations.append(
                    (
                        name,
                        field_type,
                        lines[required_index] == "Yes",
                    )
                )
                index = required_index
                break
        index += 1
    return tuple(observations)


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


def _detail_full_path(catalog_entry: ApiIdentity) -> str:
    full_path = catalog_entry.full_path.strip()
    if full_path.startswith("/document/"):
        return full_path.removeprefix("/document")
    if full_path == "/document":
        return ""
    return full_path


def _rendered_url(catalog_entry: ApiIdentity) -> str:
    doc_path = catalog_entry.doc_path.strip()
    if re.fullmatch(r"https?://\S+", doc_path):
        return doc_path
    full_path = catalog_entry.full_path.strip()
    if full_path.startswith("/"):
        return urllib.parse.urljoin("https://open.feishu.cn", full_path)
    raise InvalidEvidenceRequest("Catalog Entry 缺少 Rendered Document URL")


def _now_iso8601() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def _raw_structured_snapshot(
    catalog_entry: ApiIdentity,
    source_uri: str,
    raw: str,
) -> RecordedSnapshot:
    return RecordedSnapshot._create(
        version=1,
        source_kind=EvidenceSource.STRUCTURED_DETAIL,
        catalog_entry=catalog_entry,
        acquired_at=_now_iso8601(),
        source_uri=source_uri,
        raw_representation=raw,
    )


def _is_timeout(error: Exception | None) -> bool:
    if isinstance(error, (TimeoutError, socket.timeout)):
        return True
    return (
        isinstance(error, urllib.error.URLError)
        and isinstance(error.reason, (TimeoutError, socket.timeout))
    )


def _snapshot_time(snapshot: RecordedSnapshot) -> datetime:
    return datetime.fromisoformat(
        snapshot.acquired_at.replace("Z", "+00:00")
    )


def _snapshot_key(snapshot: RecordedSnapshot) -> str:
    identity = "\0".join(
        (
            snapshot.source_uri,
            snapshot.acquired_at,
            snapshot.content_digest,
        )
    )
    return hashlib.sha256(identity.encode("utf-8")).hexdigest()


def _diagnostic(code: str) -> EvidenceDiagnostic:
    messages = {
        "snapshot_unavailable": "未提供可用的 Recorded Snapshot",
        "adapter_unavailable": "Rendered Document adapter 不可用",
        "acquisition_timeout": "Official Source 获取超时",
        "acquisition_failed": "Official Source 获取失败",
        "document_not_found": "Official Source 未找到对应文档",
        "document_unhealthy": "原始官方文档表示未通过健康检查",
        "structure_incomplete": "相关官方文档结构无法完整解释",
        "structure_unsupported": "相关官方文档结构暂不受解释器支持",
    }
    return EvidenceDiagnostic(code=code, message=messages[code])


def _unavailable(
    source: EvidenceSource,
    code: str = "snapshot_unavailable",
) -> _Candidate:
    return _Candidate(
        status=EvidenceStatus.UNAVAILABLE,
        source=source,
        observations=(),
        diagnostics=(_diagnostic(code),),
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
