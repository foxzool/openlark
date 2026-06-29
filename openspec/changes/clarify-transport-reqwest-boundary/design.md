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
