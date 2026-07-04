# Comet Design Handoff

- Change: type-okr-v2-responses
- Phase: design
- Mode: compact
- Context hash: ddf95eb62893bedec3fa7cdbc5b159ef7fd16624599a737ca7c778f36762a08b

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/type-okr-v2-responses/proposal.md

- Source: openspec/changes/type-okr-v2-responses/proposal.md
- Lines: 1-36
- SHA256: 76fa0138013f31b14b5fc0fc2dec93cb24da333742772109447ed5baf5ebbac5

```md
## Why

`openlark-hr` 的 8 个域 seam 分裂成两半，deep-module 接口的"可导航"与"类型安全"两项美德被劈开：

- **okr/v2**：唯一带完整资源访问器链的域（`HrClient.okr.v2().objective().get(id)` 可导航 ✅），但 25 个叶子的 `execute()` **双向 `serde_json::Value`**（写操作吃 Value body、返回 Value）❌
- **另 7 域**（attendance/ehr/hire/corehr/payroll/performance/compensation）：typed Response ✅，但不可经 facade 到达（用户须写 `crate::hire::hire::v1::offer_application_form::list::ListRequest::new(config)` 5 段深路径）❌

用户被迫在"可导航但全无类型"与"有类型但写深路径"间二选一。

非对称规模决定 lean 方向：okr/v2 仅 25 叶，另 7 域数百叶。故**把 okr/v2 拉入 typed 世界**（给 25 叶加 typed Response），而非给 7 域手写资源访问器——以最小改动收敛到"可导航 + typed"。这是补审 #313 confirmed 的新模式"导航工效 vs 类型安全二律背反"，是 #327（删 HR 死节点）的同族后续。

## What Changes

- 给 okr/v2 的 **25 个叶子**各加 typed Response struct（如 `objective::get::GetObjectiveResponse`），`execute()` 返回 `SDKResult<TypedResponse>` 而非 `SDKResult<serde_json::Value>`
- typed Response 字段对齐飞书 OKR v2 官方文档（每叶已有 `docPath` 链接，用 `openlark-api-field-verify` skill 抽样核对）
- 保留 okr/v2 已有导航链不变（`OkrV2 → {Alignment,Category,Cycle,Indicator,KeyResult,Objective}Resource → 叶子 Request`）
- **BREAKING**：25 叶 `execute()` 返回类型从 `serde_json::Value` 改为 typed Response（外部消费方需改类型；okr/v2 现为零外部引用的导航终点，影响可控）
- 写操作请求 body 是否一并 typed、端点 enum 一致性（叶子 inline `format!` vs `OkrApiV2`）留 design 阶段定夺

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `v1-sub-api-accessors`：新增 HR okr/v2 专属 requirement。现有 requirement 覆盖"版本节点 SHALL 暴露链式 accessor"（platform/ai 补全）与"HR 零 accessor 死节点 SHALL 删除"（#327）；本 change 补第三条——**navigable 链的叶子 SHALL 返回 typed Response**：经 facade 链式可达的版本节点（okr/v2）其叶子 `execute()` SHALL 返回与飞书官方文档字段一致的 typed Response struct，不得返回 `serde_json::Value`。构成对"非破坏性补全"的 HR 专属补充（typed 化是 RETURN 类型 breaking 变更，非纯加法）。

## Impact

- **crates/openlark-hr**（全部改动集中于此）：
  - `src/okr/okr/v2/<resource>/...` 25 个叶子：新增 typed Response struct（inline，对齐 okr v1 / attendance 模式）+ 改 `execute()` 返回类型 + 反序列化
  - 6 资源批次：alignment(2) / category(1) / cycle 含 objective 子树(6) / indicator(1) / key_result 含 indicator/progress(5) / objective 含 alignment/indicator/key_result/progress(10)
  - 不动 `v2/mod.rs` 资源 accessor 链、不动端点 URL、不动其他 7 域
- **公开 API（v0.18 breaking）**：25 叶 `execute()` 返回类型变更（Value → typed）
- **依赖**：无新增；沿用 `serde`/`openlark_core`
- **字段准确性**：typed Response 必须与飞书 OKR v2 官方文档一致（field-verify 抽样验证），否则静默破坏反序列化
```

