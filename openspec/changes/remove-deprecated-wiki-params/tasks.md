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
