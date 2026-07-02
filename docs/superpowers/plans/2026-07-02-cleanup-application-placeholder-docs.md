---
change: cleanup-application-placeholder-docs
design-doc: docs/superpowers/specs/2026-07-02-cleanup-application-placeholder-docs-design.md
base-ref: c1313b2a129f0fa74df1a10b1bd02f417204fc06
---

# cleanup-application-placeholder-docs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all 578 `/// 待补充文档。` placeholder doc comments in the `openlark-application` crate with meaningful Chinese doc, and fix the position of 190 struct doc comments (move `///` from after `#[derive(...)]` to before it).

**Architecture:** Pure mechanical recipe from #1 analytics (no logic changes). Each placeholder becomes `<API 中文名 from file //! header> + <item 角色>` per an 8-row recipe table. Structs additionally get a 3-line swap (doc moved above `#[derive]`). Executed subagent-driven, grouped by version × sub-domain into 8 groups (G1–G8) with hard boundaries = exact file sets, so coverage is trivially verifiable via per-group `grep`. A pilot file (G0) validates the recipe and position transform before batch work.

**Tech Stack:** Rust, serde, openlark-core. Verification via `cargo doc`, `cargo fmt --check`, `just lint`, `grep` gates.

## Global Constraints

These apply to every task; copy values verbatim:

- **No logic changes.** Only doc-comment text edits + struct doc-position swaps. Do not touch function bodies, signatures, derives, serde attrs, imports, or any non-doc line.
- **Only `crates/openlark-application/src/`.** Do not edit any other crate. The 91 files are all under `crates/openlark-application/src/application/application/v{1,5,6}/<sub-domain>/` plus `crates/openlark-application/src/application/application/mod.rs`.
- **Recipe text is fixed.** Use exactly the 8-row recipe table below. Do not invent placeholders, do not write generic prose, do not translate the recipe wording. Chinese doc only.
- **`<API 中文名>` comes from the file's `//!` header line.** E.g. `//! app create` → "创建应用". Read each file's `//!` first line to get the API name.
- **named field doc (17 distinct names): read the field name, translate to Chinese per Feishu domain sense.** Reference translations below; implementer applies, reviewer spot-checks.
- **struct doc position: `///` moves from between `#[derive(...)]` and `pub struct` to ABOVE `#[derive(...)]`.** 3-line swap, 190 sites, all single-`#[derive]` (0 multi-attr boundaries verified).
- **No `unwrap()`/`expect()`** (naturally satisfied — this change writes no code, only doc).
- **Commit after each task** (after both reviewer gates pass). Commit message format: `docs(application): G<n> <sub-domain> 占位→有义 doc (<count>)` (or `docs(application): pilot <file> recipe 验证` for G0).
- **Each implementer self-verifies group placeholder count = 0 before handing to reviewer.**

### The Recipe (fixed 8-row table — verbatim)

| item | doc text (replace `/// 待补充文档。` with) | trigger (the line immediately after the placeholder) |
|------|------------------------------------------|------------------------------------------------------|
| Request struct | `<API 中文名>的请求。` | `pub struct XxxRequest` |
| Response struct | `<API 中文名>的响应。` | `pub struct XxxResponse` |
| field `data` | `响应数据。` | `pub data:` |
| field (named) | `<字段中文名>。` | any other `pub <name>:` — read `name`, translate |
| `fn new` | `创建请求实例。` | `pub fn new` |
| `fn execute` | `执行<API>请求。` | `pub async fn execute` |
| `fn execute_with_options` | `带自定义请求选项执行。` | `pub async fn execute_with_options` |
| module | `<子模块 API 说明>。` | `pub mod` |

API 中文名 取自文件第一行 `//! ...` 头（去掉版本前缀，翻译成中文动词+宾语）。

### named field translation reference (17 distinct names — implementer applies, reviewer spot-checks)

| field name | Chinese |
|------------|---------|
| `app_id` | 应用 ID |
| `app_name` | 应用名称 |
| `app_type` | 应用类型 |
| `description` | 描述 |
| `badge` | 徽标 |
| `scope_type` | 权限范围类型 |
| `reason` | 原因说明 |
| `contacts_range` | 通讯录范围 |
| `apps` | 应用列表 |
| `rules` | 规则列表 |
| `rule_id` | 规则 ID |
| `rule_name` | 规则名称 |
| `new_owner_id` | 新所有者 ID |
| `total_push_count` | 总推送数 |
| `success_count` | 成功数 |
| `failed_count` | 失败数 |
| (any other named) | read + translate per Feishu sense |

