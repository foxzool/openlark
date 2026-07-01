# feature-naming-convention Specification

## Purpose
TBD - created by archiving change feature-naming-convention. Update Purpose after archive.
## Requirements
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

**版本链 feature（`v1`/`v2`/`v3`/`v4`）** MUST 门控有实际用途的代码。定义了但在整个 crate 的 `src/` 中门控 0 行代码、或仅门控 vestigial（零引用）代码/测试入口的版本链 feature（特别是 `v1 = [...]` → `v2 = ["v1"]` → `v3 = ["v2"]` → `v4 = ["v3"]` 这类链式占位）SHALL 连同其 vestigial 门控代码一并移除。

此约束仅适用于版本链 feature。模块分组 marker feature（如 `core`、`full`、`default`、`all-*` 别名）虽不直接 `cfg` 门控代码，但作为合法的 grouping/aggregation 节点被其他 feature 引用，不在此约束范围内。

#### Scenario: 混合 crate 的死版本链被移除

- **WHEN** 检查 `openlark-platform`、`openlark-analytics`、`openlark-security`、`openlark-docs`、`openlark-user` 的 `Cargo.toml`
- **THEN** 这些 crate 中的死/vestigial 版本链 feature MUST 被移除——platform/analytics 的 `v1`-`v4`、security 的 `v1`-`v3`、user 的 `v2`-`v4` 真正门控 0 行；docs 的 `v1`-`v3` 仅门控孤立的 vestigial `versions` 模块（零引用）；platform 的 `v1` 仅门控测试入口且所测 `.v1()` API 无条件公开——其 vestigial 门控代码与测试门控一并清理；`full` feature MUST NOT 再引用已移除的版本 feature

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

