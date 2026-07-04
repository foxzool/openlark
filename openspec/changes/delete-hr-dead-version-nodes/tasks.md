# Tasks

## 1. 删除死版本节点 struct + returning accessor（按 domain，每 task 原子可验证）

每个 domain task 内**先删 facade returning accessor、再删 inner struct**（避免临时编译失败），同 commit 完成。`pub mod v1;` / `pub mod v2;` 模块声明保留（真实资源在其下）。

- [ ] 1.1 **attendance**：删 `attendance/attendance/mod.rs` 中 `AttendanceV1` struct + impl + 同文件 generic tests（保留 `pub mod v1;`）；删 `attendance/mod.rs` 中 `pub fn v1(&self) -> attendance::AttendanceV1` 方法
- [ ] 1.2 **okr**：删 `okr/okr/mod.rs` 中 `OkrV1` struct + impl + generic tests（**保留** `pub type OkrV2 = v2::OkrV2;` 与 `pub mod v1;` `pub mod v2;`）；删 `okr/mod.rs` 中 `pub fn v1`（**保留** `pub fn v2`）
- [ ] 1.3 **ehr**：删 `ehr/ehr/mod.rs` 中 `EhrV1` struct + impl + generic tests（保留 `pub mod v1;`）；删 `ehr/mod.rs` 中 `pub fn v1`
- [ ] 1.4 **hire**：删 `hire/hire/mod.rs` 中 `HireV1` + `HireV2` struct + impl + generic tests（保留 `pub mod v1;` `pub mod v2;`）；删 `hire/mod.rs` 中 `pub fn v1` 与 `pub fn v2`
- [ ] 1.5 **feishu_people/corehr**：删 `feishu_people/corehr/mod.rs` 中 `CorehrV1` + `CorehrV2` struct + impl + generic tests（保留 `pub mod v1;` `pub mod v2;`）；删 `feishu_people/mod.rs` 中 `pub fn v1` 与 `pub fn v2`
- [ ] 1.6 **payroll**：删 `payroll/payroll/mod.rs` 中 `PayrollV1` struct + impl + generic tests（保留 `pub mod v1;`）；删 `payroll/mod.rs` 中 `pub fn v1`
- [ ] 1.7 **performance**：删 `performance/performance/mod.rs` 中 `PerformanceV1` + `PerformanceV2` struct + impl + generic tests（保留 `pub mod v1;` `pub mod v2;`）；删 `performance/mod.rs` 中 `pub fn v1` 与 `pub fn v2`
- [ ] 1.8 **compensation_management**：删 `compensation_management/compensation/mod.rs` 中 `CompensationV1` struct + impl + generic tests（保留 `pub mod v1;`）；删 `compensation_management/mod.rs` 中 `pub fn v1`

## 2. 修正 facade doc example

- [ ] 2.1 对照 `attendance/attendance/v1/group/create.rs` 验证 `CreateGroupRequest::new(config: Config)` 签名与 `.group_name(String)` builder 方法存在；将 `lib.rs:15-27` 的 `rust,ignore` example 改为 `no_run` doctest，展示 Config-direct 真实路径 `CreateGroupRequest::new(client.config().clone()).group_name(...)`，`# hide` 行隐藏 config 占位

## 3. 验证（issue 验收四条）

- [ ] 3.1 `cargo build -p openlark-hr --all-features` 通过
- [ ] 3.2 `cargo test -p openlark-hr --all-features` 通过（现有 facade 字段测试不破坏）
- [ ] 3.3 `cargo doc -p openlark-hr` 通过（修正后 doctest 编译通过，不再 `rust,ignore`）
- [ ] 3.4 `cargo fmt --check` + `cargo clippy -p openlark-hr --all-features --all-targets -D warnings` 通过
- [ ] 3.5 真实 API 路径未受影响验证：grep 确认 `CreateGroupRequest` / `QueryRequest` 等 Config-direct Request struct 与端点枚举未被改动
