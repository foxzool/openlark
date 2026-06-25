---
change: fix-test-feature-gating
design-doc: docs/superpowers/specs/2026-06-25-test-feature-gating-design.md
base-ref: 86482802de7e2ebf1977ad21e2207cdd0fae2526
---

# 实施计划 — fix-test-feature-gating（#251）

## 目标

使 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。

## 执行策略

按 crate 并行（Workflow 工具，每 agent 一个 crate），套用 3 类修复模式（见 design-doc）。每 agent 改完跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 自验；主窗口汇总后跑全量目标命令 + 回归。

## 任务（对应 tasks.md）

### Task 1 — hr（10 文件 + 内联 mod）
补 `#![cfg(feature)]`：attendance→attendance、compensation→compensation、ehr→ehr、feishu_people→corehr、hire×3→hire、okr→okr、payroll→payroll、performance→performance；核查 lib.rs:230 内联 `mod tests`。自验 `-p openlark-hr`。

### Task 2 — helpdesk
`helpdesk_contract_models.rs` 加 `#![cfg(feature = "v1")]`。自验 `-p openlark-helpdesk`。

### Task 3 — webhook
`integration_webhook.rs` 加 `#![cfg(feature = "robot")]`（不动文件内 per-test `card`/`signature` cfg）；2 examples（webhook_error_handling、webhook_text_message）补门控。自验 `-p openlark-webhook`。

### Task 4 — docs
`docs_contract_models.rs`/`helper_snapshots.rs` 门控修正（去冗余 `v3`、核查为何仍 E0433）；`docs_readme_examples` example 补门控。自验 `-p openlark-docs`。

### Task 5 — 模式 B（//! 交换）
analytics、application、communication(×2)、mail、meeting、platform、workflow(×2)：`//!` 移到 `#![cfg]` 之上。各 crate 自验。

### Task 6 — 模式 C（次要 lint）
按 clippy 修 E0609（×4）、unused import（×2）。

### Task 7 — 全量验收
- 核心：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- 回归：`cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- `cargo build --workspace --all-features` exit 0
- `cargo fmt --all --check` exit 0

## 风险

- docs_contract_models gate 未生效需 build 阶段定位（可能 //! 顺序 + gate 表达式）。
- 仅测试/示例代码，低风险；门控只「不编译」不删测试。
