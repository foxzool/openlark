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
