---
change: dedup-okr-v2-models
design-doc: docs/superpowers/specs/2026-07-04-dedup-okr-v2-models-design.md
base-ref: 3a462c410caf37b4c94d8d872587b093da26c420
---

# dedup-okr-v2-models 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 openlark-hr okr/v2 中跨叶 byte-identical 重复的 9 个 domain struct 抽到共享模块 `common/models.rs`，11 个叶子改 import 引用，行为零变化。

**Architecture:** 纯机械挪位重构。新建 `okr/okr/v2/common/{mod.rs, models.rs}` 收纳 9 struct 各一份；11 叶删除 inline 定义、改 `use crate::okr::okr::v2::common::models::{...}` 显式具名引用；per-leaf Response wrapper（各包不同 shape）保持 inline。无字段调和、无行为变更。

**Tech Stack:** Rust, serde (Deserialize), openlark-core。MSRV 1.88。

**Design Doc:** `docs/superpowers/specs/2026-07-04-dedup-okr-v2-models-design.md`（D1-D4 决策，本计划据此落地）

## Global Constraints

- **D2 显式具名 import（覆盖 tasks.md 第 10 行的 glob 写法）：** 所有叶子用 `use crate::okr::okr::v2::common::models::{StructA, StructB};` **逐个列出该叶实际用到的 struct**。禁止 `use ...::*` glob——repo `[workspace.lints.clippy]` 启用 `wildcard_imports` 且 CI `-D warnings`，glob 会致 clippy fail。各叶 import 清单见 Task 2-4。

- **D2 实施细化（实施时核实）**：「实际用到」= 叶源码中**直接出现**该 struct 名（如 Response wrapper 字段类型 `Objective` / `Vec<Objective>`）。Owner/Unit 等子 struct 是主 struct 的字段类型、在 `common/models` 内部引用，叶内不直接出现——**不要导入**它们，否则 `unused_imports` 致 clippy fail。Task 2-4 body 的清单（如 `{Objective, ObjectiveOwner}`）是过列示例；**以「叶内直接引用的 struct」为准**。Task 2 四叶经核实均只导入 `Objective`；Task 3/4 同理须按各叶直接引用核实（通常每叶只导入 1 个主 struct）。
- **D3 clean break：** 9 struct 规范路径改为 `common::models::<Struct>`，**不留 `pub use` re-export** 做向后兼容。okr/v2 零外部引用（#327/#328 确认）。
- **D4 Response wrapper 保持 inline：** `GetObjectiveResponse`/`ListObjectivesResponse`/`PatchIndicatorResponse` 等 per-leaf wrapper 不挪动，仅其内部嵌套的 domain entity 改引用共享定义。
- **零字段变更、零行为变更：** 9 struct 跨叶已 byte-identical 确认（Design Doc PYTHONHASHSEED 验证 + 本计划作者 diff 抽样复验通过），整块迁移。
- **保留 `use serde::Deserialize;`：** 每叶 Response wrapper 仍 `#[derive(Deserialize)]`，删除 domain struct 后 serde import **不能误删**。
- **commit message 用中文**（匹配仓库风格，参考 recent commits "chore: 归档..."）。
- **base-ref:** `3a462c410caf37b4c94d8d872587b093da26c420`
- **跨叶 consumer 修正（Task 2 实施时发现，原计划文件清单遗漏）**：部分叶子通过 `use super::...::get::<Struct>` 跨叶复用 canonical 叶的 struct。迁移时这些 consumer 的 import 路径也要改成 `common::models`（**仅改路径，无 inline 删除**）。完整清单（全仓 `use super::` 审计 + 零外部引用确认）：
  - **Task 2 (Objective)**：+ `objective/patch.rs:16`（`use super::get::Objective` → `common::models::Objective`）。已授权并入 Task 2。
  - **Task 4 (KeyResult)**：+ `objective/key_results_weight.rs:16`、`objective/key_results_position.rs:16`、`objective/key_result/list.rs:16`（三者 `use super::...key_result::get::KeyResult` → `common::models::KeyResult`；**仅 KeyResult，不含 KeyResultOwner**）。
  - **Task 3 (Indicator)**：无跨叶 consumer（原 3 叶清单完整）。
  - **Task 4 (Alignment)**：无跨叶 consumer（原 2 叶清单完整）。
  - 零外部（非 okr/v2）引用，与 #327/#328 一致。

