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
