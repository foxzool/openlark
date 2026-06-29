# no-deprecated-im-alias Specification

## Purpose
TBD - created by archiving change remove-deprecated-im-alias. Update Purpose after archive.
## Requirements
### Requirement: communication 不保留 im::im legacy 别名
openlark-communication SHALL 不保留 `im::im` 嵌套别名（`pub mod im { ... }`）。用户/内部代码 SHALL 用 `im::v1` / `im::v2`。

#### Scenario: im 别名块移除
- **WHEN** 在 `crates/openlark-communication/src/im/mod.rs` 中 grep 精确模式 `pub mod im\b`（word-boundary；等价 `grep -w 'pub mod im'`）
- **THEN** 命中数为 0（deprecated 别名块 `pub mod im { ... }` 移除）。注意：不能用裸 `grep 'pub mod im'`——它会匹配 `pub mod im_ephemeral` / `pub mod im_message` 前缀导致永远 >0；word-boundary 排除二者。

#### Scenario: 依赖别名测试块移除
- **WHEN** 在 `crates/openlark-communication/src/im/mod.rs` 中 grep `nested_im_path_remains_a_compatibility_alias`
- **THEN** 命中数为 0（引用 `super::im::v1::...` 的兼容性测试随别名一同删除，否则编译失败）

#### Scenario: 内部导入路径迁移
- **WHEN** 在 `crates/openlark-communication/src/` 中 grep `im::im::`
- **THEN** 命中数为 0（47 文件全部改为 `im::v1`/`im::v2`）

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 三组 feature + `-D warnings`
- **THEN** 三组 exit 0

#### Scenario: tests 通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过

