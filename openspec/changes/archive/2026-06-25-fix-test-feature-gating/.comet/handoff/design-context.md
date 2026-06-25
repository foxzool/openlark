# Comet Design Handoff

- Change: fix-test-feature-gating
- Phase: design
- Mode: compact
- Context hash: 2b77664b8fa0ddbce176159ba0415212c8b9cd1dba9d6b86dff00e1cf3443c0b

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/fix-test-feature-gating/proposal.md

- Source: openspec/changes/fix-test-feature-gating/proposal.md
- Lines: 1-30
- SHA256: f02f50e3172050fcda32dd1af442599ed99889aa9169ab0ef50f10c7879b6795

```md
## Why

`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` 失败（27 个 test/example target）——这是解锁 `ci.yml` line 116 + feature-combinations 矩阵从 `--lib` 升级到 `--all-targets`（#250 deferred 部分）的前置。失败分三类，都是测试/示例代码在 feature 关闭时的门控/文档问题，被 CI 长期用 `--lib`（不编译 test target）掩盖。

## What Changes

按 3 种修复模式处理约 17 个 test/example target（不改业务代码、不改 Cargo.toml feature 定义、不改 ci.yml）：

- **E0433（约 13 文件）**：test/example 引用 feature-gated 模块但自身未门控（或门控不准）→ 加/修 `#![cfg(feature = "...")]`。涉及 helpdesk、hr(×10 子系统)、webhook、docs(含 1 example)、webhook(2 examples)。
- **missing_docs（9 文件）**：test 文件 `//!` 文档在 `#![cfg]` 之后，clippy 报「missing documentation for the crate」→ 把 `//!` 移到 `#![cfg]` 之上（spike 已验证有效）。涉及 analytics、application、communication(×2)、mail、meeting、platform、workflow(×2)。
- **次要（E0609 ×4、unused import ×2）**：个别文件的少量 lint，逐个修。

## Capabilities

### New Capabilities
<!-- 无：纯测试/门控修复 -->

### Modified Capabilities
<!-- 无：不改变产品 spec 的验收行为 -->

## Impact

- 涉及 12 个 crate 的 tests/ 与 examples/（仅测试/示例代码）。
- 验收：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
- 回归：`cargo clippy --workspace --all-targets --all-features -- -D warnings` 仍 exit 0；CI 现有检查不回退。
- 解锁 #250 deferred：`ci.yml` line 116 + 矩阵 `--all-targets` 升级（后续小 change）。

## 成本/收益说明

#250 已让 CI 在 all-features 维度覆盖 test target（line 107，能抓 #248 类回归）。本 change 进一步覆盖 **no-default-features 维度**的 test target，属边际增益。修复机械（门控 + `//!` 顺序 + 少量 lint），低风险（仅测试代码）。
```

## openspec/changes/fix-test-feature-gating/design.md

- Source: openspec/changes/fix-test-feature-gating/design.md
- Lines: 1-55
- SHA256: a6bd17371877c768e9e80f8431330068b2537cc6593c68af5d396a4db6bc3c7e

```md
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
```

## openspec/changes/fix-test-feature-gating/tasks.md

- Source: openspec/changes/fix-test-feature-gating/tasks.md
- Lines: 1-12
- SHA256: 778161cbe9baf07381dae2fb6d6bbdea997d9e94699c031d944c7e07b9fedac1

```md
# Tasks — fix-test-feature-gating（#251）

> 目标：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
> build 阶段用 workflow 按 crate 并行；每 agent 改完跑 `cargo clippy --tests --no-default-features -p <crate> -- -D warnings` 验证本 crate 绿。

- [ ] 1. **hr**（10 个 tests/*.rs）：按 subsystem 补 `#![cfg(feature)]`（attendance/compensation/ehr/corehr/hire×3/okr/payroll/performance）+ 核查内联 `mod tests`（lib.rs:230 等）；本 crate clippy 验证
- [ ] 2. **helpdesk**：`helpdesk_contract_models.rs` 加 `#![cfg(feature = "v1")]`；验证
- [ ] 3. **webhook**：`integration_webhook.rs` 加 `#![cfg(feature = "robot")]` + 2 个 examples（webhook_error_handling、webhook_text_message）补门控；验证
- [ ] 4. **docs**：`docs_contract_models.rs`/`helper_snapshots.rs` 门控修正（去冗余 `v3`、补齐）+ `docs_readme_examples` example 门控；验证
- [ ] 5. **模式 B（`//!` 交换）**：analytics、application、communication(×2)、mail、meeting、platform、workflow(×2) 的 `//!` 移到 `#![cfg]` 之上；各 crate clippy 验证
- [ ] 6. **模式 C（次要 lint）**：按 clippy 指示修 E0609（×4）与 unused import（×2）
- [ ] 7. **全量验收**：`cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0；回归 `--all-features --all-targets`、`cargo build --workspace --all-features`、`cargo fmt --all --check` 全 exit 0
```

