# OpenLark API Contract

This context defines the language used to compare OpenLark's typed Rust contracts with authoritative Feishu/Lark documentation.

## Language

**Catalog Entry**:
The authoritative identity and metadata for one Feishu/Lark operation and its official document. Official Document Evidence is always produced for exactly one Catalog Entry.
_Avoid_: API record, CSV row

**Official Document Evidence**:
Normalized observations derived from the authoritative official document for a Catalog entry, including provenance, document health, and confidence. It excludes the Rust contract comparison and any pass/fail verdict.
_Avoid_: fetched document, page text, verification result

**Official Document Snapshot**:
An immutable capture of an official document together with its acquisition provenance. It is input to document-health assessment and interpretation, and is not Official Document Evidence until those checks succeed.
_Avoid_: cache file, page text, evidence

**Recorded Snapshot**:
A versioned, immutable Official Document Snapshot fixture containing the raw official representation and acquisition provenance for offline evidence production. It never stores parsed Evidence.
_Avoid_: expected Evidence, inline mock response

**Trusted Evidence**:
Official Document Evidence whose provenance and document health are established and whose relevant structure was interpreted successfully. It may legitimately contain no fields.
_Avoid_: successful fetch, non-empty evidence

**Incomplete Evidence**:
Official Document Evidence from an authoritative, healthy document whose relevant structure could not be interpreted completely. Its partial observations may support diagnostic findings, but cannot establish a match or passing verdict.
_Avoid_: parse warning, empty result

**Unavailable Evidence**:
The state in which no official document snapshot could be acquired for a Catalog entry.
_Avoid_: missing fields, empty evidence

**Rejected Evidence**:
An acquired document snapshot that fails provenance or document-health requirements and therefore cannot serve as Official Document Evidence.
_Avoid_: fetch error, incomplete evidence

**Strict Evidence Gate**:
A verification mode in which only Trusted Evidence can support a passing verdict. Incomplete, Unavailable, and Rejected Evidence are always non-passing, even when they still provide diagnostic information.
_Avoid_: no errors found, warning-only pass

**Evidence Acquisition Policy**:
The caller-declared requirement for acquiring an Official Document Snapshot: fresh from the official source, preferably from a provenance-matching snapshot, or exclusively from a designated recorded snapshot. The policy does not alter the resulting Evidence Status.
_Avoid_: cache mode, global TTL

**Official Evidence Source**:
The official representation actually used to produce Official Document Evidence. Structured Detail is primary; Rendered Document is a fallback when the primary source cannot produce Trusted Evidence for the requested dimension, and observations from different sources are never silently merged.
_Avoid_: blended evidence, source-agnostic fields

**Evidence Dimension**:
An independently assessed category of official contract knowledge: Endpoint, Request Fields, Response Fields, or Tokens. Each dimension has its own Evidence Status, provenance, and observations.
_Avoid_: document-wide status, validation category

**Field Observation**:
A normalized request or response field identified by its canonical hierarchical path, location, requiredness, type, and Official Evidence Source. Attributes not established by the source remain unknown rather than inferred.
_Avoid_: flat field name, guessed field

**Evidence Provenance**:
The reproducibility record for Official Document Evidence. Snapshot provenance identifies the Catalog entry, actual source, acquisition time, and content digest; interpretation provenance identifies the snapshot digest, interpreter revision, and Evidence Dimension.
_Avoid_: source URL only, report timestamp

**Acquisition Trail**:
The ordered record of source attempts made while obtaining an Evidence Dimension. A successful fallback may produce Trusted Evidence while retaining failed primary attempts for diagnosis; only the selected source contributes observations.
_Avoid_: merged evidence, discarded fallback reason

**Evidence Diagnostic**:
A stable, source-agnostic reason attached to an Evidence Status or Acquisition Trail entry. It describes evidence production and is distinct from caller-specific finding codes and comparison verdicts.
_Avoid_: finding code, error message, verdict
