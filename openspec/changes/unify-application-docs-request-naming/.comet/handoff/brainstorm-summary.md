# Brainstorm Summary

- Change: unify-application-docs-request-naming
- Date: 2026-06-29

## 确认的技术方案

#271 后续小批次：application(3)+docs(1) 共 4 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` type alias。**直接应用 auth pilot（PR #280 已归档）已验证的模式**，无新决策。

每类型：struct+impl 重命名 → `XxxRequestBuilder`；`#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）；docs `PatchFormFieldQuestionBuilder` 的 `docs/base/bitable/mod.rs` re-export 双块（新名 + `#[allow(deprecated)]` 旧名）。

4 目标：AccessDataSearchBlockBuilder / AccessDataSearchCustomBuilder / AccessDataSearchWorkplaceBuilder（application，无 re-export）、PatchFormFieldQuestionBuilder（docs，1 re-export）。

## 关键取舍与风险

- 4 个全撞 body 模型名 → 全 → RequestBuilder（body `XxxRequest` 不动）。
- 4 个均无 trait impl、无 service 方法返回 → alias 无兼容性顾虑。
- **alias 放 `#[cfg(test)]` 前**（auth pilot clippy items_after_test_module 教训）。
- **push 前跑 `cargo fmt --check`**（auth pilot CI lint 教训）。
- 软 breaking，alias 源码兼容。

## 测试策略

build --all-features + clippy×3 + test（application+docs）+ `cargo fmt --check` + grep（4 RequestBuilder struct / 4 deprecated alias / RecordFieldsBuilder 未动）。

## Spec Patch

无。delta spec `application-docs-request-naming`（3 Requirement / 8 Scenario，含 fmt scenario）已覆盖。
