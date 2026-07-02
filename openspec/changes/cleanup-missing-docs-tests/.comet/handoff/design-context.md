# Comet Design Handoff

- Change: cleanup-missing-docs-tests
- Phase: design
- Mode: compact
- Context hash: d85cc9f39c7f797cabc16c54506708859c872978b39b1cedef31e2beaa2ca6ee

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-missing-docs-tests/proposal.md

- Source: openspec/changes/cleanup-missing-docs-tests/proposal.md
- Lines: 1-35
- SHA256: ebd2f2fafe8f1ef2a533a61c6743ecb2c7e884b350702e789797659f77daf7be

```md
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
```

## openspec/changes/cleanup-missing-docs-tests/design.md

- Source: openspec/changes/cleanup-missing-docs-tests/design.md
- Lines: 1-63
- SHA256: 6b9dbb4148b55612f037c58d48beeccc79b03eb9a6d1c4ea8e9a3cf8115d09e5

```md
## Context

`tools/tests/` 有 **18 个 per-crate `test_openlark_*_missing_docs.py`** 全不在 CI（死测试）。勘探分两类：
- **`has_no_missing_docs_warnings`**（18 个）：`cargo test -p <crate> --all-features --no-run` 断言无 missing_docs。**冗余**于 #1 的 `test_workspace_has_no_missing_docs_warnings`（workspace `--all-features` 编译覆盖全部 crate）。慢（18× 编译）。全绿。
- **`do_not_suppress` 结构变体**（10 个，crate 特定）：硬编码"已清理文件/模块根"列表断言无 `#![allow]` 回归。**不冗余**、快（0.27s 文件扫描）、全绿。

承接 #1（PR #293 workspace 级测试接 CI）+ #4（PR #294 codegen 闭环）。

## Goals / Non-Goals

**Goals**：删 18 冗余 `has_no_warnings`（消除死测试 + 冗余 + CI 拖慢）；接 10 结构变体进 CI（激活有价值的 crate 特定回归守卫）；更新 spec 反映新 CI 范围。

**Non-Goals**：不改业务 crate 源码；不动 #1 的 workspace 级测试；不动 codegen（#4）；不治理占位 doc（#3）；不改测试的断言逻辑（只删冗余/接 CI）。

## Decisions

### D1: 删 18 冗余 `has_no_warnings`
- **8 文件整删**（仅含 has_no_warnings）：ai/analytics/application/auth/cardkit/client/core/webhook。
- **9 文件删 has_no_warnings 方法、留结构变体**：communication/docs/helpdesk/hr/mail/meeting/platform/protocol/workflow。
- 保留 `workflow_narrow`（本就只有结构变体 `mod_roots`）。

**冗余论证**：`has_no_warnings` 跑 `cargo test -p <crate> --all-features --no-run`，断言"无 missing_docs 警告"。#1 的 workspace 级测试跑 `cargo test --workspace --all-features --no-run` 同样断言，且 workspace `--all-features` 编译覆盖全部 crate 的全部 pub 项 → per-crate 测试是 workspace 测试的真子集，必然同时通过/失败。故冗余，删除安全。

**Alternative**：保留 has_no_warnings 接进 CI（belt-and-suspenders）——否决，18× 冗余 cargo 编译拖慢 CI，且不增覆盖。

### D2: 接 10 结构变体进 CI
ci.yml 在 `test_check_mod_reachability` + `test_workspace_missing_docs`（#1）旁，加一行 `python3 -m unittest` 跑 10 模块：
```
test_openlark_communication_missing_docs
test_openlark_docs_missing_docs
test_openlark_helpdesk_missing_docs
test_openlark_hr_missing_docs
test_openlark_mail_missing_docs
test_openlark_meeting_missing_docs
test_openlark_platform_missing_docs
test_openlark_protocol_missing_docs
test_openlark_workflow_missing_docs
test_openlark_workflow_narrow_missing_docs
```
删冗余后每模块仅剩结构变体（workflow_narrow 本就如此），故跑模块 = 跑结构变体。fast（0.27s）。

### D3: 验证策略
- 删除安全性：被删的 has_no_warnings 是死测试（本就不在 CI），删除不影响任何 CI/local 行为；workspace 级测试（#1，在 CI）持续覆盖"无 missing_docs"。
- 结构变体接 CI：本地实跑 10 模块全绿（已验证 0.27s）+ CI 接线后实跑。
- 回归：workspace `cargo doc --workspace --all-features` 仍 0（#1 守门不变）；`just lint` 不破（测试文件改动不影响 Rust lint）。

