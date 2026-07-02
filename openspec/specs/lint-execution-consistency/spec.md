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

`missing_docs` lint 的级别 MUST 由根 `Cargo.toml` 的 `[workspace.lints.rust]` 单一声明（当前 `warn`），各 crate 经 `[lints] workspace = true` 继承。crate 级 `#![deny(missing_docs)]` / `#![allow(missing_docs)]` 属性属于绕过 workspace 的 outlier，MUST 清理（回落 workspace 级别），唯一例外是已登记的 vendored 生成模块 item 级 `#[allow]`。被 `allow` 压制的未文档化公开项 MUST 回补文档至 workspace 规范，使移除 outlier 后 missing_docs 警告为 0。

> 本 requirement 现覆盖全部 crate（含 `openlark-analytics`）。analytics 的 crate 级 `#![allow(missing_docs)]`（曾为独立 change 延期）已由 change `fix-analytics-missing-docs` 收口——移除该 allow 并回补其被压制的未文档化公开项。

#### Scenario: security/client crate 级 outlier 已清

- **WHEN** 运行 `grep -rn 'deny(missing_docs)' crates/openlark-security/src crates/openlark-client/src`
- **THEN** 输出 MUST 为空（security 的 `#![deny]` 已移除回落 workspace warn；client 的死注释已删）

#### Scenario: analytics crate 级 allow outlier 已清

- **WHEN** 运行 `grep -rn '#!\[allow(missing_docs)\]' crates/`
- **THEN** 输出 MUST 为空（analytics 的 `crates/openlark-analytics/src/lib.rs` crate 级 `#![allow(missing_docs)]` 已移除；其被压制的未文档化公开项已回补 doc，`cargo doc -p openlark-analytics --all-features` 无 missing_docs 警告）

#### Scenario: protocol item 级例外保留

- **WHEN** 检查 `crates/openlark-protocol/src/lib.rs`
- **THEN** vendored pbbp2 生成模块的 item 级 `#[allow(missing_docs)]` MAY 保留（已登记例外，对应 `tools/tests/test_workspace_missing_docs.py` 的 item 级 allowlist 唯一条目）

#### Scenario: 移除 outlier 后 missing_docs 仍 0

- **WHEN** 运行 `cargo doc --workspace --all-features` 与 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`
- **THEN** missing_docs warning MUST 为 0（analytics 回补后无新警告暴露）

### Requirement: missing_docs 验证测试 MUST 在 CI 运行

missing_docs 验证测试 MUST 在 CI（`.github/workflows/ci.yml`）执行，不得作为只在本地存在、CI 不跑的“死测试”。CI 须覆盖两层：

1. **workspace 级**（`tools/tests/test_workspace_missing_docs.py`）：workspace 无 missing_docs 警告、无 crate 级 missing_docs 抑制、item 级抑制仅限 protocol 生成模块。
2. **crate 特定结构变体**（`tools/tests/test_openlark_*_missing_docs.py` 的 `do_not_suppress` / `mod_roots` / `cleaned_slices` / `v1_root` 等方法）：各 crate 硬编码的“已清理文件/模块根”回归守卫，断言这些位置无 `#![allow(missing_docs)]` 回归。

此约束消除虚假强制感，确保 missing_docs 治理（含 crate 级 `allow` outlier 与各 crate 历史清理契约的回归）被 CI 持续守门。

> **冗余测试已删**：per-crate `has_no_missing_docs_warnings`（跑 `cargo test -p <crate> --no-run`）与 workspace 级测试完全冗余（workspace `--all-features` 编译已覆盖全部 crate 的 missing_docs），故删除、不得再加回。workspace 级测试是“无 missing_docs 警告”断言的单一来源。

#### Scenario: workspace 级 missing_docs 测试在 CI 执行

- **WHEN** 检查 `.github/workflows/ci.yml`
- **THEN** MUST 包含执行 `tools/tests/test_workspace_missing_docs.py`（覆盖其全部测试方法）的步骤，与已有的 `test_check_mod_reachability` 同级运行

#### Scenario: crate 级 allow 回归被 CI 捕获

- **WHEN** 有人重新向任一 crate 引入 `#![allow(missing_docs)]`
- **THEN** CI 执行的 `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions` MUST 失败，阻断合入

#### Scenario: crate 特定结构变体在 CI 执行

- **WHEN** 检查 `.github/workflows/ci.yml`
- **THEN** MUST 包含执行 10 个 crate 特定结构变体测试（`test_openlark_{communication,docs,helpdesk,hr,mail,meeting,platform,protocol,workflow}_missing_docs` + `test_openlark_workflow_narrow_missing_docs`）的步骤

#### Scenario: 无冗余 per-crate 编译测试

- **WHEN** 检查 `tools/tests/test_openlark_*_missing_docs.py`
- **THEN** MUST NOT 存在 `has_no_missing_docs_warnings` 方法（per-crate cargo 编译断言，已被 workspace 级 `test_workspace_has_no_missing_docs_warnings` 覆盖，冗余故删除）

