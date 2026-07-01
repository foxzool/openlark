---
comet_change: fix-analytics-missing-docs
role: technical-design
canonical_spec: openspec
---

# Design: fix-analytics-missing-docs

> 承接 issue #273 missing_docs 深度治理子项 #1。移除 `openlark-analytics` 的 crate 级 `#![allow(missing_docs)]`，回补其被压制的 122 个未文档化公开项，并把 3 个 missing_docs 验证测试接进 CI。
>
> Canonical spec：`openspec/changes/fix-analytics-missing-docs/specs/lint-execution-consistency/spec.md`（delta）。本 Design Doc 不重复需求，仅记录技术方案。

## 1. Context

`openlark-analytics`（`crates/openlark-analytics/src/lib.rs:36`）有工作区里最后一个 crate 级 `#![allow(missing_docs)]`，actively 压制 **122 个** missing_docs 警告（实测：临时移除该行后 `cargo doc -p openlark-analytics --all-features` 报 122 warning）。

**警告分布（已勘探）**：

| 维度 | 分布 |
|------|------|
| 按 item 类型 | 68 struct（含 struct field）/ 36 method / 18 associated function |
| 按文件 | `search/v2/*`（14 API）+ `report/v1/*`（3 API），每文件约 6 项，`get.rs` 类 9–13 |
| 文件级 doc | **已在**：17 文件含 `//!` + Feishu `docPath`（仅 `doc_wiki/search.rs:2` 的 docPath URL 为空，待补） |
| 误报 | **无**：trait impl（如 `ApiResponseTrait::data_format`）不被 missing_docs 标记（docs 从 trait 继承）；122 全是合法公开 API |

前序 #273 Part B/A2/A1（PR #290/#291/#292）已清零其余 lint 维度。捕获此类 outlier 的 3 个 missing_docs Python 测试当前不在 CI（死测试，含 `test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions`——它正失败但 CI 不跑）。

**workspace 文档基准**：`crates/openlark-communication/src/contact/contact/old/default/v2/task/get.rs`——每项一行有意义中文，文件级 `//!` + docPath。

## 2. 目标 / 非目标

**目标**：移除 allow；回补 122 doc 至 workspace 规范；接 3 测试进 CI；更新 spec 移除 analytics 豁免。

**非目标**：不动 `#![allow(clippy::module_inception)]`（lib.rs:34）；不治理 1057 占位 doc（#3）；不修 codegen（#4）；不接其余 14 死测试（#2）；不升 workspace `deny`；不改 API 签名/行为。

## 3. 核心方案：doc recipe

**关键洞察**：122 项不是 122 个独立创作，而是 **~18 文件 × 同一 recipe**。每文件从自己的 `//!` 标题取 API 名，因此产出是 **API 特定的、有意义的**，不是占位符。

**Recipe**：`<item doc> = <文件 //! 标题> + <item 角色>`。

| item 类型 | doc 模板 |
|-----------|---------|
| Request struct | `/// <标题>请求。` |
| Response struct | `/// <标题>响应。` |
| Response named field | `/// <字段含义>。`（如 `data` → `/// 响应数据。`） |
| `pub fn new`（关联函数） | `/// 创建新的请求构建器。` |
| `pub async fn execute` | `/// 执行<标题>请求。` |
| `pub async fn execute_with_options` | `/// 使用指定请求选项执行<标题>请求。` |
| builder 方法（如 `query_param`） | `/// <动作>。`（如 `/// 添加查询参数。`） |

**docPath 位置（决策 A）**：只留文件级 `//!`，**不在 `execute` 方法 doc 里重复**。更简洁，文件级 `//!` 已是 rustdoc 导航主体。

**工作样例**（`search/v2/schema/create.rs`，标题="创建数据范式"，6 warning → 6 doc）：

```rust
//! 创建数据范式
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/schema/create>

/// 创建数据范式请求。
pub struct CreateSchemaRequest { config: Arc<Config> }

/// 创建数据范式响应。
pub struct CreateSchemaResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

// impl ApiResponseTrait for ... 不加 doc（trait 继承）

impl CreateSchemaRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self { ... }

    /// 执行创建数据范式请求。
    pub async fn execute(self) -> SDKResult<CreateSchemaResponse> { ... }

    /// 使用指定请求选项执行创建数据范式请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CreateSchemaResponse> { ... }
}
```

**质量保证**：
- API 特定（引用真实 API 名，非占位符）。
- 占位符守门（D2）：`grep -rnE '待补充文档|公开项说明' crates/openlark-analytics/src/` MUST 为空。
- Pilot 前置验证（见 §4）。

