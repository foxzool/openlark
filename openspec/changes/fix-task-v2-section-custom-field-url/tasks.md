## 修复任务

- [x] T1: `common/api_endpoints.rs` — 10 个 variant 定义去掉 `tasklist_guid` 参数、URL 模板改为全局端点、同步 URL 断言
- [x] T2: `v2/section/` + `v2/custom_field/` 两组 CRUD（10 个 Request 文件）— 去掉 `tasklist_guid`、variant 调用改新签名；`CreateCustomFieldBody` 加 `resource_type`/`resource_id`，`create.rs` 的 `new(config, resource_id)` 自动填 `resource_type="tasklist"`；连带修 `update.rs` 的 PUT→PATCH（官方 section/custom_field 更新是 PATCH）
- [x] T3: `v2/section/mod.rs` + `v2/custom_field/mod.rs` — builder 去掉 `tasklist_guid` 与 `with_tasklist()`、CRUD 方法签名跟随；同步单测；修集成测试 `workflow_contract_models.rs` 的 `CreateCustomFieldBody` 构造
- [x] T4: `cargo fmt` + `cargo clippy --all-targets`（干净）；`cargo test -p openlark-workflow` 全绿（354 lib + 1 snapshot + 4 contract + 2 doctest）。validator 为路径制，文件路径未改故仍报 75 缺失（路径噪音，非 URL 问题，另案）；URL 正确性由 api_endpoints 单测断言守卫
