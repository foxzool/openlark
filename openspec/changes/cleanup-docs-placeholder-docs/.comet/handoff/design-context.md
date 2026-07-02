# Comet Design Handoff

- Change: cleanup-docs-placeholder-docs
- Phase: design
- Mode: compact
- Context hash: 141f208cb761b0d5aba36b1374810903fcd172eec42a2b7b9660bd6cc3e24979

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/cleanup-docs-placeholder-docs/proposal.md

- Source: openspec/changes/cleanup-docs-placeholder-docs/proposal.md
- Lines: 1-24
- SHA256: a5088e31015a6fb992daa973997fe108799801e7a6b059a1e7e88344fc222b3b

```md
## Why

`openlark-docs` crate 有 **144 行 `/// 公开项说明。` 占位 doc**（14 文件），撑着 0 missing_docs 警告但无实际文档价值——这是 #273 missing_docs 深度治理的最后一块（#1/#4/#2 已归档）。占位 doc 是 legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案，非当前 codegen 风格，#4 已让 codegen 产真 doc）。本 change 清理 docs crate 的占位，回补真 doc。

## What Changes

- 替换 docs crate 全部 144 行 `/// 公开项说明。` 占位为有意义 doc（recipe 仿 #1：`<文件//! 标题>+<item 角色>`，对齐 workspace 规范）。
- 修正 doc 位置到 `#[derive]` 前（legacy 把 `///` 放在 `#[derive]` 后，非标准）。
- 新 capability `missing-docs-quality`（ADDED：公开项 MUST 有有意义 doc，非占位符 + docs crate 无占位场景）。

**非破坏性**：仅 doc 文本 + 位置，不改逻辑/签名。承接 #273 #1（recipe）+ #4（codegen 不再产占位）+ #2（测试治理）。

## Capabilities

### New Capabilities
- `missing-docs-quality`: 公开项 MUST 有有意义 doc（禁止 `/// 待补充文档。`/`/// 公开项说明。` 等占位）；docs crate 占位已清。

### Modified Capabilities
（无）

## Impact
- 代码：`crates/openlark-docs/src/**` 14 文件（144 占位 → 真 doc + 位置修正）。
- Spec：新建 `openspec/specs/missing-docs-quality/spec.md`。
- 无 API/依赖变更。
```

## openspec/changes/cleanup-docs-placeholder-docs/design.md

- Source: openspec/changes/cleanup-docs-placeholder-docs/design.md
- Lines: 1-19
- SHA256: 8f7ede9e21977c7b87b44ba530b2fe6baf50ce5ef939bee2597e7c1478112de4

```md
## Context

docs crate 有 144 行 `/// 公开项说明。` 占位（14 文件），legacy codegen 产物（`///` 在 `#[derive]` 后 + 占位文案）。承接 #273 #1（recipe）+ #4（codegen 已修）+ #2（测试）。

## Goals / Non-Goals
**Goals**：替换 docs 144 占位为真 doc；修正 doc 位置到 `#[derive]` 前；建立 `missing-docs-quality` capability。
**Non-Goals**：不改逻辑/签名；不动其它 crate（application/small-crates 各自 change）；不动作 TestCheck。

## Decisions
- **D1 recipe**：仿 #1，`<文件//! 标题>+<item 角色>`（struct/field/fn 等）。逐文件读 `//!` 标题套 recipe。
- **D2 doc 位置**：`///` 移到 `#[derive]` 前（标准 + 对齐 #1/communication 规范）。
- **D3 执行**：14 文件、144 项，subagent-driven 按域分组（docs crate 内的 sub-domain）；占位符 grep 守门。

## Risks
- [doc 准确性] → 派生自文件级 `//!` + item 角色；占位符 grep + review 守门。
- [漏改位置] → 自验 `grep -A1` 确认无 `#[derive]` 后紧跟 `///`。

## Migration
纯 doc 改动。回滚 = revert。顺序：逐文件回补 → grep 守门 → cargo doc 0 警告。
```

## openspec/changes/cleanup-docs-placeholder-docs/tasks.md

- Source: openspec/changes/cleanup-docs-placeholder-docs/tasks.md
- Lines: 1-12
- SHA256: 391eb6aaf298c5edccd89e9bcbdc469d7252a4012466d44df7f9e0c46d45c340

```md
## 1. 回补 docs crate 占位 doc（144 项 / 14 文件）
- [ ] 1.1 勘探 docs crate 14 文件的占位项分布（按 sub-domain 分组）。
- [ ] 1.2 按 sub-domain 组回补：替换 `/// 公开项说明。` 为 `<//! 标题>+<item 角色>` 真 doc；修正 `///` 位置到 `#[derive]` 前。
- [ ] 1.3 逐组自验：`cargo doc -p openlark-docs --all-features` 该组文件无 warning。

## 2. 守门
- [ ] 2.1 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/` 为空。
- [ ] 2.2 doc 位置守门：`#[derive]` 后不紧跟 `///`。

## 3. 验证
- [ ] 3.1 `cargo doc --workspace --all-features` missing_docs=0；`cargo fmt --check` + `just lint` 通过。
- [ ] 3.2 docs crate 现有测试不破。
```

## openspec/changes/cleanup-docs-placeholder-docs/specs/missing-docs-quality/spec.md

- Source: openspec/changes/cleanup-docs-placeholder-docs/specs/missing-docs-quality/spec.md
- Lines: 1-15
- SHA256: f8778bb5aa105cf339242ef50658fcdb8952538d8c23d7b35bbb77f6b2531dfc

```md
## ADDED Requirements

### Requirement: 公开项 MUST 有有意义 doc（禁止占位符）

OpenLark 公开项（struct/enum/fn/method/field 等）的 doc 注释 MUST 是有意义的、描述该项的内容，MUST NOT 是 `/// 待补充文档。`、`/// 公开项说明。` 等占位符。占位符 doc 虽能让 missing_docs lint 通过，但无实际文档价值，违反 doc 质量治理。doc 注释 MUST 放在 `#[derive]`/属性**之前**（标准 Rust 规范）。

#### Scenario: docs crate 无占位符 doc

- **WHEN** 运行 `grep -rnE '/// (待补充文档|公开项说明)。' crates/openlark-docs/src/`
- **THEN** 输出 MUST 为空（docs crate 的 144 行 `公开项说明` 占位已替换为有意义 doc）

#### Scenario: 占位符 doc 不在 #[derive] 后

- **WHEN** 检查 docs crate 的 doc 注释位置
- **THEN** `///` MUST 在 `#[derive(...)]`/属性之前（修正 legacy 把 `///` 放 `#[derive]` 后的非标准位置）
```

