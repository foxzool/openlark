# ADR: 官方文档证据集中为深模块

- **状态**: Accepted
- **日期**: 2026-08-11
- **决策者**: 架构评审 + 用户 grilling 共识
- **来源**: `/improve-codebase-architecture` 的 Official Document Evidence 候选

## 背景

官方飞书文档的获取、缓存、健康判定、结构解释与字段提取目前横跨 `tools/verify_api_fields.py`、`tools/api_contracts/official.py` 和 Playwright 脚本。单 API 与全量字段核对还分别实现抓取流程，使调用方必须理解文件缓存、页面长度、404 文本、section 结构和解析顺序等细节。连续修复曾分别处理抓取失败假绿、404 误判、空解析假绿、合法无字段假阳性和目录文本假绿，说明这些规则缺少一个可测试的职责边界。

## 决策

在 `tools/api_contracts/official_evidence/` 建立 Official Document Evidence 深模块。它以现有 `ApiIdentity` 表达的单个 Catalog Entry 为输入，隐藏官方来源获取、原始快照缓存、文档健康判定、结构解释、字段标准化和 provenance；输出按 Evidence Dimension 划分的不可变证据。Rust 合约比较、finding code、通过判定、批量调度、并发、进度和报告仍由调用方负责。

外部 interface 采用组合入口加单一 `collect()` 行为。调用方声明需要的维度和 Evidence Acquisition Policy；timeout/retry 等运行参数在 composition root 配置。Structured Detail、Rendered Document、Recorded Snapshot 与 snapshot store 是模块内部的 ports，生产环境可复用 HTTP 或浏览器生命周期，但调用方不能指定 parser、直接选择来源或混合 observation。

每个 Endpoint、Request Fields、Response Fields、Tokens 维度独立返回以下状态、provenance、observations、diagnostics 和 Acquisition Trail：

- `Trusted`：来源和健康已建立，相关结构解释成功；空 observations 是合法证据。
- `Incomplete`：权威且健康的文档只能被部分解释；保留诊断观察，但不能证明匹配。
- `Unavailable`：未能取得官方文档快照。
- `Rejected`：已取得的快照不满足来源或健康要求。

Strict Evidence Gate 下只有 `Trusted` 可以支持通过。若所有来源均非 Trusted，选择最有信息量的单一结果：`Incomplete` 优先于 `Rejected`，`Rejected` 优先于 `Unavailable`；同状态优先 Structured Detail。所有尝试保留在 Acquisition Trail，但 observations 只来自最终选定的一个来源。

Structured Detail 是首选来源；仅当某个请求维度无法获得 Trusted Evidence 时，才尝试 Rendered Document。回退成功可以产生 Trusted Evidence，同时保留首选来源的失败诊断。不同来源的 observations 不得静默合并。

缓存只保存不可变的原始 Official Document Snapshot，每次使用时重新执行健康判定和解释。快照 provenance 包含 Catalog Entry、实际来源、获取时间和内容摘要；解释 provenance 包含快照摘要、解释器 revision 和 Evidence Dimension。`Rejected` 快照被移除，`Unavailable` 不缓存。Recorded Snapshot 是带版本的原始 fixture，包含来源类型、Catalog identity、获取 provenance 和原始 structured/rendered 内容，绝不保存解析后的 Evidence。

Field Observation 使用规范化的层级路径，并携带 location、requiredness、type 和来源；来源未证明的属性保持 unknown，不作推断。当前比较器可以暂时只消费顶层字段。

预期的官方来源结果，例如网络超时、文档不存在、内容不健康或结构暂不支持，转换为 Evidence 状态并记录稳定、来源无关的 Evidence Diagnostic。无效 Evidence Request、snapshot-store I/O 失败、adapter 契约或内部 invariant 违例、解释器缺陷直接中止，不得伪装成证据不完整。

## 迁移与兼容性

字段核对与 API contract 验证两个调用方在同一批改造中切换到新模块，同时删除重复的抓取、缓存、健康检查、Catalog record/CSV loader、简化路径公式和针对旧内部 helper 的测试。现有 CLI 参数、退出码用途、finding codes 与 JSON 顶层结构保持不变；调用方将 Evidence Diagnostic 映射到既有 finding codes，并在报告中追加 Evidence 状态、provenance 和 trail。

CI 本批不安装 Playwright。模块测试使用 Recorded Snapshot；CI 的 live composition 仅提供 Structured Detail，若所需 rendered fallback 不可用，则记录 `Unavailable` 并严格失败。人工及 full composition 可使用 live Playwright。Recorded Snapshot 不得充当 fresh acquisition 的隐式回退。

验收必须覆盖：

1. Recorded Snapshot 驱动的 interface matrix，包括四种状态、合法零字段、逐维度回退、层级字段和缓存 policy；
2. 本地 adapter contracts，包括临时 snapshot store、HTTP mock 和 Playwright subprocess 失败；
3. 至少一个 live Structured Detail 与一个 live Rendered Document smoke；
4. 两个现有 CLI 的代表性端到端命令。

## 理由

该边界把反复变化的官方文档规则集中到一个可替换、可录制且可通过公共行为测试的模块。证据获取与业务裁决分离，使失败来源、证据置信度和“合法空字段”不再被压成同一个空集合；内部 ports 保留真实 HTTP、浏览器和离线 fixture 的替换能力，同时不把扩展机制暴露给调用方。

## 非目标

- 不把 Rust contract comparison、通用 validation run policy 或 `--strict` category isolation 纳入本模块。
- 不在本次工作中完成完整 Catalog provenance/path policy 深化；只复用现有 `ApiIdentity` 并删除字段核对侧重复实现。
- 不设计 batch API、parser registry、source override、跨来源 observation 合并或自动 verdict。
- 不改变 Playwright 作为 rendered production adapter，也不改变已批准的 Python 正则与结构解析技术选择。