## 关于 TDD 的说明（重构特例）

本 change 是**纯机械重构，零行为变化**：9 struct 整块挪位，反序列化结果不变。每个 canonical 叶**已存在** typed Response 反序列化测试（如 `test_get_objective_response_deserialize`、`test_patch_indicator_response_deserialize`、`test_get_key_result_response_deserialize`、`test_get_alignment_response_deserialize`），它们就是回归基线。

因此本计划不新增测试（新测试只能验证"挪位本身"，无行为价值）。每个 task 的"测试周期"= **运行现有测试确认重构前后均通过**（red-green 变体：基线绿 → 改 → 仍绿）。这与 writing-plans 的 TDD 精神一致：测试行为而非实现，重构靠现有行为测试守护。

## File Structure

| 文件 | 动作 | 职责 |
|------|------|------|
| `crates/openlark-hr/src/okr/okr/v2/common/mod.rs` | 新建 | 声明 `pub mod models;` + 模块 `//!` 说明 |
| `crates/openlark-hr/src/okr/okr/v2/common/models.rs` | 新建 | 9 个跨叶共享 domain struct 各一份（DRY 单一定义） |
| `crates/openlark-hr/src/okr/okr/v2/mod.rs` | 修改 | 在 `pub mod` 区加 `pub mod common;`（字母序：category 与 cycle 之间） |
| `crates/openlark-hr/src/okr/okr/v2/objective/get.rs` | 修改 | 删 Objective + ObjectiveOwner inline 定义；加 import |
| `crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs` | 修改 | 删 Indicator + IndicatorOwner + IndicatorUnit；加 import |
| `crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/key_result/get.rs` | 修改 | 删 KeyResult + KeyResultOwner；加 import |
| `crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs` | 修改 | 同上 |
| `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs` | 修改 | 删 Alignment + AlignmentOwner；加 import |
| `crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs` | 修改 | 同上 |

**不改动：** `key_result/progress/list.rs`（`KeyResultProgress*` 是不同 struct）、各 `mod.rs` 的 `*Resource` service 结构、per-leaf Response wrapper、okr/v1、其他 crate。

---

## Task 1: 建共享模块 + 迁移 9 struct

**Files:**
- Create: `crates/openlark-hr/src/okr/okr/v2/common/mod.rs`
- Create: `crates/openlark-hr/src/okr/okr/v2/common/models.rs`
- Modify: `crates/openlark-hr/src/okr/okr/v2/mod.rs`（`pub mod` 区，约第 6-12 行）

**Interfaces:**
- Produces: `crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner, Indicator, IndicatorOwner, IndicatorUnit, KeyResult, KeyResultOwner, Alignment, AlignmentOwner}` —— Task 2-4 全部依赖这些路径。

- [ ] **Step 1: 建运行基线，确认起点干净**

先确认 base-ref 状态下 openlark-hr 全绿（后续每个 task 都以此为基线对照）。

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -5
```
Expected: `Finished` 无错误。

Run:
```bash
cd /Users/zool/workspace/openlark
cargo test -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: 所有测试 PASS（含 `test_get_objective_response_deserialize`、`test_patch_indicator_response_deserialize`、`test_get_key_result_response_deserialize`、`test_get_alignment_response_deserialize`）。

如果基线已有失败，停下来先报告（不要让重构背锅）。

- [ ] **Step 2: 新建 `common/mod.rs`**

Create `crates/openlark-hr/src/okr/okr/v2/common/mod.rs`：

```rust
//! okr/v2 跨叶共享 domain struct。
//!
//! 收纳在多个叶子（如 objective/get、cycle/objective/list 等）中
//! 重复出现的同一飞书实体的 typed 表示，避免逐字重复定义（Shotgun Surgery）。

pub mod models;
```

