# Comet Design Handoff

- Change: remove-deprecated-wiki-params
- Phase: design
- Mode: compact
- Context hash: 8e3145fb6ed3bb7377acc835988e181323e672897782c6f7a72d58991895bf80

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/remove-deprecated-wiki-params/proposal.md

- Source: openspec/changes/remove-deprecated-wiki-params/proposal.md
- Lines: 1-24
- SHA256: 73f9d8a12aba7551c430952b27fcc599ecfcf38dca81aade3c43809662b78355

```md
## Why

openlark-docs 的 4 个 wiki `Params` struct（deprecated since 0.16.0，"请使用 XxxRequest 流式 Builder 模式"）待移除：`SearchWikiParams`、`ListWikiSpacesParams`、`CreateWikiSpaceParams`、`MoveDocsToWikiParams`。共 ~6 处用法需迁移到对应 Builder。

来源：#268 剩余的 B 类（#278 跟踪；A+E+D+C 已完成，G functional 保留）。本 change 是已确认拆分项 B。

## What Changes

- **BREAKING**：迁移 4 个 deprecated `Params` 的 ~6 处用法到对应 `XxxRequest` Builder，然后移除 4 个 `Params` struct。
- 迁移映射：`SearchWikiParams` → `SearchWikiRequest` builder；`ListWikiSpacesParams` → `ListWikiSpacesRequest`；`CreateWikiSpaceParams` → `CreateWikiSpaceRequest`；`MoveDocsToWikiParams` → `MoveDocsToWikiRequest`。

## Capabilities

### New Capabilities
- `no-deprecated-wiki-params`: openlark-docs wiki 模块 SHALL 不保留 deprecated `Params` 兼容 struct；用户 SHALL 使用流式 Builder。

### Modified Capabilities
<!-- 无 -->

## Impact

- **openlark-docs**：`ccm/wiki/v1/node/search.rs`、`v2/space/list.rs`、`v2/space/create.rs`、`v2/space/node/move_docs_to_wiki.rs` 删除 4 个 `Params` struct + 迁移 ~6 处用法。
- **破坏性**：移除公开 deprecated struct（since 0.16.0）。CHANGELOG breaking + 迁移指引。
- **非目标**：不动 F（im 别名，另一拆分项）；不动非 deprecated 的 Params（如 CreateWikiSpaceNodeParams）；不改 Builder 实现。
```

## openspec/changes/remove-deprecated-wiki-params/design.md

- Source: openspec/changes/remove-deprecated-wiki-params/design.md
- Lines: 1-30
- SHA256: 8b4d5ebd0af0dc639e18d652725cffb2b110c2d21e7abdefb00f87a65accf265

```md
## Context

4 个 deprecated wiki `Params` struct（since 0.16.0）待移除，~6 处用法迁移到 Builder。deprecation 决策已做出，本 change 执行迁移 + 删除。

## Goals / Non-Goals

**Goals:** 迁移 4 个 Params 的 ~6 处用法到 Builder；删除 4 个 deprecated struct。

**Non-Goals:** 不动 F（im 别名）；不动非 deprecated Params；不改 Builder 实现。

## Decisions

**D1（迁移方式）**：每处 `XxxParams { field: value }` 用法 → `XxxRequest::builder().field(value)...`；迁移完删除 Params struct。逐文件处理。

## Risks

- **[Breaking]** 移除公开 struct → 编译失败；缓解：CHANGELOG 迁移；用法少（~6），影响小。
- **[迁移正确性]** Params → Builder 字段映射需逐个核对（字段名/类型可能略异）。build 阶段 clippy 验证。

## Migration Plan

1. 逐文件迁移 ~6 处 Params 用法到 Builder。
2. 删除 4 个 Params struct。
3. 三组 clippy + test。
4. CHANGELOG breaking + 迁移表。
5. 回滚：git revert。

## Open Questions

- 无（deprecation pre-decided；用法少）。
```

## openspec/changes/remove-deprecated-wiki-params/tasks.md

- Source: openspec/changes/remove-deprecated-wiki-params/tasks.md
- Lines: 1-20
- SHA256: 6c73e5d3ae2c0965f28c7baf1e193ec395ceab5a7f44100811cfeebce5cda279

```md
# Tasks — remove-deprecated-wiki-params

> 已确认拆分项 B（#268 剩余）。4 个 deprecated wiki Params → Builder 迁移 + 删除。~6 用法。BREAKING。

## 1. 迁移用法 + 删除 4 个 Params struct

- [ ] 1.1 `search.rs`：SearchWikiParams 用法（2）→ SearchWikiRequest builder；删除 struct
- [ ] 1.2 `list.rs`：ListWikiSpacesParams 用法（1）→ ListWikiSpacesRequest；删除 struct
- [ ] 1.3 `create.rs`：CreateWikiSpaceParams 用法（1）→ CreateWikiSpaceRequest；删除 struct
- [ ] 1.4 `move_docs_to_wiki.rs`：MoveDocsToWikiParams 用法（2）→ MoveDocsToWikiRequest；删除 struct

## 2. 验证

- [ ] 2.1 4 个 Params struct grep = 0
- [ ] 2.2 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] 2.3 `cargo test --workspace` 通过

## 3. CHANGELOG

- [ ] 3.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射（Params → Builder）
```

## openspec/changes/remove-deprecated-wiki-params/specs/no-deprecated-wiki-params/spec.md

- Source: openspec/changes/remove-deprecated-wiki-params/specs/no-deprecated-wiki-params/spec.md
- Lines: 1-23
- SHA256: 2eb215f473c5502e78b91eb7562a010a214fa4af75eaa44d88f82e8bc62e8958

```md
## ADDED Requirements

### Requirement: wiki 模块不保留 deprecated Params struct
openlark-docs wiki 模块 SHALL 不保留 `SearchWikiParams`/`ListWikiSpacesParams`/`CreateWikiSpaceParams`/`MoveDocsToWikiParams` deprecated struct。用户 SHALL 使用对应 `XxxRequest` 流式 Builder。

#### Scenario: 4 个 Params struct 移除
- **WHEN** 在 `crates/openlark-docs/src/ccm/wiki/` 中 grep `pub struct SearchWikiParams|pub struct ListWikiSpacesParams|pub struct CreateWikiSpaceParams|pub struct MoveDocsToWikiParams`
- **THEN** 命中数为 0（4 个 deprecated Params 全部移除）

#### Scenario: 用法迁移到 Builder
- **WHEN** 迁移后以 default feature 构建 openlark-docs
- **THEN** 原 Params 用法改为 `XxxRequest::builder()...`，编译通过

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 三组 feature + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: tests 通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过
```

