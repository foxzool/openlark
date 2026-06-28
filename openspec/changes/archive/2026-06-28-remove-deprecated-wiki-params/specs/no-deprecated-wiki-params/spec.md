## ADDED Requirements

### Requirement: wiki 模块不保留 deprecated Params struct
openlark-docs wiki 模块 SHALL 不保留 `SearchWikiParams`/`ListWikiSpacesParams`/`CreateWikiSpaceParams`/`MoveDocsToWikiParams` deprecated struct。用户 SHALL 使用对应 `XxxRequest` 流式 Builder。

#### Scenario: 4 个 Params struct 移除
- **WHEN** 在 `crates/openlark-docs/src/ccm/wiki/` 中 grep `pub struct SearchWikiParams|pub struct ListWikiSpacesParams|pub struct CreateWikiSpaceParams|pub struct MoveDocsToWikiParams`
- **THEN** 命中数为 0（4 个 deprecated Params 全部移除）

#### Scenario: 用法迁移到 Builder
- **WHEN** 迁移后以 default feature 构建 openlark-docs
- **THEN** 原 Params 用法改为 `XxxRequest::builder()...`，编译通过

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 三组 feature + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: tests 通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过
