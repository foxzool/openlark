# Tasks — fix-test-feature-gating（#251）

> 目标：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
> build 阶段用 workflow 按 crate 并行；每 agent 改完跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 验证本 crate 绿。

- [x] 1. **hr**（10 个 tests/*.rs）：按 subsystem 补 `#![cfg(feature)]`（attendance/compensation/ehr/corehr/hire×3/okr/payroll/performance）+ 核查内联 `mod tests`（lib.rs:230 等）；本 crate clippy 验证
- [x] 2. **helpdesk**：`helpdesk_contract_models.rs` 加 `#![cfg(feature = "v1")]`；验证
- [x] 3. **webhook**：`integration_webhook.rs` 加 `#![cfg(feature = "robot")]` + 2 个 examples（webhook_error_handling、webhook_text_message）补门控；验证
- [x] 4. **docs**：`docs_contract_models.rs`/`helper_snapshots.rs` 门控修正（去冗余 `v3`、补齐）+ `docs_readme_examples` example 门控；验证
- [x] 5. **模式 B（`//!` 交换）**：analytics、application、communication(×2)、mail、meeting、platform、workflow(×2) 的 `//!` 移到 `#![cfg]` 之上；各 crate clippy 验证
- [x] 6. **模式 C（次要 lint）**：按 clippy 指示修 E0609（×4）与 unused import（×2）
- [x] 7. **全量验收**：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0；回归 `--all-features --all-targets`、`cargo build --workspace --all-features`、`cargo fmt --all --check` 全 exit 0
