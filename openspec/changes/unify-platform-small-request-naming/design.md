## Context

#271 platform 第 1 批（小批）。模式已在 auth/application/docs 3 批完全验证：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。本批 12 个 platform 类型（trust_party/mdm/tenant/spark 子系统），均无 trait impl、无 re-export → 最简实现。

## Goals / Non-Goals

**Goals:** platform 小批 12 个请求 builder 统一 `XxxRequestBuilder` + `#[deprecated]` alias 软迁移。

**Non-Goals:** 不动 platform 其他子系统（app_engine/directory/admin）；不动 body 模型；不在 v0.18 硬移除旧名。

## Decisions

### 决策 1：方向 Builder → RequestBuilder（沿用 #271 既定方向）
12 个统一 →RequestBuilder（与 auth/application/docs 一致；RequestBuilder 目标永不撞 body 模型名）。

### 决策 2：#[deprecated] type alias 软迁移（沿用前序）
`pub struct XxxRequestBuilder` + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

### 决策 3：无 re-export（本批最简）
12 个均无 re-export → 仅定义文件 struct+impl+测试改名 + alias，无 re-export 链同步。

## Risks / Trade-offs

- alias 放 `#[cfg(test)]` 前（clippy items_after_test_module 教训）。
- push 前跑 `cargo fmt --check`（CI lint 教训）。
- 软 breaking，回滚 = revert。

## Migration Plan

v0.18：重命名 + alias。v1.0：移除 alias。CHANGELOG v0.18 breaking 段记录。

## Open Questions

无（12 个均无 trait impl/re-export，已核实）。
