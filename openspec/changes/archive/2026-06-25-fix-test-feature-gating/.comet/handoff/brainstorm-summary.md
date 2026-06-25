# Brainstorm Summary

- Change: fix-test-feature-gating
- Date: 2026-06-25

## 确认的技术方案

3 类修复模式，目标使 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0：

- **A (E0433)**：补 `#![cfg(feature)]`——helpdesk→v1、hr→10 子系统（attendance/compensation/ehr/corehr/hire×3/okr/payroll/performance）、webhook→robot、docs+examples（docs_readme_examples、webhook_error_handling、webhook_text_message）。
- **B (missing_docs)**：`//!` 移到 `#![cfg]` 之上（spike 验证有效）。
- **C**：E0609×4、unused import×2 按 clippy 逐个修。

按 crate 并行实施（build 用 workflow），每 agent 改完跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 自验。

## 关键取舍与风险

- missing_docs 用「`//!` 交换」而非 `#[allow(missing_docs)]`（保留 crate 文档、更干净）。
- `docs_contract_models` 当前 gate 含冗余 `v3` 且仍 E0433，build 阶段核查其 gate 为何未生效（可能 //! 顺序 + gate 需修正）。
- 风险低：仅测试/示例代码，门控只「不编译」、不删测试、不改业务代码。

## 测试策略

- 核心：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
- 回归：`--all-features --all-targets`、`cargo build --workspace --all-features`、`cargo fmt --all --check` 全 exit 0；CI 现有检查不回退。

## Spec Patch

无（不涉及 delta spec / 验收场景）。
