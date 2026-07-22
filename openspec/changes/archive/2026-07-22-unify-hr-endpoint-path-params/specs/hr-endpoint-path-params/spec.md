## ADDED Requirements

### Requirement: HR 端点枚举 path-param SHALL 由 variant 携带

`openlark-hr` 端点枚举（`common/api_endpoints.rs`）中带路径参数的端点 SHALL 用 **variant 携带参数**的约定（Convention A：`Variant(param) => format!("/.../{param}")`，参数编译期检查），不得用 unit variant + URL 字面 `{}` 占位 + 叶子 `.to_url().replace("{}", id)` 的约定（Convention B）。Convention B 丢类型安全、`{}` 是魔法串、且叶子可能忘记 `.replace` 致带 `{}` 的坏 URL 静默发往服务器（运行时 bug，编译期不可见）。新增/重构带 path-param 的端点 SHALL 走 Convention A。

#### Scenario: 6 个原 Convention B 端点改为 variant 带参

- **WHEN** 变更后在 `crates/openlark-hr/src/common/api_endpoints.rs` 中检查 `UserFlowGet / FileDownload / LeaveAccrualRecordPatch / LeaveEmployExpireRecordGet / UserStatsViewUpdate / ProcessFormVariableDataGet`
- **THEN** 均为 tuple variant `(String)`，`to_url` arm 用 `format!` 绑定具名参数，无字面 `{}`

#### Scenario: 叶子不再 `.replace("{}")`

- **WHEN** 在 `crates/openlark-hr/src/` 中 grep `\.replace\("\{\}"`
- **THEN** 命中数为 0（path-param 经 variant 传入，不经字符串替换）

#### Scenario: URL 行为逐字不变

- **WHEN** 运行 6 个相关 wiremock e2e 测试（`test_attendance_v1_user_flow_get_*` / `*_file_download_*` / `*_leave_accrual_record_patch_*` / `*_leave_employ_expire_record_get_*` / `*_user_stats_view_update_*` / corehr `*_form_variable_data_get_*`）
- **THEN** 均通过；断言的请求 URL path 与迁移前逐字一致（参数已正确填入，非 `{}`）

#### Scenario: HR crate 编译与测试通过

- **WHEN** 运行 `cargo build -p openlark-hr --all-features` 与 `cargo test -p openlark-hr --all-features`
- **THEN** 均通过