- [ ] **Step 3: 新建 `common/models.rs`（9 struct 整块迁移）**

Create `crates/openlark-hr/src/okr/okr/v2/common/models.rs`，内容为下面**完整 9 struct 定义**（从 4 个 canonical 叶逐字复制：Objective/ObjectiveOwner ← objective/get.rs；Indicator/IndicatorOwner/IndicatorUnit ← indicator/patch.rs；KeyResult/KeyResultOwner ← key_result/get.rs；Alignment/AlignmentOwner ← alignment/get.rs）：

```rust
//! okr/v2 跨叶共享的 domain entity struct。
//!
//! 这些 struct 代表同一飞书实体，跨多个 API 叶子重复出现（已确认 byte-identical）。
//! 为避免逐字重复（#336），各只在此处定义一次，叶子通过 `use` 引用。

use serde::Deserialize;

/// OKR 目标。
#[derive(Debug, Clone, Deserialize)]
pub struct Objective {
    /// 目标的 ID。
    pub id: String,
    /// 目标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 目标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: ObjectiveOwner,
    /// 目标的用户周期 ID。
    pub cycle_id: String,
    /// 目标的序号：从 1 开始计数。
    pub position: i32,
    /// 目标的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 目标的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的备注。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub notes: Option<serde_json::Value>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 目标的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
    /// 目标的分类 ID。
    #[serde(default)]
    pub category_id: Option<String>,
}

/// 目标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectiveOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 量化指标。
#[derive(Debug, Clone, Deserialize)]
pub struct Indicator {
    /// 指标的 ID。
    pub id: String,
    /// 指标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 指标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: IndicatorOwner,
    /// 指标所属的实体类型。
    pub entity_type: i32,
    /// 指标所属的实体 ID。
    pub entity_id: String,
    /// 指标的状态。
    pub indicator_status: i32,
    /// 指标的状态的计算方式。
    pub status_calculate_type: i32,
    /// 指标的起始值。
    #[serde(default)]
    pub start_value: Option<f64>,
    /// 指标的目标值。
    #[serde(default)]
    pub target_value: Option<f64>,
    /// 指标的当前值。
    #[serde(default)]
    pub current_value: Option<f64>,
    /// 指标的当前值的计算方式。
    #[serde(default)]
    pub current_value_calculate_type: Option<i32>,
    /// 指标的单位。
    #[serde(default)]
    pub unit: Option<IndicatorUnit>,
}

/// 指标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 指标单位。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorUnit {
    /// 指标的单位类型。
    pub unit_type: i32,
    /// 指标单位的值。
    pub unit_value: String,
}

/// 关键结果。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResult {
    /// 关键结果的 ID。
    pub id: String,
    /// 关键结果的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 关键结果的修改时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: KeyResultOwner,
    /// 关键结果的目标 ID。
    pub objective_id: String,
    /// 关键结果的序号：从 1 开始计数。
    pub position: i32,
    /// 关键结果的内容。
    // TODO: 飞书文档 block 深度嵌套结构暂留 Value，后续可单独抽取 typed 模型。
    #[serde(default)]
    pub content: Option<serde_json::Value>,
    /// 关键结果的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 关键结果的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
}

/// 关键结果所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// OKR 对齐。
#[derive(Debug, Clone, Deserialize)]
pub struct Alignment {
    /// 对齐的 ID。
    pub id: String,
    /// 对齐的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 对齐的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 发起对齐的所有者。
    pub from_owner: AlignmentOwner,
    /// 被对齐的所有者。
    pub to_owner: AlignmentOwner,
    /// 发起对齐的实体类型。
    pub from_entity_type: i32,
    /// 发起对齐的实体 ID。
    pub from_entity_id: String,
    /// 被对齐的实体类型。
    pub to_entity_type: i32,
    /// 被对齐的实体 ID。
    pub to_entity_id: String,
}

/// 对齐所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct AlignmentOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}
```

- [ ] **Step 4: 在 `v2/mod.rs` 声明 `pub mod common;`**

