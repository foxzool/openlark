# Comet Design Handoff

- Change: protocol-workspace-deps
- Phase: design
- Mode: compact
- Context hash: d7e31caa44def5206725fa3ac402e00ab48fc59221678f5be3f5e48e92229c5f

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/protocol-workspace-deps/proposal.md

- Source: openspec/changes/protocol-workspace-deps/proposal.md
- Lines: 1-33
- SHA256: 239a29a8541639e95cba8ccbf30eec242d9e273b326ceaa6a92343d86025b457

```md
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
```

## openspec/changes/protocol-workspace-deps/design.md

- Source: openspec/changes/protocol-workspace-deps/design.md
- Lines: 1-59
- SHA256: c2a29e656ff75f3570227a1b64ea8425db99b6e7e012e51fc8ef5dd66107e256

```md
## Context

OpenLark 是多 crate workspace（18 业务 crate）。依赖治理有两种并存形态：

| 形态 | 代表 | 特征 |
|------|------|------|
| workspace 统一 | core / client（core 38 个依赖走 `workspace = true`） | 版本在根 `[workspace.dependencies]` 声明，crate 用 `{ workspace = true }` 消费 |
| crate 级钉版本 | protocol（`bytes = "1.6.0"`、`prost = "0.13.1"`） | 版本钉在 crate Cargo.toml，绕过 workspace |

`prost` 已在 `[workspace.dependencies]` 声明为 `0.13`，但 protocol 仍自钉 `0.13.1`（同 minor，当前无冲突，但治理上重复）；`bytes` 完全未进 workspace，仅 protocol 一处使用。

issue #273 Part B 要求 protocol 依赖走 workspace，消除多版本共存风险。

约束：MSRV 1.88；CI msrv job 用 `.github/msrv/Cargo.lock`（pinned）跑 `--locked`，依赖 resolved 版本变化须同步该 lockfile；`cargo deny check` 校验依赖图。

## Goals / Non-Goals

**Goals:**

- `openlark-protocol` 的 `bytes`/`prost` 改为 `{ workspace = true }` 消费
- `bytes` 加入 `[workspace.dependencies]`
- 落盘 `workspace-dependency-policy` capability，给出可检查的判定准则
- 全 CI 通过（含 msrv `--locked`、cargo deny）

**Non-Goals:**

- 不升级 `bytes`/`prost` 大版本（保 1.x / 0.13.x）
- 不动 lint 策略（missing_docs/bare_urls）—— issue #273 Part A，另开 change
- 不批量改其他 crate 的依赖（仅 protocol 这一处不一致）
- 不改 protocol 功能代码或 API

## Decisions

### D1: `bytes` workspace 版本 = `1.6`

protocol 当前钉 `bytes = "1.6.0"`。workspace 声明用 `"1.6"`（同 major.minor），保证 resolved 版本尽量不变，最小化 lockfile/MSRV 影响。若 `cargo update -p bytes` 后 resolved 版本仍变，按 D3 同步 lockfile。

### D2: `prost` 复用已有 workspace 声明

`[workspace.dependencies]` 已有 `prost = { version = "0.13" }`。protocol 从 `prost = "0.13.1"` 直接改为 `prost = { workspace = true }`，版本由 workspace `0.13` 统一（同 minor，无破坏）。

### D3: MSRV lockfile 同步策略

`bytes`/`prost` resolved 版本若因迁移变化，CI msrv `--locked` 会失败（本地复现不出，见 [[msrv-pinned-lockfile]] 教训）。处理：改完 Cargo.toml 后本地 `cargo update -p bytes -p prost --workspace`，看 `Cargo.lock` diff；若变化，把同样的版本钉进 `.github/msrv/Cargo.lock`（手动编辑对应 `[package]` 条目，或 `cp Cargo.lock .github/msrv/Cargo.lock` 局部对齐——build 阶段决定）。

### D4: 落盘 policy capability（对齐 feature-naming-convention 范式）

新建 `workspace-dependency-policy` capability spec，记录「跨 crate 共享依赖 MUST 走 workspace」准则与判定命令（`cargo tree -d` 查多版本、grep crate 级钉版本）。供后续 code-review/design-review 与 codegen 引用，防复发。

## Risks / Trade-offs

- **[bytes resolved 版本变化]** → Mitigation：D3 显式同步 `.github/msrv/Cargo.lock`；CI msrv `--locked` 兜底。
- **[cargo deny 新冲突]** → Mitigation：`cargo deny check` 在验证阶段跑；预期无新冲突（版本范围未变）。
- **[policy 与后续 crate 漂移]** → Mitigation：spec 落盘 + 判定准则，后续可加 CI 脚本（本 change 不做，留待后续）。
- **[范围蔓延到其他 crate 依赖]** → Mitigation：Non-Goals 明确仅 protocol；其他 crate 已用 workspace。

## Open Questions

- **Q1（lockfile 同步方式）**：`.github/msrv/Cargo.lock` 同步用「整文件 cp 覆盖」还是「手动改对应 package 条目」？→ build 阶段看 Cargo.lock diff 后决定（cp 更省事但可能带入无关变化；手动更精准）。
```

## openspec/changes/protocol-workspace-deps/tasks.md

- Source: openspec/changes/protocol-workspace-deps/tasks.md
- Lines: 1-21
- SHA256: 5e8079c106c96a910b9bc5dda67e6dba6ed6bb1b516ff15c6c1a9982b702ca05

```md
## 1. workspace.dependencies 声明 bytes

- [ ] 1.1 根 `Cargo.toml` 的 `[workspace.dependencies]` 段新增 `bytes = "1.6"`（prost 已存在，不动）

## 2. openlark-protocol 改消费 workspace

- [ ] 2.1 `crates/openlark-protocol/Cargo.toml`：`bytes = "1.6.0"` → `bytes = { workspace = true }`
- [ ] 2.2 `crates/openlark-protocol/Cargo.toml`：`prost = "0.13.1"` → `prost = { workspace = true }`

## 3. lockfile 与 MSRV 同步

- [ ] 3.1 本地 `cargo update -p bytes -p prost --workspace` 后看 `Cargo.lock` diff，确认 resolved 版本是否变化
- [ ] 3.2 若变化：同步 `.github/msrv/Cargo.lock` 对应 `[package]` 条目（按 Q1 决策：cp 覆盖或手动精准改），保证 msrv `--locked` 可过

## 4. 验证

- [ ] 4.1 `cargo fmt --check` 过
- [ ] 4.2 `just lint` 过（CI 双模式：`--all-features` 与 `--no-default-features` 均过）
- [ ] 4.3 `cargo build --workspace --all-features` 过
- [ ] 4.4 msrv 验证：用 `.github/msrv/Cargo.lock` 跑 `--locked` 过（docker rust:1.88 或本地）
- [ ] 4.5 `cargo deny check` 过（依赖图无新冲突）+ `cargo tree -d`（无 bytes/prost 多版本）
```

## openspec/changes/protocol-workspace-deps/specs/workspace-dependency-policy/spec.md

- Source: openspec/changes/protocol-workspace-deps/specs/workspace-dependency-policy/spec.md
- Lines: 1-30
- SHA256: df22d5d032bbcdfd9c1a9963745a0a09d9bc6af9246b7d845f7971d20b63ed59

```md
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
```