## openspec/changes/type-okr-v2-responses/design.md

- Source: openspec/changes/type-okr-v2-responses/design.md
- Lines: 1-69
- SHA256: ab15c74d0bc6620677848a156da4df37b046f6cc3cd55c621e8b8cfad786bc0c

```md
## Context

`openlark-hr` okr/v2 是唯一带完整资源访问器链的域：`HrClient.okr.v2().objective().get(id).execute()` 可链式导航。但 25 个叶子的 `execute()` 双向 `serde_json::Value`：

```rust
// 现状（objective/get.rs）
pub async fn execute(self) -> SDKResult<serde_json::Value> {
    let req: ApiRequest<serde_json::Value> = ApiRequest::get(path);
    let resp = Transport::request(req, &self.config, Some(option)).await?;
    resp.data.ok_or_else(...)  // Value
}
```

对照 okr v1（typed 模式可参考）：
```rust
// okr v1（progress_record/get.rs）
pub async fn execute(self) -> SDKResult<GetResponse> {
    let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
    let response = Transport::request(request, &self.config, Some(option)).await?;
    response.data.ok_or_else(...)  // typed GetResponse
}
```

okr v1 typed Response 为**每叶 inline 定义**（无 shared models.rs），attendance 则用 `models.rs`。本 change 把 okr/v2 25 叶从前者（Value）转为后者（typed），消除"导航工效 vs 类型安全"二律背反。

约束：okr/v2 是零外部引用的导航终点（#327 已确认 okr 导航链零跨 crate 引用）；v0.18 breaking 窗口正开启。每叶已有 `docPath` 指向飞书官方文档。

## Goals / Non-Goals

**Goals:**
- okr/v2 25 叶 `execute()` 返回 typed Response（非 `serde_json::Value`）
- typed Response 字段与飞书 OKR v2 官方文档一致
- 导航链 `client.okr.v2().<resource>().<leaf>()` 全 typed

**Non-Goals:**
- 不给 7 个 typed 域补导航 accessor（lean 方向相反，规模 25 << 数百）
- 不改 okr/v2 已有导航链（`OkrV2 → *Resource → 叶子`）
- 不动其他 7 域、端点 URL、`OkrApiV2` enum 定义
- 不重构 okr/v1（已是 typed）

## Decisions

### D1: Response struct 每叶 inline 定义（对齐 okr v1 模式）
每叶在自身文件内定义 typed Response struct（+ 必要的子类型），不抽 shared `models.rs`。

**为什么**：okr v1（直接同族参考）即此模式，每叶自包含、字段与该 API 一一对应；抽 shared models.rs 会引入跨叶耦合且 okr v2 各叶响应字段差异大（objective vs alignment vs indicator 无共享结构）。attendance 的 models.rs 模式适合单资源多操作，okr/v2 是多资源多操作，inline 更合适。

### D2: schema 来源 = 飞书官方 doc（docPath）+ field-verify 抽样
每叶文件头已有 `docPath`（如 `https://open.feishu.cn/document/server-docs/okr-v2/objective/get`）。typed Response 字段从该 doc 转录。用 `openlark-api-field-verify` skill（playwright 渲染飞书 doc）对关键叶抽样核对字段名/类型。

**为什么**：`api_list_export.csv` 只含 API 元数据/URL，**无字段级 response schema**；飞书 doc 是唯一权威字段源。无 TS/其他 SDK 可直接 port。

### D3: 请求 body typing 视成本决定（response 优先满足验收）
issue 验收只要求"execute() 返回 typed Response"。10 个写操作现吃 `body: serde_json::Value`。

**决策**：response typed 化是硬性目标（满足验收）；请求 body typed 化在深设计阶段按"低成本则一并做、高成本则记为后续"原则决定。初步倾向：若某写操作的 body 字段简单且 doc 清晰则一并 typed，否则保留 Value body 并记 TODO。