Modify `crates/openlark-hr/src/okr/okr/v2/mod.rs`。当前 `pub mod` 区（约第 6-12 行）按字母序：

```rust
pub mod alignment;
pub mod category;
pub mod cycle;
pub mod indicator;
pub mod key_result;
pub mod objective;
```

在 `category` 与 `cycle` 之间插入 `pub mod common;`（字母序 c-o-m-m-o-n 在 c-y-c-l-e 之前）：

```rust
pub mod alignment;
pub mod category;
pub mod common;
pub mod cycle;
pub mod indicator;
pub mod key_result;
pub mod objective;
```

- [ ] **Step 5: build 验证（中间状态：common 已建，叶子仍用 inline）**

此时 `common/models.rs` 的 9 struct 是 pub 的（暂无引用方，但 pub 项不报 dead_code），叶子仍用各自 inline 定义。同名 struct 在不同模块路径下合法，编译应通过。

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -5
```
Expected: `Finished` 无错误、无新 warning。

若失败：检查 `common/mod.rs` 和 `models.rs` 路径/语法，以及 `v2/mod.rs` 的 `pub mod common;` 位置。

- [ ] **Step 6: 跑现有测试确认零回归**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo test -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: 全部 PASS（与 Step 1 基线一致）。

- [ ] **Step 7: clippy + fmt 预检**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -5
cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: fmt 无 diff；clippy 无 warning。

- [ ] **Step 8: Commit**

```bash
cd /Users/zool/workspace/openlark
git add crates/openlark-hr/src/okr/okr/v2/common/ crates/openlark-hr/src/okr/okr/v2/mod.rs
git commit -m "refactor(hr): 新建 okr/v2 共享模块 common/models.rs 收纳 9 struct 单一定义 (#336)"
```

---

## Task 2: Objective 组 4 叶改 import

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/get.rs`（删 Objective line ~71-106 + ObjectiveOwner line ~108-116，加 import）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs`（删 Objective line ~78-113 + ObjectiveOwner line ~115-123，加 import）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs`（删 Objective + ObjectiveOwner，加 import）
- Modify: `crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs`（删 Objective + ObjectiveOwner，加 import）

**Interfaces:**
- Consumes: `crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner}`（来自 Task 1）
- Produces: 4 叶的 `GetObjectiveResponse`/`ListObjectivesResponse`/etc. wrapper 内部 `Objective` 字段类型不变（仅来源改为 import）。

**转换模式（每叶相同）：**
1. 在 `use` 区（`use serde::Deserialize;` 之后）加一行：
   ```rust
   use crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner};
   ```
2. 删除 inline 的 `/// OKR 目标。` + `#[derive(Debug, Clone, Deserialize)] pub struct Objective { ... }` 整块
3. 删除 inline 的 `/// 目标所有者。` + `#[derive(Debug, Clone, Deserialize)] pub struct ObjectiveOwner { ... }` 整块
4. **保留** `use serde::Deserialize;`（Response wrapper 仍需要）
5. **保留** per-leaf Response wrapper（如 `GetObjectiveResponse`、`ListObjectivesResponse`）及其 `impl ApiResponseTrait`

- [ ] **Step 1: 改 `objective/get.rs`**

按转换模式处理 `crates/openlark-hr/src/okr/okr/v2/objective/get.rs`：
- 在 `use serde::Deserialize;`（第 13 行）之后加：
  ```rust
  use crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner};
  ```
- 删除第 71-116 行（`/// OKR 目标。` 到 `ObjectiveOwner` struct 闭合 `}`）—— 即 `Objective` 和 `ObjectiveOwner` 两个 struct 定义块及其上方 doc 注释。
- 保留 `GetObjectiveResponse`（line 58-69）不动。

- [ ] **Step 2: 改 `cycle/objective/list.rs`**

同模式处理 `crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs`：
- 加 import（同上）
- 删除 `Objective`（约 line 78-113）+ `ObjectiveOwner`（约 line 115-123）整块
- 保留该叶的 list Response wrapper（`ListObjectivesResponse` 或同名）及 `ApiResponseTrait` impl

