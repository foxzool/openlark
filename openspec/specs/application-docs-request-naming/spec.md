# application-docs-request-naming Specification

## Purpose
TBD - created by archiving change unify-application-docs-request-naming. Update Purpose after archive.
## Requirements
### Requirement: application+docs 请求 builder 统一 RequestBuilder 后缀
openlark-application 与 openlark-docs 的请求类型 builder SHALL 统一使用 `RequestBuilder` 后缀（对齐 auth pilot 与 helpdesk 模式）。4 个原 `XxxBuilder`（均撞 body 模型 `XxxRequest`）SHALL 重命名为 `XxxRequestBuilder`。

#### Scenario: 4 个请求类型重命名为 RequestBuilder
- **WHEN** grep `pub struct XxxRequestBuilder`（AccessDataSearchBlock/AccessDataSearchCustom/AccessDataSearchWorkplace/PatchFormFieldQuestion）
- **THEN** 4 个新 `XxxRequestBuilder` struct 存在

#### Scenario: RecordFieldsBuilder 不被误改
- **WHEN** 检查 `crates/openlark-docs/src/base/bitable/v1/field_types.rs`
- **THEN** `RecordFieldsBuilder`（真 builder，无 execute）保留原样

### Requirement: 旧 Builder 名作 #[deprecated] type alias 保留
原 4 个 `XxxBuilder` 名 SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（v0.18→v1.0 软迁移）。alias 放 `#[cfg(test)]` 前（避免 clippy items_after_test_module）。

#### Scenario: 旧名 alias 存在且标 deprecated
- **WHEN** grep `pub type XxxBuilder`（4 个旧名）
- **THEN** 4 个 type alias 存在，各自带 `#[deprecated]`

#### Scenario: alias 放在 test 模块前
- **WHEN** 检查 4 个定义文件的 alias 与 `#[cfg(test)]` 顺序
- **THEN** alias 在 `#[cfg(test)]` 之前（不触发 clippy items_after_test_module）

### Requirement: 重命名不破坏构建、lint 与测试
本次重命名 + alias SHALL 不破坏 workspace 构建/clippy/测试，SHALL 同步 docs 的 PatchFormFieldQuestion re-export。

#### Scenario: 全 feature 构建通过
- **WHEN** `cargo build --workspace --all-features`
- **THEN** exit 0

#### Scenario: 三组 feature clippy 通过
- **WHEN** clippy default / --all-features / --no-default-features + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: application+docs 测试通过
- **WHEN** `cargo test -p openlark-application -p openlark-docs`
- **THEN** 0 failed

#### Scenario: fmt 通过
- **WHEN** `cargo fmt --all -- --check`
- **THEN** exit 0

