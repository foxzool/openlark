# Comet Design Handoff

- Change: fix-analytics-missing-docs
- Phase: design
- Mode: compact
- Context hash: 465b7c93a145fa814f4c5f790265e17c6ed5f33d5afe4901cd4d5ea2e9acf13f

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/fix-analytics-missing-docs/proposal.md

- Source: openspec/changes/fix-analytics-missing-docs/proposal.md
- Lines: 1-30
- SHA256: a44a43932ba421a2558debb0f183458f024dc2ae2afda31094ed29681fb8dc1d

```md
## Why

`openlark-analytics` 是工作区里最后一个 crate 级 `#![allow(missing_docs)]` outlier（`crates/openlark-analytics/src/lib.rs:36`），actively 隐藏 **122 个**未文档化公开项（68 struct / 36 method / 18 associated fn）。这是 issue #273 missing_docs 深度治理的最后一块——前序 Part B/A2/A1（PR #290/#291/#292）已清零其余 lint 维度，仅剩 analytics 因“移除须回补 doc”被 `lint-execution-consistency` spec 的范围边界条款显式延期。与此同时，捕获这类 outlier 的 3 个 missing_docs Python 测试当前不在 CI 运行（死测试，给虚假强制感）。现在收口。

## What Changes

- 移除 `crates/openlark-analytics/src/lib.rs:36` 的 `#![allow(missing_docs)]`，回落 workspace 级 `warn`（与其余 crate 单一来源一致）。
- 回补 analytics 全部 122 个缺失文档的公开项，符合 workspace 既有文档规范（`communication` crate 风格：每个 API 文件含文件级 `//!` 描述 + Feishu `docPath`，每个 struct/field/method 一行有意义中文）。
- 把 `tools/tests/test_workspace_missing_docs.py` 的 3 个测试接进 CI（`.github/workflows/ci.yml`），让 missing_docs 强制可执行、可回归。
- 更新 `lint-execution-consistency` spec：移除 analytics 范围边界豁免条款，新增“missing_docs 验证测试 MUST 在 CI 运行”要求。

**非破坏性**：仅新增文档 + CI 接线 + 移除 1 行抑制属性；不动公开 API 签名或运行时行为。

## Capabilities

### New Capabilities

（无——本 change 复用既有 `lint-execution-consistency` capability，收口其延期条款。）

### Modified Capabilities

- `lint-execution-consistency`: 移除 analytics outlier 的范围边界豁免（让“crate 级 allow/deny outlier MUST 清理”要求对 analytics 也生效）；新增“missing_docs 验证测试 MUST 在 CI 运行”要求，消除死测试的虚假强制感。

## Impact

- **代码**：`crates/openlark-analytics/src/**` 约 40 个 `.rs` 文件（回补 122 项 doc）；`lib.rs` 移除 1 行 `#![allow]`。
- **CI**：`.github/workflows/ci.yml` 增加 3 个 missing_docs 测试的执行步骤。
- **Spec**：`openspec/specs/lint-execution-consistency/spec.md` delta（移除豁免 + 新增 CI 要求）。
- **测试**：`tools/tests/test_workspace_missing_docs.py` 的 3 个测试从“死测试”转正为 CI 强制。
- **依赖 / 公开 API**：无变更。
```

## openspec/changes/fix-analytics-missing-docs/design.md

- Source: openspec/changes/fix-analytics-missing-docs/design.md
- Lines: 1-63
- SHA256: 6ca946585be4178c853968702d27270c15e84c45b84da5a6feecbf5c0e7cd025

```md
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
```

## openspec/changes/fix-analytics-missing-docs/tasks.md

- Source: openspec/changes/fix-analytics-missing-docs/tasks.md
- Lines: 1-34
- SHA256: 210cef998cd9ff5773799660ab42206d6c72359dddc02ec9678166cdb46b3d70

```md
## 1. 基线与准备

- [ ] 1.1 记录移除 allow 前的 missing_docs 基线：122 项（68 struct / 36 method / 18 associated fn），分布在 search/v2 与 report/v1 的 API 文件（实测 `cargo doc -p openlark-analytics --all-features` 输出）。
- [ ] 1.2 确认 analytics 文件级 `//!` + Feishu `docPath` 已覆盖（17 文件）；标记 `search/v2/doc_wiki/search.rs:2` 空 docPath 待补。

## 2. 回补 analytics 公开项文档（122 项，全在 search/ + report/ 叶子 API 文件）

> 每个文件按 D1 模板回补：Request/Response struct + named field + `new`/`execute`/关联函数各一行有意义中文，对齐 `communication/contact/.../task/get.rs` 规范。文件级 `//!`+docPath 已在，不重写。

- [ ] 2.1 回补 `search/v2/data_source/*`：`create/get/patch/delete/list` + `item/{create,get,delete}`。
- [ ] 2.2 回补 `search/v2/schema/*`：`create/get/patch/delete`。
- [ ] 2.3 回补 `search/v2/{app/create, message/create, doc_wiki/search, user, query}`；**补全 `doc_wiki/search.rs:2` 空 docPath**。
- [ ] 2.4 回补 `report/v1/*`：`rule/query`、`rule/view/remove`、`task/query`。
- [ ] 2.5 逐文件自验：每补完一组运行 `cargo doc -p openlark-analytics --all-features 2>&1 | grep <该组文件>` 无新增 warning。

## 3. 移除 crate 级抑制

