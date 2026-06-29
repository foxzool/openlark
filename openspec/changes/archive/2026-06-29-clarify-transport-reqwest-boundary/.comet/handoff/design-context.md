# Comet Design Handoff

- Change: clarify-transport-reqwest-boundary
- Phase: design
- Mode: compact
- Context hash: 2c2d7bb7ae88f9c5ce371fbb3a243699f48ceff44ec2e3e74f1e0eb2f49fea79

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/clarify-transport-reqwest-boundary/proposal.md

- Source: openspec/changes/clarify-transport-reqwest-boundary/proposal.md
- Lines: 1-35
- SHA256: 9ab91d0ef6a8592aa4c641c47038c29f86c53ced521b71d9ad979f73ff0c28d6

```md
## Why

issue #270（2026-06 架构审计 finding）指出「14 个业务 crate 直接依赖 reqwest，存在分层泄漏与配置不一致风险」。但本次 open 阶段已实证核实：**业务 crate 早已通过 core 的 `Transport<T>` 收口、源码层 0 处直接使用 reqwest**（core 62 处、webhook 10 处有意、client 3 处，其余 12 个业务 crate 全部 0 处）。issue 的「分层泄漏」「配置跨 crate 不一致」前提已不成立——真正的遗留是：

1. 12 个业务 crate 的 `Cargo.toml` 仍声明 `reqwest = { workspace = true }` 却从不引用（cargo-machete 因不遍历 `crates/` 给出假阴性），属 hygiene 问题；
2. webhook crate 直接用 reqwest（连接池复用，引用 #214）这一 by-design 例外未显式文档化；
3. ARCHITECTURE.md 的「Transport 中间件/熔断/重试」层是「🚧 规划中」的未实现草案，与 reqwest 依赖问题混为一谈。

本 change 闭环该审计 finding：做廉价且高确定性的 hygiene 清理 + 把已存在且在工作的 `Transport<T>` 边界显式文档化，不引入新抽象（trait / 中间件层推迟为独立 future change）。

## What Changes

- 删除 12 个业务 crate 的未用 `reqwest = { workspace = true }` 依赖声明：`analytics / auth / bot / application / communication / mail / hr / docs / helpdesk / platform / user / workflow`。
- **ARCHITECTURE.md** 新增/强化一节，明确「`openlark_core::http::Transport<T>` 是 HTTP 边界：业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型」。
- **ARCHITECTURE.md** 显式记录 webhook 的 by-design 例外（无鉴权推送器，进程级共享 `reqwest::Client` 复用连接池，见 #214）。
- **ARCHITECTURE.md** 将「Transport 中间件/熔断/重试」草案标注为 future change（不删除设计记录，仅澄清当前未实现、不属本次范围）。
- **CHANGELOG.md** 记一笔 v0.18 hygiene 清理（删 12 个未用 reqwest 依赖，非 breaking）。
- **webhook crate 文档注释**：强化「直接 reqwest 是 by-design 例外」说明，指向本边界约定。

## Capabilities

### New Capabilities
- `transport-reqwest-boundary`: openlark 业务 crate SHALL 通过 core 的 `Transport<T>` 抽象发起 HTTP 请求、SHALL 不在 `Cargo.toml` 中直接声明 `reqwest` 依赖（core / client / webhook 三个 crate 为显式例外，其中 webhook 为 by-design 性能优化例外并需文档化）。

### Modified Capabilities
<!-- 无（无既有 transport/http 相关 main spec） -->

## Impact

- **12 个业务 crate 的 `Cargo.toml`**：各删 1 行 `reqwest = { workspace = true }`（及其在 `[dependencies]` / `[dev-dependencies]` 中的归属）。不动任何 `.rs` 源码。
- **ARCHITECTURE.md**：新增边界约定小节 + webhook 例外 + 中间件层 future 标注。
- **CHANGELOG.md**：v0.18 hygiene 条目。
- **crates/openlark-webhook**：仅强化文档注释，不改源码逻辑。
- **破坏性**：无。删除未用依赖声明不改公开 API（业务 crate 0 处引用 reqwest）。理论上若下游依赖某业务 crate 传递出的 reqwest 可用性会受影响，但 workspace dep 的 features 集中管理、且无任何业务 crate re-export reqwest，属不支持的隐式用法；CHANGELOG 记录兜底。
- **非目标**：不把 `Transport<T>` 升级为 trait、不实现中间件/熔断/重试链、不改造 webhook 到 Transport、不动 core `Transport` 实现、不动 client。
```

