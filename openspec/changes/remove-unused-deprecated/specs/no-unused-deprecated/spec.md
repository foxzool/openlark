## ADDED Requirements

### Requirement: auth 不暴露 deprecated TenantAccessTokenBuilder 方法
openlark-auth 的 `TenantAccessTokenBuilder` SHALL 不再提供 `app_id()` / `app_secret()` / `app_ticket()` deprecated 方法。用户 SHALL 经 `app_access_token()` + `tenant_key()` 流程。

#### Scenario: auth 无目标 deprecated 方法
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `pub fn app_id|pub fn app_secret|pub fn app_ticket`
- **THEN** 命中数为 0（3 个 deprecated 方法移除）

### Requirement: docs 不暴露 deprecated to_value 与宏 new
openlark-docs SHALL 不再提供 `RecordFieldValue::to_value()` 与 `impl_required_builder!` 宏生成的 `new()`。

#### Scenario: docs to_value 移除
- **WHEN** 在 `crates/openlark-docs/src/base/bitable/v1/field_types.rs` 中 grep `pub fn to_value`
- **THEN** 命中数为 0

#### Scenario: 宏不再生成 new()
- **WHEN** 在 `crates/openlark-docs/src/common/request_builder.rs` 的 `impl_required_builder!` 宏中 grep `pub fn new`
- **THEN** 命中数为 0（宏的 new() 生成块移除）

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default 任一 feature 组合 clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: examples/tests 不引用已移除项
- **WHEN** 在 `examples/` 与 `tests/` 中 grep `.to_value()` 与 `tenant_access_token().app_id/app_secret/app_ticket`
- **THEN** 命中数为 0
