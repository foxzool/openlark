## Context

`openlark-analytics` 是工作区里最后一个 crate 级 `#![allow(missing_docs)]` outlier（`lib.rs:36`），actively 压制 **122 个**未文档化公开项（实测：移除该行后 `cargo doc -p openlark-analytics --all-features` 报 122 warning，分布为 68 struct / 36 method / 18 associated function）。前序 #273 Part B/A2/A1（PR #290/#291/#292）已清零其余 lint 维度；本 change 收口这最后一块，并把捕获此类 outlier 的 3 个 missing_docs Python 测试接进 CI。

**关键现状（已勘探）**：
- analytics 的 API 文件**已有文件级 `//!` + Feishu `docPath`**（17 文件，如 `search/v2/data_source/create.rs:1-2`）。缺的是 struct / field / method 级 `///`。
- 警告分布高度规律：每个 API 文件约 6 项（Request struct / Response struct / 字段 / `new` / `execute` / 关联函数），`get.rs` 类略多（9–13）。分布在 `search/search/v2/*`（14 API）+ `report/report/v1/*`。
- `doc_wiki/search.rs:2` 的 `//! docPath:` URL 为空（小瑕疵，回补该文件时一并修）。
- workspace 文档规范基准：`communication/contact/.../task/get.rs`——每项一行有意义中文，`execute` 可带 docPath 链接。
- `lib.rs:34` 另有 `#![allow(clippy::module_inception)]`（针对 `report/report`、`search/search` 嵌套命名），**与 missing_docs 无关，不在本 change 范围**。

## Goals / Non-Goals

**Goals:**
- 移除 analytics crate 级 `#![allow(missing_docs)]`，使其回落 workspace `warn`。
- 回补 122 个缺失 doc 至 workspace 规范（有意义一行中文，可引用已有 docPath）。
- 把 `test_workspace_missing_docs.py` 的 3 个测试接进 CI。
- 更新 `lint-execution-consistency` spec，移除 analytics 豁免。

**Non-Goals:**
- 不动 `#![allow(clippy::module_inception)]`（lib.rs:34）。
- 不治理全仓 1057 行占位 doc（#3 另案）；不修 codegen `-A`（#4 另案）；不接其余 14 个死测试（#2 另案）。
- 不升级 workspace 到 `deny(missing_docs)`（会固化占位 doc，违背 #3 治理方向）。
- 不改 analytics 公开 API 签名或运行时行为。

## Decisions

### D1: 回补策略 = 按 API 文件模板化，对齐 communication 规范

每个 API 文件按统一模板回补：Request/Response struct 各一行（如 `/// 创建数据源请求。`）、named field 各一行、`new`/`execute`/关联函数各一行。文件级 `//!`+docPath 已在，不重写（仅修 `doc_wiki/search.rs` 空 docPath）。**禁止占位符**（`/// 待补充文档。` / `/// 公开项说明。` 等）。

**Alternative**：机械批量生成无意义 doc —— 否决，正是 #3 占位 doc 问题的来源。

### D2: 占位符守门 = 回补后 grep 扫描改动的 analytics 文件

在 build 验证里加一步：`grep -nE '待补充文档|公开项说明' crates/openlark-analytics/src/` MUST 为空。防止 122 项的规模诱使偷懒塞占位符。

### D3: CI 接线 = 在现有 python 测试步骤旁加一行，跑全部 3 个测试

`.github/workflows/ci.yml` 现有 `python3 -m unittest tools.tests.test_check_mod_reachability`（ci.yml:113）。紧邻加 `python3 -m unittest tools.tests.test_workspace_missing_docs`（跑该模块全部 3 个测试）。test #1（`test_workspace_has_no_missing_docs_warnings`）内部跑 `cargo test --workspace --all-features --no-run` 较重，但它是唯一在 rustdoc 层守 missing_docs 的关卡（clippy/build 不触发 missing_docs），且 CI 已有 workspace 编译产物可复用，成本可接受。

**Alternative A**：只接 2 个结构扫描测试（#2/#3），跳过 #1 —— 否决，#1 是唯一捕获 missing_docs 回归的测试，跳过则守门失效。
**Alternative B**：把 #1 拆到独立 job —— 过度设计，先简单接进现有步骤。

### D4: 构建执行建议 = subagent-driven（按文件独立单元）

122 项分布在约 18 个同构 API 文件，每个文件是独立单元（无跨文件状态），天然适合 `subagent-driven-development`：每个 subagent 处理一个 API 文件的回补 + 自验（该文件 `cargo doc` 0 警告）。最终主会话做移除 allow + CI 接线 + 全局验证。具体执行方式由 build 阶段 plan-ready 暂停时用户选定。

## Risks / Trade-offs

- **[122 项规模诱发占位符]** → D2 grep 守门 + code review；spec 已要求“回补至 workspace 规范”。
- **[test #1 增加 CI 时长]** → 接受（D3）；CI 复用编译产物；监控 lint job 时长，若超标再考虑 D3 Alternative B。
- **[回补 doc 准确性]** → 文件级 `//!` 已含 API 用途 + docPath，struct/method 描述直接派生自该上下文，无需额外 Feishu 调研；准确性由“符合文件级 `//!` 描述”保证。
- **[analytics 可能含非公开但被 lint 标记的项]** → 信任 rustdoc `missing_docs`（仅对 crate 公开 API 触发）；若 build 阶段发现误报再降级处理。

## Migration Plan

纯增量、非破坏性，无数据/API 迁移。回滚 = revert PR。顺序：先回补 doc（使 allow 移除后无新警告）→ 移除 allow → 接 CI → 验证 3 测试绿。

## Open Questions

- build 执行方式（subagent-driven vs 直接执行）→ build 阶段 plan-ready 暂停由用户选定（D4 建议 subagent-driven）。
- 是否在本 change 内顺手把 `doc_wiki/search.rs` 空 docPath 补全 → 是（D1 已含，属该文件回补范围）。
