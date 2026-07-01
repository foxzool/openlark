# Comet Design Handoff

- Change: fix-missing-docs-lint-consistency
- Phase: design
- Mode: compact
- Context hash: 4f1799cd6adfd436c64a7615162232fde2d67c36db7a2719965fefc15845e114

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/fix-missing-docs-lint-consistency/proposal.md

- Source: openspec/changes/fix-missing-docs-lint-consistency/proposal.md
- Lines: 1-35
- SHA256: 8cff2e53f42cb1dd863726d43072b509637be9d6f0cffc9def770f8ed173d2b2

```md
## Why

issue #273 Part A1 现象 A 的最小治理。missing_docs lint 的**执行**存在两个不一致：

1. **just/CI 不一致**：`just lint`（justfile:14）用 `-A missing_docs` 放过 missing_docs，CI lint job（ci.yml:91 `RUSTFLAGS="-D warnings"` 无 `-A`）强制。开发者本地 `just lint` 绿 → 推送 → **CI 红**，且本地复现不出。
2. **源码 outlier 与 workspace.lints 分裂**：`[workspace.lints.rust] missing_docs = "warn"` 是单一基线，但 `openlark-security` 用 `#![deny]`（lib.rs:88，冗余升级）、`openlark-client` 残留死注释 `//#![deny]`（lib.rs:238，技术债）。

调研另发现更深问题（analytics `#![allow]` 隐藏未文档化项、17 个 missing_docs Python 测试不在 CI 跑、1057 行占位 doc），但**这些需独立 change 处理**（回补 doc / 决策测试去留 / 占位 doc 治理），不在本次最小范围。

现在做是因为：本次 3 处改动零风险（security 已全文档化，移除 deny 不暴露警告）、消除本地/CI 行为分裂、为后续深度治理扫清执行层。

## What Changes

- **修改** `justfile:14` 的 `just lint` recipe：移除 `-A missing_docs`（`cargo clippy --workspace --all-targets --all-features -- -Dwarnings`），对齐 CI ci.yml:91
- **删除** `crates/openlark-client/src/lib.rs:238` 的死注释 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译`
- **删除** `crates/openlark-security/src/lib.rs:88` 的 `#![deny(missing_docs)]`（回落 workspace `warn`；security 已全文档化，deny 是冗余升级）
- **保留** `crates/openlark-protocol/src/lib.rs:9` 的 `#[allow(missing_docs)]`（item 级，vendored pbbp2 生成模块，已登记例外）
- **不升** workspace `missing_docs = "deny"`（保持 `warn`）

## Capabilities

### New Capabilities

- `lint-execution-consistency`: lint 执行一致性策略——本地 `just lint` MUST 与 CI lint job 行为一致（不得在 just 用 `-A` 放过 CI 强制的 lint）；防止「本地绿 CI 红」复现。

### Modified Capabilities

（无——analytics outlier / Python 死测试 / codegen / 占位 doc 均为 Non-Goals，另案。）

## Impact

- **代码**：`justfile`（1 行 recipe）+ `crates/openlark-client/src/lib.rs`（删 1 死注释行）+ `crates/openlark-security/src/lib.rs`（删 1 `#![deny]` 行）。共 3 文件。
- **公共 API**：无变化（纯 lint 配置 + 注释/属性清理）。
- **CI**：无变化（CI 本就无 `-A`）；本地 `just lint` 行为对齐 CI（从放过 missing_docs 变为强制，但因 missing_docs 现状 0 警告，仍通过）。
- **风险**：极低——移除 security deny 后 missing_docs 仍 0（deny 当前编译通过 = security 全文档化）；build 阶段验证。
```

## openspec/changes/fix-missing-docs-lint-consistency/design.md

- Source: openspec/changes/fix-missing-docs-lint-consistency/design.md
- Lines: 1-62
- SHA256: c698b938a4b004062ca8b79b2f8523fb833a9a5366e5b0c65380375d6ffcf1a1

```md
## Context

OpenLark 的 missing_docs lint 跨 5 层治理，但执行层不一致：

| 层 | 现状 | 一致性 |
|----|------|--------|
| `[workspace.lints.rust]`（Cargo.toml:63） | `missing_docs = "warn"` | 基线 |
| `just lint`（justfile:14） | `... -Dwarnings -A missing_docs` | ❌ 放过（与 CI 相反） |
| CI lint job（ci.yml:91） | `RUSTFLAGS="-D warnings"`，无 `-A` | ✅ 强制 |
| 源码 `#![deny]` | security(lib.rs:88)、client 死注释(lib.rs:238) | ❌ 冗余/残留 |
| 源码 `#![allow]` | analytics(lib.rs:35) | ❌ 隐藏（另案） |

净效果：「同一份代码，just 绿、CI 红」——开发者本地无法复现 CI 的 missing_docs 失败。

调研另发现：17 个 missing_docs Python 测试（`tools/tests/test_*_missing_docs.py`）**不在 CI 跑**（CI ci.yml:113 只跑 `test_check_mod_reachability`），是死测试；1057 行占位 doc（`/// 待补充文档。`）撑着 0 警告。两者均**不在本 change 范围**。

