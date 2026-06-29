## Context

`TenantAccessTokenBuilder`（商店应用获取 tenant_access_token）当前有两条执行路径：

- **canonical**：`new(config).app_access_token(..).tenant_key(..).execute()`——调用方直接提供 `app_access_token`。
- **legacy**（deprecated）：`new(config).app_id(..).app_secret(..).app_ticket(..).tenant_key(..).execute()`——`app_access_token` 为空时，先用 app_id/secret/ticket 调 `/auth/v3/app_access_token` 换取 `app_access_token`，再换 `tenant_access_token`。

3 个 deprecated 方法（`app_id`/`app_secret`/`app_ticket`）仅"用于编译兼容"，deprecation note 已指引改用 canonical 流程。这是 v0.18 deprecated 清理最后一批。

## Goals / Non-Goals

**Goals:** 移除 3 个 deprecated 方法 + legacy 两步换取逻辑 + 配套字段/结构体/import/测试；`execute` 简化为始终用调用方传入的 `app_access_token`；全仓 `#[deprecated]` 清零。

**Non-Goals:** 不动 `TenantAccessTokenInternalRequestBuilder`（独立 builder，内部应用标准流程，非 deprecated）；不动 `AppAccessTokenBuilder`（合法 `app_ticket`）；不动 `Config::builder()` 的 `app_id`/`app_secret`；不改 canonical 流程的网络行为/请求体。

## Decisions

**D1（移除范围）**：整块删除 legacy 路径——3 方法（line 84-103）+ 3 legacy 字段（`legacy_app_id/secret/ticket`）+ `LegacyAppAccessTokenBody` 结构体 + `AppAccessTokenResponseData` import + `execute_with_options` 内 `if self.app_access_token.is_empty() { ... legacy 两步 ... } else { ... }` 分支（简化为直接用 `self.app_access_token`，前置 `validate_required!(self.app_access_token, ...)`）+ legacy 测试。`new()` 不再初始化 legacy 字段。

**D2（execute 简化）**：移除 legacy 分支后，`execute_with_options` 不再需要 `AppAccessTokenResponseData` 与二次 HTTP 请求；请求体直接 `{app_access_token, tenant_key}`。canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization` 覆盖此路径，行为不变。

**D3（外部 breaking）**：移除公开 `app_id/app_secret/app_ticket` 及两步链。CHANGELOG 指引两步迁移：先 `AppAccessTokenBuilder` 取 `app_access_token`，再传入 `TenantAccessTokenBuilder`。

**D4（无 since 版本）**：原 `#[deprecated(note=...)]` 无 `since` 字段（不同于 im-alias 的 since 0.15.0）。CHANGELOG 条目仅注明 "deprecated legacy chain"，不引用具体弃用版本。

## Risks / Trade-offs

- **[Breaking]** 外部用 legacy 链的代码编译失败 → CHANGELOG 两步迁移指引（drop-in：先取 app_access_token 再传入）。
- **[行为移除]** legacy 链是真实两步换取能力，非死代码 → 但已被 canonical 流程覆盖（canonical 等价于 legacy 第二步；第一步改由用户显式完成）。可接受。
- **[测试断言]** `test_tenant_access_token_builder_new` 断言 legacy 字段为空 → 删字段后须同步删除这些断言行。
- 回滚：`git revert`。

## Migration Plan

1. 删 3 方法 + legacy 字段 + `LegacyAppAccessTokenBody` + import。
2. 简化 `execute_with_options`（移除 legacy 分支，加 `app_access_token` required 校验）。
3. 删 legacy 测试 + 调整 `new()`/`builder_new` 断言。
4. 三组 clippy + `cargo test --workspace`。
5. CHANGELOG breaking + 两步迁移。

## Open Questions

- 无（移除范围已 grounded 确认；唯一调用 legacy 方法的点在将被删除的测试内）。
