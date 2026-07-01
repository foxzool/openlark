# Comet Design Handoff

- Change: feature-naming-convention
- Phase: design
- Mode: compact
- Context hash: 6e14e5f1d6c57384d6f910e64889ecb170ba7171582f096f40a1b3e78cb95a27

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/feature-naming-convention/proposal.md

- Source: openspec/changes/feature-naming-convention/proposal.md
- Lines: 1-40
- SHA256: 51331c149fc42c69336ec8a45c41295bfdd9c4f34e89c0de72d791ff56396216

```md
## Why

issue #272 指出 feature 命名方案跨 crate 不一致、存在空版本 feature。核实代码后发现 issue 的诊断部分失准，真实问题更严重：

- **三套 feature 方案并存且无文档规范**：单版本门控（ai/cardkit/application/helpdesk/mail/bot）、纯模块门控（communication/hr/meeting/auth 等）、混合（docs/platform/security/analytics/user 同时有模块 feature 和死版本链）。用户需记忆不同 crate 的规则。
- **5 个混合 crate 存在门控 0 行的死版本链**（platform/analytics 的 v1-v4、security/docs 的 v1-v3、user 的 v2-v4）：`full` 拉入这些 feature 却不门控任何代码，启用 `v4` 期望版本控制实为 no-op，误导用户。
- **issue #272 误判 `openlark-bot` 的 v4 为空**：实际 v4 门控 `pub mod bot` + `BotService::bot()`，是 live 代码，必须保留并在规范中纠正记录。

现在做是因为死版本链属 cargo-cult 遗留，趁 v0.18 breaking 窗口期清理成本最低；同时补齐规范文档防止后续 crate 继续分化。

## What Changes

- **新增** feature 命名规范文档（`docs/FEATURE_NAMING_CONVENTION.md`）与 capability spec：确立「模块 feature 为主、版本 feature 为单 API 版本 crate 的合法例外」原则，记录三套方案现状与例外清单（含 bot v4 纠正说明）。
- **移除** 5 个混合 crate 中门控 0 行的死版本链 feature：
  - `openlark-platform`：移除 `v1`/`v2`/`v3`/`v4`，`full` 去掉 `v4`
  - `openlark-analytics`：移除 `v1`/`v2`/`v3`/`v4`，`full` 去掉 `v4`
  - `openlark-security`：移除 `v1`/`v2`/`v3`，`full` 去掉 `v3`
  - `openlark-docs`：移除 `v1`/`v2`/`v3`，`full` 去掉 `v3`
  - `openlark-user`：移除 `v2`/`v3`/`v4`（保留 live 的 `v1`），`full` 去掉 `v4`
- **同步下游引用**：`openlark-client` 与根 `Cargo.toml` 中引用 `openlark-docs/v2`、`openlark-docs/v3` 的 feature 定义需更新（`docs-sheets-v2`/`docs-sheets-v3` 等）。
- **保留** 6 个单版本 crate（ai/cardkit/application/helpdesk/mail/bot）的版本 feature，作为合法例外写入规范。
- **BREAKING**：移除公开 feature（如 `openlark-docs/v2`、`openlark-docs/v3`、`openlark-platform/v4` 等）。这些 feature 当前不门控任何代码、启用后行为无变化，但仍属公开 API；design 阶段决定直接删除还是走弃用别名周期。

## Capabilities

### New Capabilities

- `feature-naming-convention`: 飞书 SDK 各 crate 的 Cargo feature 命名规范——模块 feature 为主、版本 feature 为单 API 版本 crate 的合法例外；包含三套方案现状定义、合法例外清单、死版本链判定与清理准则。

### Modified Capabilities

（无现有 spec-level 行为变更——本次清理移除的是门控 0 行的死 feature，不改变任何 crate 的编译行为或 spec 要求。）

## Impact

- **代码**：5 个 crate 的 `Cargo.toml`（platform/analytics/security/docs/user）+ 2 个下游 `Cargo.toml`（openlark-client、根 crate）。无 `src/` 逻辑变更（user/v1 保留门控）。
- **公共 API**：移除多个公开 Cargo feature 属向后兼容性变更（BREAKING）。design 阶段评估外部用户依赖面，决定直接删 vs 弃用别名。
- **文档**：新增 `docs/FEATURE_NAMING_CONVENTION.md`；视情况更新 `FEATURE_MATRIX.md` 与 `AGENTS.md` SKILLS/CONVENTIONS 段。
- **CI**：`just lint` 含 `--no-default-features` 与 `--all-features` 两种构建，须确认移除 feature 后两种模式均通过。
- **依赖/性能**：无影响。
```