约束：MSRV 1.88；security crate 当前以 `#![deny(missing_docs)]` 编译通过（=全文档化）。

## Goals / Non-Goals

**Goals:**

- `just lint` 与 CI lint job 行为一致（移除 justfile 的 `-A missing_docs`）
- 移除 security 的冗余 `#![deny]`（回落 workspace warn）
- 移除 client 的死注释残留
- missing_docs 仍 0 警告（迁移不暴露新问题）

**Non-Goals（均另案，记为已知债务）：**

- analytics `#![allow(missing_docs)]`（lib.rs:35，隐藏未文档化项，移除须回补 doc——独立 change）
- 17 个 missing_docs Python 测试不在 CI（死测试，加入 CI 或删除——独立决策）
- codegen `tools/codegen.py:185` 的 `-A missing_docs` 闭环（codegen 范围）
- 1057 行占位 doc 治理（独立 change）
- 不升 workspace `missing_docs = "deny"`（保持 warn）

## Decisions

### D1: just lint 移除 `-A missing_docs`，对齐 CI

`justfile:14` 从 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs` 改为 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`。CI ci.yml:91 是事实标准（`RUSTFLAGS="-D warnings"` 无 `-A`），just 应对齐而非反向。移除后 `just lint` 会强制 missing_docs，但因现状 0 警告（其他 crate 全文档化 + analytics 自身 allow 压制），仍通过。

### D2: 移除 security 的 `#![deny(missing_docs)]`（回落 warn）

security lib.rs:88 的 `#![deny]` 是对 workspace `warn` 的冗余升级。security 当前以 deny 编译通过 → 全文档化 → 移除 deny 后回落 workspace warn，missing_docs 仍 0。统一到 `[workspace.lints.rust]` 单一来源。

### D3: 移除 client 死注释

client lib.rs:238 的 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译` 是历史未完成迁移的注释残留（被注释掉，无实际作用）。直接删除，清理技术债。

### D4: 保留 protocol 的 item 级例外

protocol lib.rs:9 的 `#[allow(missing_docs)]` 是 item 级（vendored pbbp2 生成模块），已登记例外（`test_workspace_missing_docs.py:37` 的 item 级 allowlist 唯一条目）。保留。

## Risks / Trade-offs

- **[移除 security deny 暴露 missing_docs]** → Mitigation：security 当前 deny 编译通过 = 全文档化，移除后 0 警告；build 阶段 `cargo doc -p openlark-security` + `cargo clippy -p openlark-security` 验证。
- **[just lint 移除 -A 后本地失败]** → Mitigation：现状 missing_docs = 0（workspace 全文档化除 analytics 自带 allow），`just lint` 移除 -A 仍通过；build 阶段验证。
- **[范围克制]**：analytics / Python 测试 / codegen / 占位 doc 全 Non-Goals，避免范围蔓延；本 change 仅做零风险执行层对齐。

## Open Questions

- 无（3 处改动均零风险，调研已确认事实）。analytics 处理方式属下一个 change 的 design 决策。
```

## openspec/changes/fix-missing-docs-lint-consistency/tasks.md

- Source: openspec/changes/fix-missing-docs-lint-consistency/tasks.md
- Lines: 1-21
- SHA256: 1c714be95a24d7aa30baf56b74f2825d009635047a9c53b7246e566fd569d46e

```md
## 1. just lint 对齐 CI（D1）

- [ ] 1.1 `justfile:14` 的 `just lint` recipe 移除 `-A missing_docs`（改为 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`）
- [ ] 1.2 跑 `just lint` 确认仍通过（missing_docs 现状 0 警告，移除 -A 不破坏）

## 2. 源码 outlier 清理（D2 + D3）

- [ ] 2.1 删除 `crates/openlark-security/src/lib.rs:88` 的 `#![deny(missing_docs)]`
- [ ] 2.2 删除 `crates/openlark-client/src/lib.rs:238` 的死注释 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译`
- [ ] 2.3 确认 `crates/openlark-protocol/src/lib.rs:9` 的 `#[allow(missing_docs)]`（item 级例外）保留不动

## 3. 验证

- [ ] 3.1 `cargo fmt --check` 过
- [ ] 3.2 `cargo doc --workspace --all-features` 过（missing_docs warning = 0，deny/warn 不变）
- [ ] 3.3 `cargo clippy -p openlark-security --all-features -- -Dwarnings` 过（移除 deny 后 security 仍 0 警告）
- [ ] 3.4 `just lint` 过（移除 -A missing_docs 后与 CI 一致）
- [ ] 3.5 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` 过（CI 同款）
- [ ] 3.6 `cargo build --workspace --all-features` 过
- [ ] 3.7 msrv `--locked` 验证（`.github/msrv/Cargo.lock`，rustup +1.88）无回归
- [ ] 3.8 确认 outlier 清理结果：`grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client` 无命中
```

## openspec/changes/fix-missing-docs-lint-consistency/specs/lint-execution-consistency/spec.md

- Source: openspec/changes/fix-missing-docs-lint-consistency/specs/lint-execution-consistency/spec.md
- Lines: 1-36
- SHA256: af7ea3b91f3af82d96ecc52598033303caec0f358706c1034f6446c873433133

```md
## ADDED Requirements

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
```

