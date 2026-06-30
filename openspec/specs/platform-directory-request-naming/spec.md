# platform-directory-request-naming Specification

## Purpose
TBD - created by archiving change unify-platform-directory-request-naming. Update Purpose after archive.
## Requirements
### Requirement: directory 请求 builder 统一 RequestBuilder 后缀
platform directory 子系统 21 个请求类型 builder SHALL 统一 `RequestBuilder` 后缀。

#### Scenario: 21 重命名
- **WHEN** grep `pub struct XxxRequestBuilder`（directory 21 类型）
- **THEN** 21 个存在

### Requirement: 旧名 #[deprecated] alias
21 个旧 `XxxBuilder` SHALL 作 `#[deprecated] pub type XxxBuilder = XxxRequestBuilder;`（放 `#[cfg(test)]` 前）。

#### Scenario: alias 存在
- **WHEN** grep 21 个 `pub type XxxBuilder =`
- **THEN** 21 个带 `#[deprecated]`

### Requirement: 不破坏 build/clippy/test/fmt
本次重命名 SHALL 不破坏 workspace build/clippy/test/fmt。

#### Scenario: build/clippy/fmt 全绿
- **WHEN** build --all-features / clippy×3 / test / fmt --check
- **THEN** 均 exit 0

