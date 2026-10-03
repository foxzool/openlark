---
change: remove-deprecated-accessors
design-doc: docs/superpowers/specs/2026-06-28-remove-deprecated-accessors-design.md
base-ref: e9c4b0267bd69ffdbf400524fd440c9a2b755b31
archived-with: 2026-06-28-remove-deprecated-accessors
---

# remove-deprecated-accessors 实施计划

> **事实源**：delta spec `no-deprecated-compat-accessors`。技术决策见 design doc。

**Goal**：删除 10 个 `#[deprecated] pub fn`（HR 8 service 访问器 + analytics 2 未接线存根）。BREAKING，0 内部调用。关联 #268（A+E）。

## Global Constraints

- **base-ref**：`e9c4b0267`。
- 不要用 `git stash`（仓库有历史 stash）。临时改动用 `cp`/`git checkout -- <file>`。
- D2：保留 `QueryApi`/`UserSearchApi` 类型与 `v2/query.rs`/`v2/user.rs` 模块（仅删 `SearchV2::query()`/`user()` 方法）。
- 不动 B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）——另开 follow-up issue。
- 不提交 git（主会话负责）；每个 task 完成由主会话勾选 tasks.md + commit。

## Task 1: 删除 HR 8 个 deprecated 访问器

**Files:** `crates/openlark-hr/src/lib.rs`

- [x] **Step 1: 定位 8 个方法**：`grep -n '#\[deprecated' crates/openlark-hr/src/lib.rs`（约 L148-225，含每个上方的 `#[cfg(feature = "xxx")]` 门控 + `#[deprecated(...)]` + 文档注释 + `pub fn xxx(&self)` 方法体）。
- [x] **Step 2: 删除 8 个方法块**：attendance/corehr/compensation/payroll/performance/okr/hire/ehr，连同各自 `#[cfg(feature)]` + `#[deprecated]` + `///` 文档注释 + 方法体。保留 struct 的同名**字段**（`attendance`/`corehr`/... 是字段，不删）。
- [x] **Step 3: 验证**：`grep -c 'pub fn attendance\|pub fn corehr\|pub fn compensation\|pub fn payroll\|pub fn performance\|pub fn okr\|pub fn hire\|pub fn ehr' crates/openlark-hr/src/lib.rs` = 0。

## Task 2: 删除 analytics 2 个 deprecated 存根

**Files:** `crates/openlark-analytics/src/search/search/v2.rs`

- [x] **Step 1: 删除 `query()` 与 `user()`**：连同各自 `#[deprecated]` + `///` 注释 + 方法体（约 L20-36）。**保留** `pub mod query; pub mod user;` 模块声明与 `SearchV2` struct（D2）。
- [x] **Step 2: 验证**：`grep -c 'pub fn query\|pub fn user' crates/openlark-analytics/src/search/search/v2.rs` = 0；`QueryApi`/`UserSearchApi` 类型仍在（`v2/query.rs`/`v2/user.rs` 保留）。

## Task 3: 验证

- [x] **Step 1: 三组 clippy**：`cargo clippy --workspace --all-targets -- -D warnings`（default / `--all-features` / `--no-default-features`）均 exit 0。
- [x] **Step 2: test**：`cargo test --workspace` 通过。
- [x] **Step 3: examples 不引用已移除方法**：`grep -rn '\.attendance()\|\.corehr()\|\.query()\|\.user()' examples/` = 0。
- [x] **Step 4: 目标 deprecated 已删**：HR+analytics 的目标 10 个 = 0（剩余 B/C/D/F/G 不在范围）。

## Task 4: CHANGELOG + follow-up

- [x] **Step 1: CHANGELOG** `[Unreleased] > Breaking Changes` 加条目 + 迁移映射表（方法→字段 / 其他 surface），见 design doc「迁移映射」。
- [x] **Step 2: 开 follow-up issue**：B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder）共 10 个 deprecated 异构项的迁移清理。
- [x] **Step 3: 评论 #268**：本 change 处理 A+E（10），B/C/D/F/G 拆至 follow-up；#268 待 follow-up 完成后关闭（或本 change 关 A+E 部分）。

## Self-Review

- Spec 覆盖：req1（HR 不暴露）→ Task 1；req2（analytics 去存根）→ Task 2；req3（构建/examples 不破坏）→ Task 3。✓
- D2 保留类型：Task 2 Step 2 明确保留 `QueryApi`/`UserSearchApi`。✓
- 无 placeholder；每步带验证命令。
