# Tasks — unify-auth-request-naming（#271 pilot）

> issue #271 命名统一 pilot：openlark-auth 请求类型 builder 统一 `XxxRequestBuilder`（body 模型保持 `XxxRequest`）+ `#[deprecated]` type alias 软迁移。
> **方向变更**：open/design 原「→Request」，build 阶段发现 5/13 撞 body 模型名（E0255），用户确认改「→RequestBuilder」（零撞名 + 对齐 helpdesk）。
> 12 个 `XxxBuilder` → `XxxRequestBuilder`；`TenantAccessTokenInternalRequestBuilder` 已是目标形式不动。

## 1. 重命名 + alias + re-export + service 方法（按子系统批）

- [x] 1.1 v3 auth 批（AppAccessToken/AppAccessTokenInternal/AppTicketResend/TenantAccessToken 4 个 → RequestBuilder；TenantAccessTokenInternalRequestBuilder 已符合不动）：struct+impl+测试重命名 + `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;` + AuthServiceV3 4 方法返回类型 + 2 层 re-export 双块（新名 + `#[allow(deprecated)]` 旧名 alias）
- [x] 1.2 authen v1 批（UserAccessTokenV1/RefreshUserAccessTokenV1/OidcAccessToken/OidcRefreshAccessToken/UserInfo 5 个 → RequestBuilder）：同模式 + AuthenServiceV1 2 方法 + OidcService 2 方法 + 3 层 re-export
- [x] 1.3 oauth/human_auth/verification 批（Authorization/IdentityCreate/VerificationGet 3 个 → RequestBuilder）：同模式；Authorization 3 层 re-export 链；VerificationGet 无 re-export

## 2. alias deprecation 测试

- [x] 2.1 app_access_token.rs 追加 alias 可调用测试（`#[allow(deprecated)]`）+ 新名无 warning 测试

## 3. CHANGELOG

- [x] 3.1 CHANGELOG v0.18 Breaking Changes 段记录 12 个重命名（→RequestBuilder）+ `#[deprecated]` alias + body 模型不动 + 撞名发现说明 + v1.0 移除指引

## 4. 全量验证

- [ ] 4.1 `cargo build --workspace --all-features` exit 0
- [ ] 4.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 4.3 `cargo test -p openlark-auth` 全部通过（0 failed）
- [ ] 4.4 alias warning 实证（旧名 → 成功+warning；新名 → 成功无 warning）+ grep 确认 12 RequestBuilder struct + 12 deprecated alias + AuthorizationUrlBuilder 未动
