# Comet Design Handoff

- Change: cleanup-application-placeholder-docs
- Phase: design
- Mode: compact
- Context hash: 43268aa1f6e9fb928808ac383aee7f95e017fdc842f008449d9584c911f60e04

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-application-placeholder-docs/proposal.md

- Source: openspec/changes/cleanup-application-placeholder-docs/proposal.md
- Lines: 1-20
- SHA256: c6e1b0c23e69ecf1f3805ca5cc6f4aed60e08fda8ed46f4a36bcf2ec6c826057

```md
## Why

`openlark-application` crate 有 **578 行 `/// 待补充文档。` 占位 doc**（91 文件，占全仓占位 55%）——legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。撑着 0 missing_docs 但无文档价值。本 change 清理 application 占位，回补真 doc。承接 #273 #1/#4/#2。

## What Changes
- 替换 application 全部 578 行 `/// 待补充文档。` 占位为有意义 doc（recipe 仿 #1：`<文件//! 标题>+<item 角色>`）。
- 修正 doc 位置到 `#[derive]` 前。
- delta：ADDED `missing-docs-quality` 的 application crate 场景（capability 由 change `cleanup-docs-placeholder-docs` 先建）。

非破坏性：仅 doc 文本 + 位置。

## Capabilities
### New Capabilities
（无——复用 `missing-docs-quality`，本 change 加 application 场景，依赖 `cleanup-docs-placeholder-docs` 先归档）
### Modified Capabilities
（本 change ADDED 新 requirement 到 `missing-docs-quality`：application crate 无占位）

## Impact
- 代码：`crates/openlark-application/src/**` 91 文件（578 占位 → 真 doc + 位置修正）。
- 无 API/依赖变更。
```

## openspec/changes/cleanup-application-placeholder-docs/design.md

- Source: openspec/changes/cleanup-application-placeholder-docs/design.md
- Lines: 1-18
- SHA256: 4cadcec760666ec91c63bbcf638d7976ca11ed0a99a0ec1de8d6131043150c1f

```md
## Context
application crate 578 行 `待补充文档` 占位（91 文件），legacy codegen 产物。占位项 = `pub fn new`/`execute`/`execute_with_options`/struct/field（同 #1 analytics 同构）。承接 #1/#4/#2。

## Goals / Non-Goals
**Goals**：替换 578 占位为真 doc；修正 doc 位置；占位符 grep 守门。
**Non-Goals**：不改逻辑；不动其它 crate；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<//! 标题>+<item 角色>`。
- **D2 执行**：578 项/91 文件，subagent-driven 按 version（v1/v5/v6/v7）+ sub-domain 分组（application 最大，分组并行）；占位符 + 位置双守门。
- **D3 验证**：逐组 `cargo doc -p openlark-application` 该组文件无 warning；全 crate 0 占位。

## Risks
- [规模 578 项诱发偷懒] → recipe 引用真实 API 名 + 占位符 grep 守门 + pilot 先行（仿 #1）。
- [doc 位置漏修] → 位置守门 grep。

## Migration
纯 doc。回滚 = revert。顺序：pilot → 按 version/domain 组回补 → 双守门 → 全局验证。
```

## openspec/changes/cleanup-application-placeholder-docs/tasks.md

- Source: openspec/changes/cleanup-application-placeholder-docs/tasks.md
- Lines: 1-11
- SHA256: 7c3e1f94bfdb5d72601c439b913f7c4711985f7045e1170118863d1697ed7dae

```md
## 1. 勘探 + pilot
- [ ] 1.1 勘探 application 91 文件占位分布（按 version v1/v5/v6/v7 + sub-domain 分组）。
- [ ] 1.2 Pilot 1 文件验证 recipe + 位置修正。

## 2. 批量回补（subagent-driven 按组）
- [ ] 2.1 按 version/domain 组回补 578 占位为真 doc + 修正 `///` 位置到 `#[derive]` 前。
- [ ] 2.2 逐组自验 `cargo doc -p openlark-application` 该组文件无 warning。

## 3. 守门 + 验证
- [ ] 3.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/` 为空；位置守门（`#[derive]` 后不紧跟 `///`）。
- [ ] 3.2 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint`；application 现有测试不破。
```

## openspec/changes/cleanup-application-placeholder-docs/specs/missing-docs-quality/spec.md

- Source: openspec/changes/cleanup-application-placeholder-docs/specs/missing-docs-quality/spec.md
- Lines: 1-15
- SHA256: 88926fc2065fc7c50093dffa2cc41e52188446542155e63996e7ef561c1bad51

```md
## ADDED Requirements

### Requirement: application crate 公开项 MUST 无占位符 doc

`openlark-application` crate 的公开项 MUST 有有意义 doc，MUST NOT 含 `/// 待补充文档。` 占位符。doc 注释 MUST 在 `#[derive]`/属性之前。

#### Scenario: application crate 无占位符 doc

- **WHEN** 运行 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-application/src/`
- **THEN** 输出 MUST 为空（578 行 `待补充文档` 占位已替换为有意义 doc）

#### Scenario: application doc 不在 #[derive] 后

- **WHEN** 检查 application crate 的 doc 注释位置
- **THEN** `///` MUST 在 `#[derive(...)]`/属性之前
```

