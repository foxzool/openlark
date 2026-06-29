# Tasks — remove-deprecated-tenant-token-legacy-chain

> 移除 TenantAccessTokenBuilder 的 3 个 deprecated 方法（app_id/app_secret/app_ticket）+ legacy 两步换取逻辑 + 依赖测试。v0.18 deprecated 清零收尾。BREAKING。

## 1. 移除 legacy 方法与字段

- [x] 1.1 删除 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 的 3 个 deprecated 方法 `app_id`/`app_secret`/`app_ticket`（line 84-103）
- [x] 1.2 删除 `TenantAccessTokenBuilder` 的 3 个 legacy 字段 `legacy_app_id`/`legacy_app_secret`/`legacy_app_ticket`，并从 `new()` 移除其初始化
- [x] 1.3 删除 `LegacyAppAccessTokenBody` 结构体与 `use super::app_access_token::AppAccessTokenResponseData;` import

## 2. 简化 execute 逻辑

- [x] 2.1 移除 `execute_with_options` 内的 legacy 两步换取分支（`if self.app_access_token.is_empty() { ... } else { ... }`），简化为始终用 `self.app_access_token`，前置 `validate_required!(self.app_access_token, "应用访问凭证不能为空")`（去 `.clone()`，move 字段——`validate_required!` 借用语义已验证）

## 3. 测试清理

- [x] 3.1 删除依赖测试 `test_execute_legacy_chain_fetches_app_token_then_tenant_token`（line 285-344，含其 `#[allow(deprecated)]` 标注）
- [x] 3.2 调整 `test_tenant_access_token_builder_new` 断言（删除对 3 个 legacy 字段的 `is_empty()` 断言）

## 4. 验证

- [x] 4.1 `grep -rn '#\[deprecated' crates/ --include='*.rs'` = 0（**全仓清零达成**）；`grep 'LegacyAppAccessTokenBody' <file>` = 0；`grep '#\[deprecated' <file>` = 0；canonical 方法计数 = 2
- [x] 4.2 三组 feature clippy（default/all-features/no-default）`-- -Dwarnings -A missing_docs` 全 exit 0
- [x] 4.3 `cargo test --workspace` 通过（0 failed；canonical 正向测试 `test_execute_sends_app_token_tenant_key_and_no_authorization` 通过）

## 5. CHANGELOG

- [x] 5.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 两步迁移指引（先 `AppAccessTokenBuilder` 取 app_access_token，再传入 `TenantAccessTokenBuilder`）