### struct doc position transform (verbatim, applied to all 190 struct placeholders)

```rust
// BEFORE (current — doc after derive)
#[derive(Debug, Clone)]
/// 待补充文档。
pub struct CreateAppRequest {

// AFTER (target — doc above derive, with recipe text filled in)
/// 创建应用的请求。
#[derive(Debug, Clone)]
pub struct CreateAppRequest {
```

For fn / field / module placeholders (388 sites), position is already correct — only swap the text in place.

---

## File Structure & Grouping

All 91 files live under `crates/openlark-application/src/application/application/`. The 8 groups partition them by version × sub-domain with hard file-set boundaries. Group totals: G1=65, G2=70, G3=78, G4=13, G5=138, G6=80, G7=84, G8=50. Sum = 578. ✓

**Note on G5 sizing:** G5 (`v6/application/**`, 138 placeholders) runs above the ~88/group target. Kept intact because `v6/application/` is one cohesive sub-domain with shared `mod.rs` and sub-subdirs; splitting it would cut across module boundaries for marginal balance. Implementer proceeds normally; reviewer gates are the same.

The base path prefix for every file below is `crates/openlark-application/src/application/application/`.

---

## Task 0 (G0): Pilot — validate recipe + position transform on 1 file

**Files:**
- Modify: `crates/openlark-application/src/application/application/v6/app/create.rs` (6 placeholders)

**Why pilot first:** 578-site mechanical batch is high-risk for偷懒 (lazy/generic doc). One file proves the recipe produces real, per-item doc and the struct swap is clean before committing to the batch.

- [ ] **Step 1: Read the pilot file and its `//!` header**

Run: `cat crates/openlark-application/src/application/application/v6/app/create.rs`
Header is `//! app create` → API 中文名 = "创建应用".

Expected placeholders (6, all confirmed by exploration):
- 2 struct placeholders (`CreateAppRequest`, `CreateAppResponse`) — both currently after `#[derive(...)]`, need position swap + recipe text.
- 1 field `data` placeholder (`pub data:`) → `响应数据。`, in place.
- `fn new` → `创建请求实例。`, in place.
- `fn execute` → `执行创建应用请求。`, in place.
- `fn execute_with_options` → `带自定义请求选项执行。`, in place.

- [ ] **Step 2: Apply the 6 edits**

For the 2 structs, swap doc above derive AND fill recipe text:

```rust
/// 创建应用的请求。
#[derive(Debug, Clone)]
pub struct CreateAppRequest {
```
```rust
/// 创建应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAppResponse {
```

For the field `data` and the 3 fns, replace text in place (`响应数据。`, `创建请求实例。`, `执行创建应用请求。`, `带自定义请求选项执行。`).

- [ ] **Step 3: Self-verify — pilot file has zero placeholders**

Run: `grep -n '/// 待补充文档。' crates/openlark-application/src/application/application/v6/app/create.rs`
Expected: no output (empty).

- [ ] **Step 4: Verify pilot file compiles clean under cargo doc**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | grep -E "(create\.rs|missing_docs|warning)" || echo "clean"`
Expected: `clean` (no warning mentioning `create.rs` or `missing_docs`).

- [ ] **Step 5: Verify position gate on the pilot file**

Run: `grep -nA1 '^#\[derive' crates/openlark-application/src/application/application/v6/app/create.rs | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v6/app/create.rs
git commit -m "docs(application): pilot v6/app/create recipe 验证 (6)"
```

**Reviewer gates (G0):**
- spec compliance: the 6 doc texts match the recipe table exactly; struct doc is above `#[derive]`.
- quality: doc reads as natural Chinese, references the real API name "创建应用".

---

## Task 1 (G1): v1 — app / app_version / app_badge

**Files (10, prefix `v1/`):**
- `app/create.rs`, `app/delete.rs`, `app/list.rs`, `app/patch.rs`
- `app_version/create.rs`, `app_version/delete.rs`, `app_version/get.rs`, `app_version/list.rs`, `app_version/patch.rs`
- `app_badge/set.rs`

