# no-deprecated-tenant-token-legacy-chain Specification

## Purpose
TBD - created by archiving change remove-deprecated-tenant-token-legacy-chain. Update Purpose after archive.
## Requirements
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

