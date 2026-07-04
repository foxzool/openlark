# Brainstorm Summary

- Change: dedup-okr-v2-models
- Date: 2026-07-04

## 确认的技术方案

把 okr/v2 跨叶 byte-identical 的 9 个 domain struct 抽到共享 `okr/okr/v2/common/models.rs`，11 叶改 import 引用。纯机械挪位零字段调和。

**D2 修正（brainstorming 关键发现）**：design.md 原 D2 写 glob import `use ...::*`——错误。repo `clippy::all = warn`（含 `wildcard_imports`）+ CI `-D warnings` 会 deny glob；且代码库约定是**显式具名 import**（openlark-workflow `use ...models::{A, B}`）。改为显式具名：`use crate::okr::okr::v2::common::models::{Objective, ObjectiveOwner};`。

## 关键取舍与风险

- D1 路径 `okr/okr/v2/common/models.rs`（呼应 crate 顶层 `common/`）
- D2 **显式具名 import**（修正后；非 glob）
- D3 不留 backward-compat re-export（clean break，okr/v2 零外部引用）
- D4 Response wrapper 保持 inline（各包不同 shape，非重复）
- 风险：9 struct 跨叶 byte-identical（已 PYTHONHASHSEED=0 单进程确认）→ 机械挪位零调和；struct 路径 breaking 但零外部引用

## 测试策略

- `cargo build/test -p openlark-hr --all-features` + `cargo clippy --all-features --all-targets -- -D warnings` + `cargo fmt --check`
- grep 9 struct 名在 okr/v2 下各只 1 处 `pub struct`（common/models.rs）
- byte-identical 抽样：common/models.rs 的 struct 与变更前 canonical 叶逐字一致
- `cargo check --workspace --all-features`（跨 crate 无破坏）

## Spec Patch

无（delta spec `v1-sub-api-accessors` DRY requirement + 4 场景已含；D2 import 风格是实现细节，不改 spec 语义）。
