---
comet_change: feature-naming-convention
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-01-feature-naming-convention
status: final
---

# Feature 命名规范与死版本链清理 — 技术设计

> 需求事实源（canonical spec）：`openspec/changes/feature-naming-convention/specs/feature-naming-convention/spec.md`
> 本文档只描述 **HOW（技术实现方案）**，不复述需求。

## 1. 背景与目标

飞书 SDK 各 crate 的 Cargo feature 长期演化出三套并存方案，无统一文档：

| 方案 | 代表 crate | 特征 |
|------|-----------|------|
| A. 单版本门控 | ai / cardkit / application / helpdesk / mail / bot | 唯一版本 feature（`v1` 或 `v4`）门控真实代码 |
| B. 纯模块门控 | communication / hr / meeting / auth / core / client | 按业务域命名，无版本 feature |
| C. 混合（模块 + 死版本链） | docs / platform / security / analytics / user | 模块 feature 之外挂了门控 0 行的 `v1→v2→v3→v4` 链 |

issue #272 指出"空版本 feature"问题但诊断部分失准：把 **live 的 `openlark-bot` v4** 误判为空。实际证据（`crates/openlark-bot/src/lib.rs:14` 与 `service.rs:20` 均有 `#[cfg(feature = "v4")]`，分别门控 `pub mod bot` 与 `BotService::bot()`）证明 bot v4 门控真实代码，是合法例外，必须保留。

真正需要清理的是 5 个 C 类（混合）crate 中门控 0 行的死版本链。

**目标**：建立 feature 命名规范文档（模块为主、版本为例外）+ 移除 5 crate 死版本链 + 同步所有下游引用 + 纠正 issue #272 误判。

## 2. 执行策略

**Approach A：本 change 一次性直接删除，单原子 PR，不走弃用别名周期。**

理由：
- 被移除的版本 feature 在各自 crate 的 `src/` 中门控 **0 行代码**，启用后无任何编译差异，外部用户无法依赖其产生行为。
- 全仓引用面已核实（仅 `openlark-docs/v2`、`openlark-docs/v3` 各有 2 处下游引用，其余版本 feature 完全孤立）。
- v0.18 本就是 breaking 窗口期，弃用别名只增噪音不增价值。

## 3. 文件级改动清单

### 3.1 新增 `docs/FEATURE_NAMING_CONVENTION.md`

与现有 `docs/CLIENT_NAMING_CONVENTION.md` 同目录同风格。内容须包含：
- 「模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外」原则
- 三套方案现状清单（A/B/C）
- 6 个单版本 crate 合法例外清单（ai/cardkit/application/helpdesk/mail/bot）
- **显式纠正 issue #272**：说明 `openlark-bot` 的 `v4` 门控 live 代码（`pub mod bot` + `BotService::bot()`），非空 feature
- 死版本链判定准则（供后续 code-review/design-review skill 引用）

### 3.2 `AGENTS.md` CONVENTIONS 段补一行引用指向 `docs/FEATURE_NAMING_CONVENTION.md`。

### 3.3 移除 5 个混合 crate 的死版本链

每个 crate 移除整段死版本链定义，并从 `full` feature 去掉对最高版本的引用。`core = []` marker **保留不动**（见 D1）。

| Crate | 移除的版本 feature | `full` 去掉 | 保留 |
|-------|-------------------|------------|------|
| `openlark-platform` | `v1`/`v2`/`v3`/`v4` | `"v4"` | `core`、全部模块 feature |
| `openlark-analytics` | `v1`/`v2`/`v3`/`v4` | `"v4"` | `core`、全部模块 feature |
| `openlark-security` | `v1`/`v2`/`v3` | `"v3"` | `core`、全部模块 feature |
| `openlark-docs` | `v1`/`v2`/`v3` | `"v3"` | `core`、全部模块 feature |
| `openlark-user` | `v2`/`v3`/`v4` | `"v4"` | **`v1`（live 门控 `preferences.rs`/`settings.rs`）**、`core`、模块 feature |

典型死链形态（以 platform 为例）：
```toml
# 移除整段
v1 = ["core"]
v2 = ["v1"]
v3 = ["v2"]
v4 = ["v3"]
# full 从 [...] "v4"] 改为 [...]
```

### 3.4 同步下游 docs feature 引用

移除 `openlark-docs/v2`、`openlark-docs/v3` 后，两处下游必须同步，否则 `cargo build --all-features` 会因悬空 feature 引用失败：

**根 `Cargo.toml`**（workspace）：
- `docs-ccm` 数组中移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` 两行
- **移除整个 `docs-sheets-v2` 与 `docs-sheets-v3` feature 定义**（见 D2）

**`crates/openlark-client/Cargo.toml`**：
- `docs` feature 数组中移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` 两行

## 4. 关键技术决策

### D1: `core` marker 保留（Spec Patch 排除项）

5 个混合 crate 的 `core = []` feature 在 `src/` 中门控 0 行代码，字面看与死版本链同类。但 `core` 被模块 feature 引用（如 platform：`app-engine = ["app-engine-core", "core"]`、`directory = ["directory-core", "core"]` 等），是合法的 **grouping/aggregation marker**。

→ 已通过 Spec Patch 将 spec Requirement 3 收窄为仅约束「版本链 feature（`v1`/`v2`/`v3`/`v4`）」，显式排除 `core`/`full`/`default`/`all-*` 等合法 marker。本次清理 **不动任何 `core` marker**。

