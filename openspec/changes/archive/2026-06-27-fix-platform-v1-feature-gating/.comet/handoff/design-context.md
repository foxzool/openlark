# Comet Design Handoff

- Change: fix-platform-v1-feature-gating
- Phase: design
- Mode: compact
- Context hash: f9048a49b9e6a5555193c575d7d1f81db91681a5e9437626253f4c4fa28a14ed

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/fix-platform-v1-feature-gating/proposal.md

- Source: openspec/changes/fix-platform-v1-feature-gating/proposal.md
- Lines: 1-25
- SHA256: 9e559499e996b76521a2d52d6aece99ac0750715de1e6157ff7a2e750eb43348

```md
## Why

openlark-platform 的四个 service（admin、app_engine、directory、spark）在 `default` 和 `full` feature 构建下是**空壳**：service 结构体只有 `new()` / `config()`，唯一的 API 入口 `pub fn v1()` 及其实现子树（`admin/admin`、`app_engine/apaas`、`directory/directory`、`spark/spark`，共 ~163 文件、96 个 API）全部挂在 `#[cfg(feature = "v1")]` 后面，而 `v1` 既不在 `default` 也不在 `full`（`full` 只含 `v4`，且 `v4` 不门控任何代码）。根因是 commit `995066e5a` 为通过 clippy 给 facade 加门控时的**意外后果**——这让平台 96 个真实 API 对用户彻底不可达。需要让这些 API 在标准 feature 组合下可达。

## What Changes

- **修正 platform facade 的 v1 门控**：让 admin / app_engine / directory / spark 四个 service 的 API 实现在标准 feature 组合下可达（具体修法——移除门控 vs 把 `v1` 纳入 `default`/`full`——在 design 阶段 brainstorm 确定）。
- 处置 `full` 中不门控任何代码的空 `v4` feature，并在 design 阶段明确 `v1`/`v2`/`v3`/`v4` 的语义。
- **保留** `v1`/`v2`/`v3`/`v4` feature flag 本身（测试 clippy 门控仍依赖，见 `docs/superpowers/plans/2026-06-25-test-feature-gating.md`），只动 facade 上的门控点。

## Capabilities

### New Capabilities
- `platform-service-access`: openlark-platform 的 admin / app_engine / directory / spark 四个 service SHALL 在标准 feature 组合（至少 `full`，目标含 `default`）下暴露其完整 API 实现，不再是无 API 的空壳。

### Modified Capabilities
<!-- 无现有 platform spec 需要修改 -->

## Impact

- **openlark-platform**: `src/admin.rs`、`src/app_engine.rs`、`src/directory/mod.rs`、`src/spark/mod.rs` 四处 facade 的 `#[cfg(feature = "v1")]` 门控点；`Cargo.toml` `[features]` 的 `default` / `full` / `v1` / `v2` / `v3` / `v4` 定义。
- **编译影响**: 若把 API 纳入 `default`，默认构建会多编译 ~163 文件（构建时间上升）；若仅纳入 `full`，影响仅限 full 构建。取舍在 design 阶段定。
- **测试约束**: 必须保持 `cargo clippy --workspace --all-targets --no-default-features -D warnings` 与 `--all-features` 均 exit 0（不破坏现有 test-gating），4 个 service 现有单元测试仍通过。
- **非目标**: 不重构双层目录（`admin/admin` 等）；不改其他 crate 的 feature 方案；不新增或删除 API。
- **兼容性**: 让原本不可达的 API 变可达，不移除任何公开符号——视为行为补全，非 breaking。
```

## openspec/changes/fix-platform-v1-feature-gating/design.md

- Source: openspec/changes/fix-platform-v1-feature-gating/design.md
- Lines: 1-59
- SHA256: 23f38c1c1ccab8bd4ab56b76b7da79bbaf1130197553bf3c2d3ff49c2265f6b4

```md
## Context

openlark-platform 用**业务模块 feature**（`admin`、`app-engine`、`directory`、`spark`）控制 service 是否编译，这些都在 `default` 里。但四个 service 的**真正 API 实现**额外被 `#[cfg(feature = "v1")]` 门控，而 `v1` 不在 `default`/`full`——导致 service 虽然默认启用，却只剩空壳 facade。

