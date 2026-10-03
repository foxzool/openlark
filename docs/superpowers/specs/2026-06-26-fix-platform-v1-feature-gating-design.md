---
comet_change: fix-platform-v1-feature-gating
role: technical-design
canonical_spec: openspec
archived-with: 2026-06-27-fix-platform-v1-feature-gating
status: final
---

# Design — fix-platform-v1-feature-gating

> OpenSpec delta spec `platform-service-access` 是事实源（canonical）。本文档为技术设计，不重复需求。

## 背景与根因

openlark-platform 用**业务模块 feature**（`admin`/`app-engine`/`directory`/`spark`，都在 `default`）控制 service 是否编译。但四个 service 的 API 实现额外被 `#[cfg(feature = "v1")]` 门控，而 `v1` 不在 `default`/`full`——导致 service 默认启用却是空壳 facade。

根因：commit `995066e5a`「Stabilize platform API inventory and unblock platform warning gates」为通过 clippy 给 facade 加 v1 门控，意外把 96 个 platform API 排除在标准构建外。对比 hr/communication/meeting（service 在 default → API 直接可达，无版本门控），platform 是唯一特例。

约束：`v1` flag 必须保留——`lib.rs~161` 测试 `#[cfg(all(feature = "spark", feature = "v1"))]` 与 `lib.rs:166` 调用 `service.spark().v1()...` 依赖它。

## 决策：方案 A — 移除 facade/intermediate 门控

移除 6 个文件里共 **10 处** `#[cfg(feature = "v1")]` 属性，其余完全不动（不改 Cargo.toml、不删 feature flag、不动 API）。

| 文件 | 行 | 门控对象 |
|------|----|---------|
| `src/admin.rs` | 29 | `pub fn v1()` 方法 |
| `src/admin.rs` | 35 | `pub mod admin` |
| `src/app_engine.rs` | 29 | `pub fn v1()` |
| `src/app_engine.rs` | 35 | `pub mod apaas` |
| `src/directory/mod.rs` | 29 | `pub fn v1()` |
| `src/directory/mod.rs` | 35 | `pub mod directory` |
| `src/directory/directory/mod.rs` | 3 | `pub mod v1`（intermediate） |
| `src/spark/mod.rs` | 26 | `pub fn v1()` |
| `src/spark/mod.rs` | 32 | `pub mod spark` |
| `src/spark/spark/mod.rs` | 3 | `pub mod v1`（intermediate） |

admin/app_engine 的门控只在 facade 层（其子树 `mod.rs` 无条件声明 `pub mod v1;`）；directory/spark 多一层 intermediate 门控（`<svc>/<svc>/mod.rs:3`）。

**为何 A 优于 B/C：**
- **B（v1 加进 default+full）**：留 facade 门控（异味）+ 引入 platform 本不用的版本 feature 间接层；`--no-default-features --features v1` 时 service 模块不编译却启用了 v1，语义混乱。
- **C（仅 full）**：default 仍是空壳，bug 对默认用户依旧存在，与 service 默认启用矛盾。
- **A**：service 已由模块 feature 门控且在 default，`v1` 是多余第二层；移除后"service 启用 = API 可达"，与 hr/communication/meeting 一致。`--no-default-features` 下 service 模块本就不编译，移除 facade 门控不影响 test-gating clippy。

## D2：v4 空特征保持现状

`v4` 不门控任何代码（grep 仅命中自身 `full = [... "v4"]` 定义；`Cargo.toml:85` 的 `uuid v4` 无关）。保持现状：v1 必留（测试依赖），v2/v3/v4 留作无害 no-op。最小改动原则不顺手清理。

## 风险与缓解

- **[默认构建变重]** default 多编译 ~163 文件 → 可接受（service 默认启用的应有语义）。
- **[latent 编译错误]** v1 子树长期未被标准构建编译 → build 阶段先 `cargo check -p openlark-platform`（default + full）全量验证，逐个修复暴露问题。
- **[clippy test-gating 回归]** → 强制跑 `--no-default-features` 与 `--all-features` 两组 clippy。

## 测试策略

1. `cargo check -p openlark-platform`（default）通过。
2. `cargo check -p openlark-platform --all-features`（full）通过。
3. `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` exit 0。
4. `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0。
5. `cargo test -p openlark-platform` 通过（含 `lib.rs:166` spark v1 链路与 4 service 单测）。
6. 确认 `lib.rs~161` `cfg(all(feature="spark", feature="v1"))` 测试仍编译。

## 迁移与回滚

纯 cfg 属性删除，无数据/依赖迁移；`git revert` 即可回滚。让原本不可达的 API 变可达，不移除任何公开符号——行为补全，非 breaking。CHANGELOG 记录一笔。
