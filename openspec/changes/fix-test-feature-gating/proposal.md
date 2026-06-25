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
