# Brainstorm Summary

- Change: unify-platform-small-request-naming
- Date: 2026-06-30

## 确认的技术方案

#271 platform 第 1 批（小批）。platform trust_party/mdm/tenant/spark 12 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。**直接应用 auth/application/docs 3 批已验证模式**，无新决策。

每类型：struct+impl 重命名 → RequestBuilder；`#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。12 个均无 re-export、无 trait impl → 仅定义文件改。

## 关键取舍与风险

- 全 →RequestBuilder（与 #271 既定方向一致；永不撞 body 模型名）。
- 无 trait impl/re-export → alias 无兼容性顾虑，无链同步。
- alias 放 `#[cfg(test)]` 前（clippy 教训）；push 前跑 `cargo fmt --check`（CI 教训）。

## 测试策略

build --all-features + clippy×3 + test platform + cargo fmt --check + grep（12 RequestBuilder struct / 12 deprecated alias / 0 旧 struct 残留）。

## Spec Patch

无。delta spec `platform-small-request-naming`（3 Requirement / 8 Scenario，含 fmt）已覆盖。