- [ ] **Step 3: 改 `cycle/objectives_position.rs`**

同模式。删 `Objective` + `ObjectiveOwner`，加 import，保留 Response wrapper。

- [ ] **Step 4: 改 `cycle/objectives_weight.rs`**

同模式。删 `Objective` + `ObjectiveOwner`，加 import，保留 Response wrapper。

- [ ] **Step 5: build 验证**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: `Finished` 无错误。

若报 "cannot find type `Objective`" → 漏加 import；若报 "cannot find type `ObjectiveOwner`" → import 缺 `ObjectiveOwner`；若报 duplicate/E025 → 某叶没删干净 inline 定义。

- [ ] **Step 6: 跑测试确认零回归**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo test -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: 全部 PASS。重点确认 `test_get_objective_response_deserialize` 仍 PASS（证明反序列化不变）。

- [ ] **Step 7: clippy + fmt**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -3
cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: fmt 无 diff；clippy 无 warning（显式具名 import 不触发 `wildcard_imports`）。

- [ ] **Step 8: Commit**

```bash
cd /Users/zool/workspace/openlark
git add crates/openlark-hr/src/okr/okr/v2/objective/get.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objective/list.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objectives_position.rs \
        crates/openlark-hr/src/okr/okr/v2/cycle/objectives_weight.rs
git commit -m "refactor(hr): okr/v2 Objective 组 4 叶改引用 common::models（删 inline 重复定义）(#336)"
```

---

## Task 3: Indicator 组 3 叶改 import

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs`（删 Indicator + IndicatorOwner + IndicatorUnit，加 import）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs`（同上）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs`（同上）

**Interfaces:**
- Consumes: `crate::okr::okr::v2::common::models::{Indicator, IndicatorOwner, IndicatorUnit}`（来自 Task 1）
- Produces: 3 叶 Response wrapper（如 `PatchIndicatorResponse`、`ListIndicatorsResponse`）内部 `Indicator` 字段类型不变。

**转换模式（每叶相同）：**
1. 加 import：
   ```rust
   use crate::okr::okr::v2::common::models::{Indicator, IndicatorOwner, IndicatorUnit};
   ```
2. 删除 inline 的 `Indicator`、`IndicatorOwner`、`IndicatorUnit` 三 struct 定义块（含各自 doc 注释）
3. 保留 `use serde::Deserialize;` 与 per-leaf Response wrapper

- [ ] **Step 1: 改 `indicator/patch.rs`（canonical 叶）**

处理 `crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs`：
- 在 `use serde::Deserialize;`（第 13 行）后加：
  ```rust
  use crate::okr::okr::v2::common::models::{Indicator, IndicatorOwner, IndicatorUnit};
  ```
- 删除第 82-135 行：`/// 量化指标。` + `Indicator` struct、`/// 指标所有者。` + `IndicatorOwner` struct、`/// 指标单位。` + `IndicatorUnit` struct
- 保留 `PatchIndicatorResponse`（line 68-80）不动

- [ ] **Step 2: 改 `objective/indicator/list.rs`**

同模式。删 3 个 Indicator struct，加 import，保留 list Response wrapper。

- [ ] **Step 3: 改 `key_result/indicator/list.rs`**

同模式。删 3 个 Indicator struct，加 import，保留 list Response wrapper。

- [ ] **Step 4: build 验证**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: `Finished` 无错误。

- [ ] **Step 5: 跑测试确认零回归**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo test -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: 全部 PASS。重点确认 `test_patch_indicator_response_deserialize` + `test_patch_indicator_response_deserialize_empty`（indicator/patch.rs 的两个测试）仍 PASS。

- [ ] **Step 6: clippy + fmt**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -3
cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: 全绿。

- [ ] **Step 7: Commit**

```bash
cd /Users/zool/workspace/openlark
git add crates/openlark-hr/src/okr/okr/v2/indicator/patch.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/indicator/list.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/indicator/list.rs
git commit -m "refactor(hr): okr/v2 Indicator 组 3 叶改引用 common::models（删 inline 重复定义）(#336)"
```

