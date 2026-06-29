# Tasks — unify-application-docs-request-naming（#271 批次）

> application(3)+docs(1) 共 4 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。auth pilot 模式直接应用。

## 1. 重命名 + alias + re-export

- [x] 1.1 application 3 个（AccessDataSearchBlock/Custom/Workplace）：struct+impl+测试重命名 + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（alias 放 `#[cfg(test)]` 前）；无 re-export
- [x] 1.2 docs 1 个（PatchFormFieldQuestion）：struct+impl+测试重命名 + alias；同步 `docs/base/bitable/mod.rs` re-export 双块（新名 + `#[allow(deprecated)]` 旧名）

## 2. 验证

- [x] 2.1 `cargo build --workspace --all-features` exit 0
- [x] 2.2 三组 feature clippy（default / --all-features / --no-default-features + `-D warnings`）均 exit 0
- [x] 2.3 `cargo test -p openlark-application -p openlark-docs` 0 failed
- [x] 2.4 **`cargo fmt --all -- --check` exit 0**（auth pilot CI 教训：push 前必跑）
- [x] 2.5 grep 确认 4 RequestBuilder struct + 4 deprecated alias + RecordFieldsBuilder 未动
