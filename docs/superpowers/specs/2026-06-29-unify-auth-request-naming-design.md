---
comet_change: unify-auth-request-naming
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-29-unify-auth-request-naming
status: final
---

# Design — unify-auth-request-naming（#271 pilot）

> 技术 HOW。需求 WHAT 以 OpenSpec delta spec `openspec/changes/unify-auth-request-naming/specs/auth-request-naming/spec.md` 为 canonical。

> ⚠️ **方向变更（build 阶段）**：open/design 原定 builder「→ Request」，build 实证发现 5/13 目标 `XxxRequest` 名与 `crate::models` 已存在的请求体 body 模型撞名（E0255）。用户确认改为 **builder「→ RequestBuilder」**（body 保持 `XxxRequest`），零撞名 + 对齐 helpdesk。下文「XxxRequest」处均应理解为「XxxRequestBuilder」（除明确指 body 模型外）。本文档其余「→ Request」描述以本说明为准。

## 1. 背景与目标

issue #271（架构审计立项「Request vs Builder 命名统一」）：请求类型命名跨 crate 不一致——模式 A（platform/ai/auth/hr）请求类型用 `Builder` 后缀，模式 B（docs/communication）用 `Request`，模式 C（helpdesk）用 `XxxRequest`+`XxxRequestBuilder` 分离。全局 1670 个已是 `Request`，仅约 167 个模式 A 用 `Builder`。

**目标**：pilot 统一 **openlark-auth** 的 13 个请求类型 `Builder`→`Request`，确立 `#[deprecated]` type-alias 软迁移机制，验证后套用到其余 crate。

**非目标**：不动其他 crate / core 真·builder / helpdesk RequestBuilder / auth 的 `AuthorizationUrlBuilder`（URL builder）。

## 2. 关键技术验证（spike + 代码核实）

| 断言 | 验证方式 | 结论 |
|------|---------|------|
| `#[deprecated] type alias` 触发 warning | /tmp 一次性 cargo spike：`#[deprecated] pub type Old = New;` 用 Old → `use of deprecated type alias Old` warning；用 New → 无 warning | ✅ 机制成立 |
| alias 仍可调用方法 | spike：`Old::new()` 经 alias 解析到 New 的方法，正常调用 + warning | ✅ |
| 无 trait impl 冲突 | grep `impl.*for XxxBuilder` auth → 13 类型均无 trait impl（AppAccessTokenBuilder 核实） | ✅ alias 透明继承方法 |
| re-export 传播 warning | spike 跨 crate 使用 alias → warning 触发 | ✅ |
| `AuthorizationUrlBuilder` 非请求类型 | grep 无 `execute` | ✅ 排除 |

## 3. 实现步骤（每类型 3 步 × 13）

### 3.1 重命名 struct + impl
对 13 个请求类型，`pub struct XxxBuilder` → `pub struct XxxRequest`，`impl XxxBuilder` → `impl XxxRequest`。方法签名/字段/逻辑不变。
- `AppAccessTokenBuilder` → `AppAccessTokenRequest`（`auth/auth/v3/auth/app_access_token.rs:28`）
- 其余 12 个同理（AppAccessTokenInternal/AppTicketResend/Authorization/IdentityCreate/OidcAccessToken/OidcRefreshAccessToken/RefreshUserAccessTokenV1/TenantAccessToken/UserAccessTokenV1/UserInfo/VerificationGet）
- **特例**：`TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（去 `Builder`，已有 `Request`）

### 3.2 添加 #[deprecated] type alias
每个原文件末尾（或集中 aliases 模块）：
```rust
#[deprecated(note = "renamed to XxxRequest, will be removed in v1.0 (#271)")]
pub type XxxBuilder = XxxRequest;
```
保留旧名源码兼容，调用方编译通过 + warning。

### 3.3 同步 re-export
`crates/openlark-auth/src/auth/mod.rs`（及 prelude 若有）的 `pub use` 列表：新名 `XxxRequest` + 旧名 `XxxBuilder`（alias）同时导出。根 crate `openlark::` 未直接再导出这些类型（核实无需改根 crate）。

### 3.4 examples/docs/tests
auth 相关 examples/docs/tests 更新到新名。为 alias 的 deprecation warning 行为加针对性测试（`#[allow(deprecated)]` 标注旧名测试，避免污染 `cargo test` 输出）。

### 3.5 CHANGELOG
v0.18 Breaking Changes 段记录 13 个重命名 + `#[deprecated]` alias + v1.0 移除指引。

## 4. 测试策略

- `cargo build --workspace --all-features` exit 0
- 三组 feature clippy（default / --all-features / --no-default-features + `-D warnings`）均 exit 0
- `cargo test -p openlark-auth` 0 failed
- alias 行为：旧名 build 成功 + warning（`#[allow(deprecated)]` 测试），新名成功无 warning
- grep：13 `XxxRequest` struct + 13 `XxxBuilder` deprecated alias + `AuthorizationUrlBuilder` 未动

## 5. 风险与回滚

- **公开性精确化**（build 阶段）：13 个 Builder 是否全 pub？非 pub 直接改名省 alias。
- **clippy 与 deprecated lint**：`#[deprecated]` alias 在 clippy `-D warnings` 下不触发（deprecated 属独立 lint，非 clippy）；build 阶段核实三组 clippy 干净。
- 非破坏性（软）：alias 保证源码兼容。回滚 = revert 单 commit。v1.0 移除 alias 时才硬 breaking。

## 6. pilot 可复用性

验证的模式（struct+impl 重命名 + #[deprecated] alias + re-export 同步）直接套用到其余 crate。后续 change 建议：
- platform（98）：单独 change，可能再分批（如 app_engine / directory / admin 各一批）
- ai（29）/ hr（13）/ cardkit（10）：各一个 change
- application（3）/ docs（2）：合并一个小 change
