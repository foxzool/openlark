## Why

`openlark-auth` 的 `TenantAccessTokenBuilder` 仍保留 3 个 deprecated 方法（`app_id`/`app_secret`/`app_ticket`，无 `since` 版本），驱动一条 legacy 两步换取链（app_id/secret/ticket → app_access_token → tenant_access_token）。canonical 流程已是 `app_access_token(...).tenant_key(...)`。这是 v0.18 deprecated 清理的**最后一批**——前 6 个 change（config / docs-deprecated / accessors / im-alias / wiki-params / unused-deprecated）已归档，本 change 完成后全仓 `#[deprecated]` 清零。

## What Changes

- **BREAKING**：移除 `TenantAccessTokenBuilder` 的 3 个 deprecated 方法 `app_id` / `app_secret` / `app_ticket`（`crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs:84-103`）。
- 移除其驱动的 legacy 两步换取逻辑（`execute_with_options` 内 `app_access_token.is_empty()` 分支）。
- 移除配套的 `legacy_app_id`/`legacy_app_secret`/`legacy_app_ticket` 字段、`LegacyAppAccessTokenBody` 结构体、`AppAccessTokenResponseData` import、依赖测试 `test_execute_legacy_chain_fetches_app_token_then_tenant_token`。
- `execute_with_options` 简化为始终使用调用方直接传入的 `app_access_token`（canonical 流程，行为不变）。

## Capabilities

### New Capabilities
- `no-deprecated-tenant-token-legacy-chain`: openlark-auth SHALL 不保留 `TenantAccessTokenBuilder` 的 legacy app_id/app_secret/app_ticket 链式入口及其两步换取逻辑；商店应用 SHALL 直接提供 `app_access_token` + `tenant_key`。

### Modified Capabilities
<!-- 无（无既有 auth 相关 main spec） -->

## Impact

- **openlark-auth**：单文件 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs`——删 3 方法 + legacy 字段 + legacy 结构体 + import + execute 内 legacy 分支 + legacy 测试；调整 `new()` 与 `test_tenant_access_token_builder_new` 断言。
- **破坏性**：移除公开 `TenantAccessTokenBuilder::app_id/app_secret/app_ticket` 及其驱动的两步换取。外部若用 `.app_id(..).app_secret(..).app_ticket(..)` 链需改为：先 `AppAccessTokenBuilder` 取 `app_access_token`，再 `TenantAccessTokenBuilder::new(config).app_access_token(..).tenant_key(..)`。CHANGELOG breaking + 两步迁移指引。
- **非目标**：不动 `TenantAccessTokenInternalRequestBuilder`（独立 builder，非 deprecated，内部应用标准流程）；不动 `AppAccessTokenBuilder`（合法拥有 `app_ticket`）；不动 `Config::builder()` 的 `app_id`/`app_secret`；不改 canonical 流程行为。