---

## Task 4: KeyResult + Alignment 组 4 叶改 import

**Files:**
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/get.rs`（删 KeyResult + KeyResultOwner）
- Modify: `crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs`（删 KeyResult + KeyResultOwner）
- Modify: `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs`（删 Alignment + AlignmentOwner）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs`（删 Alignment + AlignmentOwner）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_weight.rs`（**仅改 import 路径**：`use super::super::key_result::get::KeyResult` → `use crate::okr::okr::v2::common::models::KeyResult;`，无 inline 删除）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_results_position.rs`（**仅改 import 路径**，同上）
- Modify: `crates/openlark-hr/src/okr/okr/v2/objective/key_result/list.rs`（**仅改 import 路径**：`use super::super::super::key_result::get::KeyResult` → `use crate::okr::okr::v2::common::models::KeyResult;`）

**Interfaces:**
- Consumes:
  - KeyResult 组：`crate::okr::okr::v2::common::models::{KeyResult, KeyResultOwner}`
  - Alignment 组：`crate::okr::okr::v2::common::models::{Alignment, AlignmentOwner}`
- Produces: 4 叶 Response wrapper（`GetKeyResultResponse`/`PatchKeyResultResponse`/`GetAlignmentResponse`/`ListAlignmentsResponse`）内部字段类型不变。

**转换模式：**
- KeyResult 叶（get.rs / patch.rs）加：
  ```rust
  use crate::okr::okr::v2::common::models::{KeyResult, KeyResultOwner};
  ```
  删除 `KeyResult` + `KeyResultOwner` inline 定义。
- Alignment 叶（get.rs / list.rs）加：
  ```rust
  use crate::okr::okr::v2::common::models::{Alignment, AlignmentOwner};
  ```
  删除 `Alignment` + `AlignmentOwner` inline 定义。

- [ ] **Step 1: 改 `key_result/get.rs`（canonical）**

处理 `crates/openlark-hr/src/okr/okr/v2/key_result/get.rs`：
- `use serde::Deserialize;`（第 13 行）后加：
  ```rust
  use crate::okr::okr::v2::common::models::{KeyResult, KeyResultOwner};
  ```
- 删除第 70-108 行：`/// 关键结果。` + `KeyResult` struct、`/// 关键结果所有者。` + `KeyResultOwner` struct
- 保留 `GetKeyResultResponse`（line 57-68）不动

- [ ] **Step 2: 改 `key_result/patch.rs`**

同模式处理 `crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs`：
- 加 KeyResult import
- 删 `KeyResult` + `KeyResultOwner` inline 定义
- 保留 patch Response wrapper

- [ ] **Step 3: 改 `alignment/get.rs`（canonical）**

处理 `crates/openlark-hr/src/okr/okr/v2/alignment/get.rs`：
- `use serde::Deserialize;`（第 13 行）后加：
  ```rust
  use crate::okr::okr::v2::common::models::{Alignment, AlignmentOwner};
  ```
- 删除第 70-101 行：`/// OKR 对齐。` + `Alignment` struct、`/// 对齐所有者。` + `AlignmentOwner` struct
- 保留 `GetAlignmentResponse`（line 57-68）不动

- [ ] **Step 4: 改 `objective/alignment/list.rs`**

同模式。删 `Alignment` + `AlignmentOwner`，加 import，保留 list Response wrapper。

- [ ] **Step 5: build 验证**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: `Finished` 无错误。

- [ ] **Step 6: 跑测试确认零回归**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo test -p openlark-hr --all-features 2>&1 | tail -10
```
Expected: 全部 PASS。重点确认 `test_get_key_result_response_deserialize`、`test_get_alignment_response_deserialize` 仍 PASS。

- [ ] **Step 7: clippy + fmt**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -3
cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: 全绿。

- [ ] **Step 8: Commit**

```bash
cd /Users/zool/workspace/openlark
git add crates/openlark-hr/src/okr/okr/v2/key_result/get.rs \
        crates/openlark-hr/src/okr/okr/v2/key_result/patch.rs \
        crates/openlark-hr/src/okr/okr/v2/alignment/get.rs \
        crates/openlark-hr/src/okr/okr/v2/objective/alignment/list.rs
