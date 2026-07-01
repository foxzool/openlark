# lint-execution-consistency Specification

## Purpose
TBD - created by archiving change fix-missing-docs-lint-consistency. Update Purpose after archive.
## Requirements
### Requirement: 本地 `just lint` MUST 与 CI lint job 行为一致

OpenLark 的本地 lint 命令（`just lint`）MUST 与 CI lint job（`.github/workflows/ci.yml` 的 lint job）对 `missing_docs`（及所有 lint）的处理保持一致——不得在 `just lint` 用 `-A` 放过 CI 强制的 lint。此约束消除「本地绿、CI 红」的复现盲区，保证开发者本地通过即 CI 通过。

#### Scenario: just lint 不放过 CI 强制的 lint

- **WHEN** 检查 `justfile` 的 `just lint` recipe 命令行参数
- **THEN** MUST NOT 出现 CI lint job 未使用的 `-A <lint>` 抑制标志（特别是 `-A missing_docs`）；`just lint` MUST 与 CI 一致使用 `-Dwarnings` 而不额外放过 missing_docs

#### Scenario: 本地通过即 CI 通过

- **WHEN** 开发者本地运行 `just lint` 并通过
- **THEN** CI lint job（`RUSTFLAGS="-D warnings"` 无 `-A missing_docs`）MUST 也通过——本地与 CI 行为对齐，无 missing_docs 复现盲区

### Requirement: missing_docs lint 治理收归 workspace.lints 单一来源（安全 outlier 收编）

`missing_docs` lint 的级别 MUST 由根 `Cargo.toml` 的 `[workspace.lints.rust]` 单一声明（当前 `warn`），各 crate 经 `[lints] workspace = true` 继承。crate 级 `#![deny(missing_docs)]` / `#![allow(missing_docs)]` 属性属于绕过 workspace 的 outlier，MUST 清理（回落 workspace 级别），唯一例外是已登记的 vendored 生成模块 item 级 `#[allow]`。

#### Scenario: security/client crate 级 outlier 已清

- **WHEN** 运行 `grep -rn 'deny(missing_docs)' crates/openlark-security/src crates/openlark-client/src`
- **THEN** 输出 MUST 为空（security 的 `#![deny]` 已移除回落 workspace warn；client 的死注释已删）

#### Scenario: protocol item 级例外保留

- **WHEN** 检查 `crates/openlark-protocol/src/lib.rs`
- **THEN** vendored pbbp2 生成模块的 item 级 `#[allow(missing_docs)]` MAY 保留（已登记例外，对应 `tools/tests/test_workspace_missing_docs.py` 的 item 级 allowlist 唯一条目）

#### Scenario: 移除 outlier 后 missing_docs 仍 0

- **WHEN** 运行 `cargo doc --workspace --all-features` 与 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`
- **THEN** missing_docs warning MUST 为 0（移除 security deny 不暴露新问题，因 security 已全文档化）

> **范围边界（诚实限制）**：本 requirement 仅覆盖 security/client 的安全 outlier 收编。`openlark-analytics` 的 crate 级 `#![allow(missing_docs)]`（lib.rs:35，隐藏未文档化项）**不在本 requirement 范围**——移除须回补 analytics 文档，属独立 change。该 outlier 的存在不代表本 requirement 失效，而是后续治理项。

