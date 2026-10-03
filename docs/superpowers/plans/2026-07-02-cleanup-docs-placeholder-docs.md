---
change: cleanup-docs-placeholder-docs
design-doc: docs/superpowers/specs/2026-07-02-cleanup-docs-placeholder-docs-design.md
base-ref: c1939c253c693bd82f7e7b13a1f3f7b45b9be842
archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

# cleanup-docs-placeholder-docs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all 144 `/// 公开项说明。` placeholder doc-comments in `openlark-docs` with meaningful per-item semantic Chinese doc-comments, and fix 4 `#[derive(...)]` blocks whose trailing `///` must move above the derive.

**Architecture:** Pure doc-comment edits across 14 files in the docs crate. Method is **per-item semantic** (read each item's name + its enum/struct context + Feishu domain knowledge, then write a meaningful Chinese description) — NOT a mechanical `<//! title>+<role>` recipe. 92% of items are enum variants (74) and struct fields (58); the rest are pub struct (4), pub const (3), other (3), pub type (2). 4 of 14 files lack a `//!` header — their doc is derived purely from item context. Work is grouped into 3 domain cohorts (baike / ccm / common+base) sized for subagent dispatch with isolated context.

**Tech Stack:** Rust, `rustdoc`, `cargo doc`, OpenLark codebase conventions (中文-first doc, `///` for items).

## Global Constraints

Copied verbatim from the spec (Design Doc §2 非目标 + §3 方案):

- **Do NOT change logic or signatures.** Only doc-comments and (for 4 sites) doc-comment position. No `impl`, no `pub fn`, no struct field, no enum body changes.
- **Do NOT touch `application` or `small-crates`.** Those are separate changes (mechanical `<//! title>+<role>` pattern via #1). This plan is `openlark-docs` only.
- **Do NOT name-pile.** A doc like `/// OpenId` (echoing the variant name) is a failure. Read the item's name + its enum/struct context + Feishu common sense and write a *meaningful* description. When genuinely uncertain, prefer a concise accurate description over a verbose guess.
- **Chinese doc-comments only.** Project convention is 中文-first for docs. Format: `/// <中文描述>` (single line, trailing period optional, match surrounding style).
- **Only 4 derive-position fixes** — exactly the 4 sites identified in勘探 (baike/baike/v1/entity/create.rs:18-19, baike/baike/v1/entity/update.rs:18-19, baike/lingo/v1/entity/match.rs:56-57, ccm/drive/v1/file/delete.rs:17-18). Do NOT hunt for more derive cases.
- **No `unwrap()`/`expect()` additions** (none expected — pure doc edits).
- **Frequent commits.** One commit per completed task cohort.

## File Structure

14 files, grouped into 3 execution cohorts. Per-file placeholder counts (verified by `grep -c`):

### Cohort A — baike (7 files, 17 placeholders, 3 derive-fixes)
| File | Count | Has `//!`? | Derive-fix? |
|------|-------|-----------|-------------|
| `crates/openlark-docs/src/baike/lingo/v1/models.rs` | 9 | No | — |
| `crates/openlark-docs/src/baike/lingo/v1/entity/update.rs` | 1 | Yes | — |
| `crates/openlark-docs/src/baike/lingo/v1/entity/create.rs` | 1 | Yes | — |
| `crates/openlark-docs/src/baike/lingo/v1/entity/match.rs` | 1 | Yes | **YES (L56-57)** |
| `crates/openlark-docs/src/baike/lingo/v1/entity/get.rs` | 1 | Yes | — |
| `crates/openlark-docs/src/baike/baike/v1/entity/create.rs` | 2 | Yes | **YES (L18-19)** |
| `crates/openlark-docs/src/baike/baike/v1/entity/update.rs` | 2 | Yes | **YES (L18-19)** |

### Cohort B — ccm (4 files, 48 placeholders)
| File | Count | Has `//!`? | Derive-fix? |
|------|-------|-----------|-------------|
| `crates/openlark-docs/src/ccm/sheets/v3/spreadsheet/models.rs` | 43 | No | — |
| `crates/openlark-docs/src/ccm/docx/models.rs` | 3 | No | — |
| `crates/openlark-docs/src/ccm/docs/v1/content/get.rs` | 1 | No | — |
| `crates/openlark-docs/src/ccm/drive/v1/file/delete.rs` | 1 | Yes | **YES (L17-18)** |

### Cohort C — common + base (2 files, 78 placeholders)
| File | Count | Has `//!`? | Derive-fix? |
|------|-------|-----------|-------------|
| `crates/openlark-docs/src/common/api_endpoints.rs` | 73 | Yes (rich) | — |
| `crates/openlark-docs/src/common/chain.rs` | 5 | Yes | — |
| `crates/openlark-docs/src/base/bitable/v1/field_types.rs` | 1 | No | — |

**Total:** 14 files, 144 placeholders, 4 derive-fixes. (3 cohorts are the subagent dispatch units; the table above is also each implementer's index of which files to touch.)

### Doc derivation method (apply uniformly)

For each `/// 公开项说明。` placeholder, read up 1–5 lines to know the item:

| Item type | How to derive the doc | Concrete example |
|-----------|----------------------|------------------|
| **enum variant** (74) | variant name + enclosing enum's doc + Feishu semantics | `UserIdType::OpenId` → `/// 开放平台用户 ID` |
| **struct field** (58) | field name + enclosing struct's doc + type | `pub app_id: String` → `/// 应用 ID` |
| **pub struct** (4) | struct name + the API it belongs to | `CreateEntityReq` → `/// 创建词条请求体。` |
| **pub const** (3) | name + what it constant-folds | (per name) |
| **other** (3, e.g. trait/impl items) | name + role | (per name) |
| **pub type** (2) | name + alias meaning | (per name) |

Files without `//!` header (4): doc derives purely from item context — do NOT add a `//!` header (out of scope; would change the file).

### Derive-position fix (exact rewrite, all 4 sites)

**Before:**
```rust
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// 公开项说明。
pub struct CreateEntityReq {
```
**After** (semantic doc replaces placeholder AND moves above derive):
```rust
/// 创建词条请求体。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEntityReq {
```
The exact derive list varies per file (`Debug, Clone, PartialEq` for the enum in match.rs etc.) — preserve the existing derive list verbatim; only the doc line moves up and gets its text replaced.

archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

## Task 1: Cohort A — baike (7 files, 17 placeholders, 3 derive-fixes)

**Files:**
- Modify: `crates/openlark-docs/src/baike/lingo/v1/models.rs` (9 placeholders — largest in cohort)
- Modify: `crates/openlark-docs/src/baike/lingo/v1/entity/update.rs` (1)
- Modify: `crates/openlark-docs/src/baike/lingo/v1/entity/create.rs` (1)
- Modify: `crates/openlark-docs/src/baike/lingo/v1/entity/match.rs` (1, **derive-fix L56-57**)
- Modify: `crates/openlark-docs/src/baike/lingo/v1/entity/get.rs` (1)
- Modify: `crates/openlark-docs/src/baike/baike/v1/entity/create.rs` (2, **derive-fix L18-19**)
- Modify: `crates/openlark-docs/src/baike/baike/v1/entity/update.rs` (2, **derive-fix L18-19**)

**Interfaces:**
- Consumes: nothing (first task, pure doc edits).
- Produces: baike domain files with zero placeholders and 3 derive-fixes applied. Later cohorts are independent; no cross-cohort type contract.

- [x] **Step 1: Read each of the 7 files to map every placeholder to its item**

Run for each file (example for the largest):
```bash
grep -n '公开项说明' crates/openlark-docs/src/baike/lingo/v1/models.rs
```
Then for each line number, read ~6 lines of context above to identify the item (enum variant / struct field / struct). Read the enclosing `enum`/`struct` declaration and its existing doc (if any) to inform the per-item text.

Expected: a per-item mental map of all 17 placeholders in the cohort.

- [x] **Step 2: Replace placeholders with per-item semantic Chinese doc**

For each placeholder, apply the doc-derivation method (table in File Structure):
- enum variant → `/// <变体代表的 Feishu 实体>`
- struct field → `/// <字段含义>`
- pub struct (the 4 derive-fix sites are pub struct) → `/// <struct 职责>。`

Apply the **4-rewrite derive-fix** for the 3 cohort sites (baike/baike create + update, baike/lingo match): move `///` above `#[derive(...)]` AND replace the placeholder text with semantic doc. Preserve the existing derive list verbatim.

Concrete instance for `baike/baike/v1/entity/create.rs:18-19`:
```rust
// BEFORE
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// 公开项说明。
pub struct CreateEntityReq {
// AFTER
/// 创建词条请求体。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEntityReq {
```
(The 2nd placeholder in the same file, and the items in update.rs/match.rs, follow the same pattern — read each item's name to pick the right semantic text.)

- [x] **Step 3: Verify cohort grep gate is empty**

Run:
```bash
grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/baike/
```
Expected: **empty output** (no matches).

- [x] **Step 4: Verify derive-position gate holds for cohort**

Run:
```bash
cd /Users/zool/workspace/openlark
for f in crates/openlark-docs/src/baike/baike/v1/entity/create.rs \
         crates/openlark-docs/src/baike/baike/v1/entity/update.rs \
         crates/openlark-docs/src/baike/lingo/v1/entity/match.rs; do
  awk '/^#\[derive\(/ { d=NR; next } /\/\/\// && d==NR-1 { print FILENAME": derive-then-doc at line "NR }' "$f"
done
```
Expected: **empty output** (no derive-then-doc sites remaining).

- [x] **Step 5: Verify cargo doc passes for the crate**

Run:
```bash
cargo doc -p openlark-docs --all-features --no-deps 2>&1 | grep -E 'warning|error' || echo "OK: no warnings"
```
Expected: `OK: no warnings` (or only pre-existing unrelated warnings — no new `missing_docs`).

- [x] **Step 6: Commit**

```bash
git add crates/openlark-docs/src/baike/
git commit -m "docs(baike): 替换 17 处占位 doc 为逐项语义 + 修 3 处 derive 后置

cleanup-docs-placeholder-docs Cohort A (baike/lingo + baike/baike)。
- 替换 /// 公开项说明。 为 enum variant / struct field / pub struct 语义 doc
- 修正 3 处 #[derive] 后置 /// 位置（baike create/update + lingo match）
- grep 守门 + 位置守门 + cargo doc 0 警告通过"
```

archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

## Task 2: Cohort B — ccm (4 files, 48 placeholders, 1 derive-fix)

**Files:**
- Modify: `crates/openlark-docs/src/ccm/sheets/v3/spreadsheet/models.rs` (43 — largest in whole change, no `//!`)
- Modify: `crates/openlark-docs/src/ccm/docx/models.rs` (3, no `//!`)
- Modify: `crates/openlark-docs/src/ccm/docs/v1/content/get.rs` (1, no `//!`)
- Modify: `crates/openlark-docs/src/ccm/drive/v1/file/delete.rs` (1, **derive-fix L17-18**, has `//!`)

**Interfaces:**
- Consumes: nothing from Task 1 (independent files).
- Produces: ccm domain files with zero placeholders and 1 derive-fix. Task 3 is also independent; no cross-cohort contract.

**Note on the 43-item `sheets/v3/spreadsheet/models.rs`:** This file is the bulk of the cohort (43 of 48). It has no `//!` header — every doc derives from the enclosing struct/enum. Read struct-by-struct; most items are spreadsheet model fields (sheet_id, row, column, cell, range, etc. — Feishu Sheets domain). Do NOT rush — each of the 43 needs a per-item semantic line.

- [x] **Step 1: Map all 48 placeholders to their items**

Run for each file:
```bash
grep -n '公开项说明' crates/openlark-docs/src/ccm/sheets/v3/spreadsheet/models.rs
grep -n '公开项说明' crates/openlark-docs/src/ccm/docx/models.rs
grep -n '公开项说明' crates/openlark-docs/src/ccm/docs/v1/content/get.rs
grep -n '公开项说明' crates/openlark-docs/src/ccm/drive/v1/file/delete.rs
```
For each line number, read ~6 lines above to identify the item. The 43 in sheets/models.rs are mostly struct fields within spreadsheet model structs — read each enclosing struct's name + the field's type to derive the doc.

- [x] **Step 2: Replace placeholders with per-item semantic Chinese doc**

Apply the doc-derivation method. For the no-`//!` files, the doc text is purely item-context-derived — do NOT add a `//!` header.

Apply the **derive-fix** for `ccm/drive/v1/file/delete.rs:17-18`:
```rust
// BEFORE
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// 公开项说明。
pub struct DeleteFileReq {   // (actual struct name may differ — read it)
// AFTER
/// <按 struct 名+API 语义的中文 doc>。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteFileReq {
```
(Preserve the existing derive list verbatim; only the doc line moves up + text replaces.)

- [x] **Step 3: Verify cohort grep gate is empty**

Run:
```bash
grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/ccm/
```
Expected: **empty output**.

- [x] **Step 4: Verify derive-position gate holds for cohort**

Run:
```bash
awk '/^#\[derive\(/ { d=NR; next } /\/\/\// && d==NR-1 { print FILENAME": derive-then-doc at line "NR }' \
  crates/openlark-docs/src/ccm/drive/v1/file/delete.rs
```
Expected: **empty output**.

- [x] **Step 5: Verify cargo doc passes for the crate**

Run:
```bash
cargo doc -p openlark-docs --all-features --no-deps 2>&1 | grep -E 'warning|error' || echo "OK: no warnings"
```
Expected: `OK: no warnings`.

- [x] **Step 6: Commit**

```bash
git add crates/openlark-docs/src/ccm/
git commit -m "docs(ccm): 替换 48 处占位 doc 为逐项语义 + 修 1 处 derive 后置

cleanup-docs-placeholder-docs Cohort B (ccm sheets/docx/docs/drive)。
- sheets/v3/spreadsheet/models.rs 43 处 (主要工作)
- docx/models 3 + docs/content/get 1 + drive/file/delete 1
- 修正 ccm/drive/v1/file/delete.rs derive 后置
- 4 个无 //! 头文件从 item 上下文派生 doc
- grep + 位置守门 + cargo doc 0 警告通过"
```

archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

## Task 3: Cohort C — common + base (3 files, 79 placeholders)

**Files:**
- Modify: `crates/openlark-docs/src/common/api_endpoints.rs` (73 — largest in whole change, rich `//!` header)
- Modify: `crates/openlark-docs/src/common/chain.rs` (5)
- Modify: `crates/openlark-docs/src/base/bitable/v1/field_types.rs` (1, no `//!`)

**Interfaces:**
- Consumes: nothing from Tasks 1–2 (independent).
- Produces: common + base files with zero placeholders. This task completes the placeholder cleanup — Task 4's whole-crate gates depend on all 3 cohorts being done.

**Note on `common/api_endpoints.rs` (73 items):** This file is a typed enum endpoint registry with rich `//!` and many already-documented sibling variants. The 73 placeholders are the *undocumented* variants scattered across many enums. Each one needs a short semantic doc derived from the variant name + the enum's existing doc + the URL pattern the variant represents. Lean on already-documented siblings in the same enum for style — e.g. if `RoleCreate` is `/// 新增自定义角色`, then `RoleDelete(String)` → `/// 删除自定义角色`. **Do NOT bulk-replace with a uniform phrase** — read each variant.

- [x] **Step 1: Map all 79 placeholders to their items**

Run:
```bash
grep -n '公开项说明' crates/openlark-docs/src/common/api_endpoints.rs
grep -n '公开项说明' crates/openlark-docs/src/common/chain.rs
grep -n '公开项说明' crates/openlark-docs/src/base/bitable/v1/field_types.rs
```
For each `api_endpoints.rs` line, read the enclosing `enum` declaration and the variant name + any already-documented siblings. This is the bulk of the work; allocate the most context budget here.

- [x] **Step 2: Replace placeholders with per-item semantic Chinese doc**

Apply the doc-derivation method. For `api_endpoints.rs`, mirror the style of existing sibling variant docs in the same enum (single line `/// <动词>+<资源>` like `/// 新增自定义角色`).

For `base/bitable/v1/field_types.rs` (1 item, no `//!`): derive purely from the item (likely a Bitable field-type variant or constant — read it).

- [x] **Step 3: Verify cohort grep gate is empty**

Run:
```bash
grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/common/ crates/openlark-docs/src/base/
```
Expected: **empty output**.

- [x] **Step 4: Verify cargo doc passes for the crate**

Run:
```bash
cargo doc -p openlark-docs --all-features --no-deps 2>&1 | grep -E 'warning|error' || echo "OK: no warnings"
```
Expected: `OK: no warnings`.

- [x] **Step 5: Commit**

```bash
git add crates/openlark-docs/src/common/ crates/openlark-docs/src/base/
git commit -m "docs(common,base): 替换 79 处占位 doc 为逐项语义

cleanup-docs-placeholder-docs Cohort C (common api_endpoints + chain + base bitable)。
- common/api_endpoints.rs 73 处 (typed enum endpoint 变体, 主工作)
- common/chain.rs 5 + base/bitable/v1/field_types.rs 1
- base/bitable 无 //! 头从 item 上下文派生
- grep + cargo doc 0 警告通过"
```

archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

## Task 4: Final workspace gates + regression (after all 3 cohorts merged)

**Files:**
- Verify-only (no edits) — runs the whole-crate gates from Design Doc §3 验证.

**Interfaces:**
- Consumes: Tasks 1–3 all complete and committed (144 placeholders replaced, 4 derive-fixes applied).
- Produces: verified-clean docs crate, ready for the change's verify phase.

- [x] **Step 1: Whole-crate grep gate is empty**

Run:
```bash
grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/
```
Expected: **empty output** (all 144 replaced). This is the primary completion signal.

- [x] **Step 2: Whole-crate derive-position gate**

Run (sweeps all docs crate, not just the 4 known sites):
```bash
cd /Users/zool/workspace/openlark
# Find any #[derive(...)] line whose immediate next line starts with ///
found=0
while IFS= read -r f; do
  awk '/^#\[derive\(/ { d=NR; next } /^\/\/\// && d==NR-1 { print FILENAME":"NR; found=1 } END {}' "$f"
done < <(grep -rl '公开项说明\|^#\[derive' crates/openlark-docs/src/ 2>/dev/null | sort -u)
echo "---done---"
```
Expected: only `---done---` (no derive-then-doc lines anywhere). The 4 known sites were fixed in Tasks 1–2; this confirms no others regressed.

- [x] **Step 3: Workspace cargo doc — 0 missing_docs warnings**

Run:
```bash
cargo doc -p openlark-docs --all-features --no-deps 2>&1 | tee /tmp/docs-crate-doc.log
grep -c 'missing_docs' /tmp/docs-crate-doc.log
```
Expected: `0` (no missing_docs warnings in the docs crate).

Then workspace-wide (slower, but the spec calls for it):
```bash
cargo doc --workspace --all-features --no-deps 2>&1 | tee /tmp/workspace-doc.log
grep -c 'warning: missing_docs\|warning: unresolved' /tmp/workspace-doc.log
```
Expected: `0` for missing_docs in docs-crate items (unrelated pre-existing warnings elsewhere are out of scope — note them but don't try to fix).

- [x] **Step 4: Regression — fmt + lint + docs tests**

Run:
```bash
cargo fmt --check
just lint
cargo test -p openlark-docs
```
Expected: all pass. (`cargo fmt --check` is mandatory — CI lint job runs it first; pure doc edits must not perturb formatting. `just lint` runs clippy across the workspace. docs-crate tests must not break.)

If `just lint` reports pre-existing failures unrelated to this change, note them and continue — do NOT fix unrelated clippy lints (scope creep).

- [x] **Step 5: Semantic quality spot-check (review, not mechanical)**

This step is for the reviewer (human or code-review subagent), not the implementer. Sample 5–10 items per cohort and confirm the doc is meaningful Chinese (not the placeholder, not a bare name-pile):
```bash
# Sample some variant/field docs from each cohort
sed -n '<sample-range>p' crates/openlark-docs/src/common/api_endpoints.rs
sed -n '<sample-range>p' crates/openlark-docs/src/ccm/sheets/v3/spreadsheet/models.rs
sed -n '<sample-range>p' crates/openlark-docs/src/baike/lingo/v1/models.rs
```
Expected: every sampled doc reads as a meaningful Feishu-domain description. If any reads as a name-pile (e.g. `/// AppId`), rewrite it.

- [x] **Step 6: Final commit (only if Step 5 required rewrites)**

If the spot-check found name-pile docs that were rewritten, commit the fixes:
```bash
git add -A crates/openlark-docs/src/
git commit -m "docs(openlark-docs): 语义质量抽查修正

cleanup-docs-placeholder-docs Task 4 Step 5 抽查发现的名称堆砌修正。"
```
If no rewrites were needed, skip this step (no empty commits).

- [x] **Step 7: Confirm tasks.md checkboxes can be ticked**

Verify against `openspec/changes/cleanup-docs-placeholder-docs/tasks.md`:
- §1.1–1.3 done by Tasks 1–3 ✓
- §2.1 (grep gate) = Task 4 Step 1 ✓
- §2.2 (derive-position gate) = Task 4 Step 2 ✓
- §3.1 (workspace cargo doc + fmt + lint) = Task 4 Steps 3–4 ✓
- §3.2 (docs tests not broken) = Task 4 Step 4 ✓

The tasks.md text still says `<//! 标题>+<item 角色>` (legacy from open phase) — the Design Doc §1 overrides this with **per-item semantic** (the open-phase assumption was推翻'd). Do NOT edit tasks.md prose; the勾选 semantics still map cleanly. (If the change owner prefers, tasks.md can be patched separately — out of this plan's scope.)

Expected: all 7 sub-items in tasks.md are tickable. Hand off to the change's verify phase.

archived-with: 2026-07-02-cleanup-docs-placeholder-docs
---

## Self-Review (run after writing, before handoff)

**1. Spec coverage (Design Doc §3 验证):**
- grep 守门 → Task 4 Step 1 ✓
- 位置守门 (`#[derive]` 后不紧跟 `///`) → Task 4 Step 2 ✓ (per-cohort gates in Tasks 1–2 Steps 4)
- cargo doc 0 警告 (docs crate) → Tasks 1–3 Step 5 + Task 4 Step 3 ✓
- workspace 0 → Task 4 Step 3 ✓
- 回归 (fmt + lint + tests) → Task 4 Step 4 ✓
- 语义质量抽查 → Task 4 Step 5 ✓
- 14 文件全覆盖 → Tasks 1–3 file lists enumerate all 14 ✓
- 144 占位全替换 → sum of per-file counts = 144, all in tasks ✓
- 4 derive-fix → Task 1 (3 sites) + Task 2 (1 site) = 4 ✓

**2. Placeholder scan:** No "TBD"/"implement later" in this plan. Every step has the exact command or rewrite pattern. The only variance is per-item doc *text* (inherently variable — the method table and concrete example pin the approach).

**3. Type/name consistency:** No new types/functions introduced (pure doc edits). The 4 derive-fix sites all use the same before/after shape; the only per-site variance is the derive list (preserved verbatim) and the struct name (read from file).

**Gaps:** None. The plan covers all 14 files, all 144 placeholders, all 4 derive-fixes, and all 5 verification gates from the Design Doc.
