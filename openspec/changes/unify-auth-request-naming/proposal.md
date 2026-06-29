## Why

issue #271（2026-06 架构审计立项「Request vs Builder 命名统一」）：请求类型命名跨 crate 不一致——模式 A（platform/ai/auth/hr）的请求类型用 `Builder` 后缀（如 `AppAccessTokenBuilder`），模式 B（docs/communication）用 `Request` 后缀（如 `ListClassificationRequest`），模式 C（helpdesk）用 `XxxRequest` + `XxxRequestBuilder` 分离模式。用户需记忆不同 crate 的命名惯例，SDK 表面不一致。

本 change 是 **pilot**：只统一 **openlark-auth** 的请求类型命名（`Builder` → `Request`），并确立 `#[deprecated]` type-alias 迁移机制（v0.18→v1.0 渐进 breaking）。验证模式后，其余 crate（platform 98 / ai 29 / hr 13 / cardkit 10 等）按批次另开 change。全局统计：229 个 `Builder` 中约 167 个是模式 A 请求类型，但 1670 个已是 `Request`——方向 `Builder → Request` 有数据支撑。

## What Changes

> ⚠️ **方向变更（build 阶段）**：原定「→ `XxxRequest`」，build 实证发现 5/13 与 `crate::models` 的请求体 body 模型 `XxxRequest` 撞名（E0255）。改为「**→ `XxxRequestBuilder`**」（body 保持 `XxxRequest`）。下文「→ XxxRequest」均指「→ XxxRequestBuilder」，`TenantAccessTokenInternalRequestBuilder` 已是目标形式（不动）。零撞名 + 对齐 helpdesk。

- 将 openlark-auth 的 **12 个请求类型 `XxxBuilder` 重命名为规范 `XxxRequestBuilder`**（`TenantAccessTokenInternalRequestBuilder` 已符合不动；经核实均有 `execute()`，确为请求类型）：
  - `AppAccessTokenBuilder` → `AppAccessTokenRequest`
  - `AppAccessTokenInternalBuilder` → `AppAccessTokenInternalRequest`
  - `AppTicketResendBuilder` → `AppTicketResendRequest`
  - `AuthorizationBuilder` → `AuthorizationRequest`
  - `IdentityCreateBuilder` → `IdentityCreateRequest`
  - `OidcAccessTokenBuilder` → `OidcAccessTokenRequest`
  - `OidcRefreshAccessTokenBuilder` → `OidcRefreshAccessTokenRequest`
  - `RefreshUserAccessTokenV1Builder` → `RefreshUserAccessTokenV1Request`
  - `TenantAccessTokenBuilder` → `TenantAccessTokenRequest`
  - `TenantAccessTokenInternalRequestBuilder` → `TenantAccessTokenInternalRequest`（已有 `Request`，仅去 `Builder`，避免双 `Request`）
  - `UserAccessTokenV1Builder` → `UserAccessTokenV1Request`
  - `UserInfoBuilder` → `UserInfoRequest`
  - `VerificationGetBuilder` → `VerificationGetRequest`
- **保留旧名作为 `#[deprecated]` type alias**（`pub type XxxBuilder = XxxRequest;` + `#[deprecated(note="...")]`），实现 v0.18→v1.0 渐进迁移：调用方仍可编译，但用旧名会有 deprecation warning。
- 同步更新 auth 内部引用、prelude/re-export、examples、docs、tests 到新名。
- **BREAKING**（软）：公开类型重命名；但通过 `#[deprecated]` alias 保持源码兼容（仅 warning，非硬破坏），v1.0 移除 alias。

## Capabilities

### New Capabilities
- `auth-request-naming`: openlark-auth 的请求类型 SHALL 统一使用 `Request` 后缀；旧 `Builder` 名 SHALL 作为 `#[deprecated]` type alias 保留至 v1.0，调用方使用旧名时编译可通过但产生 deprecation warning。

### Modified Capabilities
<!-- 无（无既有 auth 命名相关 main spec） -->

## Impact

- **openlark-auth**：13 个请求类型 struct 重命名 + 13 个 `#[deprecated]` type alias + 内部引用/prelude 同步 + 模块路径调整。
- **examples/docs/tests**：auth 相关引用更新到新名（旧名仍可编译但有 warning，可渐进）。
- **破坏性**：软 breaking——公开类型重命名，但 `#[deprecated]` type alias 保证源码兼容（编译通过 + warning）。二进制兼容性不受影响（type alias 是零成本）。v1.0 移除 alias 时变硬 breaking。
- **非目标**：不动其他 crate（platform/ai/hr/cardkit 等，后续 change）；不动 core 真·builder（`ConfigBuilder`/`HeaderBuilder`/`MultipartBuilder`/...）；不动 helpdesk `XxxRequestBuilder`（已是 Request 模式）；不动 auth 的 `AuthorizationUrlBuilder`（无 `execute()`，是 URL builder 非请求类型）。