```
default = ["app-engine","directory","admin","mdm","tenant","trust_party","spark"]   ← service 启用
full    = [...同上..., "v4"]                                                          ← v4 不门控任何代码

src/admin.rs:   AdminService { new(), config(), #[cfg(feature="v1")] v1() }   ← v1() 是唯一 API 入口
                #[cfg(feature="v1")] pub mod admin;                            ← admin/admin/** (~25 文件) 被锁
（app_engine/directory/spark 同构，合计 ~163 文件 / 96 API）
```

根因：commit `995066e5a`「Stabilize platform API inventory and unblock platform warning gates」为通过 clippy 给 facade 加 v1 门控，意外把全部 platform API 排除在标准构建外。对比 hr/communication/meeting（service 在 default → API 直接可达，无版本门控），platform 是唯一的特例。

约束：`v1`/`v2`/`v3`/`v4` feature flag 不能删除——`docs/superpowers/plans/2026-06-25-test-feature-gating.md` 显示测试用它们做 clippy 门控。本设计只动 **facade 门控点**，不动 test 门控。

## Goals / Non-Goals

**Goals:**
- admin / app_engine / directory / spark 四 service 的 96 个 API 在标准 feature 组合下可达、可编译、可测试。
- 保持 `cargo clippy --workspace --all-targets --no-default-features -D warnings` 与 `--all-features` 均 exit 0。
- 不破坏现有公开 API 符号。

**Non-Goals:**
- 不重构双层目录（`admin/admin`、`spark/spark` 等）。
- 不改其他 crate 的 feature 方案。
- 不删除 `v1`/`v2`/`v3`/`v4` feature flag（测试门控仍依赖）。
- 不新增或删除任何 API。

## Decisions

> 本节为初步设计，**关键选型留 design 阶段 brainstorming 确认**。

**D1（核心修法，待 brainstorm 确认）——倾向方案 A：移除 facade 上的 `#[cfg(feature="v1")]` 门控。**
- 理由：四个 service 已由模块 feature（`admin`/`app-engine`/`directory`/`spark`）门控，且这些已在 `default`。`v1` 门控是多余的第二层——移除后，service 启用即 API 可达，与 hr/communication/meeting 模式一致。`--no-default-features` 下 service 模块本就不编译，移除 facade 门控不影响 test-gating clippy。
- 备选 B：把 `v1` 加入 `default`/`full`。也能让 API 可达，但引入 platform 本不使用的版本 feature 间接层，语义更混乱。
- 备选 C：仅纳入 `full`（保守，default 保持轻量）。代价：默认构建仍无 platform API。

**D2（`v4` 空特征，待 brainstorm 确认）**：`full` 中的 `v4` 不门控任何代码。方案 A 下可保留（无害，测试门控可能用到）或清理。倾向保留并在 design 阶段核实其用途。

## Risks / Trade-offs

- **[默认构建变重]** 移除门控后 default 多编译 ~163 文件 → 缓解：这正是"service 默认启用"的应有语义，可接受；若不可接受则退回复选 C（仅 full）。
- **[潜伏编译错误暴露]** v1 子树长期未被标准构建编译，可能含latent 问题 → 缓解：先 `cargo check -p openlark-platform`（门控移除后）全量验证，逐个修复暴露的问题。
- **[clippy test-gating 回归]** → 缓解：实现后强制跑 `--no-default-features` 与 `--all-features` 两组 clippy。

## Migration Plan

1. 按 brainstorm 确认的方案修改 4 处 facade 门控 + Cargo.toml（若需）。
2. `cargo check -p openlark-platform`（default 与 full）确认编译。
3. 跑两组 clippy（`--no-default-features` / `--all-features`）+ 4 service 单测。
4. 回滚：纯 cfg/feature 改动，git revert 即可，无数据/依赖迁移。

## Open Questions

- D1 最终选型（A 移除门控 / B 纳入 default+full / C 仅纳入 full）。
- `v4` 空特征是否清理。
- 是否需要在 CHANGELOG 记录（行为补全，非 breaking）。
```

## openspec/changes/fix-platform-v1-feature-gating/tasks.md

- Source: openspec/changes/fix-platform-v1-feature-gating/tasks.md
- Lines: 1-35
- SHA256: 36eba4fe1a76277ecaa7ad50436002367a228757fd5f0127e1b5756b5583414b

```md
# Tasks — fix-platform-v1-feature-gating

> 注：以下为初步任务分解。具体修法（移除 facade 门控 vs 把 `v1` 纳入 default/full）经 design 阶段 brainstorming 确认后，第 2、3 组任务可能微调。

## 1. 调研与边界确认（build 前）

