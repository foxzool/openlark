# Comet Design Handoff

- Change: dedup-okr-v2-models
- Phase: design
- Mode: compact
- Context hash: a6a06478cb08a6c9126a56a632eceb66c0b86d422567876aaee8b2c7609c69a3

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/dedup-okr-v2-models/proposal.md

- Source: openspec/changes/dedup-okr-v2-models/proposal.md
- Lines: 1-36
- SHA256: bc8b286c5e62842372478df5a6e9a4e891a613ae708a16b47ae8820eabc22d49

```md
## Why

`#328` 给 okr/v2 25 叶加 typed Response 时，每叶 inline 定义了 domain struct。跨叶共享实体被**逐字重复定义多处**（code review Standards axis 主 smell：Duplicated Code）。已核实 9 个 struct 跨叶**完全 byte-identical**（字段名/顺序/serde 属性/doc 全同），是纯冗余：

- `Objective`（32 字段）4 处、`ObjectiveOwner` 4 处
- `Indicator`（31 字段）3 处、`IndicatorOwner` 3 处、`IndicatorUnit` 3 处
- `KeyResult`（25 字段）2 处、`KeyResultOwner` 2 处
- `Alignment`（18 字段）2 处、`AlignmentOwner` 2 处

约 25 处冗余定义、~200 重复行。后果：跨端点传 `Objective` 需类型转换；任何飞书字段变更须改 N 处（Shotgun Surgery）；`#339`（深字段 typed 化）若先于此做会把同一改动重复 N 次。

## What Changes

- 新增 `crates/openlark-hr/src/okr/okr/v2/common/models.rs`，收纳 9 个跨叶共享 domain struct（`Objective`/`KeyResult`/`Alignment`/`Indicator` + `ObjectiveOwner`/`KeyResultOwner`/`AlignmentOwner`/`IndicatorOwner`/`IndicatorUnit`）——各**只定义一次**
- 11 个叶子删除 inline 的这 9 个 struct 定义，改 `use crate::okr::okr::v2::common::models::*` 引用
- per-leaf Response wrapper struct（`GetObjectiveResponse`/`ListObjectivesResponse` 等）保持 inline 不动（各包不同 shape，非重复）
- **纯结构挪位 + import 替换，零字段变更、零行为变更**（反序列化结果完全不变）
- 模块路径：`okr/okr/v2/common/mod.rs` 声明 `pub mod models;`，呼应 crate 顶层 `common/` 约定

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `v1-sub-api-accessors`：新增 DRY requirement。`#328` 加了"navigable 链叶子 SHALL 返 typed Response"；本 change 补维护性约束——**okr/v2 跨叶共享的 domain entity struct（同一飞书实体的 typed 表示，如 `Objective`/`KeyResult`/`Alignment`/`Indicator` 及其子 struct）SHALL 在共享模块单一定义，不得跨叶逐字重复**。各叶 Response wrapper（包特定响应 shape）仍可 inline。构成对 `#328` typed requirement 的维护性补充。

## Impact

- **crates/openlark-hr**（全部改动集中于此）：
  - 新增 `src/okr/okr/v2/common/mod.rs` + `src/okr/okr/v2/common/models.rs`（9 个 struct 各一份）
  - `src/okr/okr/v2/mod.rs`：声明 `pub mod common;`
  - 11 叶改 import：`objective/get`、`cycle/objective/list`、`cycle/objectives_position`、`cycle/objectives_weight`、`indicator/patch`、`objective/indicator/list`、`key_result/indicator/list`、`key_result/get`、`key_result/patch`、`alignment/get`、`objective/alignment/list`
- **公开 API**：9 个 struct 的规范路径从 `<leaf>::<Struct>` 变为 `common::models::<Struct>`。okr/v2 零外部引用（#327/#328 确认），路径变更无外部消费方。非 breaking 行为变更（类型本身不变）。
- **依赖**：无新增；纯内部重组
- **`#339` 解锁**：本 change 完成后，深字段 typed 化只改 `common/models.rs` 一处而非 N 处
```

## openspec/changes/dedup-okr-v2-models/design.md

- Source: openspec/changes/dedup-okr-v2-models/design.md
- Lines: 1-56
- SHA256: ed3a4c110ef9c35d9e5dd64054f81fe6a31c229be445c5b0b736b6db0095a6bd

```md
## Context

`#328` 给 okr/v2 25 叶加 typed Response，每叶 inline 定义 domain struct。9 个跨叶共享实体被逐字重复（byte-identical，已核实）：

```
Objective(32字段) ×4  ┐ ObjectiveOwner ×4
Indicator(31)    ×3  ├ IndicatorOwner ×3, IndicatorUnit ×3
KeyResult(25)    ×2  ├ KeyResultOwner ×2
Alignment(18)    ×2  ┘ AlignmentOwner ×2
```

