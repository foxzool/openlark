---
comet_change: type-okr-v2-responses
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-04-type-okr-v2-responses
status: final
---

# Design: type-okr-v2-responses

## Context

`openlark-hr` 的 8 个域 seam 分裂：okr/v2 是唯一带完整资源访问器链的域（`HrClient.okr.v2().<resource>().<leaf>().execute()` 可导航），但 25 叶 `execute()` 双向 `serde_json::Value`；另 7 域 typed Response 但不可经 facade 到达（深路径）。用户被迫在"可导航但全无类型"与"有类型但写深路径"间二选一。

**Value 根因（已查实）**：飞书 apiSchema 信息极其丰富（含完整 `responses["200"]...properties` 字段 schema），但 codegen MVP 刻意先输出 Value（`SCHEMA_FINDINGS.md`："typed response 数据齐全（MVP 仍先 Value，IR 完整解析存起来）"）。即数据齐全、生成被推迟。本 change 把这推迟的 typed 生成补回 okr/v2。

**pilot 端到端验证**（`objective/get`，api_id `7644764969658567628`）：
```
schema_cache.get_or_fetch(api)  # source=fetch，零鉴权 urllib 拉 open.feishu.cn/document_portal/v1/document/get_detail
→ apiSchema.responses["200"].content.application/json.schema.properties = [code, msg, data]
→ data.properties = [objective(object)]
```
网络可达、schema 程序化可得、response 字段结构化。typed Response 即 `data` 的 shape（`GetObjectiveResponse { objective: Objective }`），envelope `{code,msg}` 由现有 `ApiResponseTrait::ResponseFormat::Data` 提取（对齐 attendance `CreateGroupResponse`）。

约束：okr/v2 是零外部引用的导航终点（#327 确认 okr 导航链零跨 crate 引用）；v0.18 breaking 窗口。

## Goals / Non-Goals

**Goals:**
- okr/v2 25 叶 `execute()` 返回 typed Response（从 apiSchema 派生，非手工转录 doc）
- typed Response 字段精确匹配飞书 OKR v2 apiSchema
- 导航链全 typed（无 `serde_json::Value` 泄漏）

**Non-Goals:**
- 不给 7 个 typed 域补导航 accessor（lean 方向相反）
- 不改 okr/v2 导航链（`OkrV2 → *Resource → 叶子`）与端点 URL
- 不重构 okr/v1（已 typed）、不动其他 7 域
- 端点 enum 一致性（inline `format!` vs `OkrApiV2`）不在范围

## Decisions

### D1: Response struct 每叶 inline + 嵌套 struct
每叶在自身文件内定义 typed Response struct，嵌套 object 生成嵌套 struct（对齐 okr v1 inline 模式；codegen IR 支持 depth=3）。不抽 shared models.rs（okr/v2 各叶响应字段差异大，无共享结构）。

### D2: schema 源 = apiSchema（程序化派生，非 doc 转录）
每叶 `schema_cache.get_or_fetch(api)` 取 `responses["200"]...properties` → 推导 typed Response 字段（name/type/required/description）。这是 codegen 同源的权威结构化源，**非手工转录飞书 doc**。`openlark-api-field-verify` skill 降为辅助抽样核对（非主验证）。

> 这是相对 open 阶段 design.md D2 的关键升级：open 阶段写"飞书 doc 转录"，brainstorming 验证后发现 apiSchema 程序化可得且更可靠。

### D3: response typed 硬目标，请求 body 视成本
issue 验收聚焦 response。apiSchema 同样含 requestBody（body typing 同样可行）。决策：
- response typed = 硬目标（满足验收）
- 请求 body：单层简单 struct 一并 typed；嵌套复杂 body 保留 `serde_json::Value` + `// TODO: typed body` 注释，记为后续

### D4: 端点 enum 一致性 out of scope
叶子混用 inline `format!` 与 `OkrApiV2` enum。本 change 不动端点用法（统一是独立 concern，混入扩 diff、模糊主线）。各叶保持现有端点构造。

### D5: 6 资源批次滚动
按资源分 6 批实施 + 增量 build/test：
- alignment(2)：get, delete
- category(1)：list
- cycle(5)：list, objective/create, objective/list, objectives_position, objectives_weight
- indicator(1)：patch（写操作含 body）
- key_result(5)：get, delete, patch, indicator/list, progress/list
- objective(11)：get(试点已做), delete, patch, alignment/{create,list}, indicator/list, key_result/{create,list}, key_results_{position,weight}, progress/list

试点先做 objective/get 确立模板，其余 24 叶按 6 批铺开。

## 转换模式（每叶）

```rust
// 现状（objective/get.rs）
pub async fn execute(self) -> SDKResult<serde_json::Value> {
    let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
    let resp = Transport::request(req, &self.config, Some(option)).await?;
    resp.data.ok_or_else(...)
}

// 目标
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse {
    pub objective: Objective,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Objective {
    // 字段从 apiSchema data.objective.properties 派生
}

impl ApiResponseTrait for GetObjectiveResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}

pub async fn execute(self) -> SDKResult<GetObjectiveResponse> {
    let req: ApiRequest<GetObjectiveResponse> = ApiRequest::get(path);
    let resp = Transport::request(req, &self.config, Some(option)).await?;
    resp.data.ok_or_else(...)
}
```

## Risks / Trade-offs

- **[字段准确性]** → 缓解：apiSchema 是结构化权威源（codegen 已信任），非 doc 转录；可选字段 `Option<T>` + `#[serde(default)]` 宽容
- **[BREAKING 返回类型]** 25 叶 Value→typed → 缓解：okr/v2 零外部引用，v0.18 窗口
- **[嵌套 struct 深度]** → 缓解：apiSchema 提供完整嵌套；codegen IR depth=3 经验；`MAX_STRUCT_DEPTH=3` 兜底
- **[schema fetch 网络依赖]** → 缓解：`schema_cache` 持久缓存（fetch 一次后落盘 `.cache/`），build 阶段批量预取 25 schema 一次，后续离线可用
- **[范围蔓延]** body typing / enum 统一 → D3/D4 显式约束

## Test Strategy

- 每叶：fetch apiSchema → 定义 typed Response（+ 嵌套 struct）→ 改 execute 返回 typed → `cargo build/test`
- 试点 objective/get 先行，确立 inline typed 模板供 24 叶复用
- 全量：`cargo build/test -p openlark-hr --all-features` + `cargo fmt --check` + `cargo clippy --all-features --all-targets -D warnings`
- 25 叶 grep 无残留 `SDKResult<serde_json::Value>`
- `openlark-api-field-verify` 对 ≥3 叶跨资源抽样核对（辅助）
- 导航链 + 端点 URL 不变（git diff `okr/okr/v2/mod.rs` + 各叶 path 构造零改动）
- 跨 crate 引用回归（okr/v2 typed Response 未破坏外部消费）
