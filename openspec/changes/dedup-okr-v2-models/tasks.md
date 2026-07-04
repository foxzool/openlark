# Tasks

## 1. 创建共享模块 + 迁移 9 struct

- [x] 1.1 新建 `crates/openlark-hr/src/okr/okr/v2/common/mod.rs`（`pub mod models;` + 顶部 `//!` 模块说明），并在 `okr/okr/v2/mod.rs` 加 `pub mod common;` 声明
- [x] 1.2 新建 `crates/openlark-hr/src/okr/okr/v2/common/models.rs`，从 canonical 叶子整块迁移 9 个 struct（byte-identical 直接挪）：`Objective`/`ObjectiveOwner`（自 objective/get）、`Indicator`/`IndicatorOwner`/`IndicatorUnit`（自 indicator/patch）、`KeyResult`/`KeyResultOwner`（自 key_result/get）、`Alignment`/`AlignmentOwner`（自 alignment/get）。顶部 `//!` + `use serde::Deserialize;`

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