## openspec/changes/clarify-transport-reqwest-boundary/design.md

- Source: openspec/changes/clarify-transport-reqwest-boundary/design.md
- Lines: 1-57
- SHA256: 68834e2f6f070190729623c65895045abdc3a3ecc36cf38d719830a74c7ad2c9

```md
## Context

issue #270 的架构审计 finding 原描述「14 个业务 crate 直接依赖 reqwest，存在分层泄漏与配置不一致风险」。open 阶段已对全仓实证核实，结论与原 finding 部分相反：

- 业务 crate 的真实 HTTP 调用路径是 `*Request::execute()` → `openlark_core::http::Transport::request(req, &config, option)` → core 内部 `ReqTranslator` 组装 `reqwest::RequestBuilder`。**业务 crate 源码层不出现 reqwest 类型**。
- reqwest 在 src/ 的实际引用分布：core 62 处（抽象本体）、webhook 10 处（有意，#214 连接池复用）、client 3 处；12 个业务 crate（analytics/auth/bot/application/communication/mail/hr/docs/helpdesk/platform/user/workflow）全部 **0 处**。
- 因此「分层泄漏」「配置跨 crate 不一致」两条影响不成立（配置统一走 core `Config`，请求统一走 core `Transport`）。真正遗留是：12 个 Cargo.toml 的未用依赖声明、webhook 例外未文档化、ARCHITECTURE.md 中间件层是未实现草案。

约束：v0.18 正在做一轮 breaking/hygiene 清理（前序 7 个 deprecated/config change 已归档），是 hygiene 清理的合适窗口。

## Goals / Non-Goals

**Goals:**
- 闭环 #270 审计 finding：让「`Transport<T>` 是 HTTP 边界」成为显式、可追溯的架构约定。
- 移除 12 个业务 crate 的未用 `reqwest` 依赖声明，消除 hygiene 噪音与误导（避免未来审计再次误判）。
- 把 webhook 的 by-design 例外与中间件层的 future 状态显式记录，消除 ARCHITECTURE.md 文档与现实的偏差。

**Non-Goals:**
- 不把 `Transport<T>` 从 concrete struct 升级为 trait、不引入可插拔 transport。
- 不实现 ARCHITECTURE.md 的中间件/熔断/重试中间件链（推迟为独立 future change，待有具体插拔/中间件需求驱动）。
- 不改造 webhook 到 core `Transport`（保留其直接 reqwest 例外）。
- 不改动 core `Transport` 实现、不动 client、不动任何业务 crate 的 `.rs` 源码逻辑。
- 不改动任何公开 API。

## Decisions

### 决策 1：清理未用依赖 + 文档化边界，而非引入 Transport trait（选 C 而非 A）
**选择**：删 12 个未用 reqwest 依赖 + 文档化现有 `Transport<T>` 边界。
**理由**：A 方案（升级为 trait + 中间件层）想解决的「业务越过 core 直接用 reqwest」问题经核实**并不存在**——业务早已走 `Transport::request()`。为已不存在的问题引入新抽象属过度工程（YAGNI），且 trait 化是大架构投资。现有 concrete `Transport<T>` 已在工作、已收口，文档化它即足以闭环 finding。
**备选**：A（trait + 中间件层）——拒绝，无具体需求驱动；B（仅写一句文档、不删依赖）——拒绝，留下 12 个未用依赖，审计 finding 未真正闭环。

### 决策 2：webhook 例外保留并文档化，不收口到 Transport
**选择**：保留 webhook 直接用 reqwest，在 ARCHITECTURE.md + crate 文档注释显式标注为 by-design 例外。
**理由**：webhook 是无鉴权的机器人推送器（不走标准 OpenAPI 鉴权链），其 `shared_client()` 进程级单例 `reqwest::Client` 是为连接池复用做的性能优化（见 #214 调研结论）。强行收口到 core `Transport` 会引入与 #214 结论冲突的复杂度，且 webhook 路径本就与带鉴权的业务 API 不同。

### 决策 3：中间件层标注为 future change，不实现也不删除草案
**选择**：ARCHITECTURE.md 中间件/熔断/重试草案保留，但显式标注「规划中 / future change，当前未实现」。
**理由**：删除设计记录会丢失有价值的前期设计；实现它超出本次范围且无需求驱动。标注 future 状态即可消除「文档描述 vs 实际实现」的偏差（审计专门指出了 RetryMiddleware 文档与 RetryPolicy 实现的不一致）。

### 决策 4：边界约定写入 ARCHITECTURE.md，不新建独立文档
**选择**：在现有 ARCHITECTURE.md 新增边界小节。
**理由**：架构约定应集中在 ARCHITECTURE.md，避免文档碎片化；该文件已有 Transport/reqwest 相关段落，补充边界约定天然契合。

## Risks / Trade-offs

- **[删依赖导致下游构建失败]** → 已核实 12 crate 源码 0 处 reqwest 引用、tests/examples/build.rs 均无；workspace dep 的 features 集中管理（root Cargo.toml:90），删业务 crate 的声明不影响 feature 合并。build 阶段以 `cargo build --workspace --all-features` 实证兜底。
- **[下游隐式依赖业务 crate 传递出的 reqwest]** → 业务 crate 无任何 re-export reqwest；属不支持的隐式用法。CHANGELOG v0.18 条目记录兜底。
- **[审计再次误判「业务依赖 reqwest」]** → 本次不仅删依赖，还在 ARCHITECTURE.md 显式写明边界与例外，从根上消除「看 Cargo.toml 误判」的可能。
- **[中间件层 future 标注被遗忘]** → future change 若需启动，会经新 open 流程重新评估；标注本身已足够澄清现状。

## Migration Plan

无运行时迁移（非 breaking、不改 API）。部署 = 合并代码；回滚 = revert 该 commit。CHANGELOG v0.18 hygiene 条目记录「12 个业务 crate 移除未用 reqwest 依赖声明」。

## Open Questions

无（policy 方向 C + webhook 保留已在 open 阶段确认；中间件层明确推迟）。build 阶段唯一需实证的是删依赖后全量 build 是否全绿（预期是）。
```

