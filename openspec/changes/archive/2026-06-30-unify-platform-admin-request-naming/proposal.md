## Why
issue #271 platform 第 2 批（admin）。前 4 批已归档（auth 12 + application/docs 4 + platform 小批 12），模式验证。admin 14 个裸 Builder，全最简（无 trait impl/re-export/service）。

## What Changes
- 14 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 类型：CreateBadge/CreateBadgeGrant/CreateBadgeImage/DeleteBadgeGrant/GetBadge/GetBadgeGrant/ListAdminDeptStat/ListAdminUserStat/ListAuditInfo/ListBadge/ListBadgeGrant/ResetPassword/UpdateBadge/UpdateBadgeGrant
- **BREAKING**（软）：alias 源码兼容 + warning，v1.0 移除

## Capabilities
### New Capabilities
- `platform-admin-request-naming`: platform admin 子系统请求 builder SHALL 统一 RequestBuilder 后缀。

## Impact
- openlark-platform admin 子系统 14 个定义文件改名 + alias。无 re-export/service/trait impl。
