## ADDED Requirements

### Requirement: HR 不暴露 deprecated service 访问器方法
openlark-hr 的 `Hr` 结构体 SHALL 不再提供 `attendance()/corehr()/compensation()/payroll()/performance()/okr()/hire()/ehr()` 兼容访问器方法。用户 SHALL 经字段访问（`client.attendance` 等）获取各业务模块客户端。

#### Scenario: HR crate 无 deprecated 访问器
- **WHEN** 在 `crates/openlark-hr/src/lib.rs` 中 grep `pub fn attendance|pub fn corehr|...|pub fn ehr`
- **THEN** 命中数为 0（8 个兼容访问器方法全部移除）

#### Scenario: 字段访问等价可用
- **WHEN** 用户以 default feature 构建，访问 `hr.attendance`（字段）
- **THEN** 返回 `Attendance` 客户端（与原 `hr.attendance()` 方法等价），编译通过

### Requirement: analytics SearchV2 不暴露 deprecated 未接线存根
openlark-analytics 的 `SearchV2` SHALL 不再提供 `query()`/`user()` deprecated 存根方法。

#### Scenario: SearchV2 无 query/user 存根
- **WHEN** 在 `crates/openlark-analytics/src/search/search/v2.rs` 中 grep `pub fn query|pub fn user`
- **THEN** 命中数为 0（2 个 deprecated 存根访问器移除）

#### Scenario: QueryApi/UserSearchApi 类型保留
- **WHEN** 移除存根访问器后构建 openlark-analytics
- **THEN** `v2/query.rs`、`v2/user.rs` 模块与 `QueryApi`/`UserSearchApi` 类型仍 `pub` 可用（经完整路径），仅便捷存根方法被移除

### Requirement: 移除不破坏构建与测试
本次移除 SHALL 不导致 default/full/no-default 任一 feature 组合的 clippy 或测试失败。

#### Scenario: 三组 feature clippy 通过
- **WHEN** 运行 `cargo clippy --workspace --all-targets` 分别以 default、`--all-features`、`--no-default-features` + `-D warnings`
- **THEN** 三组均 exit 0

#### Scenario: examples 不引用已移除方法
- **WHEN** 在 `examples/` 中 grep `\.attendance()|\.corehr()|\.query()` 等
- **THEN** 命中数为 0（examples 不使用已移除的访问器方法）
