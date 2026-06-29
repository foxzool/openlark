---
change: unify-application-docs-request-naming
design-doc: docs/superpowers/specs/2026-06-29-unify-application-docs-request-naming-design.md
base-ref: 1ca5d9e0fe9ca2a9e8c6e8855cb3a6dbae60cf30
---

# unify-application-docs-request-naming 实施计划（#271 application+docs 批次）

> auth pilot（PR #280）已验证完全相同模式，本计划是其直接应用。4 类型，无新决策。

**Goal:** application(3)+docs(1) 共 4 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias，不破坏 build/clippy/test/fmt。

## Global Constraints

- 仅改名 + alias，方法签名/字段/逻辑不变。
- 不动 `RecordFieldsBuilder`（docs 真 builder 无 execute）、body 模型、其他 crate。
- alias 放 `#[cfg(test)]` 前（clippy items_after_test_module 教训）。
- **push 前必跑 `cargo fmt --all -- --check`**（CI lint 第一步）。
- commit message 中文。

## 4 类型旧名 → 新名映射

| # | 旧名 | 新名 | 定义文件 | re-export |
|---|------|------|---------|-----------|
| 1 | AccessDataSearchBlockBuilder | AccessDataSearchBlockRequestBuilder | application/workplace/workplace/v1/workplace_block_access_data/search.rs | 无 |
| 2 | AccessDataSearchCustomBuilder | AccessDataSearchCustomRequestBuilder | application/workplace/workplace/v1/custom_workplace_access_data/search.rs | 无 |
| 3 | AccessDataSearchWorkplaceBuilder | AccessDataSearchWorkplaceRequestBuilder | application/workplace/workplace/v1/workplace_access_data/search.rs | 无 |
| 4 | PatchFormFieldQuestionBuilder | PatchFormFieldQuestionRequestBuilder | docs/base/bitable/v1/app/table/form/field/patch.rs | docs/base/bitable/mod.rs |

## Task 1：4 类型重命名 + alias + re-export

- [ ] **Step 1:** application 3 个 search.rs：各 `pub struct XxxBuilder`+`impl` → `XxxRequestBuilder`，文件内 `XxxBuilder::` 引用（含测试）同步；在 `#[cfg(test)]` 前加 `#[deprecated(note="renamed to XxxRequestBuilder, will be removed in v1.0 (#271)")] pub type XxxBuilder = XxxRequestBuilder;`
- [ ] **Step 2:** docs patch.rs：`PatchFormFieldQuestionBuilder` → `PatchFormFieldQuestionRequestBuilder`（struct+impl+测试），`#[cfg(test)]` 前加 alias
- [ ] **Step 3:** docs/base/bitable/mod.rs re-export 双块：`pub use ...::PatchFormFieldQuestionRequestBuilder;` + `#[allow(deprecated)] pub use ...::PatchFormFieldQuestionBuilder;`
- [ ] **Step 4:** 增量验证 `cargo build -p openlark-application -p openlark-docs` exit 0
- [ ] **Step 5:** commit

## Task 2：全量验证

- [ ] **Step 1:** `cargo build --workspace --all-features` exit 0
- [ ] **Step 2:** 三组 clippy（default/all/no-default + `-D warnings`）exit 0
- [ ] **Step 3:** `cargo test -p openlark-application -p openlark-docs` 0 failed
- [ ] **Step 4:** **`cargo fmt --all -- --check` exit 0**（auth pilot CI 教训）
- [ ] **Step 5:** grep 确认 4 RequestBuilder struct + 4 deprecated alias + RecordFieldsBuilder 未动
- [ ] **Step 6:** CHANGELOG v0.18 breaking 段记录 4 个重命名

## Task 3：commit + 完成

- [ ] **Step 1:** 提交 CHANGELOG + tasks 勾选
