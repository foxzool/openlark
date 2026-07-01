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
