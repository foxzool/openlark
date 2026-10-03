---
comet_change: dedup-okr-v2-models
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-04-dedup-okr-v2-models
status: final
---

# Design: dedup-okr-v2-models

## Context

`#328` 给 okr/v2 25 叶加 typed Response 时，每叶 inline 定义了 domain struct。9 个跨叶共享实体被**逐字重复**（byte-identical，已 `PYTHONHASHSEED=0` 单进程确认字段名/顺序/serde 属性/doc 全同）：

| struct | 字段数 | 出现次数 | canonical 来源 |
|---|---|---|---|
| `Objective` / `ObjectiveOwner` | 32 / 5 | 4 × / 4 × | objective/get |
| `Indicator` / `IndicatorOwner` / `IndicatorUnit` | 31 / 5 / 4 | 3 × / 3 × / 3 × | indicator/patch |
| `KeyResult` / `KeyResultOwner` | 25 / 5 | 2 × / 2 × | key_result/get |
| `Alignment` / `AlignmentOwner` | 18 / 5 | 2 × / 2 × | alignment/get |

约 25 处冗余定义、~200 重复行。后果：跨端点传 `Objective` 需类型转换；飞书字段变更须改 N 处（Shotgun Surgery）；`#339`（深字段 typed 化）先做会把同改动重复 N 次。

约束：9 struct 跨叶 byte-identical → 消重纯机械挪位零字段调和；okr/v2 零外部引用（#327/#328 确认）。

## Goals / Non-Goals

**Goals:** 9 struct 各只定义一次（`common/models.rs`）；11 叶改 import；行为零变化。

**Non-Goals:** 不动 per-leaf Response wrapper；不改字段；不动 okr/v1 / 其他 crate / 9 个之外结构；不做深字段 typed 化（`#339`）；不动端点/导航链。

## Decisions

### D1: 共享模块 `okr/okr/v2/common/models.rs`
`okr/okr/v2/common/mod.rs`（`pub mod models;`）+ `models.rs` 收纳 9 struct。`v2/mod.rs` 加 `pub mod common;`。呼应 crate 顶层 `common/`（`crate::common::api_endpoints`）约定，scoped 到 okr/v2。

### D2: 显式具名 import（修正：非 glob）
每叶 `use crate::okr::okr::v2::common::models::{<StructA>, <StructB>};` 显式列出该叶用到的 struct。

**为什么不 glob**：repo `[workspace.lints.clippy] all = warn`（含 `wildcard_imports` style lint）+ CI `-D warnings` 会 deny `use ...::*`；代码库约定也是显式具名（openlark-workflow: `use ...models::{CreateCustomFieldBody, CreateCustomFieldResponse}`）。glob 会致 clippy fail 且违约定。

各叶 import 清单：
- objective/get、cycle/objective/list、cycle/objectives_position、cycle/objectives_weight → `{Objective, ObjectiveOwner}`
- indicator/patch、objective/indicator/list、key_result/indicator/list → `{Indicator, IndicatorOwner, IndicatorUnit}`
- key_result/get、key_result/patch → `{KeyResult, KeyResultOwner}`
- alignment/get、objective/alignment/list → `{Alignment, AlignmentOwner}`

### D3: 不留 backward-compat re-export
9 struct 规范路径从 `<leaf>::<Struct>` 改为 `common::models::<Struct>`，**不留 `pub use` re-export**。okr/v2 零外部引用（#327/#328 确认），clean break 避免保留半冗余。

### D4: Response wrapper 保持 inline
`GetObjectiveResponse`/`ListObjectivesResponse`/`DeleteAlignmentResponse` 等 per-leaf wrapper 仍 inline——各包**不同响应 shape**（get 单个、list `Vec`+分页、delete 空），非重复。仅内部嵌套 domain entity 跨叶相同才消重。

## 转换模式（每叶）

```rust
// 现状（objective/get.rs，inline 定义 + Response wrapper）
#[derive(Debug, Clone, Deserialize)]
pub struct Objective { /* 32 字段 */ }     // ← 删除
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectiveOwner { /* 5 字段 */ } // ← 删除
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse { pub objective: Objective } // ← 保留（wrapper）

// 目标
use crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner}; // ← 显式 import
// Objective/ObjectiveOwner 定义移到 common/models.rs
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse { pub objective: Objective } // wrapper 仍 inline
```

## Risks / Trade-offs

- **[struct 路径 breaking]** 9 struct 公开路径变更 → 缓解：okr/v2 零外部引用，v0.18 窗口；类型本身不变
- **[机械挪位出错]** 复制漏字段/改属性 → 缓解：9 struct 跨叶 byte-identical（已确认），整块迁移；build/test/clippy 验证反序列化不变
- **[glob import clippy]** → D2 已用显式具名规避

## Test Strategy

- 每叶：删 inline struct 定义 → 加显式 import → build
- 全量：`cargo build/test -p openlark-hr --all-features` + `cargo fmt --check` + `cargo clippy --all-features --all-targets -- -D warnings`
- grep 9 struct 名在 `okr/okr/v2/` 下各只 1 处 `pub struct`（`common/models.rs`）
- byte-identical 抽样：`common/models.rs` 的 struct 与变更前 canonical 叶逐字一致
- `cargo check --workspace --all-features`（跨 crate 无破坏）
