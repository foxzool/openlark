# Tasks — unify-platform-small-request-naming（#271 platform 第 1 批小批）

> platform trust_party/mdm/tenant/spark 12 个请求 builder → XxxRequestBuilder + #[deprecated] alias。#271 既定模式，最简实现（无 trait impl、无 re-export）。

## 1. 重命名 + alias

- [x] 1.1 12 个定义文件：struct+impl+测试 `XxxBuilder` → `XxxRequestBuilder`；在 `#[cfg(test)]` 前加 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`。12 类型：AssignInfoListQuery/CollaborationDepartmentGet/CollaborationTenantGet/CollaborationTenantList/CollaborationUserGet/CountryRegionBatchGet/CountryRegionList/DirectoryUserIdConvert/TenantQuery/UserAuthDataRelationBind/UserAuthDataRelationUnbind/VisibleOrganization

## 2. 验证

- [x] 2.1 `cargo build --workspace --all-features` exit 0
- [x] 2.2 三组 clippy（default/all/no-default + `-D warnings`）均 exit 0
- [x] 2.3 `cargo test -p openlark-platform` 0 failed
- [x] 2.4 **`cargo fmt --all -- --check` exit 0**（CI lint 教训）
- [x] 2.5 grep 确认 12 RequestBuilder struct + 12 deprecated alias + 0 旧 struct 残留

## 3. CHANGELOG

- [x] 3.1 CHANGELOG v0.18 breaking 段记录 12 个重命名（platform 小批）

## 代码审查（review_mode: standard）
Ready to merge: Yes（0 Critical/Important/Minor）。
**未来提示（directory 批次）**：`CollaborationTenantListBuilder` 在 trust_party（本批已改）和 directory 子系统都有（不同 struct）。directory 批次重命名时两者都会变 `CollaborationTenantListRequestBuilder` → crate 根命名冲突，需特殊处理（如 directory 那个保持原名或换前缀）。
