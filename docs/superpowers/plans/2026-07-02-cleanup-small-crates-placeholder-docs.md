---
change: cleanup-small-crates-placeholder-docs
design-doc: docs/superpowers/specs/2026-07-02-cleanup-small-crates-placeholder-docs-design.md
base-ref: 1e6a23807aab7cb819c4e682e164f840dcd02009
archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

# cleanup-small-crates-placeholder-docs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace all 335 `/// 待补充文档。` placeholder doc comments across 5 small/medium crates (mail 104 / workflow 78 / meeting 65 / user 47 / hr 41) with meaningful Chinese doc, and fix the position of 63 struct doc comments (move `///` from between `#[derive(...)]` and `pub struct` to above `#[derive(...)]`).

**Architecture:** Pure mechanical recipe inherited from #1 analytics and re-validated by the just-archived application crate (578 sites, PR #297). Each placeholder becomes `<API 中文名 from file //! header> + <item 角色>` per an 11-row recipe table. Structs additionally get a 3-line swap (doc moved above `#[derive]`). Executed subagent-driven, grouped by crate into 5 groups (G1–G5) — each crate is an independent compilation unit, so per-crate `cargo doc -p` and `cargo check -p` give clean hard boundaries. A pilot file (Task 0, mail) validates the recipe and position transform before batch work.

**Tech Stack:** Rust, serde, openlark-core. Verification via `cargo doc`, `cargo check`, `cargo fmt --check`, `just lint`, `grep` gates.

## Global Constraints

These apply to every task; copy values verbatim:

- **No logic changes.** Only doc-comment text edits + struct doc-position swaps. Do not touch function bodies, signatures, derives, serde attrs, imports, or any non-doc line.
- **Only the 5 crate source trees.** `crates/openlark-{mail,workflow,meeting,user,hr}/src/`. Do not edit any other crate. Do not touch TestCheck, do not edit `mod.rs` factory lines beyond the recipe.
- **Recipe text is fixed.** Use exactly the 11-row recipe table below. Do not invent placeholders, do not write generic prose, do not translate the recipe wording. Chinese doc only.
- **`<API 中文名>` comes from the file's `//!` header line(s).** E.g. mail `//! 批量获取邮件组管理员` → use "批量获取邮件组管理员" verbatim as the API name root. Read each file's `//!` header to get the API name.
- **named field doc (33 distinct names + 14 builder setters): read the field name, translate to Chinese per the shared translation table.** Same name must use the same Chinese across crates. Reference translations below; implementer applies, reviewer spot-checks.
- **struct doc position: `///` moves from between `#[derive(...)]` and `pub struct` to ABOVE `#[derive(...)]`.** 3-line swap, 63 sites, all single-`#[derive]` (0 multi-attr boundaries verified). Other-item placeholders (fn/field/module/impl) stay in place — only swap text.
- **No `unwrap()`/`expect()`** (naturally satisfied — this change writes no code, only doc).
- **Each implementer self-verifies crate placeholder count = 0 AND `cargo check -p openlark-<crate>` exit 0 before handing to reviewer.** (`cargo check` is the hard proof of signature integrity — `cargo doc` does NOT report a deleted `pub async fn execute_with_options(` signature line; `cargo check` does.)
- **Commit after each task** (after both reviewer gates pass). Commit message format: `docs(<crate>): <crate> 占位→有义 doc (<count>)` (e.g. `docs(mail): mail 占位→有义 doc (104)`); pilot uses `docs(mail): pilot mailgroup/manager/list recipe 验证 (11)`.

### CRITICAL — struct position swap Edit safety (lesson from application crate)

When swapping a struct's doc position, the Edit tool's `old_string` AND `new_string` must each contain the **full 3 lines**: `#[derive(...)]` + `///` + `pub struct`. Anchoring on just the `///` line risks accidentally deleting the `pub async fn execute_with_options(` signature line below. Pattern:

```text
# WRONG (anchor too narrow — may eat the fn signature below)
old: /// 待补充文档。\npub struct Foo
new: /// <doc>。\npub struct Foo

# RIGHT (full 3-line block, both sides)
old:
#[derive(Debug, Clone)]
/// 待补充文档。
pub struct Foo {

new:
/// <doc>。
#[derive(Debug, Clone)]
pub struct Foo {
```

Every struct edit in this plan must use the 3-line block form on both sides.

### The Recipe (fixed 11-row table — verbatim from Design Doc)

