## ADDED Requirements

### Requirement: 跨 crate 共享依赖 MUST 经 workspace 统一治理

飞书 SDK workspace 内被 2 个及以上 crate 消费的第三方依赖 SHALL 在根 `Cargo.toml` 的 `[workspace.dependencies]` 段统一声明版本，各 crate 通过 `{ workspace = true }` 消费，不得在 crate 级 `Cargo.toml` 自钉版本。此约束防止同一依赖出现多个 resolved 版本（cargo tree 多版本共存），保证 workspace 级版本一致性。

#### Scenario: 多 crate 共享依赖走 workspace

- **WHEN** 一个第三方依赖被 workspace 内 ≥2 个 crate 消费（如 `prost` 被 client/core/protocol 消费）
- **THEN** 该依赖 MUST 在根 `Cargo.toml [workspace.dependencies]` 声明版本，每个消费 crate 的 `Cargo.toml` 用 `{ workspace = true }` 引用；MUST NOT 在任何 crate 级 `Cargo.toml` 出现自钉版本号（如 `prost = "0.13.1"`）

#### Scenario: 单 crate 专用依赖鼓励走 workspace

- **WHEN** 一个第三方依赖当前仅被 1 个 crate 消费但属于通用基础库（如 `bytes` 仅 protocol 用）
- **THEN** 该依赖 SHOULD 也经 `[workspace.dependencies]` 声明并 `{ workspace = true }` 消费，便于未来跨 crate 复用时统一版本、避免日后迁移成本

### Requirement: 依赖声明一致性（无 crate 级钉版本绕过）

已在 `[workspace.dependencies]` 声明的依赖，消费 crate MUST 以 `{ workspace = true }` 引用，不得在 crate 级重复钉版本。

#### Scenario: openlark-protocol 的 bytes/prost 走 workspace

- **WHEN** 检查 `crates/openlark-protocol/Cargo.toml` 的 `bytes` 与 `prost` 依赖声明
- **THEN** 两者 MUST 为 `{ workspace = true }` 形态；MUST NOT 出现 `bytes = "1.6.0"` 或 `prost = "0.13.1"` 这类 crate 级钉版本；`bytes` MUST 已在根 `Cargo.toml [workspace.dependencies]` 声明

#### Scenario: 迁移不引入新多版本

- **WHEN** 对比本迁移前后的 `cargo tree -d --workspace` 输出
- **THEN** `bytes` 与 `prost` 的重复条目集合 MUST 为迁移前的子集（即本迁移 MUST NOT 引入 `bytes`/`prost` 的**新**多版本；`bytes` 仍为单一 resolved 版本）
- **边界**：`prost` 当前已存在 `0.13.5`（runtime）与 `0.12.6`（由 vendored `lark-websocket-protobuf` 的 `prost-build 0.12.6` build-dep 间接引入）的既存 split，属独立问题，不在本 capability 范围；本约束仅要求「迁移行为不新增多版本」，不宣称消除该既存 split