git commit -m "refactor(hr): okr/v2 KeyResult + Alignment 组 4 叶改引用 common::models（删 inline 重复定义）(#336)"
```

---

## Task 5: 最终验证（issue #336 / spec scenario 验收）

**Files:** （只读验证，不改文件）

本 task 对应 tasks.md §3 全部 7 项验收，与 spec `v1-sub-api-accessors` 的 4 个 Scenario 一致。

- [ ] **Step 1: 完整 build + test**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo build -p openlark-hr --all-features 2>&1 | tail -3
cargo test -p openlark-hr --all-features 2>&1 | tail -15
```
Expected: build `Finished`；test 全 PASS。

- [ ] **Step 2: fmt + clippy 全检**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -3
cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings 2>&1 | tail -10
```
Expected: fmt 无 diff；clippy 无 warning（对应 spec Scenario "行为零变化"）。

- [ ] **Step 3: grep 单一定义（spec Scenario "9 跨叶共享 struct 各只定义一次"）**

Run:
```bash
cd /Users/zool/workspace/openlark/crates/openlark-hr/src/okr/okr/v2
grep -rn "^pub struct \(Objective\|ObjectiveOwner\|Indicator\|IndicatorOwner\|IndicatorUnit\|KeyResult\|KeyResultOwner\|Alignment\|AlignmentOwner\) " --include="*.rs" . | grep -v "common/models.rs"
```
Expected: **空输出**（9 struct 在 common/models.rs 之外零残留）。

Run:
```bash
cd /Users/zool/workspace/openlark/crates/openlark-hr/src/okr/okr/v2
for s in Objective ObjectiveOwner Indicator IndicatorOwner IndicatorUnit KeyResult KeyResultOwner Alignment AlignmentOwner; do
  n=$(grep -rn "^pub struct $s " --include="*.rs" . | wc -l | tr -d ' ')
  echo "$s: $n"
done
```
Expected: 每个 struct 输出 `: 1`（仅 `common/models.rs` 一处）。

- [ ] **Step 4: 确认 11 叶显式具名 import（spec Scenario "11 叶子 import 引用共享定义"）**

Run:
```bash
cd /Users/zool/workspace/openlark/crates/openlark-hr/src/okr/okr/v2
echo "=== 应有 Objective/ObjectiveOwner import（4 叶）==="
grep -l "use crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner};" objective/get.rs cycle/objective/list.rs cycle/objectives_position.rs cycle/objectives_weight.rs
echo "=== 应有 Indicator 组 import（3 叶）==="
grep -l "use crate::okr::okr::v2::common::models::{Indicator, IndicatorOwner, IndicatorUnit};" indicator/patch.rs objective/indicator/list.rs key_result/indicator/list.rs
echo "=== 应有 KeyResult 组 import（2 叶）==="
grep -l "use crate::okr::okr::v2::common::models::{KeyResult, KeyResultOwner};" key_result/get.rs key_result/patch.rs
echo "=== 应有 Alignment 组 import（2 叶）==="
grep -l "use crate::okr::okr::v2::common::models::{Alignment, AlignmentOwner};" alignment/get.rs objective/alignment/list.rs
```
Expected: 每组列出正好 4/3/2/2 个文件名，无缺失。共 11 文件。

- [ ] **Step 5: 确认 per-leaf Response wrapper 仍 inline（spec Scenario "per-leaf Response wrapper 保持 inline"）**

Run:
```bash
cd /Users/zool/workspace/openlark/crates/openlark-hr/src/okr/okr/v2
grep -rn "^pub struct GetObjectiveResponse \|^pub struct ListObjectivesResponse \|^pub struct PatchIndicatorResponse \|^pub struct GetKeyResultResponse \|^pub struct GetAlignmentResponse " --include="*.rs" .
```
Expected: 各 wrapper 仍在各自叶文件内（非 common/models.rs）。例如 `objective/get.rs: GetObjectiveResponse`、`indicator/patch.rs: PatchIndicatorResponse` 等。

- [ ] **Step 6: byte-identical 抽样（spec Scenario "行为零变化" 的定义一致性）**

抽验 common/models.rs 的 struct 与 git 历史中重构前的 canonical 叶逐字一致。由于 Task 1 已从 canonical 叶整块复制且未改字段，此处抽样确认 9 struct 无字段改动：

Run（对比工作区 common/models.rs 的 `Objective` 与 git base-ref 的 objective/get.rs 的 `Objective`）:
```bash
cd /Users/zool/workspace/openlark
diff <(sed -n '/^\/\/\/ OKR 目标\./,/^}/p' crates/openlark-hr/src/okr/okr/v2/common/models.rs) \
     <(git show 3a462c410caf37b4c94d8d872587b093da26c420:crates/openlark-hr/src/okr/okr/v2/objective/get.rs | sed -n '/^\/\/\/ OKR 目标\./,/^}/p') \
  && echo "Objective: IDENTICAL" || echo "Objective: DIFFERENT"