## openspec/changes/feature-naming-convention/design.md

- Source: openspec/changes/feature-naming-convention/design.md
- Lines: 1-58
- SHA256: 893b75cca86fb492a9caad64e2b57986f1c0e45d9feb1886cbd88d1245954f84

```md
## Context

飞书 SDK 各 crate 的 Cargo feature 长期无统一规范，演化出三套并存方案：

| 方案 | 代表 crate | 特征 |
|------|-----------|------|
| A. 单版本门控 | ai/cardkit/application/helpdesk/mail/bot | 唯一版本 feature（`v1` 或 `v4`）门控真代码，无 v2 可选 |
| B. 纯模块门控 | communication/hr/meeting/auth/core/client | 按业务域命名，无版本 feature |
| C. 混合（模块 + 死版本链） | docs/platform/security/analytics/user | 模块 feature 之外还挂了门控 0 行的 `v1→v2→v3→v4` 链 |

issue #272 诊断部分失准：把 live 的 `openlark-bot` v4 误判为空，且只点了 platform/bot 两个空 feature，实际 5 个混合 crate 都有死版本链。本 change 据实纠正并清理。

约束：MSRV 1.88；CI lint 跑 `--no-default-features` 与 `--all-features` 两种模式；AGENTS.md 要求不破坏向后兼容性（公开 feature 亦是公开 API）。

## Goals / Non-Goals

**Goals:**

- 建立 feature 命名规范（模块为主、版本为例外），落盘 `docs/FEATURE_NAMING_CONVENTION.md` 与 capability spec
- 移除 5 个混合 crate 门控 0 行的死版本链，同步所有下游 `Cargo.toml` 引用
- 纠正 issue #272 对 `bot` v4 的误判记录

**Non-Goals:**

- 不动 6 个单版本 crate 的版本 feature（保留为合法例外）
- 不把三套方案统一/重命名为单一方案
- 不改变任何 crate 的实际编译行为（死链本就不门控代码）
- 不新增 API、不重命名 feature（除移除死链外）

## Decisions

### D1: 规范力度 = 文档 + 清理死链

**选择**：既写规范文档，也清理违规死链。**理由**：仅写文档不清理，死版本链继续误导用户（启用 `v4` 期望版本控制实为 no-op）；文档化现状才有判定准则。**备选**：仅文档（零风险但留垃圾）/ 全面标准化（重命名统一，breaking 面太大且非必要）。

### D2: 死链处理 = 全部移除（含 docs，同步下游）

**选择**：5 个混合 crate 的死版本链 feature 一律移除，docs 同步更新 `openlark-client` 与根 `Cargo.toml`。**理由**：死 feature 门控 0 行代码，移除是纯清理无行为变化；保留只会持续误导。**备选**：改 `reserved-vN` 占位（保留扩展位但仍是门控 0 行的噪音）/ 仅文档标注（不清理）。

### D3: 单版本 crate = 保留现状

**选择**：ai/cardkit/application/helpdesk/mail/bot 的版本 feature 保留，作为合法例外写入规范。**理由**：这些 feature 门控真实代码且反映服务端 API 版本（bot=v4 搜索、其余=v1），符合「版本 feature 门控真代码」的合法条件。**备选**：移除冗余版本门控让代码常开（改 src + Cargo，范围大且丢失版本语义）。

### D4: 规范文档位置 = `docs/FEATURE_NAMING_CONVENTION.md`

**选择**：与现有 `docs/CLIENT_NAMING_CONVENTION.md` 同目录、同命名风格。**理由**：保持文档发现一致性，便于在 AGENTS.md CONVENTIONS 段统一引用。

## Risks / Trade-offs

- **[移除公开 feature 属 BREAKING]** → 这些 feature（`openlark-docs/v2`、`openlark-platform/v4` 等）门控 0 行代码，启用后行为无变化，外部用户实际无法依赖其产生任何编译差异；但仍属公开 API。Mitigation：design 阶段评估是否需弃用别名周期（见开放问题 Q1）。
- **[docs 下游引用遗漏]** → 移除 `openlark-docs/v2`/`v3` 后若漏改下游 feature 定义，`cargo build` 会因悬空 feature 引用直接失败。Mitigation：tasks 中显式列出下游同步项 + CI `--all-features`/`--no-default-features` 双模式验证。
- **[规范与后续 crate 漂移]** → 新 crate 可能再次引入死版本链。Mitigation：spec 落盘为持久约束 + 文档给出判定准则，后续 code-review/design-review skill 可引用。

## Open Questions

- **Q1（BREAKING 处理）**：被移除的版本 feature 是否需要先走一个弃用别名周期（如保留 `openlark-docs/v2` 为空别名 + `#[deprecated]` 等价物）再删？还是鉴于 v0.18 本就是 breaking 窗口、且这些 feature 无实际编译效果，直接删？→ `/comet-design` 阶段 brainstorming 决定。
- **Q2（docs-sheets-v2/v3 去留）**：根 `Cargo.toml` 的 `docs-sheets-v2`/`docs-sheets-v3` feature 移除 `openlark-docs/v2`/`v3` 引用后，是退化为 `docs-ccm` 别名保留，还是直接移除这两个根 feature？→ design 阶段决定。
- **Q3（FEATURE_MATRIX.md 同步）**：`docs/FEATURE_MATRIX.md` 是否需要同步更新以反映移除的 feature？→ build 阶段核实。
```

## openspec/changes/feature-naming-convention/tasks.md

- Source: openspec/changes/feature-naming-convention/tasks.md
- Lines: 1-24
- SHA256: f40bc4db56554f5ca80bc6a2446265dbc597fdcaa64643a5de4aefc2bc0bc25a

```md
## 1. 规范文档落盘

