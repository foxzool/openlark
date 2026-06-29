## Why

openlark-communication 的 `im/mod.rs` 有个 deprecated `pub mod im { pub use super::project::{v1, v2}; }` 别名（since 0.15.0），造成冗余嵌套路径 `im::im::v1`。新路径是 `im::v1`/`im::v2`（经 `pub use project::{v1, v2}`）。**47 个文件**用旧 `im::im::` 路径，迁移是机械的路径缩短（`im::im::` → `im::`）。

来源：#268 剩余的 F 类（#278 跟踪；A+E+D+C+B 各自分项）。本 change 是已确认拆分项 F。

## What Changes

- **BREAKING**：47 个文件的 `im::im::v1::x` / `im::im::v2::x` 导入 → `im::v1::x` / `im::v2::x`（机械 sed，路径缩短，无语义变化）。
- 删除 `im/mod.rs` 的 `pub mod im { ... }` deprecated 别名块。

## Capabilities

### New Capabilities
- `no-deprecated-im-alias`: openlark-communication SHALL 不保留 `im::im` legacy 嵌套别名；用户/内部代码 SHALL 用 `im::v1`/`im::v2`。

### Modified Capabilities
<!-- 无 -->

## Impact

- **openlark-communication**：47 个 `im/im/**/*.rs` 文件 import 路径 `im::im::` → `im::`；`im/mod.rs` 删除 `pub mod im` 别名块。
- **破坏性**：移除公开 `im::im` 路径别名（since 0.15.0）。外部若用 `openlark_communication::im::im::v1` 需改为 `im::v1`。CHANGELOG breaking + 迁移指引。
- **非目标**：不动 B（wiki Params）；不改 im 的 v1/v2 实现；不重命名模块。
