# Brainstorm Summary

- Change: unify-auth-request-naming
- Date: 2026-06-29

## 确认的技术方案

issue #271 命名统一 pilot：openlark-auth 13 个请求类型 `XxxBuilder` → `XxxRequest` + `#[deprecated]` type alias 软迁移。方向（Builder→Request）、pilot-first、deprecation alias 三项已在 open 阶段确认。

实现机制（每类型 3 步，已 spike 验证）：
1. `pub struct XxxBuilder` → `pub struct XxxRequest`；`impl XxxBuilder` → `impl XxxRequest`（方法签名不变）
2. `#[deprecated(note="renamed to XxxRequest, removed in v1.0 (#271)")] pub type XxxBuilder = XxxRequest;`
3. re-export（`auth/mod.rs` + prelude）同时导出新名 + 旧名 alias

特例：`TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（去 Builder，已有 Request，避免双 Request）。

13 个目标：AppAccessToken / AppAccessTokenInternal / AppTicketResend / Authorization / IdentityCreate / OidcAccessToken / OidcRefreshAccessToken / RefreshUserAccessTokenV1 / TenantAccessToken / TenantAccessTokenInternal(特例) / UserAccessTokenV1 / UserInfo / VerificationGet。排除 `AuthorizationUrlBuilder`（URL builder，无 execute，非请求类型）。

## 关键取舍与风险

- **spike 实证（/tmp 一次性实验）**：`#[deprecated] pub type Old = New;` 使用旧名触发 `use of deprecated type alias Old` warning；新名无 warning；alias 仍可调用 New 的方法。机制成立。
- **无 trait impl 冲突**：13 类型均无 `impl Trait for XxxBuilder`（以 AppAccessTokenBuilder 为例核实），type alias 透明继承方法，无兼容性顾虑。
- **软 breaking**：源码兼容（编译通过 + warning），v1.0 移除 alias 才硬 breaking。CHANGELOG v0.18 breaking 段记录。
- **re-export 传播**：`pub use` 带 `#[deprecated]` alias 的 warning 会传播到调用方（spike 证实跨 crate 使用也触发）。
- **公开性**（build 阶段精确化）：13 个 Builder 是否全 pub？非 pub 的无需 alias 直接改名（减少 alias 数量）。
- **pilot 可复用**：验证的模式直接套用到其余 crate（platform 98 / ai 29 / hr 13 / cardkit 10），但量大需单独 change。

## 测试策略

- `cargo build --workspace --all-features` exit 0
- 三组 feature clippy（default / --all-features / --no-default-features + `-D warnings`）均 exit 0（注：`#[deprecated]` alias 的 warning 属 `deprecated` lint，不属 clippy `-D warnings` 抑制范围，但 alias 自身在 clippy 下不触发——build 阶段核实）
- `cargo test -p openlark-auth` 0 failed
- alias 行为实证：用旧名 `AppAccessTokenBuilder` 写临时代码 build → 成功 + deprecation warning；新名 → 成功无 warning；还原
- grep 确认：13 个 `XxxRequest` struct、13 个 `XxxBuilder` type alias 带 `#[deprecated]`、`AuthorizationUrlBuilder` 未动

## Spec Patch

无。delta spec `auth-request-naming`（3 Requirement / 9 Scenario）已覆盖重命名、alias、不破坏构建、alias warning 行为。
