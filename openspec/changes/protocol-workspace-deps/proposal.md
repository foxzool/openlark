## Why

issue #273 Part B 指出：`openlark-protocol` 的 `bytes`/`prost` 依赖在 crate 级直接钉版本（`bytes = "1.6.0"`、`prost = "0.13.1"`），未走 workspace 的 `[workspace.dependencies]`，与其余 crate 不一致（`openlark-core` 已有 38 个依赖走 `workspace = true`）。

风险：同一依赖在 protocol 钉死版本、在其他 crate 走 workspace，可能在将来版本演化时产生**多版本共存**（cargo tree 里同时出现两个 `bytes`/`prost` 版本），引发编译产物膨胀或符号冲突。同时削弱了 workspace 统一版本治理。

现在做是因为：改动小、低风险、范式成熟（core/client 已用 workspace deps），趁 v0.18 周期补齐这条依赖卫生；并落盘一条 workspace-dependency-policy 防止后续 crate/codegen 再次绕过 workspace 钉版本。

## What Changes

- **新增** `workspace-dependency-policy` capability：要求跨 crate 共享的依赖 MUST 在根 `Cargo.toml` 的 `[workspace.dependencies]` 声明，并通过 `{ workspace = true }` 消费。
- **修改** `openlark-protocol/Cargo.toml`：`bytes`、`prost` 从钉版本改为 `{ workspace = true }`。
- **修改** 根 `Cargo.toml [workspace.dependencies]`：新增 `bytes` 声明（`prost` 已存在）；protocol 改为消费 workspace。
- **同步**（如需）`.github/msrv/Cargo.lock`：若 `bytes` 移到 workspace 后 resolved 版本变化，须同步 MSRV pinned lockfile（否则 CI msrv `--locked` 失败）。
- **保留** protocol 的功能代码与 API 不变；不升级 `bytes`/`prost` 大版本。

## Capabilities

### New Capabilities

- `workspace-dependency-policy`: workspace 依赖治理策略——跨 crate 共享的依赖 MUST 经 `[workspace.dependencies]` 声明并以 `{ workspace = true }` 消费，禁止 crate 级钉版本；含现状定义与判定准则。

### Modified Capabilities

（无现有 spec-level 行为变更——本次仅依赖声明方式调整，不改任何 crate 的编译行为或公开 API。）

## Impact

- **代码**：根 `Cargo.toml`（新增 bytes workspace 声明）+ `openlark-protocol/Cargo.toml`（bytes/prost 改 workspace=true）。无 `src/` 变更。
- **依赖**：`bytes`/`prost` 版本范围不变（`bytes` 1.x、`prost` 0.13.x），仅声明位置迁移；resolved 版本可能微调 → 视情况同步 `.github/msrv/Cargo.lock`。
- **公共 API**：无变化。
- **CI**：须确认 `cargo build --all-features`、msrv `--locked`、lint 双模式、`cargo deny check`（依赖图一致性）均通过。
- **性能**：无影响。
