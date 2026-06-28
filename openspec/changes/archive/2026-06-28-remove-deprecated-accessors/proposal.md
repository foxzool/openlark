## Why

10 个 `#[deprecated]` 兼容访问器方法 clutter API 表面，v1.0 breaking 窗口应移除：

- **HR（8）**：`Hr` 结构体的 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` —— v0.15.0 起标记 deprecated，提示「用字段直接访问 `client.attendance`」。方法式访问与字段访问并存致 API 混乱。
- **analytics（2）**：`SearchV2` 的 `query()/user()` —— 未接线的 runtime stub（deprecation note：「not wired to a Feishu endpoint, prefer implemented surfaces」）。

实证：这 10 个方法**零内部调用**（examples/tests/其他 crate 均未用方法式调用）→ 可干净移除，无内部迁移。

来源：架构审计 issue [#268](https://github.com/foxzool/openlark/issues/268)（20 个 deprecated 拆分；本 change 做其中 A+E 共 10 个最干净的，其余 B/C/D/F/G 10 个异构项另开 issue）。

## What Changes

- **BREAKING**：移除 HR 8 个 service 访问器方法（`attendance()`/`corehr()`/`compensation()`/`payroll()`/`performance()`/`okr()`/`hire()`/`ehr()`）。用户改用字段访问（`client.attendance` 等，字段已存在且等价）。
- **BREAKING**：移除 analytics `SearchV2::query()`/`user()` 存根方法。用户改用已实现的 search surfaces（`doc_wiki`/`schema`/`app`/`message`）。`QueryApi`/`UserSearchApi` 类型与模块（`v2/query.rs`、`v2/user.rs`）保留为 `pub`（仍可经完整路径访问），仅移除便捷存根访问器。

## Capabilities

### New Capabilities
- `no-deprecated-compat-accessors`: openlark SHALL 不暴露已废弃的兼容访问器方法；HR 业务模块 SHALL 经字段访问（`client.attendance`），analytics SHALL 经已实现的 search surfaces 而非未接线存根。

### Modified Capabilities
<!-- 无现有 spec 需要修改 -->

## Impact

- **openlark-hr**：`src/lib.rs` 删除 8 个 `#[deprecated] pub fn` 方法（各 behind feature gate）。
- **openlark-analytics**：`src/search/search/v2.rs` 删除 2 个 `#[deprecated] pub fn`（query/user）。
- **破坏性**：移除公开方法（已 deprecated ≥ 一个次版本）。外部 `client.attendance()` → `client.attendance`；`search_v2.query()` → 用其他 surface 或完整路径。
- **CHANGELOG**：记入 `[Unreleased] > Breaking Changes`，附迁移指引。
- **非目标**：不动 B/C/D/F/G（docs wiki 旧构造/macro new/to_value/im 别名/auth builder 方法）——另开 issue；不改字段访问本身；不新增 API；不移除 `QueryApi`/`UserSearchApi` 类型。