| item | doc text (replace `/// 待补充文档。` with) | trigger (the line immediately after the placeholder) |
|------|------------------------------------------|------------------------------------------------------|
| Request struct | `<API 中文名>的请求。`（+ position swap） | `pub struct XxxRequest` |
| Response struct | `<API 中文名>的响应。`（+ position swap） | `pub struct XxxResponse` |
| Other struct (Body/Data/DeviceInfo etc.) | `<结构中文名>。`（+ position swap） | any other `pub struct` |
| field `data` | `响应数据。` | `pub data:` |
| field (named) | `<字段中文名>。` | any other `pub <name>:` — read `name`, translate via table |
| `fn new` | `创建请求实例。` | `pub fn new` |
| `fn execute` | `执行<API 中文名>请求。` | `pub async fn execute` |
| `fn execute_with_options` | `带自定义请求选项执行。` | `pub async fn execute_with_options` |
| `fn <field>` (builder setter) | `设置<字段中文名>。` | `pub fn <field>(mut self, ... -> Self)` |
| module | `<子模块 API 说明>。` | `pub mod`（workflow 8 处：v4/mod.rs 6 + approval/mod.rs 1 + instance/mod.rs 1） |
| **impl 块** | `<API 中文名>请求构建器实现。` | `impl XxxRequest {`（hr 6 处 — recipe row NEW vs application） |

API 中文名 取自文件 `//!` 头第一行（去版本前缀，直接用中文头原文，如"批量获取邮件组管理员"）。implementer 读占位**下一条**非注释行确定 item 角色，套对应行。

### named field translation reference (33 distinct names — cross-crate consistent)

implementer applies; reviewer spot-checks same-name-same-Chinese across crates.

| field name | Chinese | crates |
|------------|---------|--------|
| `manager_ids` | 管理员 ID 列表 | mail |
| `managers` | 管理员 | mail |
| `manager_id` | 管理员 ID | mail |
| `manager_email` | 管理员邮箱 | mail |
| `download_url` | 下载地址 | mail |
| `expire_time` | 过期时间 | mail |
| `id` | ID | workflow |
| `name` | 名称 | workflow |
| `level` | 层级 | workflow |
| `has_sub_district` | 是否含下级区划 | workflow |
| `parent_districts` | 上级区划 | workflow |
| `version` | 版本 | workflow |
| `has_more` | 是否有更多 | workflow |
| `page_token` | 分页标记 | workflow |
| `items` | 列表项 | workflow, hr |
| `district_ids` | 区划 ID 列表 | workflow |
| `keyword` | 关键词 | workflow |
| `calendar` | 日历 | meeting |
| `capacity` | 容量 | meeting |
| `capacity_max` | 最大容量 | meeting |
| `description` | 描述 | meeting |
| `device_id` | 设备 ID | meeting |
| `device_name` | 设备名称 | meeting |
| `device_type` | 设备类型 | meeting |
| `devices` | 设备列表 | meeting |
| `room_id` | 会议室 ID | meeting |
| `room_name` | 会议室名称 | meeting |
| `status` | 状态 | meeting |
| `user_ids` | 用户 ID 列表 | user |
| `employee_id` | 员工 ID | hr |
| `probation` | 试用期 | hr |
| `from_date` | 开始日期 | hr |
| `to_date` | 结束日期 | hr |

builder setter (14 distinct names) — use `设置<字段中文名>。`:

| setter fn name | doc text |
|----------------|----------|
| `manager_ids` | 设置管理员 ID 列表。 |
| `user_id` | 设置用户 ID。 |
| `topic` | 设置主题。 |
| `user_id_type` | 设置用户 ID 类型。 |
| `page_size` | 设置分页大小。 |
| `page_token` | 设置分页标记。 |
| `root_district_id` | 设置根区划 ID。 |
| `list_type` | 设置列表类型。 |
| `locale` | 设置语言。 |
| `district_ids` | 设置区划 ID 列表。 |
| `keyword` | 设置关键词。 |
| `status_id` | 设置状态 ID。 |
| `user_ids` | 设置用户 ID 列表。 |
| `body` | 设置请求体。 |

implementer may fine-tune per Feishu domain sense if a name is missing; record any addition in the task commit body.

### struct doc position transform (verbatim from Design Doc, applied to all 63 struct placeholders)

```rust
// BEFORE (current — doc after derive)
#[derive(Debug, Clone)]
/// 待补充文档。
pub struct XxxRequest {

// AFTER (target — doc above derive, with recipe text filled in)
/// <API 中文名>的请求。
#[derive(Debug, Clone)]
pub struct XxxRequest {
```

For fn / field / module / impl placeholders (272 sites), position is already correct — only swap the text in place.

Verified: 0 multi-attr boundaries (all 63 struct placeholders have exactly one `#[derive(...)]` directly above; no `#[serde]` stacking). The transform is 100% mechanical. hr `edit.rs` struct placeholders are ALREADY in the correct position (`///` above `#[derive]`) — those serve as a reference for the target shape and need text-only replacement.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## File Structure & Grouping

All 5 crates, file lists verified by grep on base-ref. Each group = one crate (independent compilation unit). Hard file-set boundaries make coverage trivially verifiable via per-group `grep`.

