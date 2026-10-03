---
comet_change: unify-platform-small-request-naming
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-30-unify-platform-small-request-naming
status: final
---

# Design — unify-platform-small-request-naming（#271 platform 第 1 批小批）

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/unify-platform-small-request-naming/specs/platform-small-request-naming/spec.md` 为 canonical。

## 1. 背景与目标

#271 platform 第 1 批。模式已在 auth/application/docs 3 批验证：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。本批 12 个 platform 类型（trust_party/mdm/tenant/spark），均无 trait impl、无 re-export → 最简实现。

## 2. 关键核实

| 断言 | 验证 | 结论 |
|------|------|------|
| 12 个是请求类型（有 execute） | grep execute | ✅ |
| 无 trait impl | grep `impl Trait for XxxBuilder` | ✅ 12 个均 0 |
| 无 re-export | grep pub use | ✅ 12 个均 0 → 无链同步 |

## 3. 实现步骤（每类型，最简）

`pub struct XxxBuilder`+`impl` → `XxxRequestBuilder`；测试同步；`#[cfg(test)]` 前加：
```rust
#[deprecated(note = "renamed to XxxRequestBuilder, will be removed in v1.0 (#271)")]
pub type XxxBuilder = XxxRequestBuilder;
```

12 类型：AssignInfoListQuery/CollaborationDepartmentGet/CollaborationTenantGet/CollaborationTenantList/CollaborationUserGet/CountryRegionBatchGet/CountryRegionList/DirectoryUserIdConvert/TenantQuery/UserAuthDataRelationBind/UserAuthDataRelationUnbind/VisibleOrganization。

## 4. 测试策略

build --all-features + clippy×3（-D warnings）+ test platform + **cargo fmt --check** + grep（12 RequestBuilder struct / 12 alias / 0 残留）。

## 5. 风险与回滚

软 breaking（alias 源码兼容）。回滚 = revert。
