---
change: fix-platform-v1-feature-gating
design-doc: docs/superpowers/specs/2026-06-26-fix-platform-v1-feature-gating-design.md
base-ref: b92dccb95434ddb5c62c3179cdcce2d8f79dff1b
archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

# 修复 platform v1 feature gating 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 移除 openlark-platform 中 6 个文件共 10 处多余的 `#[cfg(feature = "v1")]` 门控，使 `default`/`full` feature 组合下四个 platform service 真正暴露其 API 实现（不再是空壳 facade）。

**Architecture:** service 已由业务模块 feature（`admin`/`app-engine`/`directory`/`spark`，均在 `default`）控制是否编译，`v1` 是多余的版本第二层门控。方案 A（已 brainstorming 确认）：纯 cfg 属性删除，不改 Cargo.toml、不删 feature flag、不动 API。`v1` flag 必须保留（测试依赖）；`v4` 为空 no-op，按最小改动原则保持现状。

**Tech Stack:** Rust, Cargo features, `#[cfg(feature = "...")]`。

## Global Constraints

- **方案锁定 A**：只删 `#[cfg(feature = "v1")]` 属性，不改 `crates/openlark-platform/Cargo.toml`（不把 `v1` 加进 default/full），不删任何 feature flag，不动 API 符号。
- **`v1` feature 必须保留**：`crates/openlark-platform/src/lib.rs:161` 测试 `#[cfg(all(feature = "spark", feature = "v1"))]` 与 `tests/platform_contract_models.rs:2-11`（`#![cfg(all(... feature = "v1" ...))]`）依赖它。
- **`v4` 不动**：`v4` 不门控任何代码（仅自身 `full = [... "v4"]` 定义命中），留作无害 no-op，不顺手清理（设计 D2）。
- **删除时连带移除上方 `/// V1 版本 API` 注释**（仅对 `pub fn v1()` 方法门控处；`pub mod` 门控处无注释，删属性即可），保持模块注释整洁。
- **不要提交 git**（主会话负责提交）；每个 task 验收由主会话勾选 `tasks.md` 并提交。
- **base-ref**：`b92dccb95434ddb5c62c3179cdcce2d8f79dff1b`。

## File Structure（映射到 task）

修改文件清单（**仅删除 cfg 属性 + 方法注释，无新增文件、无重构**）：

| 文件 | 修改点 | Task |
|------|--------|------|
| `crates/openlark-platform/src/admin.rs:28-29, 35` | 删 `pub fn v1()` 上方注释+属性；删 `pub mod admin;` 上方属性 | Task 2 |
| `crates/openlark-platform/src/app_engine.rs:28-29, 35` | 同上模式（`pub mod apaas;`） | Task 2 |
| `crates/openlark-platform/src/directory/mod.rs:28-29, 35` | 同上模式（`pub mod directory;`） | Task 2 |
| `crates/openlark-platform/src/directory/directory/mod.rs:3` | 删 intermediate `pub mod v1;` 上方属性 | Task 2 |
| `crates/openlark-platform/src/spark/mod.rs:25-26, 32` | 同上模式（`pub mod spark;`） | Task 2 |
| `crates/openlark-platform/src/spark/spark/mod.rs:3` | 删 intermediate `pub mod v1;` 上方属性 | Task 2 |
| `CHANGELOG.md` | `[Unreleased]` 下加 Fixed 条目 | Task 5 |

不修改：`Cargo.toml`、`lib.rs`（测试 cfg 是测试自己的门控，不删）、4 个 service 的 `tests` 子模块。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Task 1: 调研确认 — v1/v4 在测试中的引用边界

**Files:**
- Read: `crates/openlark-platform/src/lib.rs:150-168`
- Read: `crates/openlark-platform/tests/platform_contract_models.rs:1-15`
- Read: `crates/openlark-platform/Cargo.toml`（`[features]` 段）

**Interfaces:**
- Consumes: 无
- Produces: 一份"必须保留的 v1 引用清单"证据，作为 Task 2 删除时不得触碰的红线。

**目的：** 实施前用 grep+Read 把 v1/v4 的所有引用点钉死，防止 Task 2 误删测试自身的 cfg 门控（那是测试选择何时运行的门控，与我们要删的 service facade 门控是两回事）。

- [x] **Step 1: grep 全部 `feature = "v[1-4]"` 引用**

