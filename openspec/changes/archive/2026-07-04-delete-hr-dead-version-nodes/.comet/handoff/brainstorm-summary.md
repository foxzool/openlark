# Brainstorm Summary

- Change: delete-hr-dead-version-nodes
- Date: 2026-07-04

## 确认的技术方案

删除 `openlark-hr` 11 个零资源 accessor 的死版本节点 struct + 11 个 returning facade accessor，修正 `lib.rs` facade doc example 指向 Config-direct Request 真实路径。

**删除映射（每 domain 同 commit：先删 facade accessor 后删 struct）：**
- attendance: `AttendanceV1` + `.v1()`（保留 `pub mod v1`）
- okr: `OkrV1` + `.v1()`（保留 `pub mod v1/v2`、`pub type OkrV2`、`.v2()`）
- ehr: `EhrV1` + `.v1()`
- hire: `HireV1`+`HireV2` + `.v1()`+`.v2()`
- corehr (feishu_people): `CorehrV1`+`CorehrV2` + `.v1()`+`.v2()`
- payroll: `PayrollV1` + `.v1()`
- performance: `PerformanceV1`+`PerformanceV2` + `.v1()`+`.v2()`
- compensation_management: `CompensationV1` + `.v1()`

每个 inner `<domain>/<domain>/mod.rs` 的同文件 generic serde_json tests 一并删（零 HR 断言价值），`pub mod vN` 声明保留（真实资源在其下）。

**doc example（lib.rs:15-27）：** 改 `rust,ignore` 谎言 `client.attendance.v1().group().create()` 为 `no_run` doctest，展示 `CreateGroupRequest::new(client.config().clone()).group_name(...)`（已验证 `.group_name(String)` 存在于 create.rs:96）。

## 关键取舍与风险

- **BREAKING 11 公开类型 + 11 方法**：已核实零跨 crate 引用 + 零 `pub use` re-export；v0.18 breaking 窗口（同 #325/#326/#329）
- **facade struct 保留**（8 个）：删 `.v1()` 后仅剩 `new()+config()`，但是 `HrClient` 的 `pub` 字段 + `test_hr_client_*_field` 测试依赖；`config` 字段被 `config()` 读取无 dead_code 风险
- **okr 例外依据**：`okr v2::OkrV2` 有 6 个真实资源 accessor（alignment/category/cycle/indicator/key_result/objective）是活类型 → 留；`okr v1` 资源是 Config-direct Request（`Request::new(config, args)`，非 Resource accessor 范式）→ `OkrV1` 真死、删而非修

## 测试策略

- 现有 8 个 `test_hr_client_*_field` 不调用 `.v1()`，删 accessor 后零破坏
- 删除的 generic serde_json tests 无 HR 断言，不丢覆盖率
- 验证：`cargo build/test/doc -p openlark-hr --all-features` + `cargo fmt --check` + `cargo clippy -p openlark-hr --all-features --all-targets -D warnings`
- 真实 Request（`CreateGroupRequest`/`QueryRequest` 等）删除前后行为不变的 grep 验证

## Spec Patch

无。delta spec `v1-sub-api-accessors` 已含足够验收场景（11 struct 移除、okr.v2 保留、真实路径不变、HR build/test、doc example 编译、不展示死链）。brainstorming 未发现缺失场景。
