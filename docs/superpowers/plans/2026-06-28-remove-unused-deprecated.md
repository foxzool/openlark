---
change: remove-unused-deprecated
design-doc: docs/superpowers/specs/2026-06-28-remove-unused-deprecated-design.md
base-ref: 0156f201975953f58ba37eea9b28e2668770d41c
---

# remove-unused-deprecated 实施计划

> **事实源**：delta spec `no-unused-deprecated`。技术决策见 design doc。

**Goal**：删除 5 个零调用/dead `#[deprecated]` 项（G auth ×3 + D to_value + C 宏 new）。BREAKING。关联 #278（G+D+C）。

## Global Constraints

- **base-ref**：`0156f2019`。
- 不要用 `git stash`（仓库有历史 stash）。
- 不动 B（wiki Params）/ F（im 别名）——留 #278。
- C：从 `impl_required_builder!` 宏删除 `new()` 生成块（唯一调用者 TestRequest 用 builder()）。
- 不提交 git（主会话负责）。

## Task 1: 删除 G — auth 3 个 deprecated 方法

**Files:** `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`

- [ ] **Step 1:** 删除 `app_id()`/`app_secret()`/`app_ticket()` 3 个 `#[deprecated] pub fn`（约 L85-101，含各自 deprecation 注释 + 文档注释 + 方法体）。
- [ ] **Step 2:** 验证 `grep -c 'pub fn app_id\|pub fn app_secret\|pub fn app_ticket'` 该文件 = 0。

## Task 2: 删除 D — docs to_value

**Files:** `crates/openlark-docs/src/base/bitable/v1/field_types.rs`

- [ ] **Step 1:** 删除 `to_value()` 方法（约 L248-251，含 `#[deprecated]` + `pub fn to_value` + `json!(self)` 体）。
- [ ] **Step 2:** 验证 `grep -c 'pub fn to_value'` 该文件 = 0。

## Task 3: 删除 C — docs 宏 new()

**Files:** `crates/openlark-docs/src/common/request_builder.rs`

- [ ] **Step 1:** 从 `impl_required_builder!` 宏删除 `new()` 生成块（`#[deprecated]`+`#[expect(dead_code)]`+`pub fn new()`+`Self::default()` 体，约 L96-101）。
- [ ] **Step 2:** 确认 TestRequest 测试仍用 `builder()`（不依赖 new()）。

## Task 4: 验证

- [ ] **Step 1:** 三组 clippy（default / `--all-features` / `--no-default-features`）`-D warnings` exit 0。
- [ ] **Step 2:** `cargo test --workspace` 通过。
- [ ] **Step 3:** examples/tests 不引用已移除项（`.to_value()` / `tenant_access_token().app_id` 等 = 0）。
- [ ] **Step 4:** 目标 deprecated 已删（auth 3 + to_value + 宏 new = 5）。

## Task 5: CHANGELOG + 收尾

- [ ] **Step 1:** CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射表（见 design doc）。
- [ ] **Step 2:** 评论 #278：本 change 处理 G+D+C（5）；B（wiki Params ~16 用法）+ F（im 别名 47 文件）仍 open。

## Self-Review

- Spec 覆盖：req1（auth 不暴露）→ Task 1；req2（docs to_value + 宏 new）→ Task 2/3；req3（构建/examples 不破坏）→ Task 4。✓
- D2 宏 new 移除安全：Task 3 Step 2 确认。✓
- 无 placeholder；每步带验证命令。
