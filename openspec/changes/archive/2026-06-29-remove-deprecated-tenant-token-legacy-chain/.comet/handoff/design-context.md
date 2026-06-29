# Comet Design Handoff

- Change: remove-deprecated-tenant-token-legacy-chain
- Phase: design
- Mode: compact
- Context hash: 81743668119a1c2a9f21c6266c2c9880d5f53c895cec677259b65d12719cc1d5

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/remove-deprecated-tenant-token-legacy-chain/proposal.md

- Source: openspec/changes/remove-deprecated-tenant-token-legacy-chain/proposal.md
- Lines: 1-24
- SHA256: c4d479c53635fe4d9e92a8799d30e553090aaffc91e8d8aa015380552cd2abd0

```md
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
```

## openspec/changes/remove-deprecated-tenant-token-legacy-chain/design.md

- Source: openspec/changes/remove-deprecated-tenant-token-legacy-chain/design.md
- Lines: 1-43
- SHA256: bb834b067787739bce7a0e0dea98c2d7f271a96f21fea2be158f268cb7b5e404

```md
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
```

## openspec/changes/remove-deprecated-tenant-token-legacy-chain/tasks.md

- Source: openspec/changes/remove-deprecated-tenant-token-legacy-chain/tasks.md
- Lines: 1-28
- SHA256: 3a550711c6f4f48db105aeaa83a32780807d2eb0f7f6e90d8475f995812cfb90

```md
# Tasks — remove-deprecated-tenant-token-legacy-chain

> 移除 TenantAccessTokenBuilder 的 3 个 deprecated 方法（app_id/app_secret/app_ticket）+ legacy 两步换取逻辑 + 依赖测试。v0.18 deprecated 清零收尾。BREAKING。

## 1. 移除 legacy 方法与字段

- [ ] 1.1 删除 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 的 3 个 deprecated 方法 `app_id`/`app_secret`/`app_ticket`（line 84-103）
- [ ] 1.2 删除 `TenantAccessTokenBuilder` 的 3 个 legacy 字段 `legacy_app_id`/`legacy_app_secret`/`legacy_app_ticket`，并从 `new()` 移除其初始化
- [ ] 1.3 删除 `LegacyAppAccessTokenBody` 结构体与 `use super::app_access_token::AppAccessTokenResponseData;` import

## 2. 简化 execute 逻辑

- [ ] 2.1 移除 `execute_with_options` 内的 legacy 两步换取分支（`if self.app_access_token.is_empty() { ... } else { ... }`），简化为始终用 `self.app_access_token`，前置 `validate_required!(self.app_access_token, "应用访问凭证不能为空")`

## 3. 测试清理

- [ ] 3.1 删除依赖测试 `test_execute_legacy_chain_fetches_app_token_then_tenant_token`（line 285-344，含其 `#[allow(deprecated)]` 标注）
- [ ] 3.2 调整 `test_tenant_access_token_builder_new` 断言（删除对 3 个 legacy 字段的 `is_empty()` 断言）

## 4. 验证

- [ ] 4.1 `grep -rn '#\[deprecated' crates/ --include='*.rs'` = 0（全仓清零）；`grep 'LegacyAppAccessTokenBody' <file>` = 0；`grep '#\[deprecated' <file>` = 0
- [ ] 4.2 三组 feature clippy（default/all-features/no-default）`-- -Dwarnings -A missing_docs` 全 exit 0
- [ ] 4.3 `cargo test --workspace` 通过（含 canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization`）

## 5. CHANGELOG

- [ ] 5.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 两步迁移指引（先 `AppAccessTokenBuilder` 取 app_access_token，再传入 `TenantAccessTokenBuilder`）
```

## openspec/changes/remove-deprecated-tenant-token-legacy-chain/specs/no-deprecated-tenant-token-legacy-chain/spec.md

- Source: openspec/changes/remove-deprecated-tenant-token-legacy-chain/specs/no-deprecated-tenant-token-legacy-chain/spec.md
- Lines: 1-35
- SHA256: ae8c318a466468a24de6fc54957e11b8b0ad96d8d930a93f756a5ebd2983979d

```md
## ADDED Requirements

### Requirement: auth 不保留 tenant_access_token legacy 链
openlark-auth SHALL 不保留 `TenantAccessTokenBuilder` 的 deprecated `app_id` / `app_secret` / `app_ticket` 链式入口，SHALL 不保留其驱动的 legacy 两步换取逻辑（app_id/secret/ticket → app_access_token → tenant_access_token）。商店应用 SHALL 直接提供 `app_access_token` + `tenant_key`。

#### Scenario: legacy deprecated 方法移除
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `#\[deprecated`
- **THEN** 命中数为 0（3 个 deprecated 方法 `app_id`/`app_secret`/`app_ticket` 及其 `#[allow(deprecated)]` 测试标注一并移除）

#### Scenario: legacy 两步链移除
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `LegacyAppAccessTokenBody`
- **THEN** 命中数为 0（legacy 结构体与 `execute_with_options` 内的两步换取分支移除；execute 简化为始终用调用方传入的 `app_access_token`）

#### Scenario: canonical 流程保留
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `pub fn app_access_token` 与 `pub fn tenant_key`
- **THEN** 两个 canonical builder 方法均存在（canonical `app_access_token(...).tenant_key(...)` 流程不受影响）

### Requirement: 移除不破坏构建、测试与 canonical 行为
本次移除 SHALL 不导致全仓出现新的 `#[deprecated]`，SHALL 不破坏 canonical tenant_access_token 流程，SHALL 不导致 default/all/no-default clippy 或测试失败。

#### Scenario: 全仓 deprecated 清零
- **WHEN** 在 `crates/` 下递归 grep `#\[deprecated`（`--include='*.rs'`）
- **THEN** 命中数为 0（v0.18 deprecated 清理完成；本 change 是最后一批）

#### Scenario: canonical 流程行为不变
- **WHEN** 运行 `cargo test -p openlark-auth test_execute_sends_app_token_tenant_key_and_no_authorization`
- **THEN** 测试通过（canonical `app_access_token`+`tenant_key` 单步请求行为与请求体不变）

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 三组 feature + `-- -Dwarnings -A missing_docs`
- **THEN** 三组 exit 0

#### Scenario: tests 通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过（0 failed）
```