| Group | crate | placeholders | files | base path prefix |
|-------|-------|--------------|-------|------------------|
| G1 | mail | 104 | 15 | `crates/openlark-mail/src/mail/mail/v1/` |
| G2 | workflow | 78 | 34 (29 business + 5 mod.rs) | `crates/openlark-workflow/src/approval/approval/` |
| G3 | meeting | 65 | 41 | `crates/openlark-meeting/src/{calendar,meeting_room,vc}/` |
| G4 | user | 47 | 7 | `crates/openlark-user/src/personal_settings/personal_settings/v1/system_status/` |
| G5 | hr | 41 | 3 | `crates/openlark-hr/src/feishu_people/corehr/v2/` |
| **Total** | | **335** | **95** | |

Sum = 104 + 78 + 65 + 47 + 41 = 335. ✓ (matches Design Doc exploration table).

The largest group (mail 104) is well under the application crate's G5=138 proven-feasible ceiling; no group needs splitting.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 0: Pilot — validate recipe + position transform on 1 mail file

**Files:**
- Modify: `crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs` (11 placeholders)

**Why pilot first:** 335-site mechanical batch is high-risk for偷懒 (lazy/generic doc) and for Edit-anchor mistakes on the struct swap. One file proves the recipe produces real per-item doc and the struct swap is clean (no signature deletion) before committing to the batch.

- [x] **Step 1: Read the pilot file and its `//!` header**

Run: `head -2 crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs`
Header is `//! 批量获取邮件组管理员` → API 中文名 = "批量获取邮件组管理员".

Expected placeholders (11, all confirmed by exploration):
- 3 struct placeholders position-wrong (`#[derive]` → `///` → `pub struct`): `ListMailGroupManagerRequest`, `ListMailGroupManagerResponse`, `ListMailGroupManagerData`, plus `MailGroupManager` (4 struct swaps total — re-count by reading the file).
- named fields: `managers`→管理员, `manager_id`→管理员 ID, `manager_email`→管理员邮箱 (translate per table).
- `fn new` → `创建请求实例。`
- `fn execute` → `执行批量获取邮件组管理员请求。`
- `fn execute_with_options` → `带自定义请求选项执行。`

(Exact count of struct vs field sites confirmed by reading the file in Step 1; the recipe applies per the next-non-comment-line rule.)

- [x] **Step 2: Apply the 11 edits**

For each struct placeholder, use the **3-line block Edit** (both old and new include `#[derive(...)]` + `///` + `pub struct`):

```rust
/// 批量获取邮件组管理员的请求。
#[derive(Debug, Clone)]
pub struct ListMailGroupManagerRequest {
```
(Repeat for `Response` → `批量获取邮件组管理员的响应。`, for `Data`/other structs → struct Chinese name like `批量获取邮件组管理员数据。`.)

For named fields and the 3 fns, replace text in place: `管理员`、`管理员 ID`、`管理员邮箱`、`创建请求实例。`、`执行批量获取邮件组管理员请求。`、`带自定义请求选项执行。`.

- [x] **Step 3: Self-verify — pilot file has zero placeholders**

Run: `grep -n '/// 待补充文档。' crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs`
Expected: no output (empty).

- [x] **Step 4: Verify position gate on the pilot file**