- [ ] 1.1 创建 `docs/FEATURE_NAMING_CONVENTION.md`：写明「模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外」原则；附三套方案现状清单（A 单版本门控 / B 纯模块门控 / C 混合）、6 个单版本 crate 合法例外清单，并显式纠正 issue #272 对 `openlark-bot` v4 的误判（v4 门控 live 代码）
- [ ] 1.2 在 `AGENTS.md` 的 CONVENTIONS 段补一行指向 `docs/FEATURE_NAMING_CONVENTION.md`

## 2. 移除 5 个混合 crate 的死版本链

- [ ] 2.1 `openlark-platform/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [ ] 2.2 `openlark-analytics/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [ ] 2.3 `openlark-security/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [ ] 2.4 `openlark-docs/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [ ] 2.5 `openlark-user/Cargo.toml`：移除 `v2`/`v3`/`v4` 定义（**保留 live 的 `v1`**），`full` 去掉 `"v4"`

## 3. 同步下游 docs feature 引用

- [ ] 3.1 `crates/openlark-client/Cargo.toml`：`docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` 引用
- [ ] 3.2 根 `Cargo.toml`：workspace `docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"`；按 Q2 决策处理 `docs-sheets-v2`/`docs-sheets-v3`（退化为 `docs-ccm` 别名或移除）

## 4. 验证

- [ ] 4.1 `just fmt` + `cargo fmt --check` 通过
- [ ] 4.2 `just lint` 通过（CI 双模式：`--no-default-features` 与 `--all-features` 均过，确认无悬空 feature 引用）
- [ ] 4.3 `just build` + `just test` 通过
- [ ] 4.4 核实 `docs/FEATURE_MATRIX.md` 是否需同步更新移除的 feature（Q3）
```

## openspec/changes/feature-naming-convention/specs/feature-naming-convention/spec.md

- Source: openspec/changes/feature-naming-convention/specs/feature-naming-convention/spec.md
- Lines: 1-63
- SHA256: f505a730081bf33ce89a9f1e6e8ac47783882de2799a258e0991238a11c0c435

```md
## ADDED Requirements

### Requirement: 模块 feature 为主要门控方案

飞书 SDK 各业务 crate 的功能开关 SHALL 以**模块 feature**（按业务域命名，如 `attendance`、`im`、`search`、`app-engine`）为主要门控手段，而非版本 feature。

#### Scenario: 业务域功能用模块 feature 门控

- **WHEN** 某个 crate 需要按业务子域（如 hr 的考勤/薪酬/绩效）控制编译范围
- **THEN** 该 crate 的 `[features]` 段 MUST 定义以业务域命名的模块 feature（如 `attendance = []`），且该 feature 门控对应子模块的 `cfg(feature = "...")` 代码

