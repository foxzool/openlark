---
change: remove-deprecated-wiki-params
design-doc: docs/superpowers/specs/2026-06-28-remove-deprecated-wiki-params-design.md
base-ref: 402f3b66dd1adfcf418abdb7cfc83c67ddfd89ec
---

# remove-deprecated-wiki-params 实施计划

**Goal**：迁移 4 个 deprecated wiki Params（~6 用法）到 Builder + 删除 struct。BREAKING。

## Task 1: 迁移用法 + 删除 4 个 Params struct

- [ ] **Step 1:** `search.rs`：SearchWikiParams（2 用法）→ SearchWikiRequest::builder()；删除 struct
- [ ] **Step 2:** `list.rs`：ListWikiSpacesParams（1 用法）→ ListWikiSpacesRequest::builder()；删除 struct
- [ ] **Step 3:** `create.rs`：CreateWikiSpaceParams（1 用法）→ CreateWikiSpaceRequest::builder()；删除 struct
- [ ] **Step 4:** `move_docs_to_wiki.rs`：MoveDocsToWikiParams（2 用法）→ MoveDocsToWikiRequest::builder()；删除 struct

## Task 2: 验证

- [ ] **Step 1:** 4 个 Params struct grep = 0
- [ ] **Step 2:** 三组 clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] **Step 3:** `cargo test --workspace` 通过

## Task 3: CHANGELOG

- [ ] **Step 1:** CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射
