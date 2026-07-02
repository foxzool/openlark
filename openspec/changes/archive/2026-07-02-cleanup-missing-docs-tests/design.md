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
