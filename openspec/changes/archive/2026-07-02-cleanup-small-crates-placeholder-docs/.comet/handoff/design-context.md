# Comet Design Handoff

- Change: cleanup-small-crates-placeholder-docs
- Phase: design
- Mode: compact
- Context hash: 8f3dce305ad6118e227cf2e19be08298ec940297d65e90aa976895b4cfe0f781

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-small-crates-placeholder-docs/proposal.md

- Source: openspec/changes/cleanup-small-crates-placeholder-docs/proposal.md
- Lines: 1-20
- SHA256: f32e07c46221062dfb6f4c9fdc479e2ab3903af2bc6bac13011e7c8c9e91a943

```md
## Why

5 个中小 crate 有 **335 行 `/// 待补充文档。` 占位 doc**：mail 104 / workflow 78 / meeting 65 / user 47 / hr 41——legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。本 change 清理这 5 个 crate 的占位，回补真 doc。承接 #273 #1/#4/#2，是 #3 拆分的最后一批。

## What Changes
- 替换 mail/workflow/meeting/user/hr 全部 335 行 `/// 待补充文档。` 占位为有意义 doc（recipe 仿 #1）。
- 修正 doc 位置到 `#[derive]` 前。
- delta：ADDED `missing-docs-quality` 的 small-crates 场景（capability 由前序 change 先建）。

非破坏性：仅 doc 文本 + 位置。

## Capabilities
### New Capabilities
（无——复用 `missing-docs-quality`，本 change 加 small-crates 场景）
### Modified Capabilities
（本 change ADDED 新 requirement 到 `missing-docs-quality`：mail/workflow/meeting/user/hr 无占位）

## Impact
- 代码：5 个 crate 的 `src/**`（335 占位 → 真 doc + 位置修正）。
- 无 API/依赖变更。
```

## openspec/changes/cleanup-small-crates-placeholder-docs/design.md

- Source: openspec/changes/cleanup-small-crates-placeholder-docs/design.md
- Lines: 1-18
- SHA256: 0f9d5c953a320f70d460f9d5410c6dbf42c0ad800bf20e951047eea31ca18eb5

```md
## Context
5 个中小 crate 共 335 行 `待补充文档` 占位（mail 104/workflow 78/meeting 65/user 47/hr 41），legacy codegen 产物。占位项同 #1 analytics 同构。承接 #1/#4/#2。

## Goals / Non-Goals
**Goals**：替换 335 占位为真 doc；修正 doc 位置；占位符 grep 守门。
**Non-Goals**：不改逻辑；不动 application/docs（各自 change）；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<//! 标题>+<item 角色>`。
- **D2 执行**：按 crate 分 5 组 subagent-driven（每 crate 独立单元）；占位符 + 位置双守门。mail 最大（104）先行。
- **D3 验证**：逐 crate `cargo doc -p <crate>` 无 warning；5 crate 0 占位。

## Risks
- [跨 5 crate 一致性] → 同 recipe + 双守门；per-crate 自验。
- [doc 位置漏修] → 位置守门 grep。

## Migration
纯 doc。回滚 = revert。顺序：按 crate（mail→workflow→meeting→user→hr）回补 → 双守门 → 全局验证。
```

## openspec/changes/cleanup-small-crates-placeholder-docs/tasks.md

- Source: openspec/changes/cleanup-small-crates-placeholder-docs/tasks.md
- Lines: 1-11
- SHA256: 309e3291f3b261ea21a2b2b471a25b81f5ef88cc868c82b2ab86e76bc13a1e32

```md
## 1. 勘探 + pilot
- [ ] 1.1 勘探 5 crate（mail/workflow/meeting/user/hr）占位文件分布。
- [ ] 1.2 Pilot mail 1 文件验证 recipe + 位置修正。

## 2. 按 crate 回补（subagent-driven，5 组）
- [ ] 2.1 mail（104）→ workflow（78）→ meeting（65）→ user（47）→ hr（41）逐 crate 回补占位为真 doc + 修正 `///` 位置。
- [ ] 2.2 逐 crate 自验 `cargo doc -p <crate>` 无 warning。

## 3. 守门 + 验证
- [ ] 3.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-{mail,workflow,meeting,user,hr}/src/` 为空；位置守门。
- [ ] 3.2 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint`；5 crate 现有测试不破。
```

## openspec/changes/cleanup-small-crates-placeholder-docs/specs/missing-docs-quality/spec.md

- Source: openspec/changes/cleanup-small-crates-placeholder-docs/specs/missing-docs-quality/spec.md
- Lines: 1-15
- SHA256: c6357d695ca21424905a1b56a5ec18641bccd0fdc6b2e2a209f6c0a78635ceef

```md
## ADDED Requirements

### Requirement: small-crates（mail/workflow/meeting/user/hr）公开项 MUST 无占位符 doc

`openlark-{mail,workflow,meeting,user,hr}` 5 个 crate 的公开项 MUST 有有意义 doc，MUST NOT 含 `/// 待补充文档。` 占位符。doc 注释 MUST 在 `#[derive]`/属性之前。

#### Scenario: small-crates 无占位符 doc

- **WHEN** 运行 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-{mail,workflow,meeting,user,hr}/src/`
- **THEN** 输出 MUST 为空（335 行 `待补充文档` 占位已替换为有意义 doc）

#### Scenario: small-crates doc 不在 #[derive] 后

- **WHEN** 检查这 5 个 crate 的 doc 注释位置
- **THEN** `///` MUST 在 `#[derive(...)]`/属性之前
```