### D4: 端点 enum 一致性 out of scope
叶子混用 inline `format!` 与 `OkrApiV2` enum。本 change **不动**端点用法（统一是独立的 navigation/endpoint 一致性问题，混入会扩大 diff、模糊 typed 化主线）。保持各叶现有端点构造方式。

### D5: 6 资源批次滚动
按资源分 6 批实施 + 验证：alignment(2) → category(1) → cycle 子树(6) → indicator(1) → key_result 子树(5) → objective 子树(10)。每批一个 commit，`cargo build/test` 增量验证。

## Risks / Trade-offs

- **[字段准确性]** typed Response 字段与飞书 doc 不一致会静默破坏反序列化 → 缓解：每叶对照 docPath 转录；field-verify skill 抽样核对关键叶；`#[serde(default)]` 对可选字段宽容处理
- **[BREAKING 返回类型]** 25 叶 `execute()` 返回 Value→typed → 缓解：okr/v2 零外部引用（#327 已确认导航链零跨 crate 引用），v0.18 breaking 窗口
- **[范围蔓延]** body typing / enum 统一可能拖大 diff → 缓解：D3/D4 显式 out-of-scope 或视成本，主线锁定 response typed
- **[doc 转录工作量]** 25 叶 × 字段转录是机械但量大 → 缓解：6 资源批次 + codegen.py（若可辅助）+ field-verify 自动化
```

## openspec/changes/type-okr-v2-responses/tasks.md

- Source: openspec/changes/type-okr-v2-responses/tasks.md
- Lines: 1-26
- SHA256: 05255436e16fbac7b3301ed3315cb5dec3bd2a4fe71e4a76cf4338880a986131

```md
# Tasks

## 1. 模板确立（1 叶试点）

- [ ] 1.1 选 `objective/get`（docPath 清晰：`/server-docs/okr-v2/objective/get`）做试点：用 `openlark-api-field-verify` skill 渲染飞书 doc 核对字段 → inline 定义 `GetObjectiveResponse`（+ 子类型，可选字段 `Option<T>` + `#[serde(default)]`）→ 改 `execute()`/`execute_with_options()` 返回 typed → `ApiRequest::<GetObjectiveResponse>::get(...)` → `cargo build/test -p openlark-hr` 通过。该叶成为其余 24 叶的模板

## 2. 按 5 资源批次铺开（每批一个 commit，增量 build/test）

每叶按模板：飞书 doc 转录字段 → inline typed Response → 改 execute 返回类型 → 反序列化。写操作 body 是否 typed 视成本（D3）。

- [ ] 2.1 **alignment**（2 叶：`alignment/get`、`alignment/delete`）
- [ ] 2.2 **category**（1 叶：`category/list`）
- [ ] 2.3 **cycle**（5 叶：`cycle/list`、`cycle/objective/create`、`cycle/objective/list`、`cycle/objectives_position`、`cycle/objectives_weight`）
- [ ] 2.4 **indicator**（1 叶：`indicator/patch`，写操作含 body）
- [ ] 2.5 **key_result**（5 叶：`key_result/get`、`key_result/delete`、`key_result/patch`、`key_result/indicator/list`、`key_result/progress/list`）
- [ ] 2.6 **objective 剩余 10 叶**（`objective/delete`、`objective/patch`、`objective/alignment/create`、`objective/alignment/list`、`objective/indicator/list`、`objective/key_result/create`、`objective/key_result/list`、`objective/key_results_position`、`objective/key_results_weight`、`objective/progress/list`）

## 3. 验证（issue 验收）

