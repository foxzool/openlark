## ADDED Requirements

### Requirement: admin 请求 builder 统一 RequestBuilder 后缀
platform admin 子系统 14 个请求类型 builder SHALL 统一使用 `RequestBuilder` 后缀。

#### Scenario: 14 个重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（CreateBadge/CreateBadgeGrant/CreateBadgeImage/DeleteBadgeGrant/GetBadge/GetBadgeGrant/ListAdminDeptStat/ListAdminUserStat/ListAuditInfo/ListBadge/ListBadgeGrant/ResetPassword/UpdateBadge/UpdateBadgeGrant）
- **THEN** 14 个新 `XxxRequestBuilder` struct 存在

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 14 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: 旧名 alias 存在且标 deprecated
- **WHEN** grep 14 个 `pub type XxxBuilder =`
- **THEN** 14 个 alias 带 `#[deprecated]`，均在 `#[cfg(test)]` 前

### Requirement: 重命名不破坏构建、lint、测试、格式
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0
