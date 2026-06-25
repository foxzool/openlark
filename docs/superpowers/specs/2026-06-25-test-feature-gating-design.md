---
comet_change: fix-test-feature-gating
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-25-fix-test-feature-gating
status: final
---

# Design Doc — fix-test-feature-gating（#251）

## 目标

使 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0，解锁 `ci.yml` line 116 + feature-combinations 矩阵从 `--lib` 升级到 `--all-targets`（#250 deferred）。

`--keep-going` 实测失败面：27 个 test/example target，三类问题。`--all-features --all-targets` 已绿（#249），故只处理 no-default-features 维度。

## 修复模式

### A — E0433：补/修 feature 门控

test/example 引用 feature-gated 模块但自身未门控 → 文件顶部加 `#![cfg(feature = "<f>")]`。feature 映射（workflow 逐文件核对 lib.rs `#[cfg(feature)]` + Cargo.toml）：

| 文件 | 门控 |
|------|------|
| helpdesk/tests/helpdesk_contract_models.rs | `v1` |
| hr/tests/attendance_tests.rs | `attendance` |
| hr/tests/compensation_tests.rs | `compensation` |
| hr/tests/ehr_tests.rs | `ehr` |
| hr/tests/feishu_people_tests.rs | `corehr`（模块名 feishu_people ≠ feature 名 corehr） |
| hr/tests/hire_request_modeling_tests.rs / hire_response_contract_tests.rs / hire_tests.rs | `hire` |
| hr/tests/okr_tests.rs | `okr` |
| hr/tests/payroll_tests.rs | `payroll` |
| hr/tests/performance_tests.rs | `performance` |
| webhook/tests/integration_webhook.rs | `robot`（文件内已有的 per-test `card`/`signature` cfg 不动） |
| docs/tests/docs_contract_models.rs、helper_snapshots.rs + docs/examples/docs_readme_examples + webhook/examples/* | build 阶段逐个确认（docs 当前 gate 含冗余 `v3` 且仍 E0433，需核查） |

> hr 内联 `mod tests`（lib.rs:230 等，位于 feature-gated 模块内）也报 `test "mod"`——同根因，一并核查门控。

### B — missing_docs：`//!` 移到 `#![cfg]` 之上

现状 `#![cfg(...)]` 在 line 1、`//!` 在 line 2，clippy 报「missing documentation for the crate」。修法：**交换两行**让 `//!` 成 line 1。spike 验证（application_contract_models.rs 交换后 clippy exit 0）。涉及 analytics、application、communication(×2)、mail、meeting、platform、workflow(×2)。

### C — 次要 lint

`--keep-going` 报告的 E0609（×4）、unused import（×2），按 clippy 指示逐个修（多在模式 A 改动的文件里）。

## 实施方式

build 阶段用 workflow 按 crate 并行（每 agent 一个 crate：套用 A/B/C + 跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 自验），主窗口汇总后跑全量目标命令。

## 验收与回归

- 核心：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
- 回归：`--all-features --all-targets` exit 0；`cargo build --workspace --all-features` exit 0；`cargo fmt --all --check`；CI 现有检查不回退。
- 边界：门控只「不编译」、不删测试；`--all-features` 下相关测试仍正常运行。

## 非目标

不改 `src/` 业务代码、不改 Cargo.toml feature 定义、不改 `ci.yml`（CI flip 留后续小 change）。
