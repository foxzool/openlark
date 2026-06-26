## 修复任务

- [ ] T1: `common/api_endpoints.rs` — 10 个 variant 定义去掉 `tasklist_guid` 参数、URL 模板改为全局端点（见 design.md 映射表）、同步 `#[cfg(test)]` 里的 URL 断言（约 api_endpoints.rs:835-871, 959+）
- [ ] T2: `v2/section/` + `v2/custom_field/` 两组 CRUD（create/get/update/delete/list 共 10 个 Request 文件）— 去掉 `tasklist_guid` 字段、`new()` 签名、`TaskApiV2::*` 调用改为新 variant 签名；`custom_field/models.rs` 的 `CreateCustomFieldBody` 加 `resource_type`/`resource_id`，`create.rs` 的 `new(config, resource_id)` 自动填 `resource_type="tasklist"`
- [ ] T3: `v2/section/mod.rs` + `v2/custom_field/mod.rs` — builder 去掉 `tasklist_guid` 字段与 `with_tasklist()` 方法、5 个 CRUD 方法签名跟随；同步两组 builder 的单测
- [ ] T4: `just fmt && just lint`，`cargo test -p openlark-workflow`（及受影响 crate）确认全绿；validator 复跑确认这 9 项不再判缺失
