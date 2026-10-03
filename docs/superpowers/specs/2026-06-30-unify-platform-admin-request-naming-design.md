---
comet_change: unify-platform-admin-request-naming
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-30-unify-platform-admin-request-naming
status: final
---

# Design — unify-platform-admin-request-naming（#271 platform admin 批）

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec 为 canonical。

## 1. 背景
#271 platform admin 批。模式 4 批验证。14 个全无 trait impl/re-export/service → 最简。

## 2. 实现
每类型：struct+impl → RequestBuilder；`#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）；测试同步。

14 类型：CreateBadge/CreateBadgeGrant/CreateBadgeImage/DeleteBadgeGrant/GetBadge/GetBadgeGrant/ListAdminDeptStat/ListAdminUserStat/ListAuditInfo/ListBadge/ListBadgeGrant/ResetPassword/UpdateBadge/UpdateBadgeGrant。

## 3. 测试
build --all-features + clippy×3 + test platform + cargo fmt --check + grep。

## 4. 风险
软 breaking（alias 兼容）。alias 放 #[cfg(test)] 前；push 前 fmt。