### D2: `docs-sheets-v2`/`docs-sheets-v3` 直接移除

这两个根 feature 除引用 `openlark-docs/v2`、`openlark-docs/v3` 外无其他消费者（examples/tests/CI matrix/FEATURE_MATRIX 均无引用）。退化为别名只留噪音 → 直接移除整个定义。

### D3: `openlark-user` 的 `v1` 保留

`v1` 门控 `preferences.rs`/`settings.rs` 等 live 代码（user crate 中 `cfg(feature = "v1")` 命中 4 处），符合「版本 feature 门控真代码」的合法条件。只移除 `v2`/`v3`/`v4`。

### D4: BREAKING 直接删不走弃用别名

见 §2 执行策略。被移除的 feature 门控 0 行、零下游消费者（除 docs/v2、docs/v3 各 2 处），直接删安全。

## 5. BREAKING 影响分析

移除多个公开 Cargo feature（如 `openlark-docs/v2`、`openlark-docs/v3`、`openlark-platform/v4` 等）属向后兼容性变更（BREAKING）。

**为何安全**：
- 这些 feature 门控 0 行代码，启用后编译产物无差异，外部用户无法依赖其产生任何可观测行为。
- 全仓引用面已穷举核实：仅 docs 的 v2/v3 有下游引用（根 Cargo + openlark-client 两文件），其余版本 feature 完全孤立。
- v0.18 已是 breaking 窗口（`Config` 迁移等多个 breaking 已并入）。
- CI `--all-features` 与 `--no-default-features` 双模式会捕获任何漏改的悬空引用。

## 6. 验证策略

| 步骤 | 命令 | 捕获目标 |
|------|------|---------|
| 格式 | `cargo fmt`（写）+ `cargo fmt --check`（验） | 格式漂移（CI lint 首步即 `fmt --check`，clippy 通过 ≠ fmt 通过） |
| Lint 双模式 | `just lint`（CI 跑 `--no-default-features` 与 `--all-features`） | **悬空 feature 引用**（漏改下游的关键防线） |
| 构建 | `just build` | 编译错误 |
| 测试 | `just test` | 行为回归（预期无，因死链本就不门控代码） |

特别关注：CI lint 的 `--no-default-features` 模式会暴露被 `cfg(feature)` 门控的测试/代码可达性问题；`--all-features` 模式会暴露移除 feature 后留下的悬空引用。两者都过才算安全。

MSRV 1.88：本次仅改 `Cargo.toml` 的 `[features]` 段，不动依赖版本，不触发 `.github/msrv/Cargo.lock` 同步。

## 7. 风险与缓解

| 风险 | 缓解 |
|------|------|
| 漏改下游 → `cargo build --all-features` 悬空引用失败 | tasks 显式列出 2 处下游同步项 + CI `--all-features` 兜底 |
| `core` marker 被误判为死链一并移除 | Spec Patch 已收窄约束范围；tasks 明确「保留 `core`」 |
| 误删 user 的 live `v1` | tasks 明确「user 仅删 v2/v3/v4，保留 v1」；grep `cfg(feature = "v1")` 命中 4 处为证 |
| 后续新 crate 再次引入死版本链 | spec 落盘为持久约束 + `docs/FEATURE_NAMING_CONVENTION.md` 给出判定准则，供 code-review/design-review skill 引用 |
| 规范文档与后续 crate 漂移 | 文档列入 AGENTS.md CONVENTIONS 段，提升可见性 |

## 8. 非目标

- 不动 6 个单版本 crate 的版本 feature（合法例外）
- 不把三套方案统一/重命名为单一方案
- 不改变任何 crate 的实际编译行为（死链本就不门控代码）
- 不新增 API、不重命名 feature（除移除死链外）

## 9. 实现发现（Spec Patch，build 阶段）

> 本节修正 §5 与 §8 中「被移除的 feature 全部门控 0 行 / 不改变编译行为」的不准确表述。build 阶段 `cargo check --all-targets --all-features` 暴露了 7 处 `unexpected_cfgs` warning，调查发现前期 grep 用了过窄模式 `cfg(feature = "v1")`，漏掉 `cfg(all/any(...))` 形态。

**实情**（宽模式 `feature = "v[1-4]"` 重查后）：

| crate | 真实门控 | 处理 |
|-------|---------|------|
| docs v1/v2/v3 | 门控 `pub mod versions`（孤立 vestigial 模块，硬编码 `0.15.0-dev`，0 外部/内部引用） | 删除 `versions.rs` + lib.rs 门控行（BREAKING 移除未用模块） |
| platform v1 | 门控 2 个测试入口；但测试调用的 `spark .v1()` 方法（`spark/mod.rs:26`）本身是无条件 `pub fn` | 测试 cfg 从 `all(spark, v1)` 改为 `spark`，继续覆盖真实 API |
| analytics/security/user-v2-v4 | 真正 0 门控（analytics v1 仅出现在 `//!` doc 注释示例文本，非真 cfg） | 按原计划移除 ✓ |

**结论**：spec 的核心论点（这些版本链是死链 / vestigial，应清理）依然成立；只是 docs/platform 的清理附带移除/修正了 vestigial 门控代码，并非纯 `Cargo.toml` 改动。`--all-targets` 下的 `unexpected_cfgs` 已全部消除（验证：`cargo check --workspace --all-targets --all-features` 0 warning）。
