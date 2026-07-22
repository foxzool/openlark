# Tasks

## 1. api_endpoints.rs — 6 端点 variant + to_url arm 改 Convention A

- [x] 1.1 `AttendanceApiV1::UserFlowGet` → `(String)`；arm `"/user_flows/{}"` → `format!("/user_flows/{user_flow_id}")`
- [x] 1.2 `AttendanceApiV1::FileDownload` → `(String)`；arm `"/files/{}/download"` → `format!("/files/{photo_id}/download")`
- [x] 1.3 `AttendanceApiV1::LeaveAccrualRecordPatch` → `(String)`；arm → `format!("/leave_accrual_record/{record_id}")`
- [x] 1.4 `AttendanceApiV1::LeaveEmployExpireRecordGet` → `(String)`；arm → `format!("/leave_employ_expire_records/{record_key}")`
- [x] 1.5 `AttendanceApiV1::UserStatsViewUpdate` → `(String)`；arm → `format!("/user_stats_views/{view_id}")`
- [x] 1.6 `FeishuPeopleApiV1::ProcessFormVariableDataGet` → `(String)`；arm → `format!("/corehr/v1/processes/{process_id}/form_variable_data")`

## 2. 6 叶子改传参给 variant（删 .replace）

- [x] 2.1 `attendance/v1/user_flow/get.rs`：`UserFlowGet(self.user_flow_id.clone()).to_url()`
- [x] 2.2 `attendance/v1/file/download.rs`：`FileDownload(self.photo_id.clone()).to_url()`
- [x] 2.3 `attendance/v1/leave_accrual_record/patch.rs`：`LeaveAccrualRecordPatch(self.record_id.clone()).to_url()`（record_id 后续入 body，须 clone）
- [x] 2.4 `attendance/v1/leave_employ_expire_record/get.rs`：`LeaveEmployExpireRecordGet(record_key).to_url()`（本地 record_key，move）
- [x] 2.5 `attendance/v1/user_stats_view/update.rs`：`UserStatsViewUpdate(self.view_id.clone()).to_url()`（view_id 后续入 body，须 clone）
- [x] 2.6 `feishu_people/corehr/v1/process/form_variable_data/get.rs`：`CorehrApiV1::ProcessFormVariableDataGet(self.process_id.clone()).to_url()`

## 3. 验证

- [x] 3.1 `cargo build -p openlark-hr --all-features` 通过
- [x] 3.2 `cargo test -p openlark-hr --all-features` 通过（6 个 wiremock e2e URL 断言前后逐字一致 → 不变即过）
- [x] 3.3 HR 叶子无残留 `.replace("{}"` （grep = 0）
- [x]3.4 `cargo fmt --check` + clippy×2（--all-features / --no-default-features，-Dwarnings）
- [x]3.5 `cargo doc --workspace --all-features`（intra-doc）
- [x]3.6 `cargo machete`