## 4. 执行计划：pilot 先行 + 按域批量（决策 B）

```
Phase 1 — Pilot（主会话）
  └─ 回补 schema/create.rs 1 个文件 → 实跑 recipe → 自验 cargo doc grep 该文件无 warning
  └─ 审查产出符合 §3 样例 → 确认 recipe

Phase 2 — 批量回补（subagent-driven, per-domain）
  ├─ Group A: search/v2/data_source/* （create/get/patch/delete/list + item/{create,get,delete}）= 8 文件
  ├─ Group B: search/v2/schema/* （create/get/patch/delete）= 4 文件
  ├─ Group C: search/v2/{app/create, message/create, doc_wiki/search, user, query} = 5 文件
  │           （含补 doc_wiki/search.rs:2 空 docPath）
  └─ Group D: report/v1/* （rule/query, rule/view/remove, task/query）= 3 文件
  每 subagent：读本文件 //! 标题 → 套 recipe → 自验 cargo doc grep 该文件无 warning

Phase 3 — 收尾（主会话）
  ├─ 移除 lib.rs:36 #![allow(missing_docs)]（保留 line 34 module_inception）
  ├─ CI 接线：ci.yml 现有 test_check_mod_reachability 旁加 test_workspace_missing_docs
  └─ 全局验证（见 §6）
```

**粒度选择**：per-domain（3-4 组）而非 per-file（18）。调度/审查开销低，域内 coherent。极限并行度非目标。

## 5. 决策与替代方案

| 决策 | 选择 | 理由 | 否决的替代 |
|------|------|------|-----------|
| A. docPath 位置 | 只留文件级 `//!` | 简洁无冗余；rustdoc 导航主体已是文件级 | execute 方法里重复 docPath（communication 风格）—— 冗余 |
| B. 回补粒度 | pilot + per-domain 3-4 组 | 低调度开销 + 域内 coherent + pilot 守质量 | per-file 18 subagent（开销大）/ 主会话顺序做（无中间守门） |
| C. test #1 进 CI | 接受其 `cargo test --workspace --no-run` 成本 | 唯一 rustdoc 层 missing_docs 守门 | 拆独立 job（过度设计）/ 只接 2 个扫描测试（守门失效） |
| D. trait impl doc | 不加（继承 trait） | rustdoc 规则；不加即合规 | 给 trait impl 方法补 doc —— 冗余且无意义 |
| E. 占位符 | 禁止 | 正是 #3 问题的来源 | 允许 `/// 待补充文档。` 撑 0 警告 |

## 6. 测试策略

| 层级 | 验证 |
|------|------|
| 单文件（subagent 自验） | `cargo doc -p openlark-analytics --all-features 2>&1 \| grep <file>` 无 warning |
| crate 级 | `cargo doc -p openlark-analytics --all-features` 0 警告 |
| workspace 级 | `cargo doc --workspace --all-features` missing_docs=0；`cargo clippy --workspace --all-targets --all-features -- -Dwarnings` 通过 |
| 行为测试 | `python3 -m unittest tools.tests.test_workspace_missing_docs` 3 测试全绿 |
| 占位符守门 | `grep -rnE '待补充文档\|公开项说明' crates/openlark-analytics/src/` 为空 |
| 回归 | `cargo test -p openlark-analytics` 现有测试不破；`just lint` 通过 |

## 7. 风险与缓解

- **[122 项诱发占位符]** → recipe 引用真实 API 名 + D2 grep 守门 + pilot 前置审查。
- **[test #1 CI 时长]** → 接受（决策 C）；CI 复用编译产物；监控 lint job，若超标再拆。
- **[doc 准确性]** → 每文件 doc 派生自该文件已正确的 `//!` 标题，无需额外 Feishu 调研。
- **[批量 subagent 风格漂移]** → pilot 定锚 recipe；per-domain 分组降低跨 subagent 一致性压力；主会话全局 cargo doc 验证兜底。

## 8. 迁移与回滚

纯增量、非破坏性，无 API/数据迁移。顺序：回补 doc（使移除 allow 后无新警告）→ 移除 allow → CI 接线 → 验证。回滚 = revert PR。

## 9. Open Questions / Build 阶段决策

- **build_mode**：subagent-driven-development（per-domain 分组天然适合）vs 直接执行 → build 阶段 plan-ready 暂停由用户选定。
- **isolation**：branch（默认）vs worktree → build 阶段选定。
- **tdd_mode**：本 change 是 doc 回补 + 配置，无逻辑代码；tdd 不强制，build 阶段选定 direct 或 tdd（doc/CI 改动用 direct 即可）。