- [ ] 3.1 `cargo build -p openlark-hr --all-features` 通过
- [ ] 3.2 `cargo test -p openlark-hr --all-features` 通过（现有 test 不破坏）
- [ ] 3.3 `cargo fmt --check` + `cargo clippy -p openlark-hr --all-features --all-targets -D warnings` 通过
- [ ] 3.4 25 叶 grep 确认：`execute()` 返回类型无残留 `SDKResult<serde_json::Value>`（全 typed）
- [ ] 3.5 `openlark-api-field-verify` 抽样核对（≥3 叶跨资源）typed Response 字段与飞书 doc 一致
- [ ] 3.6 导航链与端点不变：`okr/okr/v2/mod.rs` 资源 accessor + 各叶端点 URL 路径零改动（git diff 确认）
- [ ] 3.7 跨 crate 引用回归：okr/v2 typed Response 未破坏外部消费（grep workspace）
```

## openspec/changes/type-okr-v2-responses/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/type-okr-v2-responses/specs/v1-sub-api-accessors/spec.md
- Lines: 1-30
- SHA256: 34206fb2c631e9198b6afd1e6558ae73f12810e8f5efd85ecb13626610077da6

```md
## ADDED Requirements

### Requirement: openlark-hr okr/v2 navigable 链叶子 SHALL 返回 typed Response

openlark-hr 中经 facade 链式可达的版本节点（`okr/v2`，由 `HrClient.okr.v2()` → `{Alignment,Category,Cycle,Indicator,KeyResult,Objective}Resource` → 叶子 `Request` 构成）其 25 个叶子的 `execute()` SHALL 返回与飞书 OKR v2 官方文档字段一致的 typed Response struct，不得返回 `serde_json::Value`。每个 typed Response SHALL 在叶子文件内 inline 定义（对齐 okr v1 模式），字段名/类型对齐该叶子 `docPath` 指向的官方文档；可选字段 SHALL 用 `Option<T>` + `#[serde(default)]` 宽容处理。本 requirement 构成对 `v1-sub-api-accessors` 现有「非破坏性补全」requirement 的 HR 专属补充：okr/v2 typed 化是 `execute()` 返回类型的 breaking 变更（Value → typed），非 platform 的纯加法。请求 body 是否 typed 不强制（write 操作可暂保留 `serde_json::Value` body）；端点 enum 一致性（inline `format!` vs `OkrApiV2`）不在本 requirement 范围。

#### Scenario: 25 叶 execute() 返回 typed Response

- **WHEN** 变更后检查 `crates/openlark-hr/src/okr/okr/v2/**/*.rs` 的 25 个叶子（alignment/category/cycle/indicator/key_result/objective 资源树下，排除 mod.rs）
- **THEN** 每叶 `execute()` 与 `execute_with_options()` 的返回类型为具体 typed Response struct（如 `objective::get::GetObjectiveResponse`），不再是 `SDKResult<serde_json::Value>`

#### Scenario: typed Response 字段对齐飞书官方文档

- **WHEN** 对关键叶子运行 `openlark-api-field-verify`（playwright 渲染该叶 `docPath` 指向的飞书 OKR v2 文档）对照 typed Response struct 字段
- **THEN** 字段名与类型与官方文档一致；可选字段为 `Option<T>` + `#[serde(default)]`；无静默反序列化破坏风险

#### Scenario: 导航链全 typed（无 Value 泄漏）

- **WHEN** 调用 `client.okr.v2().objective().get(id).execute()` 并接收返回值
- **THEN** 返回值为 typed `GetObjectiveResponse`（或等价 typed struct），整条导航链 `HrClient → OkrV2 → ObjectiveResource → Request::execute` 无 `serde_json::Value` 出现

#### Scenario: okr/v2 导航链与端点不变

- **WHEN** 变更后检查 `okr/okr/v2/mod.rs` 资源 accessor（alignment/category/cycle/indicator/key_result/objective）与各叶端点 URL 构造
- **THEN** 导航链结构（`OkrV2 → *Resource`）与端点 URL 路径（`/open-apis/okr/v2/...`）未变，仅叶子返回类型改变

#### Scenario: HR crate 编译与测试通过

- **WHEN** 运行 `cargo build -p openlark-hr --all-features` 与 `cargo test -p openlark-hr --all-features`
- **THEN** 均通过；现有 `test_hr_client_*` 与各叶 `builder_initializes` 测试不破坏；typed Response 的 `Deserialize` impl 由 serde 派生，无运行时反序列化错误
```