#### Scenario: 不引入无业务含义的版本链

- **WHEN** 一个 crate 已经用模块 feature 组织其业务域
- **THEN** 该 crate MUST NOT 额外定义 `v1`/`v2`/`v3`/`v4` 这类不门控任何代码的链式版本占位 feature

### Requirement: 版本 feature 仅作为单 API 版本 crate 的合法例外

版本 feature（`v1`/`v2`/`v4` 等）SHALL 仅在「单 API 版本 crate」场景下作为合法例外使用：该 crate 的某个服务域只有单一 API 版本，且该版本 feature MUST 门控真实代码（反映服务端实际 API 版本）。

#### Scenario: 单版本 crate 保留版本 feature

- **WHEN** 一个 crate 全量围绕单一 API 版本构建（如 `openlark-bot` 围绕 v4 机器人搜索 API、`openlark-ai` 围绕 v1）
- **THEN** 该 crate MAY 保留其唯一且门控真实代码的版本 feature（如 `bot` 的 `v4`、`ai`/`cardkit`/`application`/`helpdesk`/`mail` 的 `v1`）

#### Scenario: 版本 feature 必须门控真实代码

- **WHEN** 某个 crate 定义了版本 feature
- **THEN** 该 feature MUST 通过 `cfg(feature = "...")` 门控至少一处真实代码；不门控任何代码的版本 feature 不被允许

### Requirement: 禁止门控 0 行的死版本链 feature

**版本链 feature（`v1`/`v2`/`v3`/`v4`）** MUST 门控实际代码。定义了但在整个 crate 的 `src/` 中门控 0 行代码的版本链 feature（特别是 `v1 = [...]` → `v2 = ["v1"]` → `v3 = ["v2"]` → `v4 = ["v3"]` 这类链式占位）SHALL 被移除。

此约束仅适用于版本链 feature。模块分组 marker feature（如 `core`、`full`、`default`、`all-*` 别名）虽不直接 `cfg` 门控代码，但作为合法的 grouping/aggregation 节点被其他 feature 引用，不在此约束范围内。

#### Scenario: 混合 crate 的死版本链被移除

- **WHEN** 检查 `openlark-platform`、`openlark-analytics`、`openlark-security`、`openlark-docs`、`openlark-user` 的 `Cargo.toml`
- **THEN** 这些 crate 中门控 0 行代码的版本链 feature（platform/analytics 的 `v1`-`v4`、security/docs 的 `v1`-`v3`、user 的 `v2`-`v4`）MUST 被移除；`full` feature MUST NOT 再引用已移除的版本 feature

#### Scenario: live 版本门控保留不动

- **WHEN** 检查 `openlark-user`（`v1` 门控 `preferences.rs`/`settings.rs`）与 `openlark-bot`（`v4` 门控 `bot` 模块）
- **THEN** 这些门控真实代码的版本 feature（user 的 `v1`、bot 的 `v4`）MUST 保留不变

### Requirement: 死版本链移除须同步下游 feature 引用

移除某 crate 的版本 feature 后，所有下游 `Cargo.toml` 中引用该 feature 的定义 MUST 同步更新，不得引用已不存在的 feature。

#### Scenario: docs 版本 feature 的下游引用同步

- **WHEN** `openlark-docs` 的 `v2`/`v3` 被移除
- **THEN** `openlark-client` 的 `docs` feature 与根 `Cargo.toml` 的 `docs-sheets-v2`/`docs-sheets-v3` feature MUST 移除对 `openlark-docs/v2`、`openlark-docs/v3` 的引用，不得留下悬空引用

### Requirement: 命名规范文档落盘并维护例外清单

SHALL 在 `docs/FEATURE_NAMING_CONVENTION.md` 维护 feature 命名规范文档，记录模块 feature 为主、版本 feature 为合法例外的原则，并列出当前三套方案的现状与合法例外清单（含 `openlark-bot` v4 门控 live 代码的说明，纠正 issue #272 的误判）。

#### Scenario: 规范文档存在且内容完整

- **WHEN** 检查 `docs/FEATURE_NAMING_CONVENTION.md`
- **THEN** 文档 MUST 包含：模块 feature 为主的原则、版本 feature 合法例外的判定条件、当前三套方案现状清单、6 个单版本 crate 例外清单，并明确说明 `openlark-bot` 的 `v4` 门控 live 代码（非空 feature）
```

