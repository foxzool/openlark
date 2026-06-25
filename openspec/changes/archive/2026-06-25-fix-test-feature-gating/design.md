# Design — fix-test-feature-gating（#251）

## 目标命令

使 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。

`--keep-going` 实测失败面：27 个 test/example target，三类问题（见下）。`--all-features --all-targets` 现已绿（#249 修过 all-features 维度），故本 change 只处理 no-default-features 维度。

## 修复模式

### 模式 A — E0433：补/修 feature 门控（约 13 文件）

test/example 引用 feature-gated 模块但自身未按 feature 门控。文件顶部加 `#![cfg(feature = "<f>")]`（多 feature 用 `all(...)`）。feature 映射（workflow 逐文件核对 lib.rs 的 `#[cfg(feature)]` + Cargo.toml）：

| 文件 | 门控 |
|------|------|
| helpdesk/tests/helpdesk_contract_models.rs | `#![cfg(feature = "v1")]` |
| hr/tests/attendance_tests.rs | `#![cfg(feature = "attendance")]` |
| hr/tests/compensation_tests.rs | `#![cfg(feature = "compensation")]` |
| hr/tests/ehr_tests.rs | `#![cfg(feature = "ehr")]` |
| hr/tests/feishu_people_tests.rs | `#![cfg(feature = "corehr")]`（模块名 feishu_people，feature 名 corehr） |
| hr/tests/hire_request_modeling_tests.rs | `#![cfg(feature = "hire")]` |
| hr/tests/hire_response_contract_tests.rs | `#![cfg(feature = "hire")]` |
| hr/tests/hire_tests.rs | `#![cfg(feature = "hire")]` |
| hr/tests/okr_tests.rs | `#![cfg(feature = "okr")]` |
| hr/tests/payroll_tests.rs | `#![cfg(feature = "payroll")]` |
| hr/tests/performance_tests.rs | `#![cfg(feature = "performance")]` |
| webhook/tests/integration_webhook.rs | `#![cfg(feature = "robot")]` |
| docs/tests/docs_contract_models.rs、helper_snapshots.rs + docs/examples/* + webhook/examples/* | build 阶段逐个确认门控（docs 当前 gate 含冗余 `v3` 且仍 E0433，需核查；examples 需补门控） |

> hr 另有内联 `mod tests`（lib.rs:230 等，feature-gated 模块内的 `#[cfg(test)] mod tests`）在 `test "mod"` 报错——属同一根因，build 阶段一并核查门控。

### 模式 B — missing_docs：`//!` 移到 `#![cfg]` 之上（9 文件）

现状：`#![cfg(...)]` 在 line 1、`//!` 文档在 line 2，clippy 报「missing documentation for the crate」（`-D missing-docs` 由 `-D warnings` 隐含）。修法：**交换两行**，让 `//!` 成为 line 1。spike 已验证（application 交换后 clippy exit 0）。

涉及：analytics、application、communication(×2)、mail、meeting、platform、workflow(×2) 的 `*_contract_models.rs` / `helper_snapshots.rs`。

### 模式 C — 次要 lint（E0609 ×4、unused import ×2）

`--keep-going` 报告的零星 lint，build 阶段按 clippy 指示逐个修（多在已被模式 A 改动的文件里）。

## 实施方式

build 阶段用 workflow 按 crate 并行修（每 agent 负责一个 crate：套用模式 A/B/C + 跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 验证本 crate 绿），主窗口汇总后跑全量目标命令。

## 验收与回归

- 核心：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
- 回归：`cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0；`cargo build --workspace --all-features` exit 0；`cargo fmt --all --check`；CI 现有检查不回退。
- 边界：门控只「不编译」、不删测试；`--all-features` 下相关测试仍正常运行。

## 非目标

不改 `src/` 业务代码、不改 Cargo.toml feature 定义、不改 `ci.yml`（CI flip 留后续小 change）。
