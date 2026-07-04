# Comet Subagent Progress: dedup-okr-v2-models

change: dedup-okr-v2-models
plan: docs/superpowers/plans/2026-07-04-dedup-okr-v2-models.md
review_mode: standard
tdd_mode: direct
build_mode: subagent-driven-development
branch: refactor/issue-336-dedup-okr-v2-models
base_ref: 3a462c410caf37b4c94d8d872587b093da26c420

> standard 模式：无 per-task reviewer。implementer 自测 + 回报证据，协调者做
> 定向勾选验证。全部 task 完成后只派发一次最终轻量 code reviewer。任务之间
> 禁止暂停——勾选后立即派发下一个 task。

## Task History

- **Task 1 (建共享模块 + 迁移 9 struct)** — DONE · commit a0fbbd84 · OpenSpec 1.1/1.2 ✓
- **Task 2 (Objective 组改 import)** — DONE · commit 2c10e0d7c · OpenSpec 2.1 ✓
  - 5 文件（4 叶 + patch.rs 跨叶 consumer）；D2 只 import Objective；+9/-189
- **Task 3 (Indicator 组 3 叶改 import)** — DONE · commit ab500bd3e · OpenSpec 2.2 ✓
  - 3 文件；D2 只 import Indicator（Owner/Unit 仅嵌套字段）；+6/-165
  - import 放独立分组（外部 crate 组 + 空行 + crate 组），fmt-clean，与 Task 2 一致
  - 证据：build Finished；test 全 binary 0 failed；fmt --check exit 0；clippy -D warnings clean
  - 协调者复核：3 叶 inline Indicator/Owner/Unit×0 / import×1、commit 只含 3 文件、build 绿

## Current Task

- Plan task: Task 4 — KeyResult + Alignment 组改 import（7 文件）
- OpenSpec tasks: 2.3
- Stage: implementing（implementer 待派发）
- BASE commit: ab500bd3e（Task 3 完成后的 HEAD）
- Review-fix round: N/A
- 文件清单（7）+ D2 import（协调者审计核实）：
  - **Inline-delete（4 叶，删 struct + Owner，加 import）**：
    - `key_result/get.rs` → import `{KeyResult}`
    - `key_result/patch.rs` → import `{KeyResult}`
    - `alignment/get.rs` → import `{Alignment}`
    - `objective/alignment/list.rs` → import `{Alignment}`
  - **Cross-leaf consumer（3 叶，仅改 import 路径，无 inline 删除）**：
    - `objective/key_results_weight.rs`：`use super::super::key_result::get::KeyResult` → `{KeyResult}`
    - `objective/key_results_position.rs`：同上 → `{KeyResult}`
    - `objective/key_result/list.rs`：`use super::super::super::key_result::get::KeyResult` → `{KeyResult}`
  - D2：4 删除叶的 Owner（KeyResultOwner/AlignmentOwner）仅作主 struct 内部字段，删后无直接引用 → 不导入
  - 无 Alignment 跨叶 consumer