Run:
```bash
rg -n 'feature\s*=\s*"v[1-4]"' crates/openlark-platform/ --type rust
rg -n '"v[1-4]"' crates/openlark-platform/Cargo.toml
```
Expected: src 命中 = 我们要删的 10 处 facade/intermediate 门控（admin.rs:29,35 / app_engine.rs:29,35 / directory/mod.rs:29,35 / directory/directory/mod.rs:3 / spark/mod.rs:26,32 / spark/spark/mod.rs:3）**加上** `lib.rs:161`（测试自身门控，**保留**）和 `tests/platform_contract_models.rs:4`（集成测试自身门控，**保留**）。Cargo.toml 命中 = `v1`/`v2`/`v3`/`v4` 定义行 + `full = [... "v4"]`（**全部保留**）。

- [x] **Step 2: 确认 lib.rs:161 测试门控语义**

Read `crates/openlark-platform/src/lib.rs:150-168`。确认第 161 行 `#[cfg(all(feature = "spark", feature = "v1"))]` 是测试模块**自己**选择何时运行该 test 的门控（仅当 `spark` 与 `v1` 同时启用才编译/运行），第 166 行 `service.spark().v1().directory().user().id_convert()` 在该门控下成立。**该 cfg 不在删除清单内**（删除清单只含 6 个 service 源文件的门控）。

- [x] **Step 3: 确认 tests/platform_contract_models.rs 顶层 `#![cfg(...)]`**

Read `crates/openlark-platform/tests/platform_contract_models.rs:1-15`。确认 `#![cfg(all(feature = "admin", feature = "v1", ...))]` 是 crate 级测试门控，决定整个集成测试文件何时编译。**保留**（v1 在该门控里，移除会让 default 下该文件尝试编译，但 v1 仍不在 default —— 门控逻辑不变）。

- [x] **Step 4: 确认 v4 为空 no-op**

Run:
```bash
rg -n 'feature\s*=\s*"v4"' crates/openlark-platform/src/ crates/openlark-platform/tests/ || echo "(no v4 cfg in code)"
```
Expected: `(no v4 cfg in code)` —— 证实 v4 仅在 `Cargo.toml` 的 `full = [... "v4"]` 中出现，不门控任何代码。**保持现状**（设计 D2）。

- [x] **Step 5: 记录证据，本 task 无代码改动、无需提交**

把 Step 1-4 的命中清单记在心里（或贴到 PR 描述）。本 task 是 read-only 调研，零代码改动，主会话按 tasks.md 1.1-1.3 勾选并提交（如策略要求）。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Task 2: 移除 6 文件 10 处 v1 facade/intermediate 门控

**Files:**
- Modify: `crates/openlark-platform/src/admin.rs:28-29, 35`
- Modify: `crates/openlark-platform/src/app_engine.rs:28-29, 35`
- Modify: `crates/openlark-platform/src/directory/mod.rs:28-29, 35`
- Modify: `crates/openlark-platform/src/directory/directory/mod.rs:3`
- Modify: `crates/openlark-platform/src/spark/mod.rs:25-26, 32`
- Modify: `crates/openlark-platform/src/spark/spark/mod.rs:3`

**Interfaces:**
- Consumes: Task 1 的"必须保留 v1 引用清单"（不得触碰 lib.rs:161、tests 文件门控、Cargo.toml）。
- Produces: 四个 service 在 `default` feature 下暴露 `.v1()` 方法与完整 API 子树（`admin/admin/**`、`app_engine/apaas/**`、`directory/directory/**`、`spark/spark/**` 全部参与编译）。

**删除规则（统一）：**
- 方法门控处（`pub fn v1()`）：删除其上方紧邻的 `/// V1 版本 API` 注释行 **和** `#[cfg(feature = "v1")]` 属性行，**保留**方法签名与方法体不变。
- 模块门控处（`pub mod admin;` / `pub mod apaas;` / `pub mod directory;` / `pub mod spark;`）：仅删除 `#[cfg(feature = "v1")]` 属性行，**保留** `pub mod ...;` 声明。
- intermediate 门控处（`directory/directory/mod.rs:3` 与 `spark/spark/mod.rs:3` 的 `pub mod v1;`）：仅删除 `#[cfg(feature = "v1")]` 属性行，**保留** `pub mod v1;`。

- [x] **Step 1: admin.rs — 删 2 处**

文件 `crates/openlark-platform/src/admin.rs`：

第 28-29 行（方法门控），删掉注释 + 属性：
```rust
    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::admin::admin::v1::AdminV1 {
```
改为：
```rust
    pub fn v1(&self) -> crate::admin::admin::v1::AdminV1 {
```

