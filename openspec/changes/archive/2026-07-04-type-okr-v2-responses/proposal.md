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
