## Why

`openlark-analytics` 是工作区里最后一个 crate 级 `#![allow(missing_docs)]` outlier（`crates/openlark-analytics/src/lib.rs:36`），actively 隐藏 **122 个**未文档化公开项（68 struct / 36 method / 18 associated fn）。这是 issue #273 missing_docs 深度治理的最后一块——前序 Part B/A2/A1（PR #290/#291/#292）已清零其余 lint 维度，仅剩 analytics 因“移除须回补 doc”被 `lint-execution-consistency` spec 的范围边界条款显式延期。与此同时，捕获这类 outlier 的 3 个 missing_docs Python 测试当前不在 CI 运行（死测试，给虚假强制感）。现在收口。

## What Changes

- 移除 `crates/openlark-analytics/src/lib.rs:36` 的 `#![allow(missing_docs)]`，回落 workspace 级 `warn`（与其余 crate 单一来源一致）。
- 回补 analytics 全部 122 个缺失文档的公开项，符合 workspace 既有文档规范（`communication` crate 风格：每个 API 文件含文件级 `//!` 描述 + Feishu `docPath`，每个 struct/field/method 一行有意义中文）。
- 把 `tools/tests/test_workspace_missing_docs.py` 的 3 个测试接进 CI（`.github/workflows/ci.yml`），让 missing_docs 强制可执行、可回归。
- 更新 `lint-execution-consistency` spec：移除 analytics 范围边界豁免条款，新增“missing_docs 验证测试 MUST 在 CI 运行”要求。

**非破坏性**：仅新增文档 + CI 接线 + 移除 1 行抑制属性；不动公开 API 签名或运行时行为。

## Capabilities

### New Capabilities

（无——本 change 复用既有 `lint-execution-consistency` capability，收口其延期条款。）

### Modified Capabilities

- `lint-execution-consistency`: 移除 analytics outlier 的范围边界豁免（让“crate 级 allow/deny outlier MUST 清理”要求对 analytics 也生效）；新增“missing_docs 验证测试 MUST 在 CI 运行”要求，消除死测试的虚假强制感。

## Impact

- **代码**：`crates/openlark-analytics/src/**` 约 40 个 `.rs` 文件（回补 122 项 doc）；`lib.rs` 移除 1 行 `#![allow]`。
- **CI**：`.github/workflows/ci.yml` 增加 3 个 missing_docs 测试的执行步骤。
- **Spec**：`openspec/specs/lint-execution-consistency/spec.md` delta（移除豁免 + 新增 CI 要求）。
- **测试**：`tools/tests/test_workspace_missing_docs.py` 的 3 个测试从“死测试”转正为 CI 强制。
- **依赖 / 公开 API**：无变更。