第 35 行（模块门控），删掉属性：
```rust
#[cfg(feature = "v1")]
pub mod admin;
```
改为：
```rust
pub mod admin;
```

- [x] **Step 2: app_engine.rs — 删 2 处**

文件 `crates/openlark-platform/src/app_engine.rs`：

第 28-29 行（方法门控）：
```rust
    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::app_engine::apaas::v1::ApaasV1 {
```
→
```rust
    pub fn v1(&self) -> crate::app_engine::apaas::v1::ApaasV1 {
```

第 35 行（模块门控）：
```rust
#[cfg(feature = "v1")]
pub mod apaas;
```
→
```rust
pub mod apaas;
```

- [x] **Step 3: directory/mod.rs — 删 2 处**

文件 `crates/openlark-platform/src/directory/mod.rs`：

第 28-29 行（方法门控）：
```rust
    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::directory::directory::v1::DirectoryV1 {
```
→
```rust
    pub fn v1(&self) -> crate::directory::directory::v1::DirectoryV1 {
```

第 35 行（模块门控）：
```rust
#[cfg(feature = "v1")]
pub mod directory;
```
→
```rust
pub mod directory;
```

- [x] **Step 4: directory/directory/mod.rs — 删 intermediate 1 处**

文件 `crates/openlark-platform/src/directory/directory/mod.rs`，第 3 行：
```rust
#[cfg(feature = "v1")]
pub mod v1;
```
→
```rust
pub mod v1;
```

- [x] **Step 5: spark/mod.rs — 删 2 处**

文件 `crates/openlark-platform/src/spark/mod.rs`：

第 25-26 行（方法门控）：
```rust
    /// V1 版本 API
    #[cfg(feature = "v1")]
    pub fn v1(&self) -> crate::spark::spark::v1::SparkV1 {
```
→
```rust
    pub fn v1(&self) -> crate::spark::spark::v1::SparkV1 {
```

第 32 行（模块门控）：
```rust
#[cfg(feature = "v1")]
pub mod spark;
```
→
```rust
pub mod spark;
```

- [x] **Step 6: spark/spark/mod.rs — 删 intermediate 1 处**

文件 `crates/openlark-platform/src/spark/spark/mod.rs`，第 3 行：
```rust
#[cfg(feature = "v1")]
pub mod v1;
```
→
```rust
pub mod v1;
```

- [x] **Step 7: 复核删除计数**

Run:
```bash
rg -n 'cfg\(feature\s*=\s*"v1"\)' crates/openlark-platform/src/
```
Expected: **零命中**（6 个 service 源文件里 v1 门控全部清除）。若仍有命中，说明漏删，回 Step 1-6 补。

再确认保留项未被误删：
```bash
rg -n 'feature\s*=\s*"v1"' crates/openlark-platform/src/lib.rs crates/openlark-platform/tests/platform_contract_models.rs crates/openlark-platform/Cargo.toml
```
Expected: 3 处命中（lib.rs:161、tests/platform_contract_models.rs:4、Cargo.toml v1 定义行），全部保留。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Task 3: 编译验证（default + full）

**Files:**
- 无修改；若暴露 latent 编译错误则就地修复 v1 子树中的具体问题。

**Interfaces:**
- Consumes: Task 2 的 10 处删除。
- Produces: `cargo check` 在 default 与 full 两个 feature 组合下均通过；v1 API 子树（长期未被标准构建编译）验证可编译。

- [x] **Step 1: default 编译**

Run:
```bash
cargo check -p openlark-platform
```
Expected: 编译通过，exit 0。此时 `admin/admin/**`、`app_engine/apaas/**`、`directory/directory/**`、`spark/spark/**` 子树首次进入标准构建。

- [x] **Step 2: full 编译**

Run:
```bash
cargo check -p openlark-platform --all-features
```
Expected: 编译通过，exit 0。96 个 API 实现全部参与编译。

- [x] **Step 3: 处理 latent 编译错误（如有）**

若 Step 1 或 Step 2 失败（v1 子树长期未被标准构建编译，可能暴露遗留问题），加载 `systematic-debugging` skill 逐个定位修复。**只修暴露的真实编译错误，不做顺手重构。** 在 PR 描述记录每个修复点（文件:行 + 原因）。

若 Step 1/2 均通过，跳过本步。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Task 4: 质量门（clippy + 测试）

