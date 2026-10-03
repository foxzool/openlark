"""Resolve one Catalog Entry to its unique Rust Contract Target.

The mapping file and repository snapshot are composition concerns.  Callers only
see the four domain outcomes and never search the repository themselves.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path, PurePosixPath
from types import MappingProxyType
from typing import Mapping, TypeAlias

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - repository requires Python 3.11+
    tomllib = None

from .models import ApiIdentity


__all__ = (
    "Ambiguous",
    "Missing",
    "Resolution",
    "ResolutionConfigurationError",
    "ResolutionDiagnostic",
    "Resolved",
    "RustContractTarget",
    "Unmapped",
    "compose",
)


@dataclass(frozen=True, order=True)
class ResolutionDiagnostic:
    """One stable mapping-composition diagnostic."""

    code: str
    location: str
    message: str


class ResolutionConfigurationError(Exception):
    """The resolver could not be composed from a valid complete configuration."""

    def __init__(self, diagnostics: tuple[ResolutionDiagnostic, ...]):
        self.diagnostics = tuple(sorted(diagnostics))
        rendered = "\n".join(
            f"[{item.code}] {item.location}: {item.message}"
            for item in self.diagnostics
        )
        super().__init__(f"Rust contract resolution configuration is invalid:\n{rendered}")


@dataclass(frozen=True)
class RustContractTarget:
    """The unique repository-relative Rust source contract for a Catalog Entry."""

    crate_name: str
    repository_path: PurePosixPath
    _crate_source: PurePosixPath = field(repr=False)


@dataclass(frozen=True)
class Resolved:
    entry: ApiIdentity
    target: RustContractTarget


@dataclass(frozen=True)
class Unmapped:
    entry: ApiIdentity


@dataclass(frozen=True)
class Missing:
    entry: ApiIdentity
    crate_name: str
    checked_candidates: frozenset[PurePosixPath]


@dataclass(frozen=True)
class Ambiguous:
    entry: ApiIdentity
    crate_name: str
    candidates: frozenset[RustContractTarget]


Resolution: TypeAlias = Resolved | Unmapped | Missing | Ambiguous


@dataclass(frozen=True)
class _Rewrite:
    source_prefix: str
    target_prefix: str


@dataclass(frozen=True)
class _CrateMapping:
    crate_name: str
    source_root: PurePosixPath
    aliases: Mapping[str, str]
    rewrites: tuple[_Rewrite, ...]


class _RustContractResolver:
    def __init__(
        self,
        owners: Mapping[str, _CrateMapping],
        rust_files: frozenset[PurePosixPath],
    ) -> None:
        self._owners = owners
        self._rust_files = rust_files

    def resolve(self, entry: ApiIdentity) -> Resolution:
        if not isinstance(entry, ApiIdentity):
            raise TypeError("resolve() requires an ApiIdentity Catalog Entry")
        expected_file = _entry_path(entry)
        owner = self._owners.get(entry.biz_tag)
        if owner is None:
            return Unmapped(entry)

        relative_candidates = {expected_file}
        alias = owner.aliases.get(expected_file)
        if alias is not None:
            relative_candidates.add(alias)
        for rewrite in owner.rewrites:
            if expected_file.startswith(rewrite.source_prefix):
                relative_candidates.add(
                    rewrite.target_prefix
                    + expected_file[len(rewrite.source_prefix) :]
                )

        checked = frozenset(
            owner.source_root / PurePosixPath(candidate)
            for candidate in relative_candidates
        )
        existing = checked & self._rust_files
        if not existing:
            return Missing(entry, owner.crate_name, checked)

        targets = frozenset(
            RustContractTarget(owner.crate_name, path, owner.source_root)
            for path in existing
        )
        if len(targets) > 1:
            return Ambiguous(entry, owner.crate_name, targets)
        return Resolved(entry, next(iter(targets)))


def compose(*, repository_root: Path) -> _RustContractResolver:
    """Validate the fixed mapping and snapshot all mapped Rust source files."""

    if tomllib is None:
        raise ResolutionConfigurationError(
            (
                ResolutionDiagnostic(
                    "python_tomllib_unavailable",
                    "runtime",
                    "Python 3.11+ with tomllib is required",
                ),
            )
        )

    root = Path(repository_root)
    mapping_path = root / "tools" / "api_coverage.toml"
    diagnostics: list[ResolutionDiagnostic] = []
    if not root.is_dir():
        diagnostics.append(
            ResolutionDiagnostic(
                "repository_root_missing", str(root), "repository root is not a directory"
            )
        )
    if not mapping_path.is_file():
        diagnostics.append(
            ResolutionDiagnostic(
                "mapping_file_missing",
                "tools/api_coverage.toml",
                "fixed mapping file does not exist",
            )
        )
    if diagnostics:
        raise ResolutionConfigurationError(tuple(diagnostics))

    try:
        data = tomllib.loads(mapping_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as exc:
        raise ResolutionConfigurationError(
            (
                ResolutionDiagnostic(
                    "mapping_file_unreadable",
                    "tools/api_coverage.toml",
                    str(exc),
                ),
            )
        ) from exc

    crates = data.get("crates")
    if not isinstance(crates, dict) or not crates:
        raise ResolutionConfigurationError(
            (
                ResolutionDiagnostic(
                    "mapping_crates_empty",
                    "tools/api_coverage.toml",
                    "at least one [crates.*] entry is required",
                ),
            )
        )

    root_real = root.resolve()
    mappings: list[tuple[_CrateMapping, tuple[str, ...], Path]] = []
    tag_locations: dict[str, str] = {}

    for crate_name, raw_config in sorted(crates.items()):
        location = f"crates.{crate_name}"
        if not isinstance(raw_config, dict):
            diagnostics.append(
                ResolutionDiagnostic(
                    "crate_config_malformed", location, "crate configuration must be a table"
                )
            )
            continue

        source_value = raw_config.get("src")
        source_path = _validated_relative_path(
            source_value,
            f"{location}.src",
            diagnostics,
            code="crate_source_malformed",
        )
        source_real: Path | None = None
        if source_path is not None:
            source_real = (root / Path(source_path.as_posix())).resolve()
            if not _is_within(source_real, root_real):
                diagnostics.append(
                    ResolutionDiagnostic(
                        "crate_source_escape",
                        f"{location}.src",
                        "crate source must remain within the repository",
                    )
                )
                source_real = None
            elif not source_real.is_dir():
                diagnostics.append(
                    ResolutionDiagnostic(
                        "crate_source_missing",
                        f"{location}.src",
                        f"directory does not exist: {source_path.as_posix()}",
                    )
                )
                source_real = None

        raw_tags = raw_config.get("biz_tags")
        tags: tuple[str, ...] = ()
        if (
            not isinstance(raw_tags, list)
            or not raw_tags
            or any(not isinstance(tag, str) or not tag.strip() for tag in raw_tags)
        ):
            diagnostics.append(
                ResolutionDiagnostic(
                    "biz_tags_malformed",
                    f"{location}.biz_tags",
                    "biz_tags must be a non-empty array of non-empty strings",
                )
            )
        else:
            tags = tuple(raw_tags)
            for tag in tags:
                previous = tag_locations.get(tag)
                if previous is not None:
                    diagnostics.append(
                        ResolutionDiagnostic(
                            "duplicate_biz_tag",
                            f"{location}.biz_tags",
                            f"bizTag {tag!r} is already owned by {previous}",
                        )
                    )
                else:
                    tag_locations[tag] = crate_name

        aliases = _validate_aliases(
            raw_config.get("implementation_path_aliases"), location, diagnostics
        )
        rewrites = _validate_rewrites(
            raw_config.get("implementation_path_rewrites"), location, diagnostics
        )
        if source_path is not None and source_real is not None:
            mappings.append(
                (
                    _CrateMapping(
                        str(crate_name),
                        source_path,
                        MappingProxyType(aliases),
                        rewrites,
                    ),
                    tags,
                    source_real,
                )
            )

    rust_files: set[PurePosixPath] = set()
    for mapping, _tags, source_real in mappings:
        for path in source_real.rglob("*.rs"):
            if not path.is_file():
                continue
            resolved = path.resolve()
            if not _is_within(resolved, root_real) or not _is_within(
                resolved, source_real
            ):
                diagnostics.append(
                    ResolutionDiagnostic(
                        "rust_source_escape",
                        resolved.as_posix(),
                        "Rust source resolves outside its configured crate root",
                    )
                )
                continue
            rust_files.add(PurePosixPath(resolved.relative_to(root_real).as_posix()))

    if diagnostics:
        raise ResolutionConfigurationError(tuple(diagnostics))

    owners: dict[str, _CrateMapping] = {}
    for mapping, tags, _source_real in mappings:
        for tag in tags:
            owners[tag] = mapping
    return _RustContractResolver(MappingProxyType(owners), frozenset(rust_files))


def _entry_path(entry: ApiIdentity) -> str:
    value = entry.expected_file
    if not _is_safe_relative(value) or not value.endswith(".rs"):
        raise ValueError(
            f"Catalog Entry {entry.api_id!r} has invalid expected Rust path: {value!r}"
        )
    return PurePosixPath(value).as_posix()


def _validated_relative_path(
    value: object,
    location: str,
    diagnostics: list[ResolutionDiagnostic],
    *,
    code: str,
) -> PurePosixPath | None:
    if not isinstance(value, str) or not _is_safe_relative(value):
        diagnostics.append(
            ResolutionDiagnostic(
                code,
                location,
                "value must be a safe repository-relative path",
            )
        )
        return None
    return PurePosixPath(value)


def _validate_aliases(
    raw: object,
    crate_location: str,
    diagnostics: list[ResolutionDiagnostic],
) -> dict[str, str]:
    if raw is None:
        return {}
    if not isinstance(raw, dict):
        diagnostics.append(
            ResolutionDiagnostic(
                "implementation_aliases_malformed",
                f"{crate_location}.implementation_path_aliases",
                "aliases must be a table of Rust path to Rust path",
            )
        )
        return {}
    aliases: dict[str, str] = {}
    for source, target in raw.items():
        location = f"{crate_location}.implementation_path_aliases.{source}"
        if (
            not isinstance(source, str)
            or not isinstance(target, str)
            or not _is_safe_relative(source)
            or not _is_safe_relative(target)
            or not source.endswith(".rs")
            or not target.endswith(".rs")
        ):
            diagnostics.append(
                ResolutionDiagnostic(
                    "implementation_alias_malformed",
                    location,
                    "alias source and target must be safe relative .rs paths",
                )
            )
            continue
        aliases[PurePosixPath(source).as_posix()] = PurePosixPath(target).as_posix()
    return aliases


def _validate_rewrites(
    raw: object,
    crate_location: str,
    diagnostics: list[ResolutionDiagnostic],
) -> tuple[_Rewrite, ...]:
    if raw is None:
        return ()
    if not isinstance(raw, list):
        diagnostics.append(
            ResolutionDiagnostic(
                "implementation_rewrites_malformed",
                f"{crate_location}.implementation_path_rewrites",
                "rewrites must be an array of {from, to} tables",
            )
        )
        return ()
    rewrites: list[_Rewrite] = []
    for index, item in enumerate(raw):
        location = f"{crate_location}.implementation_path_rewrites[{index}]"
        if not isinstance(item, dict) or set(item) != {"from", "to"}:
            diagnostics.append(
                ResolutionDiagnostic(
                    "implementation_rewrite_malformed",
                    location,
                    "rewrite must contain exactly string keys 'from' and 'to'",
                )
            )
            continue
        source = item["from"]
        target = item["to"]
        if (
            not isinstance(source, str)
            or not isinstance(target, str)
            or not source
            or not source.endswith("/")
            or (target and not target.endswith("/"))
            or not _is_safe_prefix(source)
            or not _is_safe_prefix(target)
        ):
            diagnostics.append(
                ResolutionDiagnostic(
                    "implementation_rewrite_malformed",
                    location,
                    "rewrite prefixes must be safe relative directory prefixes ending in '/'",
                )
            )
            continue
        rewrites.append(_Rewrite(source, target))
    return tuple(rewrites)


def _is_safe_prefix(value: str) -> bool:
    return value == "" or _is_safe_relative(value.rstrip("/"))


def _is_safe_relative(value: str) -> bool:
    if not value or "\\" in value or "\x00" in value:
        return False
    path = PurePosixPath(value)
    return (
        not path.is_absolute()
        and ".." not in path.parts
        and ":" not in path.parts[0]
    )


def _is_within(path: Path, parent: Path) -> bool:
    try:
        path.relative_to(parent)
    except ValueError:
        return False
    return True