**Placeholders: 65** (app=24, app_version=30, app_badge=11).

**Interfaces:** none (doc-only).

- [ ] **Step 1: For each of the 10 files, read `//!` header to get API 中文名**

Example headers: `//! app create`→"创建应用", `//! app version list`→"获取应用版本列表", `//! app badge set`→"设置应用徽标". Read each file to confirm exact header before filling recipe.

- [ ] **Step 2: Apply recipe to all 65 placeholders in these 10 files**

Per the 8-row recipe table. Every struct placeholder gets BOTH the position swap (doc above `#[derive]`) and the recipe text. named fields in this set: `app_id`→"应用 ID", `badge`→"徽标" (in `app_badge/set.rs`). `data` fields → "响应数据。".

- [ ] **Step 3: Self-verify — these 10 files have zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v1/app/ crates/openlark-application/src/application/application/v1/app_version/ crates/openlark-application/src/application/application/v1/app_badge/`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — no struct doc left after derive in these files**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v1/app/ crates/openlark-application/src/application/application/v1/app_version/ crates/openlark-application/src/application/application/v1/app_badge/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check the crate**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: no `missing_docs` warning, no errors.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v1/app/ crates/openlark-application/src/application/application/v1/app_version/ crates/openlark-application/src/application/application/v1/app_badge/
git commit -m "docs(application): G1 v1 app/app_version/app_badge 占位→有义 doc (65)"
```

**Reviewer gates (G1):**
- spec compliance: all 65 sites match recipe; struct doc above derive; named fields translated per table.
- quality: API names traceable to `//!` headers; spot-check 2 files end-to-end.

---

## Task 2 (G2): v1 — collaborator / owner / app_recommend_rule / app_usage / frequently_used

**Files (6, prefix `v1/`):**
- `collaborator/create.rs`, `collaborator/delete.rs`, `collaborator/list.rs`
- `owner/recommended.rs`, `owner/transfer.rs`
- `app_recommend_rule/list.rs`, `app_usage/message_push_overview.rs`, `frequently_used/get.rs`

**Placeholders: 70** (collaborator=18, owner=20, app_recommend_rule=11, app_usage=10, frequently_used=11).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read each file's `//!` header for API 中文名**

Examples: `//! collaborator create`→"添加协作者", `//! owner transfer`→"转让所有者", `//! app recommend rule list`→"获取应用推荐规则列表".

- [ ] **Step 2: Apply recipe to all 70 placeholders in these 8 files**

named fields in this set: `new_owner_id`→"新所有者 ID" (owner/transfer.rs), `apps`→"应用列表" (owner/recommended.rs, frequently_used/get.rs), `rules`→"规则列表", `rule_id`→"规则 ID", `rule_name`→"规则名称" (app_recommend_rule/list.rs), `total_push_count`→"总推送数", `success_count`→"成功数", `failed_count`→"失败数" (app_usage/message_push_overview.rs). Struct placeholders get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders in these files**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v1/collaborator/ crates/openlark-application/src/application/application/v1/owner/ crates/openlark-application/src/application/application/v1/app_recommend_rule/ crates/openlark-application/src/application/application/v1/app_usage/ crates/openlark-application/src/application/application/v1/frequently_used/`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v1/{collaborator,owner,app_recommend_rule,app_usage,frequently_used}/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: no `missing_docs` warning, no errors.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v1/{collaborator,owner,app_recommend_rule,app_usage,frequently_used}/
git commit -m "docs(application): G2 v1 collaborator/owner/recommend/usage/freq 占位→有义 doc (70)"
```

**Reviewer gates (G2):** same shape as G1.

---

## Task 3 (G3): v1 — feedback / app_visibility / management / contacts_range / application / usage / visibility

**Files (14, prefix `v1/`):**
- `feedback/create.rs`, `feedback/delete.rs`, `feedback/get.rs`, `feedback/list.rs`
- `app_visibility/get.rs`, `app_visibility/patch.rs`
- `management/get.rs`, `management/patch.rs`
- `contacts_range/get.rs`
- `application/contacts_range_configuration.rs`
- `usage/get.rs`
- `visibility/check.rs`

**Placeholders: 78** (feedback=24, app_visibility=12, management=12, contacts_range=9, application=9, usage=6, visibility=6).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read each file's `//!` header**

