## Why

issue #271（命名统一）platform crate 的第 1 批（小批）。前序 auth/application/docs 3 批已归档，验证了模式：请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias。platform 是 #271 最后的大 crate（97 裸 Builder），按子系统分批；本批是 trust_party/mdm/tenant/spark 4 个子系统共 12 个。

## What Changes

- 将 platform 的 **12 个请求 builder `XxxBuilder` 重命名为规范 `XxxRequestBuilder`**（均有 `execute()`，无 trait impl、无 re-export → 最简模式）：
  - trust_party: `UserAuthDataRelationBindBuilder`、`UserAuthDataRelationUnbindBuilder`
  - mdm: `AssignInfoListQueryBuilder`（注：虽叫 Query，但有 execute，是请求类型）
  - tenant: `TenantQueryBuilder`、`CollaborationDepartmentGetBuilder`、`CollaborationTenantGetBuilder`、`CollaborationTenantListBuilder`、`CollaborationUserGetBuilder`、`VisibleOrganizationBuilder`（注：collaboration/visible 属 tenant 子系统目录）
  - spark: `CountryRegionBatchGetBuilder`、`CountryRegionListBuilder`、`DirectoryUserIdConvertBuilder`
- 旧名作 `#[deprecated(note="...")] pub type XxxBuilder = XxxRequestBuilder;`（v0.18→v1.0 软迁移，放 `#[cfg(test)]` 前）
- 无 re-export 链（12 个均无）→ 仅定义文件改
- **BREAKING**（软）：`#[deprecated]` alias 保证源码兼容

## Capabilities

### New Capabilities
- `platform-small-request-naming`: openlark-platform 的 trust_party/mdm/tenant/spark 子系统请求类型 builder SHALL 统一 `RequestBuilder` 后缀；旧 `Builder` 名作 `#[deprecated]` alias 保留至 v1.0。

### Modified Capabilities
<!-- 无 -->

## Impact

- **openlark-platform**：12 个 struct+impl 重命名 + 12 alias + 测试同步。无 re-export。
- **破坏性**：软 breaking（alias 源码兼容 + warning）。v1.0 移除 alias。
- **非目标**：不动 platform 其他子系统（app_engine 51/directory 21/admin 14，后续批次）；不动 body 模型；不动非请求 builder。
