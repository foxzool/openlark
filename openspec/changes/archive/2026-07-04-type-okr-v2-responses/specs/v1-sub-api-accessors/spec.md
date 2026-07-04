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
