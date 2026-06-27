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