约 25 处冗余定义、~200 重复行。`#339`（深字段 typed 化）依赖此消重作为 prefactor（消重后改一处而非 N 处）。

约束：9 struct 跨叶完全 byte-identical（字段/顺序/serde/doc 全同）→ 消重是纯机械挪位，零字段调和；okr/v2 零外部引用（#327/#328 确认）。

## Goals / Non-Goals

**Goals:**
- 9 个跨叶共享 domain struct 各只定义一次（`okr/okr/v2/common/models.rs`）
- 11 叶改 import 引用共享定义
- 行为零变化（反序列化不变）

**Non-Goals:**
- 不动 per-leaf Response wrapper（`GetObjectiveResponse` 等，各包不同 shape）
- 不改任何字段定义
- 不动 okr/v1 / 其他 crate / 9 个之外的结构
- 不做深字段 typed 化（那是 `#339`，本 change 是其 prefactor）
- 不动端点 URL / 导航链

## Decisions

### D1: 共享模块路径 = `okr/okr/v2/common/models.rs`
新模块 `okr/okr/v2/common/`（`mod.rs` 声明 `pub mod models;`）+ `models.rs` 收纳 9 struct。

**为什么**：呼应 crate 顶层 `common/`（`crate::common::api_endpoints`）的约定； scoped 到 okr/v2 不污染 crate 全局；`common/models` 语义清晰（共享模型）。备选 `okr/okr/v2/models.rs`（无 common 子层）→ 否决（与顶层 common 约定不一致，且未来若 okr/v2 有其他共享物无统一归处）。

### D2: import 用 glob `use ...common::models::*`
11 叶用 `use crate::okr::okr::v2::common::models::*;` 引用。

**为什么**：每叶用到的 struct 1-3 个，glob 简洁且避免漏引；模块是纯数据 struct 集合无命名冲突风险。备选显式列表 `use ...{Objective, ObjectiveOwner}` → 否决（11 叶各列冗长，且新增字段时需补引）。

### D3: 不做 backward-compat re-export
9 struct 规范路径从 `<leaf>::<Struct>` 改为 `common::models::<Struct>`，**不在原叶路径留 `pub use` re-export**。

**为什么**：okr/v2 零外部引用（#327/#328 已确认），无消费方依赖原路径；留 re-export 会保留一半冗余（违背消重初衷）+ 模糊 canonical 路径。clean break。

### D4: Response wrapper 保持 inline（不消重）
`GetObjectiveResponse`/`ListObjectivesResponse`/`DeleteAlignmentResponse` 等 per-leaf wrapper 仍 inline。

**为什么**：各 wrapper 包**不同响应 shape**（get 包单个 objective、list 包 `Vec<objective>` + 分页、delete 空响应）——它们不是重复，是不同类型。只有内部嵌套的 domain entity（Objective 等）是跨叶相同的，才消重。

## Risks / Trade-offs

- **[struct 路径 breaking]** 9 struct 公开路径变更 → 缓解：okr/v2 零外部引用，v0.18 窗口；非行为变更
- **[glob import 命名冲突]** `models::*` 可能与叶内其他同名冲突 → 缓解：models 是纯数据 struct，叶内无同名；clippy/build 即时捕获
- **[机械挪位出错]** 复制 struct 漏字段/改属性 → 缓解：已核实 9 struct 跨叶 byte-identical，直接整块迁移；build/test/clippy 验证反序列化不变
```

## openspec/changes/dedup-okr-v2-models/tasks.md

- Source: openspec/changes/dedup-okr-v2-models/tasks.md
- Lines: 1-24
- SHA256: dd323beddcc8e1273181a7edea89197db2d963176b0656dc7d931f362dfdee4e

```md
# Tasks

## 1. 创建共享模块 + 迁移 9 struct

- [ ] 1.1 新建 `crates/openlark-hr/src/okr/okr/v2/common/mod.rs`（`pub mod models;` + 顶部 `//!` 模块说明），并在 `okr/okr/v2/mod.rs` 加 `pub mod common;` 声明
- [ ] 1.2 新建 `crates/openlark-hr/src/okr/okr/v2/common/models.rs`，从 canonical 叶子整块迁移 9 个 struct（byte-identical 直接挪）：`Objective`/`ObjectiveOwner`（自 objective/get）、`Indicator`/`IndicatorOwner`/`IndicatorUnit`（自 indicator/patch）、`KeyResult`/`KeyResultOwner`（自 key_result/get）、`Alignment`/`AlignmentOwner`（自 alignment/get）。顶部 `//!` + `use serde::Deserialize;`

## 2. 11 叶改 import（删 inline 定义 + 加 use 引用）

每叶：删除 inline 的被消重 struct 定义 → 加 `use crate::okr::okr::v2::common::models::*;` → 保留各自 Response wrapper inline。每批一个 commit + 增量 build。

