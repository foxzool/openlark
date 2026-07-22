## Why

`openlark-hr/src/common/api_endpoints.rs` 的端点枚举长期并存两套 path-param 约定：

- **Convention A（类型安全）**：variant 携带参数 → `GroupDelete(group_id) => format!("/.../{group_id}")`。参数编译期检查，缺参即编译失败。
- **Convention B（丢类型安全）**：unit variant + URL 字面 `{}` + 叶子 `.to_url().replace("{}", id)`。`{}` 是魔法串，叶子**可能忘记 `.replace`** → 带 `{}` 的坏 URL 静默发往飞书服务器（运行时 bug，编译期不可见）。

6 个端点用 Convention B：`AttendanceApiV1::{UserFlowGet, FileDownload, LeaveAccrualRecordPatch, LeaveEmployExpireRecordGet, UserStatsViewUpdate}` + `FeishuPeopleApiV1::ProcessFormVariableDataGet`（经 `CorehrApiV1` 别名消费）。B 严格劣于 A。URL 字符串迁移前后**逐字相同**（行为保持），纯属一致性 + 防回归收敛。

源自 `/improve-codebase-architecture` 候选 3（3605 行 endpoint 表）。初看是"shallow module"，grilling 核实后重塑为"path-param 约定不一致"——deletion test 下 enum 挣得位置（类型安全端点身份），故不做全量 macro 深化（payoff 部分已实现 + 532 arm 太大），只收敛这 6 个 B→A。

## What Changes

- 6 个 variant：unit → tuple `(String)`（参数名按叶子字段：`user_flow_id / photo_id / record_id / record_key / view_id / process_id`）
- 6 个 `to_url` arm：`"/.../{}".to_string()` → `format!("/.../{param}")`
- 6 个叶子：`Variant.to_url().replace("{}", &x)` → `Variant(x.clone()).to_url()`（本地 `record_key` 用 move）；删 `.replace`
- **BREAKING**：pub variant 形态变更（unit→tuple），但行为逐字保持（URL 同串）；0 外部消费者（外部用 leaf builder，不直接构 variant）
- 不动 Convention A 端点、不动 `/open-apis/` 前缀（532 arm 机械改、`API_PATH_PREFIX` 常量采纳另案——前缀是永不改变的飞书平台串，收益不抵成本）

## Capabilities

### New Capabilities
- `hr-endpoint-path-params`：HR 端点枚举的 path-param SHALL 由 variant 携带（Convention A），不得用 `{}` 占位 + 叶子 `.replace()`（Convention B）。

### Modified Capabilities
（无）

## Impact

- **crates/openlark-hr**（改动集中于此，不跨 crate）：
  - `src/common/api_endpoints.rs`：6 variant 定义 + 6 `to_url` arm
  - `src/attendance/attendance/v1/{user_flow/get, file/download, leave_accrual_record/patch, leave_employ_expire_record/get, user_stats_view/update}.rs` + `src/feishu_people/corehr/v1/process/form_variable_data/get.rs`：端点构造改传参
- **公开 API（v0.19.0 breaking）**：6 个 pub variant 形态 unit→tuple（零外部引用）
- **运行时行为**：逐字不变（6 个 wiremock e2e 测试断言的 URL path 前后一致）
- **依赖**：无新增、无删除
