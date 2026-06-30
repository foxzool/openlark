## ADDED Requirements

### Requirement: app_engine 请求 builder 统一 RequestBuilder 后缀
platform app_engine/apaas 子系统 51 个请求类型 builder SHALL 统一 `RequestBuilder` 后缀。

#### Scenario: 51 重命名
- **WHEN** grep `pub struct XxxRequestBuilder`（app_engine 51 类型）
- **THEN** 51 个存在

### Requirement: 旧名 #[deprecated] alias
51 个旧 `XxxBuilder` SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: alias 存在
- **WHEN** grep 51 个 `pub type XxxBuilder =`
- **THEN** 51 个带 `#[deprecated]`

### Requirement: 不破坏 build/clippy/test/fmt
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: 全绿
- **WHEN** build --all-features / clippy×3 / test / fmt --check
- **THEN** 均 exit 0