- [ ] 1.1 grep 确认 `v1`/`v2`/`v3`/`v4` feature 在测试文件中的全部 cfg 用法，明确哪些必须保留
- [ ] 1.2 确认移除 facade 门控后，`--no-default-features` 下各 service 模块（`admin`/`app-engine`/`directory`/`spark`）的编译边界与 test-gating 互不影响
- [ ] 1.3 核实 `full` 中空 `v4` feature 是否被任何测试/代码引用，决定保留或清理

## 2. 实施门控修正

- [ ] 2.1 修正 `src/admin.rs` 的 `#[cfg(feature = "v1")]` 门控（`pub fn v1()` + `pub mod admin`）
- [ ] 2.2 修正 `src/app_engine.rs` 的门控（`pub fn v1()` + `pub mod apaas`）
- [ ] 2.3 修正 `src/directory/mod.rs` 的门控（`pub fn v1()` + `pub mod directory`）
- [ ] 2.4 修正 `src/spark/mod.rs` 的门控（`pub fn v1()` + `pub mod spark`）
- [ ] 2.5 按 brainstorm 选型调整 `crates/openlark-platform/Cargo.toml` `[features]`（若选 B/C：把 `v1` 纳入 default/full）

## 3. 编译验证

- [ ] 3.1 `cargo check -p openlark-platform`（default）通过
- [ ] 3.2 `cargo check -p openlark-platform --all-features`（full）通过
- [ ] 3.3 修复门控移除后暴露的 latent 编译问题（v1 子树长期未被标准构建编译，可能有遗留问题）

## 4. 质量门

- [ ] 4.1 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0
- [ ] 4.3 四个 service 现有单元测试通过（`cargo test -p openlark-platform`）

## 5. 文档与收尾

- [ ] 5.1 CHANGELOG 记录本次行为补全（非 breaking）
- [ ] 5.2 更新 AGENTS.md / 相关 feature 说明（若涉及 default/full 语义变化）
- [ ] 5.3 `cargo doc -p openlark-platform --all-features --no-deps` 确认文档可生成
```

## openspec/changes/fix-platform-v1-feature-gating/specs/platform-service-access/spec.md

- Source: openspec/changes/fix-platform-v1-feature-gating/specs/platform-service-access/spec.md
- Lines: 1-41
- SHA256: 4555cbbbd37ad894dd700f63cd292f0a4a86c4f5c368092790695e3923c746e3

```md
## ADDED Requirements

### Requirement: Platform services expose their API surface under default features
openlark-platform 的 `AdminService`、`AppEngineService`、`DirectoryService`、`SparkService` SHALL 在 `default` feature 组合下编译并暴露其 API 实现入口（不再是无 API 的空壳 facade）。

#### Scenario: AdminService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`（`cargo build -p openlark-platform`）
- **THEN** `AdminService` 暴露其 v1 API 入口并成功编译，`admin/admin/**` 子树参与编译

#### Scenario: AppEngineService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`
- **THEN** `AppEngineService` 暴露其 API 入口，`app_engine/apaas/**` 子树参与编译

#### Scenario: DirectoryService 与 SparkService 可达 API
- **WHEN** 以 `default` feature 构建 `openlark-platform`
- **THEN** `DirectoryService` 与 `SparkService` 各自暴露 API 入口，`directory/directory/**` 与 `spark/spark/**` 子树参与编译

### Requirement: Platform services expose their API surface under full features
四个 platform service SHALL 在 `full`（`--all-features`）feature 组合下同样暴露其完整 API 实现。

#### Scenario: full 构建覆盖全部 platform API
- **WHEN** 以 `--all-features` 构建 `openlark-platform`
- **THEN** 四个 service 的全部 96 个 API 实现均参与编译，无空壳 facade

### Requirement: Feature-gating 测试门控不回归
本变更 SHALL 不破坏现有基于 feature flag 的 clippy 测试门控。

#### Scenario: no-default-features clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- **THEN** 命令以 exit 0 结束

#### Scenario: all-features clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **THEN** 命令以 exit 0 结束

### Requirement: 公开 API 符号不被移除
本变更 SHALL 不移除任何现有公开 API 符号；仅让原本不可达的 API 变为可达，视为行为补全而非破坏性变更。

#### Scenario: 现有公开类型保持可用
- **WHEN** 变更后以 `default` 或 `full` feature 编译依赖 openlark-platform 的代码
- **THEN** 变更前可访问的公开类型与方法仍可访问（仅新增可达性，无删除）
```