### D4: build 执行方式
改动机械（删 8 文件 + 删 9 方法 + ci.yml 一行 + spec），无跨模块协调。倾向 executing-plans（build 阶段 plan-ready 暂停由用户选定）。

## Risks / Trade-offs

- **[删 has_no_warnings 失去 per-crate 粒度]** → workspace 级测试（#1）已 subsume 全部 crate 的 missing_docs 断言；粒度损失无实际覆盖损失。
- **[结构变体硬编码路径过期]** → 10 个结构变体现全绿（已验证），路径当前有效；若后续文件移动致路径失效，CI 接线后会被捕获（变红，提示更新）——这正是接 CI 的价值。
- **[ci.yml 10 模块 unittest 行语法]** → 沿用现有 `python3 -m unittest tools.tests.X` 模式（与 #1 一致），yaml 缩进照抄 ci.yml:113-114。

## Migration Plan

纯增量、非破坏性（删死测试 + 接 CI）。回滚 = revert。顺序：先删冗余 has_no_warnings（D1）→ 接结构变体进 CI（D2）→ 验证（D3）。

## Open Questions

- build 执行方式（executing-plans vs subagent-driven）→ build 阶段 plan-ready 暂停由用户选定（D4 倾向 executing-plans，机械改动）。
- isolation: branch（默认）。tdd_mode: direct（测试文件删除/配置，无 TDD）。
```

## openspec/changes/cleanup-missing-docs-tests/tasks.md

- Source: openspec/changes/cleanup-missing-docs-tests/tasks.md
- Lines: 1-19
- SHA256: 87fa649f8aff3ba090fe05dff97b417df6854395afc36a5c1a936559b54c73a5

```md
## 1. D1：删冗余 `has_no_warnings`

- [ ] 1.1 **删 8 整文件**（仅含 has_no_warnings）：`test_openlark_{ai,analytics,application,auth,cardkit,client,core,webhook}_missing_docs.py`。
- [ ] 1.2 **9 文件删 `has_no_warnings` 方法、留结构变体**：`test_openlark_{communication,docs,helpdesk,hr,mail,meeting,platform,protocol,workflow}_missing_docs.py`（移除 `test_*_has_no_missing_docs_warnings` 方法 + 不再需要的 `subprocess` import；保留 `do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root` 结构变体 + 其所需 import）。
- [ ] 1.3 保留 `test_openlark_workflow_narrow_missing_docs.py` 不动（本就只有结构变体）。
- [ ] 1.4 自验：`grep -rn 'has_no_missing_docs_warnings' tools/tests/` 输出为空（18 处全删）；`ls tools/tests/test_openlark_*_missing_docs.py` 剩 10 文件（9 改 + workflow_narrow）。

## 2. D2：接 10 结构变体进 CI

- [ ] 2.1 `.github/workflows/ci.yml` 在 `test_workspace_missing_docs`（ci.yml:114，#1 加）旁，加一行 `python3 -m unittest` 跑 10 模块（缩进照抄 ci.yml:113-114）：
  `tools.tests.test_openlark_communication_missing_docs tools.tests.test_openlark_docs_missing_docs tools.tests.test_openlark_helpdesk_missing_docs tools.tests.test_openlark_hr_missing_docs tools.tests.test_openlark_mail_missing_docs tools.tests.test_openlark_meeting_missing_docs tools.tests.test_openlark_platform_missing_docs tools.tests.test_openlark_protocol_missing_docs tools.tests.test_openlark_workflow_missing_docs tools.tests.test_openlark_workflow_narrow_missing_docs`
- [ ] 2.2 yaml 语法：`python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` OK。

## 3. 验证

- [ ] 3.1 **本地跑 10 结构变体全绿**：`python3 -m unittest`（D2 那行）exit 0，确认 10 测试 pass。
- [ ] 3.2 **workspace missing_docs 仍 0**（#1 守门不变）：`cargo doc --workspace --all-features 2>&1 | grep -c 'missing documentation for'` = 0。
- [ ] 3.3 `cargo fmt --check` + `just lint`（测试文件改动不影响 Rust lint）通过。
- [ ] 3.4 **无残留死测试**：`ls tools/tests/test_openlark_*_missing_docs.py | wc -l` = 10；无仅含 has_no_warnings 的文件。
```

## openspec/changes/cleanup-missing-docs-tests/specs/lint-execution-consistency/spec.md

- Source: openspec/changes/cleanup-missing-docs-tests/specs/lint-execution-consistency/spec.md
- Lines: 1-32
- SHA256: 25b2e58813043702e304dcace502203c20ed9c115c7ed62d90602b687dfb8c24

```md
## MODIFIED Requirements

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
```

