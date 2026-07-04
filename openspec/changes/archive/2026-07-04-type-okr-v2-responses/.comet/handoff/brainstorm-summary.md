# Brainstorm Summary

- Change: type-okr-v2-responses
- Date: 2026-07-04

## 确认的技术方案

给 okr/v2 25 叶加 typed Response，`execute()` 返 typed（非 `serde_json::Value`）。

**关键验证（pilot objective/get 端到端跑通）**：
- schema 源 = `schema_cache.get_or_fetch(api)`（零鉴权 urllib 拉 `open.feishu.cn/document_portal/v1/document/get_detail`），网络可达、已缓存
- response 结构 = `{code, msg, data}`，`data` 含业务字段（objective/get 的 data = `{objective: object}`）
- 现有 typed 域（attendance）用 `ApiResponseTrait::data_format() -> ResponseFormat::Data` 提取 data，typed Response 即 data 的 shape（如 `GetObjectiveResponse { objective: Objective }`，`Objective` 为嵌套 struct）

**方案选型（3 选 1）**：
- ✅ **A 从 apiSchema 派生**（采用）：`schema_cache.get_or_fetch` 取 `responses["200"].properties` → 推导 typed Response（含嵌套 struct）。可靠、codegen 同源
- ✗ B 手工转录飞书 doc：脆弱、易错
- ✗ C 扩展 codegen.py 重生成：codegen MVP 刻意输出 Value、目标 crate 是 communication，扩到 hr+typed 工程量大

## 关键取舍与风险

- **D1 Response struct 每叶 inline**（对齐 okr v1 模式），嵌套 object 生成嵌套 struct（codegen IR 支持 depth=3）
- **D2 schema 源 = apiSchema**（非 doc 转录）；`openlark-api-field-verify` 降为辅助抽样核对
- **D3 请求 body**：apiSchema 同样有 requestBody（body typing 同样可行），但 issue 验收聚焦 response。决策：response typed 为硬目标；body 视成本（单层 struct 一并 typed，嵌套复杂 body 保留 Value + TODO）
- **D4 端点 enum 一致性 out of scope**（混入会扩 diff）
- **D5 6 资源批次**：alignment(2)/category(1)/cycle(5)/indicator(1)/key_result(5)/objective(11)

**风险**：
- 字段准确性 → 缓解：apiSchema 是结构化权威源（非转录），codegen 已信任
- BREAKING 返回类型（25 叶 Value→typed）→ okr/v2 零外部引用（#327 确认），v0.18 窗口
- 嵌套 struct 深度 → apiSchema 提供完整嵌套，codegen IR depth=3 经验可循

## 测试策略

- 每叶：fetch apiSchema → 定义 typed Response → 改 execute 返回 typed → build/test
- `cargo build/test -p openlark-hr --all-features` 全绿
- field-verify skill 对 ≥3 叶跨资源抽样核对（辅助，非主验证）
- 25 叶 grep 无残留 `SDKResult<serde_json::Value>`
- 导航链 + 端点 URL 不变（git diff 确认）

## Spec Patch

无（delta spec `v1-sub-api-accessors` 已含"navigable 链叶子 SHALL 返 typed Response"requirement + 5 场景）。brainstorming 未发现缺失场景；D2 schema 源细化（apiSchema 而非 doc）是实现细节，不改变 spec 语义。
