# Tasks

## 1. 模板确立（1 叶试点）

- [x] 1.1 选 `objective/get`（docPath 清晰：`/server-docs/okr-v2/objective/get`）做试点：用 `openlark-api-field-verify` skill 渲染飞书 doc 核对字段 → inline 定义 `GetObjectiveResponse`（+ 子类型，可选字段 `Option<T>` + `#[serde(default)]`）→ 改 `execute()`/`execute_with_options()` 返回 typed → `ApiRequest::<GetObjectiveResponse>::get(...)` → `cargo build/test -p openlark-hr` 通过。该叶成为其余 24 叶的模板

## 2. 按 5 资源批次铺开（每批一个 commit，增量 build/test）

每叶按模板：飞书 doc 转录字段 → inline typed Response → 改 execute 返回类型 → 反序列化。写操作 body 是否 typed 视成本（D3）。

- [x] 2.1 **alignment**（2 叶：`alignment/get`、`alignment/delete`）
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
