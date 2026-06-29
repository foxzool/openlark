# Comet Design Handoff

- Change: remove-deprecated-im-alias
- Phase: design
- Mode: compact
- Context hash: 0eba5ae6817e9656598194def15ff7732acb708e127a7d5507e0721b455ebe5a

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/remove-deprecated-im-alias/proposal.md

- Source: openspec/changes/remove-deprecated-im-alias/proposal.md
- Lines: 1-24
- SHA256: 1b62451f39d6cd4c88d54ec4e7b0887d6ce9c2d8c10a7704a2ce488f55a7b9d7

```md
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
```

## openspec/changes/remove-deprecated-im-alias/design.md

- Source: openspec/changes/remove-deprecated-im-alias/design.md
- Lines: 1-37
- SHA256: 4c433bc71556fdb625879c52dbd697358df9600f10ae0aff7d6e42a8e86e90f4

```md
## Context

`im/mod.rs` 的 `pub mod im { pub use super::project::{v1,v2}; }` 是 deprecated 别名（since 0.15.0），造成 47 文件用冗余 `im::im::` 路径。新路径 `im::v1`/`im::v2` 已存在（`pub use project::{v1,v2}`）。迁移是**路径缩短**，机械、无语义变化。

```
im::im::v1::x  →  im::v1::x      （去掉冗余的 im:: 别名层）
im::im::v2::x  →  im::v2::x
```

## Goals / Non-Goals

**Goals:** 47 文件 `im::im::` → `im::` 迁移；删除 `pub mod im` 别名块。

**Non-Goals:** 不动 B（wiki Params）；不改 im v1/v2 实现；不重命名模块；不动 `mod project`（实际模块）。

## Decisions

**D1（迁移方式）**：`sed 's/im::im::/im::/g'` 替换 47 文件中的导入路径（机械、无语义变化），然后删除 `pub mod im { ... }` 别名块（im/mod.rs:17-20）。`mod project`（实际模块，`#[path="im/mod.rs"]`）保留。

**D2（外部 breaking）**：`openlark_communication::im::im::v1` 外部路径移除。CHANGELOG 指引改 `im::v1`。

## Risks

- **[Breaking]** `im::im` 路径移除 → 外部编译失败；缓解：CHANGELOG 迁移（`im::im::` → `im::`，drop-in）。
- **[sed 误替换]** `im::im::` 模式可能匹配非导入场景？— 实证 grep 仅命中 `use ... im::im::` 导入，无歧义。build 阶段 clippy 验证。

## Migration Plan

1. `sed -i '' 's/im::im::/im::/g'` 替换 47 文件。
2. 删除 `im/mod.rs` 的 `pub mod im { ... }` 别名块 + `#[allow(module_inception)]`。
3. 三组 clippy + test。
4. CHANGELOG breaking + 迁移。
5. 回滚：git revert。

## Open Questions

- 无（路径缩短是确定的；grep 确认无歧义）。
```

## openspec/changes/remove-deprecated-im-alias/tasks.md

- Source: openspec/changes/remove-deprecated-im-alias/tasks.md
- Lines: 1-22
- SHA256: 0de189243d92f59a91c6821e65af0a9691187c6144452a534c01ce9259f5c755

```md
# Tasks — remove-deprecated-im-alias

> 已确认拆分项 F（#268 剩余）。47 文件 `im::im::` → `im::` 路径缩短 + 删除 `pub mod im` 别名。BREAKING。

## 1. 迁移 47 文件导入路径

- [ ] 1.1 `sed -i '' 's/im::im::/im::/g'` 替换 `crates/openlark-communication/src/` 下所有命中文件（47 个）
- [ ] 1.2 验证 `grep -rn 'im::im::' crates/openlark-communication/src/` = 0

## 2. 删除 im 别名块

- [ ] 2.1 删除 `crates/openlark-communication/src/im/mod.rs` 的 `#[allow(clippy::module_inception)] #[deprecated(...)] /// 兼容历史... pub mod im { pub use super::project::{v1, v2}; }` 整块（保留 `mod project`/`pub use project::{v1,v2}`/其它 pub mod）

## 3. 验证

- [ ] 3.1 `pub mod im` grep im/mod.rs = 0；`im::im::` 全 crate = 0
- [ ] 3.2 三组 feature clippy（default/all-features/no-default）`-D warnings` exit 0
- [ ] 3.3 `cargo test --workspace` 通过

## 4. CHANGELOG

- [ ] 4.1 CHANGELOG `[Unreleased] > Breaking Changes` 加条目 + 迁移映射（`im::im::` → `im::`）
```

## openspec/changes/remove-deprecated-im-alias/specs/no-deprecated-im-alias/spec.md

- Source: openspec/changes/remove-deprecated-im-alias/specs/no-deprecated-im-alias/spec.md
- Lines: 1-27
- SHA256: 112c9fdf607e643eac0006f5cc5d44c3b9b07549f0482bd50313183bcec818dd

```md
## ADDED Requirements

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
```

