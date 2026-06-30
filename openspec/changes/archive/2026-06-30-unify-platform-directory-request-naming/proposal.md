## Why
issue #271 platform directory 批（第 6 批）。前 5 批已归档（42 类型），模式验证。directory 21 个裸 Builder，全最简（无 trait impl/re-export/service）。CollaborationTenantListBuilder 撞名已排除（platform 根不 re-export，不同模块路径）。

## What Changes
- 21 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 类型：CollaborationRule(Create/Delete/List/Update)、CollaborationShareEntityList、CollaborationTenantList、Department(Create/Delete/Filter/Mget/Patch/Search)、Employee(Create/Delete/Filter/Mget/Patch/Regular/Resurrect/Search/ToBeResigned)
- **BREAKING**（软）：alias 源码兼容

## Capabilities
### New Capabilities
- `platform-directory-request-naming`: platform directory 子系统请求 builder SHALL 统一 RequestBuilder 后缀。
