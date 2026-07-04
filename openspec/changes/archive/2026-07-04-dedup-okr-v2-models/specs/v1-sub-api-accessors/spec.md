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