Examples: `//! feedback create`→"创建反馈", `//! management get`→"获取应用管理信息", `//! visibility check`→"检查可见性".

- [ ] **Step 2: Apply recipe to all 78 placeholders**

named fields in this set: `app_id`→"应用 ID", `contacts_range`→"通讯录范围" (contacts_range/get.rs, application/contacts_range_configuration.rs). Struct placeholders get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v1/feedback/ crates/openlark-application/src/application/application/v1/app_visibility/ crates/openlark-application/src/application/application/v1/management/ crates/openlark-application/src/application/application/v1/contacts_range/ crates/openlark-application/src/application/application/v1/application/ crates/openlark-application/src/application/application/v1/usage/ crates/openlark-application/src/application/application/v1/visibility/`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v1/{feedback,app_visibility,management,contacts_range,application,usage,visibility}/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v1/{feedback,app_visibility,management,contacts_range,application,usage,visibility}/
git commit -m "docs(application): G3 v1 feedback/visibility/management/cr/app/usage 占位→有义 doc (78)"
```

**Reviewer gates (G3):** same shape.

---

## Task 4 (G4): v5 + root mod.rs

**Files (3):**
- `v5/application/recommend.rs`
- `v5/application/favourite.rs`
- `mod.rs` (the crate's `application/application/mod.rs`, 1 module placeholder)

**Placeholders: 13** (v5=12, root mod=1).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read headers**

`//! application recommend`→"推荐应用", `//! application favourite`→"收藏应用". The root `mod.rs` placeholder is a `pub mod` → fill `<子模块 API 说明>` per the module it documents.

- [ ] **Step 2: Apply recipe to all 13 placeholders**

Struct placeholders (if any in v5 files) get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders in v5 + root mod**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v5/ crates/openlark-application/src/application/application/mod.rs`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v5/ | grep '/// 待补充文档'`
Expected: no output (empty). (`mod.rs` has no derives.)

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v5/ crates/openlark-application/src/application/application/mod.rs
git commit -m "docs(application): G4 v5 + root mod 占位→有义 doc (13)"
```

**Reviewer gates (G4):** same shape.

---

## Task 5 (G5): v6/application/** (cohesive sub-domain; 138 placeholders, 23 files)

**Files (23, all under `v6/application/`):**
- standalone: `get.rs`, `patch.rs`, `list.rs`, `underauditlist.rs`, `contacts_range_configuration.rs`
- `app_version/`: `create.rs`, `delete.rs`, `get.rs`, `list.rs`, `patch.rs`, `contacts_range.rs`, `contacts_range_suggest.rs`
- `app_usage/`: `overview.rs`, `message_push_overview.rs`, `department_overview.rs`
- `collaborators/`: `get.rs`, `list.rs`, `update.rs`
- `contacts_range/`: `patch.rs`
- `feedback/`: `list.rs`, `patch.rs`
- `management/`: `update.rs`
- `owner/`: `update.rs`
- `visibility/`: `patch.rs`, `check_white_black_list.rs`

**Placeholders: 138.** This is the largest group (above ~88 target; kept cohesive because all under `v6/application/`).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read each of the 23 files' `//!` header**

API 中文名 examples: `//! application get`→"获取应用", `//! application app version list`→"获取应用版本列表", `//! application collaborators update`→"更新协作者", `//! application underauditlist`→"获取审核中应用列表".

- [ ] **Step 2: Apply recipe to all 138 placeholders**

named fields present: `app_id`→"应用 ID", `contacts_range`→"通讯录范围" (contacts_range_configuration.rs, app_version/contacts_range.rs). All struct placeholders get position swap + recipe text. The `v6/application/mod.rs` (if it has placeholders, it does not per exploration — only the listed 23 files do).

- [ ] **Step 3: Self-verify — zero placeholders under v6/application/**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v6/application/`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v6/application/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v6/application/
git commit -m "docs(application): G5 v6/application 全子域 占位→有义 doc (138)"
```

**Reviewer gates (G5):**
- spec compliance: 138 sites match recipe; struct doc above derive; named fields translated.
- quality: because this group is large, reviewer spot-checks 3 files end-to-end (one standalone, one app_version/, one collaborators/) for recipe fidelity and that no doc reads as generic.

---

## Task 6 (G6): v6 — app / app_badge / app_recommend_rule / app_usage / frequently_used / management

**Files (12, prefix `v6/`):**
- `app/`: `create.rs` (already done in G0 — skip), `delete.rs`, `get.rs`, `list.rs`, `patch.rs`, `models.rs`, `mod.rs`
- `app_badge/set.rs`
- `app_recommend_rule/list.rs`
- `app_usage/message_push_overview.rs`
- `frequently_used/get.rs`
- `management/get.rs`, `management/patch.rs`

**Placeholders: 80, MINUS 6 already done in G0 (app/create.rs) = 74 remaining.**
(app=39−6=33, app_badge=11, app_recommend_rule=6, app_usage=6, frequently_used=6, management=12 → 33+11+6+6+6+12 = 74.)

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read headers for the 11 remaining files** (skip `app/create.rs`)

- [ ] **Step 2: Apply recipe to all 74 placeholders**

`app/models.rs` has named fields `app_id`→"应用 ID", `app_name`→"应用名称", `app_type`→"应用类型", `description`→"描述". `app_badge/set.rs` has `app_id`→"应用 ID", `badge`→"徽标". `app/mod.rs` has `pub mod` placeholders → `<子模块 API 说明>。`. Struct placeholders get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders in these dirs**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v6/app/ crates/openlark-application/src/application/application/v6/app_badge/ crates/openlark-application/src/application/application/v6/app_recommend_rule/ crates/openlark-application/src/application/application/v6/app_usage/ crates/openlark-application/src/application/application/v6/frequently_used/ crates/openlark-application/src/application/application/v6/management/`
Expected: no output (empty). (This includes app/create.rs as a redundant re-check — should already be empty from G0.)

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v6/{app,app_badge,app_recommend_rule,app_usage,frequently_used,management}/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v6/{app,app_badge,app_recommend_rule,app_usage,frequently_used,management}/
git commit -m "docs(application): G6 v6 app/badge/recommend/usage/freq/mgmt 占位→有义 doc (74)"
```

**Reviewer gates (G6):** same shape; spot-check `app/models.rs` (named fields dense).

---

## Task 7 (G7): v6 — app_version / collaborator / contacts_range / owner / scope

**Files (11, prefix `v6/`):**
- `app_version/`: `create.rs`, `delete.rs`, `get.rs`, `list.rs`, `patch.rs`
- `collaborator/`: `create.rs`, `delete.rs`, `list.rs`
- `contacts_range/get.rs`
- `owner/`: `recommended.rs`, `transfer.rs`
- `scope/`: `apply.rs`, `list.rs`

**Placeholders: 84** (app_version=30, collaborator=18, contacts_range=9, owner=12, scope=15).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read headers**

Examples: `//! app version create`→"创建应用版本", `//! collaborator list`→"获取协作者列表", `//! scope apply`→"申请权限范围".

- [ ] **Step 2: Apply recipe to all 84 placeholders**

named fields: `app_id`→"应用 ID", `contacts_range`→"通讯录范围" (contacts_range/get.rs), `scope_type`→"权限范围类型", `reason`→"原因说明" (scope/apply.rs), `new_owner_id`→"新所有者 ID" (owner/transfer.rs). Struct placeholders get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v6/app_version/ crates/openlark-application/src/application/application/v6/collaborator/ crates/openlark-application/src/application/application/v6/contacts_range/ crates/openlark-application/src/application/application/v6/owner/ crates/openlark-application/src/application/application/v6/scope/`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v6/{app_version,collaborator,contacts_range,owner,scope}/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v6/{app_version,collaborator,contacts_range,owner,scope}/
git commit -m "docs(application): G7 v6 app_version/collaborator/cr/owner/scope 占位→有义 doc (84)"
```

**Reviewer gates (G7):** same shape.

---

## Task 8 (G8): v6 — feedback / app_visibility / visibility / usage + v6/mod.rs

**Files (9, prefix `v6/`):**
- `feedback/`: `create.rs`, `delete.rs`, `get.rs`, `list.rs`
- `app_visibility/`: `get.rs`, `patch.rs`
- `visibility/check.rs`
- `usage/get.rs`
- `mod.rs`

**Placeholders: 50** (feedback=24, app_visibility=12, visibility=6, usage=6, v6/mod.rs=2).

**Interfaces:** none (doc-only).

- [ ] **Step 1: Read headers**

Examples: `//! feedback create`→"创建反馈", `//! app visibility get`→"获取应用可见性", `//! usage get`→"获取用量". `v6/mod.rs` `pub mod` placeholders → `<子模块 API 说明>。`.

- [ ] **Step 2: Apply recipe to all 50 placeholders**

Struct placeholders get position swap + recipe text.

- [ ] **Step 3: Self-verify — zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-application/src/application/application/v6/feedback/ crates/openlark-application/src/application/application/v6/app_visibility/ crates/openlark-application/src/application/application/v6/visibility/ crates/openlark-application/src/application/application/v6/usage/ crates/openlark-application/src/application/application/v6/mod.rs`
Expected: no output (empty).

- [ ] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-application/src/application/application/v6/{feedback,app_visibility,visibility,usage}/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 5: Compile-check**

Run: `cargo doc -p openlark-application --no-deps 2>&1 | tail -5`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/openlark-application/src/application/application/v6/{feedback,app_visibility,visibility,usage}/ crates/openlark-application/src/application/application/v6/mod.rs
git commit -m "docs(application): G8 v6 feedback/app_vis/visibility/usage + mod 占位→有义 doc (50)"
```

**Reviewer gates (G8):** same shape.

---

## Task 9: Global gate + full verification

**Files:** none modified (verification only; runs after G0–G8 all merged/committed).

**Interfaces:** none.

- [ ] **Step 1: Placeholder gate — application crate has zero of either placeholder string**

Run: `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/`
Expected: no output (empty). If any line remains, identify which group missed it and send a fix patch to that group.

- [ ] **Step 2: Position gate — no struct doc sits after a derive**

Run: `grep -rnE -A1 '^#\[derive' crates/openlark-application/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [ ] **Step 3: Full workspace doc — missing_docs = 0**

Run: `cargo doc --workspace --all-features --no-deps 2>&1 | grep -E "missing_docs|warning: unresolved" || echo "clean"`
Expected: `clean`.

- [ ] **Step 4: Format check**

Run: `cargo fmt --check`
Expected: exit 0, no diff output.

- [ ] **Step 5: Lint**

Run: `just lint`
Expected: exit 0.

- [ ] **Step 6: Application crate tests not broken**

Run: `cargo test -p openlark-application`
Expected: all tests pass (same pass/fail count as before this change — this change writes no code, so tests must be unchanged).

- [ ] **Step 7: Final sanity — total placeholder count across whole crate = 0**

Run: `grep -rc '/// 待补充文档。' crates/openlark-application/src/ | grep -v ':0$' | wc -l`
Expected: `0`.

---

## Self-Review (post-write check against Design Doc + tasks.md)

**Spec coverage:**
- tasks.md §1.1 (勘探 distribution) → covered by this plan's exploration section + per-group counts. ✓
- tasks.md §1.2 (pilot 1 file) → Task 0 (G0). ✓
- tasks.md §2.1 (按组回补 578 + 修正位置) → Tasks 1–8 (G1–G8), total 65+70+78+13+138+74+84+50 = 572 + G0's 6 = 578. ✓ (G6 excludes the 6 done in G0; the 578 total is fully covered.)
- tasks.md §2.2 (逐组自验 cargo doc) → each group Step 5. ✓
- tasks.md §3.1 (占位 + 位置守门) → Task 9 Steps 1–2. ✓
- tasks.md §3.2 (cargo doc missing_docs=0, fmt, lint, tests) → Task 9 Steps 3–6. ✓
- Design Doc recipe (8-row) → Global Constraints + every task Step 2 references it verbatim. ✓
- Design Doc position transform (190 sites) → Global Constraints + struct steps. ✓
- Design Doc grouping (version×sub-domain, ~7-8) → 8 groups. ✓ (G5 sized 138 > 88 target, noted.)

**Placeholder scan:** no "TBD"/"TODO"/"implement later"/"similar to Task N" in this plan. Every step has concrete commands or concrete doc text. ✓

**Type consistency:** there are no types/functions defined here (doc-only change). Recipe strings are used identically across all group tasks. ✓

**Note:** Task ordering assumes G0 merged before G6 (G6 re-checks app/create.rs and excludes its 6 from its count). If executing in parallel worktrees, G0 must land first; otherwise G6's self-verify Step 3 still correctly catches any leftover.