- [ ] 3.1 移除 `crates/openlark-analytics/src/lib.rs:36` 的 `#![allow(missing_docs)]`（**保留** line 34 `#![allow(clippy::module_inception)]`，不在范围）。
- [ ] 3.2 验证 `cargo doc -p openlark-analytics --all-features` 0 missing_docs 警告。

## 4. 占位符守门（D2）

- [ ] 4.1 `grep -rnE '待补充文档|公开项说明' crates/openlark-analytics/src/` 输出为空（回补未引入占位符）。

## 5. CI 接线（D3）

- [ ] 5.1 在 `.github/workflows/ci.yml` 现有 `python3 -m unittest tools.tests.test_check_mod_reachability`（ci.yml:113）旁，加 `python3 -m unittest tools.tests.test_workspace_missing_docs`。
- [ ] 5.2 本地复现 CI 步骤：运行该模块，确认 3 个测试（`test_workspace_has_no_missing_docs_warnings` / `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions` / `test_workspace_item_level_missing_docs_exception_is_protocol_generated_module_only`）全绿。

## 6. 全局验证

- [ ] 6.1 `cargo doc --workspace --all-features` 无 missing_docs 警告（workspace 整体仍 0）。
- [ ] 6.2 `cargo fmt --check` 通过 + `just lint`（已无 `-A missing_docs`，会强制 missing_docs）通过。
- [ ] 6.3 `cargo test -p openlark-analytics` 现有测试不破。
```

## openspec/changes/fix-analytics-missing-docs/specs/lint-execution-consistency/spec.md

- Source: openspec/changes/fix-analytics-missing-docs/specs/lint-execution-consistency/spec.md
- Lines: 1-43
- SHA256: 1c1ef5d79a833810b8c02b581e03e743e2044b11ce9313b36102e383c023e1e7

```md
## MODIFIED Requirements

### Requirement: missing_docs lint 治理收归 workspace.lints 单一来源（安全 outlier 收编）

`missing_docs` lint 的级别 MUST 由根 `Cargo.toml` 的 `[workspace.lints.rust]` 单一声明（当前 `warn`），各 crate 经 `[lints] workspace = true` 继承。crate 级 `#![deny(missing_docs)]` / `#![allow(missing_docs)]` 属性属于绕过 workspace 的 outlier，MUST 清理（回落 workspace 级别），唯一例外是已登记的 vendored 生成模块 item 级 `#[allow]`。被 `allow` 压制的未文档化公开项 MUST 回补文档至 workspace 规范，使移除 outlier 后 missing_docs 警告为 0。

> 本 requirement 现覆盖全部 crate（含 `openlark-analytics`）。analytics 的 crate 级 `#![allow(missing_docs)]`（曾为独立 change 延期）已由 change `fix-analytics-missing-docs` 收口——移除该 allow 并回补其被压制的未文档化公开项。

#### Scenario: security/client crate 级 outlier 已清

- **WHEN** 运行 `grep -rn 'deny(missing_docs)' crates/openlark-security/src crates/openlark-client/src`
- **THEN** 输出 MUST 为空（security 的 `#![deny]` 已移除回落 workspace warn；client 的死注释已删）

#### Scenario: analytics crate 级 allow outlier 已清

- **WHEN** 运行 `grep -rn '#!\[allow(missing_docs)\]' crates/`
- **THEN** 输出 MUST 为空（analytics 的 `crates/openlark-analytics/src/lib.rs` crate 级 `#![allow(missing_docs)]` 已移除；其被压制的未文档化公开项已回补 doc，`cargo doc -p openlark-analytics --all-features` 无 missing_docs 警告）

#### Scenario: protocol item 级例外保留

- **WHEN** 检查 `crates/openlark-protocol/src/lib.rs`
- **THEN** vendored pbbp2 生成模块的 item 级 `#[allow(missing_docs)]` MAY 保留（已登记例外，对应 `tools/tests/test_workspace_missing_docs.py` 的 item 级 allowlist 唯一条目）

#### Scenario: 移除 outlier 后 missing_docs 仍 0

- **WHEN** 运行 `cargo doc --workspace --all-features` 与 `cargo clippy --workspace --all-targets --all-features -- -Dwarnings`
- **THEN** missing_docs warning MUST 为 0（analytics 回补后无新警告暴露）

## ADDED Requirements

### Requirement: missing_docs 验证测试 MUST 在 CI 运行

`tools/tests/test_workspace_missing_docs.py` 的 missing_docs 验证测试（workspace 无 missing_docs 警告、无 crate 级 missing_docs 抑制、item 级抑制仅限 protocol 生成模块）MUST 在 CI（`.github/workflows/ci.yml`）执行，不得作为只在本地存在、CI 不跑的“死测试”。此约束消除虚假强制感，确保 missing_docs 治理（含 crate 级 `allow` outlier 的回归）被 CI 持续守门。

#### Scenario: missing_docs 验证测试在 CI 执行

- **WHEN** 检查 `.github/workflows/ci.yml`
- **THEN** MUST 包含执行 `tools/tests/test_workspace_missing_docs.py`（覆盖其全部测试方法）的步骤，与已有的 `test_check_mod_reachability` 同级运行

#### Scenario: crate 级 allow 回归被 CI 捕获

- **WHEN** 有人重新向任一 crate 引入 `#![allow(missing_docs)]`
- **THEN** CI 执行的 `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions` MUST 失败，阻断合入
```