```
Expected: `Objective: IDENTICAL`。

重复同模式抽查 `Alignment`（base-ref alignment/get.rs）、`Indicator`（base-ref indicator/patch.rs）、`KeyResult`（base-ref key_result/get.rs）—— 4 个 canonical struct 各抽一个即可代表 9 struct（其余 5 个是同源子 struct）。全部期望 `IDENTICAL`。

- [ ] **Step 7: 跨 crate 回归（spec Scenario "行为零变化" 的外部消费）**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo check --workspace --all-features 2>&1 | tail -10
```
Expected: 全 workspace 编译通过（okr/v2 struct 路径变更未破坏外部 crate；okr/v2 零外部引用已确认）。

- [ ] **Step 8: 全量 workspace fmt/clippy 兜底（可选但推荐）**

Run:
```bash
cd /Users/zool/workspace/openlark
cargo fmt --check 2>&1 | tail -3
```
Expected: 无 diff。

- [ ] **Step 9: 更新 tasks.md 勾选 + 提交**

回到 `openspec/changes/dedup-okr-v2-models/tasks.md`，勾选 1.1、1.2、2.1、2.2、2.3、3.1-3.7 全部框。

Run:
```bash
cd /Users/zool/workspace/openlark
git add openspec/changes/dedup-okr-v2-models/tasks.md
git commit -m "chore: 勾选 dedup-okr-v2-models 全部 task（重构完成、verify 通过）(#336)"
```

---

## Self-Review 总结

**Spec coverage（design.md D1-D4 + spec.md 4 scenarios）：**
- D1 共享模块 `common/models.rs` → Task 1 ✓
- D2 显式具名 import（覆盖 tasks.md glob 写法）→ Global Constraints + Task 2-4 各 import 行 ✓
- D3 clean break 无 re-export → Task 1-4 全程未加 `pub use` ✓
- D4 Response wrapper 保持 inline → 每个 task 的"保留 wrapper"步骤 + Task 5 Step 5 验证 ✓
- spec Scenario "9 struct 各只定义一次" → Task 5 Step 3 grep 验证 ✓
- spec Scenario "11 叶 import" → Task 5 Step 4 验证 ✓
- spec Scenario "Response wrapper inline" → Task 5 Step 5 验证 ✓
- spec Scenario "行为零变化" → 每 task build/test + Task 5 Step 1/2/6/7 验证 ✓

**Placeholder scan：** 无 TBD/TODO/placeholder；每个步骤含可执行命令与预期输出；9 struct 完整代码内联于 Task 1 Step 3。

**Type consistency：** `Objective`/`ObjectiveOwner`/`Indicator`/`IndicatorOwner`/`IndicatorUnit`/`KeyResult`/`KeyResultOwner`/`Alignment`/`AlignmentOwner` 9 个名字在 Task 1（定义）与 Task 2-4（import）、Task 5（grep）中完全一致；import 路径 `crate::okr::okr::v2::common::models::{...}` 全程统一。