Run: `grep -nA1 '^#\[derive' crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: Verify pilot file compiles clean under cargo doc**

Run: `cargo doc -p openlark-mail --no-deps 2>&1 | grep -E "(list\.rs|missing_docs|warning)" || echo "clean"`
Expected: `clean` (no warning mentioning the pilot file or `missing_docs`).

- [x] **Step 6: Verify signature integrity with cargo check**

Run: `cargo check -p openlark-mail 2>&1 | tail -5`
Expected: exit 0, no errors. (This is the hard gate that catches a deleted `pub async fn execute_with_options(` line — `cargo doc` does NOT report it.)

- [x] **Step 7: Commit**

```bash
git add crates/openlark-mail/src/mail/mail/v1/mailgroup/manager/list.rs
git commit -m "docs(mail): pilot mailgroup/manager/list recipe 验证 (11)"
```

**Reviewer gates (Task 0):**
- spec compliance: the 11 doc texts match the recipe table exactly; struct doc is above `#[derive]`; named fields match the translation table (`managers`/`manager_id`/`manager_email`).
- quality: doc reads as natural Chinese, references the real API name "批量获取邮件组管理员"; no signature lines deleted (cargo check passed).

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 1 (G1): mail — full crate (104 placeholders, 15 files)

**Files (15, all under `crates/openlark-mail/src/mail/mail/v1/`):**
- `mailgroup/manager/`: `batch_create.rs` (9), `batch_delete.rs` (9), `list.rs` (11 — DONE in Task 0, skip but re-verify)
- `mailgroup/member/`: `batch_delete.rs` (6), `get.rs` (6), `list.rs` (6)
- `public_mailbox/member/`: `batch_create.rs` (6), `batch_delete.rs` (6), `clear.rs` (6)
- `public_mailbox/remove_to_recycle_bin.rs` (6)
- `user_mailbox/`: `delete.rs` (6), `rule/update.rs` (6), `event/subscription.rs` (6), `message/attachment/download_url.rs` (9)
- `user/query.rs` (6)

**Placeholders: 104, MINUS 11 already done in Task 0 (manager/list.rs) = 93 remaining.**

**Interfaces:** none (doc-only).

- [x] **Step 1: Read each of the 14 remaining files' `//!` header for API 中文名**

Example headers (read each file to confirm): `mailgroup/manager/batch_create.rs`→"批量创建邮件组管理员", `user_mailbox/message/attachment/download_url.rs`→"获取邮件附件下载地址" (named field `download_url`→下载地址, `expire_time`→过期时间), `user/query.rs`→"查询用户".

- [x] **Step 2: Apply recipe to all 93 remaining placeholders in these 14 files**

Per the 11-row recipe table. Every struct placeholder (mail has 35 struct sites total — 4 already done in Task 0, 31 remaining) gets BOTH the **3-line block position swap** (doc above `#[derive]`, full-3-line Edit on both sides) AND the recipe text. named fields in this crate: `manager_ids`→管理员 ID 列表, `managers`→管理员, `manager_id`→管理员 ID, `manager_email`→管理员邮箱, `download_url`→下载地址, `expire_time`→过期时间. builder setters: `manager_ids`→设置管理员 ID 列表. `data` fields → "响应数据。". `fn new`→`创建请求实例。`, `fn execute`→`执行<API>请求。`, `fn execute_with_options`→`带自定义请求选项执行。`.

- [x] **Step 3: Self-verify — entire mail crate has zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-mail/src/`
Expected: no output (empty). (This re-checks Task 0's file too as a redundant gate.)

- [x] **Step 4: Self-verify — position gate across whole mail crate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-mail/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: cargo doc — no missing_docs warning**

Run: `cargo doc -p openlark-mail --no-deps 2>&1 | tail -5`
Expected: no `missing_docs` warning, no errors.

- [x] **Step 6: cargo check — signature integrity (hard gate)**

Run: `cargo check -p openlark-mail 2>&1 | tail -5`
Expected: exit 0, no errors.

- [x] **Step 7: Commit**

```bash
git add crates/openlark-mail/src/
git commit -m "docs(mail): mail 占位→有义 doc (104)"
```

(Commit message counts the full 104 including Task 0's 11; the commit body can note Task 0 already landed the pilot file.)

**Reviewer gates (G1):**
- spec compliance: all 93 remaining sites match recipe; struct doc above derive (3-line block edits used); named fields translated per table; cross-file `managers`/`manager_id`/`manager_email` consistent.
- quality: API names traceable to `//!` headers; spot-check 2 files end-to-end (one manager, one user_mailbox); confirm no `pub async fn execute_with_options(` line deleted (cargo check passed).

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 2 (G2): workflow — full crate (78 placeholders, 34 files)

**Files (34, all under `crates/openlark-workflow/src/approval/approval/`):**
- `mod.rs` (1 module placeholder)
- `v4/mod.rs` (6 module placeholders: approval, external_approval, external_instance, external_task, instance, task)
- `v4/instance/mod.rs` (1 module placeholder)
- `v4/approval/`: `create.rs` (1), `get.rs` (1), `subscribe.rs` (1), `unsubscribe.rs` (1)
- `v4/external_approval/`: `create.rs` (1), `get.rs` (1)
- `v4/external_instance/`: `check.rs` (1), `create.rs` (1)
- `v4/external_task/list.rs` (1)
- `v4/task/`: `approve.rs` (1), `query.rs` (6), `reject.rs` (1), `resubmit.rs` (1), `search.rs` (1), `transfer.rs` (1)
- `v4/instance/`: `add_sign.rs` (1), `cancel.rs` (1), `cc.rs` (1), `create.rs` (1), `get.rs` (1), `list.rs` (1), `preview.rs` (1), `query.rs` (1), `search_cc.rs` (1), `specified_rollback.rs` (1), `mod.rs` (1)
- `v4/instance/comment/`: `create.rs` (1), `delete.rs` (1), `list.rs` (1), `remove.rs` (1)
- `v4/district/list.rs` (24 — struct/field dense: DistrictBaseInfo, DistrictItem, ListDistrictsResponse + named fields id/name/level/has_sub_district/parent_districts/version/has_more/page_token/items/district_ids)
- `v4/district/search.rs` (12 — named fields: keyword, district_ids, etc.)

**Placeholders: 78.** module placeholders = 8 (approval/mod.rs 1 + v4/mod.rs 6 + v4/instance/mod.rs 1). struct placeholders = 7 (all in district/list.rs + district/search.rs). field placeholders = 15 (all in district/). fn placeholders = 48 (mostly single-fn files).

**Interfaces:** none (doc-only).

- [x] **Step 1: Read each file's `//!` header for API 中文名**

Example headers (read each to confirm): `v4/district/list.rs`→"查询地理库信息", `v4/task/query.rs`→"查询任务", `v4/approval/create.rs`→"创建审批定义", `v4/instance/create.rs`→"创建审批实例", `v4/instance/comment/create.rs`→"创建评论". For `mod.rs` files, the `pub mod` placeholders take `<子模块 API 说明>。` — read the submodule's purpose from its own `//!` header or module doc.

- [x] **Step 2: Apply recipe to all 78 placeholders**

  - **module (8 sites)**: `v4/mod.rs` `pub mod approval;`→`审批定义相关 API。` (etc. for external_approval/external_instance/external_task/instance/task); `approval/mod.rs` and `v4/instance/mod.rs` similarly per submodule.
  - **struct (7 sites, district/list.rs 5 + district/search.rs 2)**: 3-line block position swap + recipe text. Other structs (`DistrictBaseInfo`/`DistrictItem`/`ListDistrictsResponse`) → `<结构中文名>。` e.g. "区划基础信息。", "区划项。", "查询区划列表的响应。".
  - **field (15 sites, all district/)**: named fields per table — `id`→ID, `name`→名称, `level`→层级, `has_sub_district`→是否含下级区划, `parent_districts`→上级区划, `version`→版本, `has_more`→是否有更多, `page_token`→分页标记, `items`→列表项, `district_ids`→区划 ID 列表, `keyword`→关键词.
  - **fn (48 sites)**: most are single `fn new` or `fn execute` per file. `task/query.rs` has 6 (new + execute + execute_with_options + setters). setters: `district_ids`→设置区划 ID 列表, `keyword`→设置关键词, `page_size`→设置分页大小, `page_token`→设置分页标记, `root_district_id`→设置根区划 ID, `user_id_type`→设置用户 ID 类型, `locale`→设置语言, `list_type`→设置列表类型, `body`→设置请求体 (apply per actual setter name found).

- [x] **Step 3: Self-verify — entire workflow crate has zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-workflow/src/`
Expected: no output (empty).

- [x] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-workflow/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: cargo doc**

Run: `cargo doc -p openlark-workflow --no-deps 2>&1 | tail -5`
Expected: no `missing_docs` warning, no errors.

- [x] **Step 6: cargo check — signature integrity (hard gate, CRITICAL for G2)**

Run: `cargo check -p openlark-workflow 2>&1 | tail -5`
Expected: exit 0, no errors. (G2 has the densest struct/field set in district/list.rs + district/search.rs — the 3-line block Edits there are the highest-risk for accidentally deleting a `pub async fn execute_with_options(` signature. cargo check is the proof.)

- [x] **Step 7: Commit**

```bash
git add crates/openlark-workflow/src/
git commit -m "docs(workflow): workflow 占位→有义 doc (78)"
```

**Reviewer gates (G2):**
- spec compliance: all 78 sites match recipe; 7 struct docs above derive (3-line block edits); 8 module placeholders have submodule-specific text (not generic); named fields in district/ match table.
- quality: spot-check `district/list.rs` (24 sites, densest) and `district/search.rs` (12) end-to-end; confirm no signature lines deleted (cargo check passed); confirm cross-crate `items`/`page_token` use the same Chinese as the table.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 3 (G3): meeting — full crate (65 placeholders, 41 files)

**Files (41):**
- `meeting_room/responses.rs` (11 — the 2 struct position-bug sites are here)
- `calendar/calendar/v4/calendar/`: `create.rs` (2), `delete.rs` (2), `get.rs` (2), `list.rs` (2), `mget.rs` (2), `patch.rs` (3), `primary.rs` (1), `primarys.rs` (1), `search.rs` (2), `subscribe.rs` (1), `subscription.rs` (2), `unsubscribe.rs` (1), `unsubscription.rs` (1)
- `calendar/calendar/v4/calendar/event/`: `attendee/batch_delete.rs` (1), `attendee/create.rs` (1), `attendee/list.rs` (1), `create.rs` (2), `delete.rs` (1), `get.rs` (1), `instance_view.rs` (1), `instances.rs` (1), `list.rs` (1), `patch.rs` (2), `reply.rs` (2), `search.rs` (1), `subscription.rs` (2), `unsubscription.rs` (2)
- `vc/vc/v1/meeting/`: `end.rs` (1), `invite.rs` (1), `kickout.rs` (1), `list_by_no.rs` (1), `set_host.rs` (1)
- `vc/vc/v1/reserve_config/`: `patch.rs` (1), `reserve_scope.rs` (1)
- `vc/vc/v1/room_level/`: `create.rs` (1), `del.rs` (1), `get.rs` (1), `mget.rs` (1), `patch.rs` (1), `search.rs` (1)

**Placeholders: 65.** struct placeholders = 2 (both position-wrong, both in `meeting_room/responses.rs`). field placeholders = 13 (meeting_room/responses.rs has device/room fields: `device_id`/`device_name`/`device_type`/`devices`/`room_id`/`room_name`/`status`/`capacity`/`capacity_max`/`description`/`calendar`). module = 0. fn = 50.

**Interfaces:** none (doc-only).

- [x] **Step 1: Read each file's `//!` header**

Example headers: `calendar/v4/calendar/get.rs`→"获取日历", `calendar/v4/calendar/event/create.rs`→"创建日程", `meeting_room/responses.rs`→(meeting room summary — read header), `vc/v1/meeting/end.rs`→"结束会议", `vc/v1/room_level/create.rs`→"创建会议室层级".

- [x] **Step 2: Apply recipe to all 65 placeholders**

  - **struct (2 sites, `meeting_room/responses.rs`)**: 3-line block position swap + recipe text (struct Chinese name per the struct, e.g. 会议室摘要响应).
  - **field (13 sites, mostly in `meeting_room/responses.rs`)**: named per table — `calendar`→日历, `capacity`→容量, `capacity_max`→最大容量, `description`→描述, `device_id`→设备 ID, `device_name`→设备名称, `device_type`→设备类型, `devices`→设备列表, `room_id`→会议室 ID, `room_name`→会议室名称, `status`→状态.
  - **fn (50 sites, mostly 1-2 per file)**: `fn new`→`创建请求实例。`, `fn execute`→`执行<API>请求。`, `fn execute_with_options`→`带自定义请求选项执行。`. setters if any: per setter name + table.

- [x] **Step 3: Self-verify — entire meeting crate has zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-meeting/src/`
Expected: no output (empty).

- [x] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-meeting/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: cargo doc**

Run: `cargo doc -p openlark-meeting --no-deps 2>&1 | tail -5`
Expected: clean.

- [x] **Step 6: cargo check — signature integrity (hard gate)**

Run: `cargo check -p openlark-meeting 2>&1 | tail -5`
Expected: exit 0, no errors.

- [x] **Step 7: Commit**

```bash
git add crates/openlark-meeting/src/
git commit -m "docs(meeting): meeting 占位→有义 doc (65)"
```

**Reviewer gates (G3):**
- spec compliance: all 65 sites match recipe; 2 struct docs in `responses.rs` above derive (3-line block); named device/room fields match table.
- quality: spot-check `meeting_room/responses.rs` (11 sites, struct+field dense) end-to-end; confirm no signature deleted.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 4 (G4): user — full crate (47 placeholders, 7 files)

**Files (7, all under `crates/openlark-user/src/personal_settings/personal_settings/v1/system_status/`):**
- `batch_close.rs` (9)
- `batch_open.rs` (6)
- `create.rs` (6)
- `delete.rs` (7)
- `get.rs` (6)
- `list.rs` (6)
- `patch.rs` (7)

**Placeholders: 47.** struct placeholders = 15 (all position-wrong). field placeholders = 8 (mostly `user_ids`→用户 ID 列表; `status` etc.). module = 0. fn = 21. The named-field set is small (`user_ids` is the only cross-crate one; others are single-crate status fields).

**Interfaces:** none (doc-only).

- [x] **Step 1: Read each file's `//!` header**

Example headers: `system_status/create.rs`→"创建系统状态", `system_status/batch_open.rs`→"批量开启系统状态", `system_status/list.rs`→"获取系统状态列表", `system_status/patch.rs`→"更新系统状态".

- [x] **Step 2: Apply recipe to all 47 placeholders**

  - **struct (15 sites, all position-wrong)**: 3-line block position swap + recipe text. Request→`<API>的请求。`, Response→`<API>的响应。`, other structs (Body etc.)→`<结构中文名>。`.
  - **field (8 sites)**: `user_ids`→用户 ID 列表 (cross-crate consistent with the table). Other status-specific fields: read name, translate per Feishu sense.
  - **fn (21 sites)**: `fn new`→`创建请求实例。`, `fn execute`→`执行<API>请求。`, `fn execute_with_options`→`带自定义请求选项执行。`. setter `user_ids`→设置用户 ID 列表.

- [x] **Step 3: Self-verify — entire user crate has zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-user/src/`
Expected: no output (empty).

- [x] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-user/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: cargo doc**

Run: `cargo doc -p openlark-user --no-deps 2>&1 | tail -5`
Expected: clean.

- [x] **Step 6: cargo check — signature integrity (hard gate)**

Run: `cargo check -p openlark-user 2>&1 | tail -5`
Expected: exit 0, no errors.

- [x] **Step 7: Commit**

```bash
git add crates/openlark-user/src/
git commit -m "docs(user): user 占位→有义 doc (47)"
```

**Reviewer gates (G4):**
- spec compliance: all 47 sites match recipe; 15 struct docs above derive (3-line block); `user_ids` uses "用户 ID 列表" consistent with the cross-crate table.
- quality: API names traceable to `//!` headers; spot-check 2 of the 7 files end-to-end.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 5 (G5): hr — full crate (41 placeholders, 3 files)

**Files (3, all under `crates/openlark-hr/src/feishu_people/corehr/v2/`):**
- `company/query_multi_timeline.rs` (14)
- `location/query_multi_timeline.rs` (14)
- `probation/edit.rs` (13 — struct placeholders here are ALREADY in correct position; 6 impl-block placeholders = recipe new role)

**Placeholders: 41.** struct placeholders = 15 (4 position-wrong, 11 position-correct — Design Doc: "hr struct 位置 bug=4"; `edit.rs` structs are the correct-position reference). field = 8 (`employee_id`/`probation`/`from_date`/`to_date`/`items` etc.). module = 0. **impl block = 6 (recipe NEW role — first time in this change; application had 0 impl-block placeholders)**. fn = 9 (mostly `new`).

**Interfaces:** none (doc-only).

- [x] **Step 1: Read each file's `//!` header**

Example headers: `probation/edit.rs`→"编辑试用期", `company/query_multi_timeline.rs`→"查询公司多时间线", `location/query_multi_timeline.rs`→"查询地点多时间线". For `edit.rs`, confirm the struct placeholders are already `///` above `#[derive]` (reference shape) — those need text-only replacement, NO position swap.

- [x] **Step 2: Apply recipe to all 41 placeholders**

  - **struct (15 sites)**: For the 11 already-correct (edit.rs) → text-only replacement (`<API>的请求。` / `<API>的响应。` / `<结构中文名>。`). For the 4 position-wrong (in company/query_multi_timeline.rs or location/query_multi_timeline.rs — confirm by reading) → 3-line block position swap + recipe text.
  - **impl block (6 sites — recipe row: `<API 中文名>请求构建器实现。`)**: trigger is `impl XxxRequest {`. e.g. in `edit.rs`, `impl EditProbationRequest {` → doc above it becomes `编辑试用期请求构建器实现。`. This is the new recipe row vs application — apply verbatim.
  - **field (8 sites)**: `employee_id`→员工 ID, `probation`→试用期, `from_date`→开始日期, `to_date`→结束日期, `items`→列表项 (cross-crate consistent with workflow).
  - **fn (9 sites)**: `fn new`→`创建请求实例。`, `fn execute`→`执行<API>请求。`, `fn execute_with_options`→`带自定义请求选项执行。`. setters: `employee_id`→设置员工 ID, `body`→设置请求体, etc.

- [x] **Step 3: Self-verify — entire hr crate has zero placeholders**

Run: `grep -rn '/// 待补充文档。' crates/openlark-hr/src/`
Expected: no output (empty).

- [x] **Step 4: Self-verify — position gate**

Run: `grep -rnA1 '^#\[derive' crates/openlark-hr/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 5: cargo doc**

Run: `cargo doc -p openlark-hr --no-deps 2>&1 | tail -5`
Expected: clean.

- [x] **Step 6: cargo check — signature integrity (hard gate)**

Run: `cargo check -p openlark-hr 2>&1 | tail -5`
Expected: exit 0, no errors.

- [x] **Step 7: Commit**

```bash
git add crates/openlark-hr/src/
git commit -m "docs(hr): hr 占位→有义 doc (41)"
```

**Reviewer gates (G5):**
- spec compliance: all 41 sites match recipe; **6 impl-block sites use the new `<API>请求构建器实现。` row** (not a generic fn doc); 4 position-wrong structs swapped via 3-line block; 11 already-correct structs get text-only; named fields (`employee_id`/`probation`/`from_date`/`to_date`/`items`) match table.
- quality: spot-check `edit.rs` (13 sites, includes impl blocks) end-to-end; confirm impl-block doc reads as "构建器实现" not as a fn doc; confirm cross-crate `items` = "列表项" same as workflow.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Task 6: Global gate + full verification

**Files:** none modified (verification only; runs after Task 0 + G1–G5 all merged/committed).

**Interfaces:** none.

- [x] **Step 1: Placeholder gate — all 5 crates have zero of either placeholder string**

Run: `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-{mail,workflow,meeting,user,hr}/src/`
Expected: no output (empty). If any line remains, identify which group missed it and send a fix patch to that group's implementer.

- [x] **Step 2: Position gate — no struct doc sits after a derive in any of the 5 crates**

Run: `grep -rnE -A1 '^#\[derive' crates/openlark-{mail,workflow,meeting,user,hr}/src/ | grep '/// 待补充文档'`
Expected: no output (empty).

- [x] **Step 3: Full workspace doc — missing_docs = 0**

Run: `cargo doc --workspace --all-features --no-deps 2>&1 | grep -E "missing_docs|warning: unresolved" || echo "clean"`
Expected: `clean`.

- [x] **Step 4: Format check**

Run: `cargo fmt --check`
Expected: exit 0, no diff output. (If diff appears, a group committed unformatted doc — run `cargo fmt` on the affected crate(s) and amend.)

- [x] **Step 5: Lint (both CI lint paths)**

Run: `just lint`
Expected: exit 0. (CI lint runs `clippy --workspace --all-targets --no-default-features` as well as the default-features path; `just lint` must pass for both. This change writes no code, so any lint failure is pre-existing and should be reported, not fixed here.)

- [x] **Step 6: 5 crate tests not broken**

Run: `cargo test -p openlark-mail -p openlark-workflow -p openlark-meeting -p openlark-user -p openlark-hr`
Expected: all tests pass (same pass/fail count as before this change — this change writes no code, so tests must be unchanged). If a test was already failing before this change, report it as pre-existing; do not fix.

- [x] **Step 7: Final sanity — total placeholder count across all 5 crates = 0**

Run: `grep -rc '/// 待补充文档。' crates/openlark-{mail,workflow,meeting,user,hr}/src/ | grep -v ':0$' | wc -l`
Expected: `0`.

- [x] **Step 8: Cross-crate named-field consistency spot-check**

Run these and confirm same Chinese across crates:
- `items` (workflow + hr): `grep -rn '/// 列表项。' crates/openlark-{workflow,hr}/src/ | wc -l` — should match the field count, all using "列表项".
- `page_token` (workflow): confirm "分页标记" everywhere.
- (No other name overlaps across these 5 crates per the table.)

Expected: no field uses a divergent Chinese for the same name.

archived-with: 2026-07-02-cleanup-small-crates-placeholder-docs
---

## Self-Review (post-write check against Design Doc + tasks.md)

**Spec coverage:**
- tasks.md §1.1 (勘探 5 crate distribution) → covered by this plan's exploration section + per-task file lists (verified by grep on base-ref). ✓
- tasks.md §1.2 (pilot 1 file recipe + position validation) → Task 0. ✓
- tasks.md §2.1 (5-crate batch回补 + struct position fix) → Tasks 1–5 (G1–G5). Totals: mail 104 + workflow 78 + meeting 65 + user 47 + hr 41 = 335. ✓ (Task 0's 11 are part of G1's 104 and counted in G1's commit.)
- tasks.md §2.2 (逐 crate 自验 cargo doc) → each group Step 5. ✓
- tasks.md §3.1 (placeholder + position gate) → Task 6 Steps 1–2. ✓
- tasks.md §3.2 (workspace cargo doc missing_docs=0, fmt, lint both paths, 5-crate tests) → Task 6 Steps 3–6. ✓
- Design Doc recipe (11-row, incl. impl-block new row) → Global Constraints + every task Step 2 references it verbatim. ✓
- Design Doc position transform (63 struct sites, 0 multi-attr) → Global Constraints (with 3-line Edit safety) + struct steps in each group. ✓
- Design Doc grouping (5 crates) → 5 groups G1–G5 + Task 0 pilot + Task 6 gate. ✓
- Design Doc named-field table (33 names) + builder setter (14 names) → Global Constraints tables, copied verbatim. ✓
- Design Doc risk mitigations (偷懒 → recipe + grep gate + pilot; position漏修 → position gate; 跨 crate 漂移 → shared table + Task 6 Step 8; impl 块新角色 → recipe new row + G5 spot-check) → all addressed. ✓

**Placeholder scan:** no "TBD"/"TODO"/"implement later"/"similar to Task N" in this plan. Every step has concrete commands or concrete doc text. The 11-row recipe and translation tables are copied verbatim, not summarized. ✓

**Type consistency:** this is a doc-only change (no types/functions defined). Recipe strings are used identically across all group tasks. Cross-crate field names (`items`, `page_token`, `user_ids`) are pinned to one Chinese each in the table and re-checked in Task 6 Step 8. ✓

**Task ordering note:** Task 0 (pilot) must land before Task 1 (G1) — G1 re-verifies the pilot file and counts its 11 in the 104 total. Tasks 1–5 (G1–G5) are independent (different crates) and could run in parallel worktrees if desired; each is a self-contained compilation unit. Task 6 (gate) runs after all of Task 0 + G1–G5 are committed.