## openspec/changes/clarify-transport-reqwest-boundary/tasks.md

- Source: openspec/changes/clarify-transport-reqwest-boundary/tasks.md
- Lines: 1-29
- SHA256: b55b74b1db33a532ff83c70119da9c626bb31b653a0658d58c759c28e11efa99

```md
# Tasks — clarify-transport-reqwest-boundary

> 闭环 issue #270：删 12 个业务 crate 未用 reqwest 依赖 + 文档化 `Transport<T>` 边界 + webhook 例外 + 中间件层 future 标注。
> 纯依赖声明 + 文档变更，不改 `.rs` 源码逻辑、不改公开 API。

## 1. 依赖清理

- [ ] 1.1 移除 12 个业务 crate 的 `Cargo.toml` 中未用的 `reqwest = { workspace = true }` 依赖声明：`openlark-analytics`、`openlark-auth`、`openlark-bot`、`openlark-application`、`openlark-communication`、`openlark-mail`、`openlark-hr`、`openlark-docs`、`openlark-helpdesk`、`openlark-platform`、`openlark-user`、`openlark-workflow`（注意区分 `[dependencies]` 与 `[dev-dependencies]` 归属；保留 core/client/webhook 三个例外）

## 2. 边界文档化（ARCHITECTURE.md）

- [ ] 2.1 新增/强化「Transport HTTP 边界」小节：明确 `openlark_core::http::Transport<T>` 是边界、业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型
- [ ] 2.2 记录 webhook by-design 例外：无鉴权推送器、进程级共享 `reqwest::Client` 复用连接池（引用 #214），不算分层泄漏
- [ ] 2.3 将「Transport 中间件 / 熔断 / 智能重试中间件」草案显式标注为「规划中 / future change / 当前未实现」，与实际（RetryPolicy 配置模式、无中间件链）对齐

## 3. webhook crate 文档注释强化

- [ ] 3.1 强化 `crates/openlark-webhook/src/robot/v1/` 中 `shared_client`/`client` 的文档注释，显式说明「直接 reqwest 是 by-design 边界例外」并指向 ARCHITECTURE.md 边界约定（不改源码逻辑）

## 4. CHANGELOG

- [ ] 4.1 在 CHANGELOG.md v0.18 段补充 hygiene 条目：12 个业务 crate 移除未用 reqwest 依赖声明（非 breaking）

## 5. 验证

- [ ] 5.1 `cargo build --workspace --all-features` exit 0（删依赖后全量构建通过）
- [ ] 5.2 三组 feature clippy（default / `--all-features` / `--no-default-features` + `-D warnings`）均 exit 0
- [ ] 5.3 `cargo test --workspace` 全部通过（0 failed）
- [ ] 5.4 grep 双重确认：12 crate 的 `Cargo.toml` 与 `src/` 中 `reqwest` 命中数均为 0；core/client/webhook 三者 Cargo.toml 仍保留 reqwest
```

