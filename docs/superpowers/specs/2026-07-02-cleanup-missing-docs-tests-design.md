---
comet_change: cleanup-missing-docs-tests
role: technical-design
canonical_spec: openspec
---

# Design: cleanup-missing-docs-tests

> #273 missing_docs 深度治理子项 #2。承接 #1（PR #293 workspace 级测试接 CI）+ #4（PR #294 codegen 闭环）。本 change 收口 `tools/tests/` 下不在 CI 的 18 个 per-crate missing_docs 测试：删冗余 + 接有价值变体。
>
> Canonical spec：`openspec/changes/cleanup-missing-docs-tests/specs/lint-execution-consistency/spec.md`（delta，MODIFIED）。

## 1. Context

`tools/tests/` 有 **18 个 per-crate `test_openlark_*_missing_docs.py`** 全不在 CI（ci.yml 只跑 `test_check_mod_reachability` + #1 接的 `test_workspace_missing_docs`）。死测试——虚假强制感。

勘探分两类（价值迥异）：
- **`has_no_missing_docs_warnings`**（18 个）：`cargo test -p <crate> --all-features --no-run` 断言无 missing_docs。**冗余**于 #1 的 workspace 级测试。慢（18× 编译）。全绿。
- **`do_not_suppress` 结构变体**（10 个，crate 特定）：硬编码"已清理文件/模块根"列表断言无 `#![allow]` 回归。**不冗余**、快（0.27s 文件扫描）、全绿。

## 2. 目标 / 非目标

**目标**：删 18 冗余 has_no_warnings（消除死测试 + 冗余 + CI 拖慢）；接 10 结构变体进 CI（激活 crate 特定回归守卫）；更新 spec。

**非目标**：不改业务 crate；不动 #1 workspace 测试；不动 codegen（#4）；不治理占位 doc（#3）；不改测试断言逻辑。

## 3. 方案

### D1: 删 18 冗余 `has_no_warnings`

| 文件 | 当前 | 操作 |
|------|------|------|
| ai/analytics/application/auth/cardkit/client/core/webhook（8） | 仅 has_no_warnings | **整文件删** |
| communication/docs/helpdesk/hr/mail/meeting/platform/protocol/workflow（9） | has_no_warnings + 结构变体 | 删 has_no_warnings 方法 **+ 删 `import subprocess`**（参照 workflow_narrow） |
| workflow_narrow（1） | 仅结构变体 | 保留不动 |

**import 清理**：workflow_narrow（无 has_no_warnings 的参照）只 `import unittest` + `from pathlib import Path`，无 `subprocess`。故 9 文件删方法时同步删 `import subprocess`（否则 unused）。

**冗余论证**：per-crate `cargo test -p <crate> --all-features --no-run` 断言无 missing_docs。#1 的 workspace `cargo test --workspace --all-features --no-run` 同样断言，且 workspace `--all-features` 编译是最严格超集（统一 features + 全 crate），per-crate 是其子集。workspace 通过 ⇒ 所有 per-crate 必通过（feature unification 只让 workspace 更宽，捕获更多非更少）。故冗余、删除安全。

### D2: 接 10 结构变体进 CI

ci.yml 现有 run block（ci.yml:112-115）加 `python3 -m unittest \` + 10 backslash 续行模块，沿用 ci.yml:69-72 api-contracts 多模块模式：

```yaml
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 -m unittest tools.tests.test_workspace_missing_docs
          python3 -m unittest \
            tools.tests.test_openlark_communication_missing_docs \
            tools.tests.test_openlark_docs_missing_docs \
            tools.tests.test_openlark_helpdesk_missing_docs \
            tools.tests.test_openlark_hr_missing_docs \
            tools.tests.test_openlark_mail_missing_docs \
            tools.tests.test_openlark_meeting_missing_docs \
            tools.tests.test_openlark_platform_missing_docs \
            tools.tests.test_openlark_protocol_missing_docs \
            tools.tests.test_openlark_workflow_missing_docs \
            tools.tests.test_openlark_workflow_narrow_missing_docs
          python3 tools/check_mod_reachability.py
```

删冗余后每模块仅剩结构变体（workflow_narrow 本就如此），跑模块 = 跑结构变体。

### D3: 验证策略

- **删除安全**：被删的 has_no_warnings 是死测试（本不在 CI），删除不改任何 CI/local 行为；#1 的 workspace 测试（在 CI）持续覆盖"无 missing_docs"。
- **结构变体接 CI**：本地实跑 10 模块全绿（已验证 0.27s）+ CI 接线后实跑。
- **回归**：workspace `cargo doc --workspace --all-features` 仍 0（#1 守门不变）；`cargo fmt --check` + `just lint`（测试文件改动不影响 Rust lint）；yaml 合法。

## 4. 决策与替代

| 决策 | 选择 | 否决的替代 |
|------|------|-----------|
| 冗余 has_no_warnings | 删除 | 接进 CI（18× 冗余编译拖慢）/ 留本地（死测试仍在） |
| 结构变体 | 接进 CI | 不接（死测试，虚假强制感） |
| ci.yml 多模块格式 | backslash 续行 | 单行长行（可读差）/ 单模块每行（冗长） |
| import 清理 | 同步删 subprocess | 留 unused import（dead code） |

## 5. 测试策略

| 层级 | 验证 |
|------|------|
| 结构变体 | 本地 + CI 跑 10 模块全绿 |
| 删除安全 | `cargo doc --workspace --all-features` missing_docs=0（#1 守门） |
| 回归 | `cargo fmt --check` + `just lint` 通过；yaml 合法 |
| 无残留死测试 | `ls test_openlark_*_missing_docs.py \| wc -l` = 10；`grep has_no_missing_docs_warnings tools/tests/` 空 |

## 6. 风险与缓解

- **[删 has_no_warnings 失去 per-crate 粒度]** → workspace 级测试 subsume 全部 crate；无覆盖损失。
- **[结构变体硬编码路径过期]** → 现全绿；接 CI 后路径失效会变红提示——接 CI 的价值所在。
- **[9 文件漏删 import subprocess]** → 自验 `python3 -c "import py_compile"` 或 ruff 查 unused import；参照 workflow_narrow。

## 7. 迁移与回滚

纯增量、非破坏性（删死测试 + 接 CI）。回滚 = revert。顺序：D1 删冗余 → D2 接变体 → D3 验证。

## 8. Open Questions / Build 阶段决策

- build_mode：机械改动（删 8 文件 + 改 9 + ci.yml），倾向 executing-plans → build plan-ready 暂停由用户选定。
- isolation: branch。tdd_mode: direct（测试文件删除/配置）。
