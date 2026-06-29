---
comet_change: remove-deprecated-wiki-params
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-28-remove-deprecated-wiki-params
status: final
---

# Design — remove-deprecated-wiki-params

> OpenSpec delta spec `no-deprecated-wiki-params` 是事实源（canonical）。本文档为技术设计。

## 背景

openlark-docs wiki 4 个 `Params` struct（deprecated since 0.16.0，"请使用 XxxRequest 流式 Builder"）待移除：`SearchWikiParams`/`ListWikiSpacesParams`/`CreateWikiSpaceParams`/`MoveDocsToWikiParams`。~6 处用法需迁移到 Builder。deprecation 决策已做出，本 change 执行迁移 + 删除。

来源：#268 剩余 B 类（#278 跟踪；已确认拆分项 B）。A+E+D+C 已完成，G functional 保留。

## 决策

**D1（迁移方式，已确认）**：逐处把 `XxxParams { field: value, .. }` 用法改为 `XxxRequest::builder().field(value)..`；迁移完删除 4 个 Params struct。无替代方案。

## 改动清单

| 文件 | 动作 |
|------|------|
| `ccm/wiki/v1/node/search.rs` | SearchWikiParams 用法（2）→ SearchWikiRequest builder；删除 struct |
| `ccm/wiki/v2/space/list.rs` | ListWikiSpacesParams 用法（1）→ ListWikiSpacesRequest；删除 struct |
| `ccm/wiki/v2/space/create.rs` | CreateWikiSpaceParams 用法（1）→ CreateWikiSpaceRequest；删除 struct |
| `ccm/wiki/v2/space/node/move_docs_to_wiki.rs` | MoveDocsToWikiParams 用法（2）→ MoveDocsToWikiRequest；删除 struct |
| `CHANGELOG.md` | `[Unreleased] > Breaking Changes` + 迁移映射 |

## 迁移映射

| 旧（移除） | 新 |
|------------|-----|
| `SearchWikiParams {..}` | `SearchWikiRequest::builder()..` |
| `ListWikiSpacesParams {..}` | `ListWikiSpacesRequest::builder()..` |
| `CreateWikiSpaceParams {..}` | `CreateWikiSpaceRequest::builder()..` |
| `MoveDocsToWikiParams {..}` | `MoveDocsToWikiRequest::builder()..` |

## 风险

- **[Breaking]** 移除公开 struct → 编译失败；缓解：CHANGELOG 迁移；用法少（~6），影响小。
- **[字段映射]** Params 字段 → Builder setter 需逐处核对；build 阶段 clippy 验证。

## 测试策略

1. 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0。
2. `cargo test --workspace` 通过。
3. 4 个 Params struct grep = 0。

## 迁移与回滚

纯迁移 + 删除 + CHANGELOG；`git revert` 即可。

## 关联

- #268（B 类）；#278（F 类 im 别名仍 open）