## openspec/changes/clarify-transport-reqwest-boundary/specs/transport-reqwest-boundary/spec.md

- Source: openspec/changes/clarify-transport-reqwest-boundary/specs/transport-reqwest-boundary/spec.md
- Lines: 1-75
- SHA256: f34c6b1336f78686f32a04e84a2777c0dd19f0b2ce741ecb0865693fe28e0cec

```md
## ADDED Requirements

### Requirement: 业务 crate 不直接依赖 reqwest
openlark 业务 crate SHALL 通过 core 的 `openlark_core::http::Transport<T>` 抽象（`Transport::request()`）发起 HTTP 请求，SHALL 不在各自 `Cargo.toml` 中声明 `reqwest` 依赖。仅 `openlark-core`（抽象本体）、`openlark-client`（客户端装配）与 `openlark-webhook`（by-design 性能例外，见下条）三个 crate 允许直接依赖 reqwest。

#### Scenario: 12 个业务 crate 的 Cargo.toml 不含 reqwest
- **WHEN** 在 `analytics / auth / bot / application / communication / mail / hr / docs / helpdesk / platform / user / workflow` 这 12 个 crate 的 `Cargo.toml` 中 grep `reqwest`
- **THEN** 命中数为 0（未用依赖声明全部移除）

#### Scenario: 业务 crate 源码不出现 reqwest 类型
- **WHEN** 在上述 12 个 crate 的 `src/` 中递归 grep `reqwest`
- **THEN** 命中数为 0（业务 crate 经 core `Transport` 发请求，不直接碰 reqwest）

#### Scenario: 允许的三个例外 crate 仍可声明 reqwest
- **WHEN** 检查 `openlark-core`、`openlark-client`、`openlark-webhook` 的 `Cargo.toml`
- **THEN** 三者保留 `reqwest = { workspace = true }`（core=抽象本体，client=装配，webhook=by-design 例外）

### Requirement: Transport 边界显式文档化
ARCHITECTURE.md SHALL 明确记录「`openlark_core::http::Transport<T>` 是 HTTP 边界：业务 crate 经 `Transport::request()` 发请求、不直接依赖 reqwest 类型」这一架构约定。

#### Scenario: ARCHITECTURE.md 含边界约定
- **WHEN** 在 `ARCHITECTURE.md` 中 grep `Transport` 与边界相关表述
- **THEN** 能定位到明确说明业务 crate 经 core `Transport` 发请求、不直接依赖 reqwest 的段落

### Requirement: webhook 直接 reqwest 作为 by-design 例外文档化
`openlark-webhook` 直接使用 `reqwest::Client`（进程级共享、连接池复用，见 GitHub issue #214）SHALL 作为 by-design 例外在 ARCHITECTURE.md 与 webhook crate 文档注释中显式记录，不被视为分层泄漏。

#### Scenario: ARCHITECTURE.md 记录 webhook 例外
- **WHEN** 在 `ARCHITECTURE.md` 中 grep `webhook`
- **THEN** 能定位到说明 webhook 直接 reqwest 是 by-design 例外（连接池复用 / 无鉴权推送器）的段落

#### Scenario: webhook crate 文档注释说明例外
- **WHEN** 在 `crates/openlark-webhook/src/robot/v1/` 中检查 `shared_client`/`client` 的文档注释
- **THEN** 注释显式说明「直接 reqwest 是 by-design 边界例外」并指向架构约定

### Requirement: Transport 边界由守卫脚本机器检验
「业务 crate 不直接依赖 reqwest」这一边界 SHALL 由 `tools/check_reqwest_boundary.sh` 守卫脚本机器检验（匹配 `check_no_dead_code_allows.sh` 先例），并接入 justfile 与 CI lint job，防止未来 PR 重新引入违规依赖。白名单仅 `openlark-core`、`openlark-client`、`openlark-webhook`。

#### Scenario: 守卫脚本存在
- **WHEN** 检查 `tools/check_reqwest_boundary.sh` 是否存在且可执行
- **THEN** 文件存在、含 `set -euo pipefail`、白名单含 core/client/webhook

#### Scenario: 清理后守卫通过
- **WHEN** 12 个业务 crate 的 Cargo.toml 移除 reqwest 后运行 `bash tools/check_reqwest_boundary.sh`
- **THEN** exit 0（业务 crate 无 reqwest 直接依赖）

#### Scenario: 守卫能抓回归
- **WHEN** 任一业务 crate 的 Cargo.toml 重新声明 `reqwest` 后运行守卫
- **THEN** exit 1 并列出违规的 Cargo.toml 路径（防止边界被未来 PR 破坏）

#### Scenario: 守卫接入 CI 与 justfile
- **WHEN** 检查 `justfile` 与 `.github/workflows/ci.yml`
- **THEN** 分别存在 `reqwest-boundary` recipe 与 CI lint job 的一步调用 `tools/check_reqwest_boundary.sh`

### Requirement: ARCHITECTURE.md 中间件层标注为 future
ARCHITECTURE.md 中「Transport 中间件 / 熔断 / 智能重试中间件」相关设计 SHALL 显式标注为「规划中 / future change、当前未实现」，与实际实现（RetryPolicy 配置模式、无中间件链）对齐，消除文档与现实的偏差。

#### Scenario: 中间件草案标注未实现状态
- **WHEN** 在 `ARCHITECTURE.md` 中定位中间件/熔断/重试中间件相关段落
- **THEN** 段落显式标注为规划中 / future / 当前未实现（不再以已实现口吻描述）

### Requirement: 清理不破坏构建、lint 与测试
本次移除 12 个未用 reqwest 依赖 SHALL 不导致 workspace 构建、clippy 或测试失败，SHALL 不改变任何公开 API。

#### Scenario: 全 feature 构建通过
- **WHEN** 运行 `cargo build --workspace --all-features`
- **THEN** exit 0（删依赖后全量构建成功）

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: 测试通过
- **WHEN** 运行 `cargo test --workspace`
- **THEN** 全部通过（0 failed；本次为纯依赖声明 + 文档变更，无行为影响）
```

