## Why

`tools/tests/` 下有 **18 个 per-crate `test_openlark_*_missing_docs.py`** 测试文件**全不在 CI**（ci.yml 只跑 `test_check_mod_reachability` + #1 接的 `test_workspace_missing_docs`）。这些是"死测试"——给虚假强制感：看似有 missing_docs 守门，实际 CI 不跑。

勘探发现这 18 文件含**两类测试**，价值迥异：
- **`has_no_missing_docs_warnings`**（18 个，每文件都有）：跑 `cargo test -p <crate> --all-features --no-run` 断言无 missing_docs 警告。**与 #1 已接的 `test_workspace_has_no_missing_docs_warnings`（workspace 级，编译全部 crate）完全冗余**——workspace 测试通过的，per-crate 必然通过。且慢（18× cargo 编译）。
- **`do_not_suppress` 结构变体**（10 个，crate 特定）：硬编码"已清理文件/模块根"列表，断言这些位置无 `#![allow(missing_docs)]` 回归。**不冗余**（crate 特定历史清理契约）、快（文件扫描 0.27s）、全绿。是有价值的回归守卫。

承接已归档的 #273 #1（PR #293，接 workspace 级测试）+ #4（PR #294，codegen 闭环）。

## What Changes

- **删除 18 个冗余 `has_no_warnings` 测试**：其中 8 个文件仅含此测试（ai/analytics/application/auth/cardkit/client/core/webhook）→ 整文件删除；9 个文件含结构变体（communication/docs/helpdesk/hr/mail/meeting/platform/protocol/workflow）→ 仅删 `has_no_warnings` 方法、留结构变体。
- **保留 `test_openlark_workflow_narrow_missing_docs.py`**（本就只有结构变体 `mod_roots`）。
- **接 10 个结构变体进 CI**：ci.yml 加一行 `python3 -m unittest` 跑这 10 个模块（删冗余后每模块仅剩结构变体）。
- **更新 `lint-execution-consistency` spec**：MODIFIED #1 加的"missing_docs 验证测试 MUST 在 CI 运行"要求——扩展覆盖结构变体测试 + 场景声明无冗余 per-crate 编译测试。

**非破坏性**：仅删/改测试文件 + ci.yml 一行；不改业务 crate 源码、不改 #1 workspace 测试、不动 codegen。

## Capabilities

### New Capabilities

（无——复用 `lint-execution-consistency`，扩展其 CI 测试要求。）

### Modified Capabilities

- `lint-execution-consistency`: MODIFIED #1 加的"missing_docs 验证测试 MUST 在 CI 运行"要求——CI 须跑 workspace 级测试 **+ 10 个 crate 特定结构变体**；新增场景声明 per-crate `has_no_warnings` 编译测试冗余（已被 workspace 级覆盖）故删除，不得再加回。

## Impact

- **测试文件**：删 8（ai/analytics/application/auth/cardkit/client/core/webhook）+ 改 9（移除 `has_no_warnings`）+ 保留 1（workflow_narrow）+ ci.yml +1 行跑 10 模块。
- **CI**：净效果 CI **变快**（去掉 18× 冗余 cargo 编译，加 10 个 fast 文件扫描 0.27s）。
- **Spec**：`openspec/specs/lint-execution-consistency/spec.md` delta（MODIFIED）。
- **业务 crate / 依赖 / 公开 API**：无变更。
