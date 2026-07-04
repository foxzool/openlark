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
