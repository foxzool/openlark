# auth-request-naming Specification

## Purpose
TBD - created by archiving change unify-auth-request-naming. Update Purpose after archive.
## Requirements
### Requirement: auth 请求类型统一使用 RequestBuilder 后缀
openlark-auth 的 API 请求 builder 类型 SHALL 统一使用 `RequestBuilder` 后缀命名（对齐 helpdesk 既有 `XxxRequest` body + `XxxRequestBuilder` builder 分离模式）。12 个原 `XxxBuilder`（无 `Request`）请求类型 SHALL 重命名为规范 `XxxRequestBuilder`；`TenantAccessTokenInternalRequestBuilder` 已是目标形式，保持不动。

> **方向变更说明**：open/design 阶段原定「→ Request」，build 阶段发现 5/13 目标 `XxxRequest` 名与 `crate::models::auth`/`models::authen` 已存在的请求体 body 模型 `XxxRequest` 撞名（E0255）。用户确认改为「→ RequestBuilder」：body 模型保持 `XxxRequest`，builder 统一 `XxxRequestBuilder`，零撞名且对齐 helpdesk（47 类型已是此模式）。

#### Scenario: 12 个请求类型重命名为 RequestBuilder
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub struct XxxRequestBuilder`（针对 AppAccessToken/AppAccessTokenInternal/AppTicketResend/Authorization/IdentityCreate/OidcAccessToken/OidcRefreshAccessToken/RefreshUserAccessTokenV1/TenantAccessToken/UserAccessTokenV1/UserInfo/VerificationGet 各 `RequestBuilder`）
- **THEN** 12 个新 `XxxRequestBuilder` struct 存在；`TenantAccessTokenInternalRequestBuilder` 原样保留（共 13 个 RequestBuilder）

#### Scenario: auth 不再有裸 Builder 请求类型
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep 请求类型的 `XxxBuilder`（排除 `AuthorizationUrlBuilder`）
- **THEN** 12 个旧 `XxxBuilder` 名不再作为 struct 定义（已重命名为 RequestBuilder）

#### Scenario: AuthorizationUrlBuilder 不被误改
- **WHEN** 检查 `crates/openlark-auth/src/models/oauth/mod.rs`
- **THEN** `AuthorizationUrlBuilder`（URL builder，非请求类型）保留原样

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 12 个 `XxxBuilder` 名 SHALL 作为 `#[deprecated]` type alias 保留（`#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`），实现 v0.18→v1.0 软迁移：调用方使用旧名编译可通过但产生 deprecation warning。v1.0 移除 alias。

#### Scenario: 旧名 type alias 存在且标 deprecated
- **WHEN** 在 `crates/openlark-auth/src/` 中 grep `pub type XxxBuilder`（12 个旧名）
- **THEN** 12 个 type alias 存在，各自带 `#[deprecated]` 属性

#### Scenario: 旧名调用产生 deprecation warning
- **WHEN** 用旧名 `AppAccessTokenBuilder`（alias）写测试代码并 `cargo build -p openlark-auth`
- **THEN** 构建成功（源码兼容）且产生 deprecation warning

#### Scenario: 新名无 deprecation warning
- **WHEN** 用新名 `AppAccessTokenRequestBuilder` 写测试代码并 build
- **THEN** 构建成功且无该类型 deprecation warning

### Requirement: 重命名不破坏构建、lint 与测试
本次重命名 + alias SHALL 不导致 workspace 构建、clippy 或测试失败，SHALL 同步更新 auth 内部引用、service 方法返回类型、re-export 链、examples/docs/tests 到新名。

#### Scenario: 全 feature 构建通过
- **WHEN** 运行 `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: auth 测试通过
- **WHEN** 运行 `cargo test -p openlark-auth`
- **THEN** 全部通过（0 failed）

