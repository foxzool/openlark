## ADDED Requirements

### Requirement: auth 请求类型统一使用 Request 后缀
openlark-auth 的 API 请求类型 SHALL 统一使用 `Request` 后缀命名（对齐 docs/communication/helpdesk 的既有惯例）。13 个原 `XxxBuilder` 请求类型 SHALL 重命名为规范 `XxxRequest`；`TenantAccessTokenInternalRequestBuilder` SHALL 重命名为 `TenantAccessTokenInternalRequest`（去 `Builder`，避免双 `Request`）。

#### Scenario: 13 个请求类型重命名为 Request
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub struct XxxRequest`（针对 AppAccessToken/AppAccessTokenInternal/AppTicketResend/Authorization/IdentityCreate/OidcAccessToken/OidcRefreshAccessToken/RefreshUserAccessTokenV1/TenantAccessToken/TenantAccessTokenInternal/UserAccessTokenV1/UserInfo/VerificationGet）
- **THEN** 13 个 `XxxRequest` struct 均存在（重命名完成）

#### Scenario: auth 不再有裸 Builder 请求类型
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep 请求类型的 `XxxBuilder`（排除 `AuthorizationUrlBuilder` 这个 URL builder）
- **THEN** 13 个请求 Builder 名不再作为 struct 定义存在（已重命名为 Request）

#### Scenario: AuthorizationUrlBuilder 不被误改
- **WHEN** 检查 `crates/openlark-auth/src/`
- **THEN** `AuthorizationUrlBuilder`（URL builder，非请求类型）保留原样不动

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 13 个 `XxxBuilder` 名 SHALL 作为 `#[deprecated]` type alias 保留（`#[deprecated] pub type XxxBuilder = XxxRequest;`），实现 v0.18→v1.0 软迁移：调用方使用旧名时编译可通过但产生 deprecation warning。v1.0 移除 alias。

#### Scenario: 旧名 type alias 存在且标 deprecated
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub type XxxBuilder`（13 个旧名）
- **THEN** 13 个 type alias 存在，且各自带 `#[deprecated]` 属性

#### Scenario: 旧名调用产生 deprecation warning
- **WHEN** 用旧名 `AppAccessTokenBuilder`（alias）写测试代码并 `cargo build -p openlark-auth`
- **THEN** 构建成功（源码兼容）且产生 `AppAccessTokenBuilder is deprecated` warning

#### Scenario: 新名无 deprecation warning
- **WHEN** 用新名 `AppAccessTokenRequest` 写测试代码并 `cargo build -p openlark-auth`
- **THEN** 构建成功且无该类型的 deprecation warning

### Requirement: 重命名不破坏构建、lint 与测试
本次重命名 + alias SHALL 不导致 workspace 构建、clippy 或测试失败，SHALL 同步更新 auth 内部引用、prelude/re-export、examples、docs、tests 到新名。

#### Scenario: 全 feature 构建通过
- **WHEN** 运行 `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`（deprecation warning 不属 clippy `-D warnings` 抑制范围，alias 自身的 `#[deprecated]` 不在 clippy lint 内）
- **THEN** 三组均 exit 0

#### Scenario: auth 测试通过
- **WHEN** 运行 `cargo test -p openlark-auth`
- **THEN** 全部通过（0 failed；测试用新名，旧名 alias 测试单独覆盖 warning 行为）
