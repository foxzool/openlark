## ADDED Requirements

### Requirement: auth TenantAccessTokenBuilder legacy 方法保留（functional）
openlark-auth 的 `TenantAccessTokenBuilder` 的 `app_id()` / `app_secret()` / `app_ticket()` deprecated 方法 **SHALL 保留**——它们喂 functional legacy two-step flow（`execute_with_options` 读取 `legacy_app_id/secret/ticket` 字段，`test_execute_legacy_chain` 验证），**非 unused**。移除该 legacy flow 是独立的更大改动（见 #278）。本 change 不动 auth。

#### Scenario: auth legacy 方法保留
- **WHEN** 在 `crates/openlark-auth/src/auth/auth/v3/auth/tenant_access_token.rs` 中 grep `pub fn app_id|pub fn app_secret|pub fn app_ticket`
- **THEN** 命中数为 3（functional legacy 方法保留，不移除）

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