**Files:**
- 无修改；纯验证。

**Interfaces:**
- Consumes: Task 2 的删除 + Task 3 的编译通过。
- Produces: 两组 clippy 零 warning + platform 单元/集成测试通过。

- [x] **Step 1: no-default-features clippy**

Run:
```bash
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
```
Expected: exit 0（无 warning）。验证移除 facade 门控不影响 `--no-default-features` 下 service 模块本就不编译的 test-gating 行为。

- [x] **Step 2: all-features clippy**

Run:
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: exit 0（无 warning）。此时 v1 子树全部参与编译，clippy 覆盖最全。

- [x] **Step 3: platform 测试**

Run:
```bash
cargo test -p openlark-platform
```
Expected: 通过。包含：
- 4 个 service 的 `test_service_creation` 单元测试（admin/app_engine/directory/spark）。
- `lib.rs:161` 的 `test_platform_service_spark` 在 default 下因缺 `v1` 被 cfg-skip（正常），在 `--all-features` 下运行并验证 `service.spark().v1().directory().user().id_convert()` 链路编译。

- [x] **Step 4: 全 feature 组合复测（确认 spark v1 链路）**

Run:
```bash
cargo test -p openlark-platform --all-features
```
Expected: 通过。`test_platform_service_spark` 此时**会运行**（spark+v1 均启用），确认 `.v1()` 调用链完整可执行。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Task 5: 文档收尾

**Files:**
- Modify: `CHANGELOG.md`（`[Unreleased]` 段）

**Interfaces:**
- Consumes: Task 1-4 全部通过。
- Produces: CHANGELOG 记录本次行为补全。

- [x] **Step 1: CHANGELOG 加 Fixed 条目**

在 `CHANGELOG.md` 的 `## [Unreleased]` 段下，按现有结构添加一个 `### Fixed`（或 `### Changed`）子段（若已有则在其列表追加）。条目内容（中文，符合项目风格）：

```markdown
### Fixed

- **fix(platform)**: 移除 `openlark-platform` 四个 service（Admin/AppEngine/Directory/Spark）
  facade 与 intermediate 层多余的 `#[cfg(feature = "v1")]` 门控。此前 `default`/`full`
  feature 下 service 启用却暴露空壳 facade（96 个 API 被排除在标准构建外）。移除后
  "service 启用 = API 可达"，与 hr/communication/meeting 一致。行为补全，非 breaking：
  仅让原本不可达的公开 API 变为可达，不移除任何符号。`v1` feature 保留（测试依赖）。
```

- [x] **Step 2: cargo doc 生成确认**

Run:
```bash
cargo doc -p openlark-platform --all-features --no-deps
```
Expected: 文档生成成功，exit 0。确认四个 service 的 `.v1()` 方法现在出现在 `--all-features` 文档中（移除门控前因 v1 子树空，方法实际不参与编译）。

- [x] **Step 3: 不提交 git**

本计划不执行 git 提交。全部 task 完成后，交主会话：勾选 `openspec/changes/fix-platform-v1-feature-gating/tasks.md` 对应项 → 由主会话按 comet 流程提交。

archived-with: 2026-06-27-fix-platform-v1-feature-gating
---

## Self-Review（plan 完成后自检）

1. **Spec 覆盖**：
   - spec「default 下暴露 API」→ Task 2（移除门控）+ Task 3 Step 1（default check）。✓
   - spec「full 下暴露 API」→ Task 2 + Task 3 Step 2（full check）。✓
   - spec「clippy 测试门控不回归」→ Task 4 Step 1-2。✓
   - spec「公开 API 符号不被移除」→ 全程只删 cfg 属性、不动签名（Task 2 明确"保留方法签名与方法体"）。✓
   - design D2「v4 保持现状」→ Task 1 Step 4 确认 + Global Constraints 声明不动 Cargo.toml。✓
   - design「v1 必留」→ Task 1 Step 2-3 + Task 2 Step 7 复核保留项。✓

2. **Placeholder 扫描**：无 TBD/TODO；每个删除点给出确切 before/after 代码块；命令均带 Expected。✓

3. **类型/符号一致性**：方法签名（`AdminV1`/`ApaasV1`/`DirectoryV1`/`SparkV1` 返回类型）在 Task 2 各步与源码逐字核对一致。✓

4. **风险预案**：Task 3 Step 3 显式处理 latent 编译错误（加载 systematic-debugging，只修真实错误）。✓
