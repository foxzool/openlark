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
