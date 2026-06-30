## ADDED Requirements

### Requirement: platform 小批请求 builder 统一 RequestBuilder 后缀
openlark-platform 的 trust_party/mdm/tenant/spark 子系统请求类型 builder SHALL 统一 `RequestBuilder` 后缀。12 个原 `XxxBuilder` SHALL 重命名为 `XxxRequestBuilder`。

#### Scenario: 12 个请求类型重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（AssignInfoListQuery/CollaborationDepartmentGet/CollaborationTenantGet/CollaborationTenantList/CollaborationUserGet/CountryRegionBatchGet/CountryRegionList/DirectoryUserIdConvert/TenantQuery/UserAuthDataRelationBind/UserAuthDataRelationUnbind/VisibleOrganization）
- **THEN** 12 个新 `XxxRequestBuilder` struct 存在

#### Scenario: 旧裸 Builder struct 不再存在
- **WHEN** grep 12 个旧 `XxxBuilder` 作 struct 定义
- **THEN** 0 命中（全退化为 type alias）

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 12 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: 旧名 alias 存在且标 deprecated + 在 test 前
- **WHEN** grep 12 个 `pub type XxxBuilder` + 检查与 `#[cfg(test)]` 顺序
- **THEN** 12 个 alias 带 `#[deprecated]`，均在 `#[cfg(test)]` 前

### Requirement: 重命名不破坏构建、lint、测试、格式
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** clippy default/all/no-default + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: platform 测试通过
- **WHEN** `cargo test -p openlark-platform`
- **THEN** 0 failed

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0
