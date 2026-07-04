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

## Implementation Divergence

在 build 阶段，通过 `tools/schema_cache/cache.py` 程序化拉取飞书 `apiSchema`（零鉴权 urllib 拉 `open.feishu.cn/document_portal/v1/document/get_detail`）并缓存到 `tools/schema_cache/.cache/`。typed Response 字段实际从该 apiSchema 派生，而非手工转录 `docPath` 指向的飞书 SPA 文档。这一变更在 Superpowers Design Doc（`docs/superpowers/specs/2026-07-04-type-okr-v2-responses-design.md` D2）中记录为关键升级：apiSchema 是结构化权威源，比 SPA 文档更可靠；`openlark-api-field-verify` 降为辅助抽样核对。本 OpenSpec design.md 的 D2 仍保留“doc 转录”原始表述，实际实现采用 apiSchema 派生。偏差原因：brainstorming 阶段发现 apiSchema 结构化数据可直接获取，且能避免 playwright 渲染 SPA 的复杂性与不稳定性。