- [ ] 2.1 **Objective 组 4 叶**：`objective/get`、`cycle/objective/list`、`cycle/objectives_position`、`cycle/objectives_weight`（删 Objective + ObjectiveOwner，加 import）
- [ ] 2.2 **Indicator 组 3 叶**：`indicator/patch`、`objective/indicator/list`、`key_result/indicator/list`（删 Indicator + IndicatorOwner + IndicatorUnit，加 import）
- [ ] 2.3 **KeyResult + Alignment 组 4 叶**：`key_result/get`、`key_result/patch`（删 KeyResult + KeyResultOwner）、`alignment/get`、`objective/alignment/list`（删 Alignment + AlignmentOwner），加 import

## 3. 验证（issue #336 验收）

- [ ] 3.1 `cargo build -p openlark-hr --all-features` 通过
- [ ] 3.2 `cargo test -p openlark-hr --all-features` 通过（现有 typed Response 反序列化 + test_hr_client_* 不破坏）
- [ ] 3.3 `cargo fmt --check` + `cargo clippy -p openlark-hr --all-features --all-targets -- -D warnings` 通过
- [ ] 3.4 grep 单一定义：9 个 struct 名在 `okr/okr/v2/` 下各只 1 处 `pub struct` 定义（`common/models.rs`），叶内零残留
- [ ] 3.5 byte-identical 抽样：确认 `common/models.rs` 的 9 struct 字段与变更前 canonical 叶子逐字一致（纯挪位无字段改动）
- [ ] 3.6 per-leaf Response wrapper 仍 inline（未挪动）
- [ ] 3.7 跨 crate 回归：`cargo check --workspace --all-features` 通过（okr/v2 struct 路径变更未破坏外部消费）
```

## openspec/changes/dedup-okr-v2-models/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/dedup-okr-v2-models/specs/v1-sub-api-accessors/spec.md
- Lines: 1-25
- SHA256: 63eca9f5fa265bdcf2ba7ba3470151a621d1eb92ecf649db86edd3db4425f2d8

```md
## ADDED Requirements

### Requirement: openlark-hr okr/v2 跨叶共享 domain struct SHALL 单一定义

openlark-hr okr/v2 中代表同一飞书实体、跨多个叶子重复出现的 domain entity struct（如 `Objective`/`KeyResult`/`Alignment`/`Indicator` 及其子 struct 如 `*Owner`/`*Unit`）SHALL 在共享模块（`okr/okr/v2/common/models.rs`）中**各只定义一次**，各叶子通过 `use` 引用，不得跨叶逐字重复定义。仅当某 struct 真为单叶私有（无跨叶复用）时方可在叶内 inline。per-leaf Response wrapper（包特定响应 shape，如 `GetObjectiveResponse` vs `ListObjectivesResponse`）不在此约束内——它们 shape 各异，非重复。本 requirement 构成对 `v1-sub-api-accessors` 现有「okr/v2 navigable 链叶子 SHALL 返 typed Response」（#328）的维护性补充：typed 化产生的 domain struct 不得散落重复，否则任何飞书字段变更触发 Shotgun Surgery（N 处改）。

#### Scenario: 9 跨叶共享 struct 各只定义一次

- **WHEN** 变更后在 `crates/openlark-hr/src/okr/okr/v2/` 中 grep `^pub struct (Objective|KeyResult|Alignment|Indicator|ObjectiveOwner|KeyResultOwner|AlignmentOwner|IndicatorOwner|IndicatorUnit) `
- **THEN** 每个 struct 名全局仅 1 处定义（`common/models.rs`），不再出现在叶子文件内

#### Scenario: 11 叶子 import 引用共享定义

- **WHEN** 变更后检查 11 个受影响叶子（objective/get、cycle/objective/list、cycle/objectives_position、cycle/objectives_weight、indicator/patch、objective/indicator/list、key_result/indicator/list、key_result/get、key_result/patch、alignment/get、objective/alignment/list）
- **THEN** 每叶含 `use crate::okr::okr::v2::common::models::*`（或等价显式 import），且叶内无被消重 struct 的 inline 定义

#### Scenario: per-leaf Response wrapper 保持 inline

- **WHEN** 变更后检查各叶 Response wrapper（如 `GetObjectiveResponse`/`ListObjectivesResponse`/`DeleteAlignmentResponse`）
- **THEN** wrapper struct 仍在各自叶文件内 inline 定义（未挪动），仅其嵌套的 domain entity 改为引用共享定义

#### Scenario: 行为零变化（反序列化不变）

- **WHEN** 运行 `cargo build/test -p openlark-hr --all-features` 与 `cargo clippy -p openlark-hr --all-features --all-targets -D warnings`
- **THEN** 均通过；现有 typed Response 反序列化测试与 `test_hr_client_*` 不破坏；9 struct 字段定义与变更前 byte-identical（纯挪位）
```

