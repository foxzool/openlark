---
comet_change: delete-hr-dead-version-nodes
role: technical-design
canonical_spec: openspec
archived-with: 2026-07-04-delete-hr-dead-version-nodes
status: final
---

# Design: delete-hr-dead-version-nodes

## Context

`openlark-hr` 的导航层存在两级 struct：

```
HrClient (lib.rs)
└── client.attendance : Attendance        ← facade struct（new + config + .v1()）
    └── .v1() → AttendanceV1              ← 版本节点 struct（new + config，零 accessor）
                 ❌ 无 .group() / .shift() / 任何资源访问器
```

真实资源**不在这条链上**。它们是 `attendance/attendance/v1/<resource>/<action>.rs` 里的自包含 Config-direct Request struct：

```
CreateGroupRequest::new(config: Config)   ← 直接吃 Config
    .group_name(String)                    ← builder（create.rs:96 已验证）
    .execute() -> SDKResult<Resp>          ← 自带 HTTP 执行
```

全 crate grep 证实：无任何 `pub fn group()` / `.shift()` / `.employee()` 资源 accessor 挂在 facade 或版本节点上。`Attendance → .v1() → AttendanceV1` 是转发到空的 Potemkin 死链。

**两种资源范式并存（解释选择性删除的依据）：**

| | 资源模式 | 版本节点状态 | 处置 |
|---|---|---|---|
| attendance / ehr / hire / corehr / payroll / performance / compensation 的 v1/v2 | Config-direct Request（`Request::new(config, args)`） | 死（无 Resource 层可服务） | **删版本节点** |
| okr v1 | Config-direct Request | 死（同上） | **删 OkrV1** |
| okr v2 | Resource accessor（`OkrV2 → AlignmentResource → 叶子`，6 个资源 accessor） | 活（Resource 图入口） | **留 OkrV2** |

11 个死版本节点 struct 经核实**零跨 crate 引用、零 `pub use` re-export、零测试引用**。`lib.rs:15-27` 的 facade doc example `client.attendance.v1().group().create()` 靠 `rust,ignore` 跳过编译——实际 `AttendanceV1` 无 `.group()`，是编译失败谎言。

约束：v0.18 breaking 窗口正开启（同 #325/#326/#329 批次）；死节点全部零外部引用。

## Goals / Non-Goals

**Goals:**
- 删除 11 个零资源 accessor 的死版本节点 struct + 11 个 returning facade accessor
- 修正 `lib.rs` facade doc example 为可编译检查形式（`no_run`），指向 Config-direct Request 真实路径
- 真实 API（`CreateGroupRequest` 等）路径与行为完全不变

**Non-Goals:**
- 不删 8 个 facade struct（`Attendance`/`Okr`/`Ehr`/`Hire`/`Corehr`/`Payroll`/`Performance`/`CompensationManagement`）——是 `HrClient` 的 `pub` 字段 + `test_hr_client_*_field` 测试依赖
- 不动 `okr.v2()` / `OkrV2` type alias——活类型
- 不重构"资源直达 Request"整体导航范式（更大后续工作）
- 不动真实 API Request/Response 模型、端点枚举、`v1/`/`v2/` 子模块资源代码

## Decisions

### D1: 文件级删除粒度——struct + impl + 同文件 generic tests，保留 `pub mod v1/v2`

每个 inner `<domain>/<domain>/mod.rs` 当前含：`pub mod v1;`(+`pub mod v2;`) + 死 struct + impl + `#[cfg(test)] mod tests`（generic serde_json 序列化测试，断言与 HR 无关）。

删除死 struct + impl + generic tests 块，**保留** `pub mod v1;`/`pub mod v2;`（真实资源在其下，必须可达）。generic tests 是 `serde_json::from_str::<Value>` 模板，零 HR 断言价值，且名义上是该 struct 的伴生测试；struct 删除后保留它们只增噪音。

### D2: okr 例外——保留 `pub type OkrV2 = v2::OkrV2;` 别名

`okr/okr/mod.rs` 同时含 `OkrV1`（死 struct，删）和 `pub type OkrV2 = v2::OkrV2;`（活类型别名，**保留**）。facade `okr/mod.rs` 的 `pub fn v2(&self) -> okr::OkrV2` 依赖此别名。仅删 `OkrV1` struct + impl，保留 `OkrV2` type alias + `pub mod v2;` + facade `.v2()`。

### D3: accessor 删除范围——11 个 returning 方法，保留 okr.v2

8 个 facade struct 删除 11 个 returning accessor：`attendance.v1` / `okr.v1` / `ehr.v1` / `hire.v1` / `hire.v2` / `corehr.v1` / `corehr.v2` / `payroll.v1` / `performance.v1` / `performance.v2` / `compensation_management.v1`。**保留** `okr.v2()`。

### D4: doc example 改为 Config-direct Request + `no_run` doctest

`lib.rs:15-27` 改为展示真实可达路径：

```rust
//! ```no_run
//! use openlark_hr::HrClient;
//! use openlark_hr::attendance::attendance::v1::group::CreateGroupRequest;
//! # let config: openlark_core::config::Config = unimplemented!();
//! let client = HrClient::new(config);
//! let _request = CreateGroupRequest::new(client.config().clone())
//!     .group_name("技术部".to_string());
//! ```
```

`no_run` 编译检查（验证类型/方法/路径真实可达）但不运行——避免需真实 config 与 tokio executor，满足 issue "doc example 编译通过" 验收，且不撒谎。`# hide` 行隐藏 config 占位。`.group_name(String)` 已对照 create.rs:96 验证存在。

否决 `rust,ignore`（issue 明确要求编译通过，ignore 等于不验证）与完整可运行 doctest（过度工程，lib.rs 顶层 doc 非端到端测试位置）。

### D5: facade struct 保留

8 个 facade struct 删 `.v1()`/`.v2()` 后仅剩 `new()` + `config()`，看似也成 Potemkin config 持有者。但保留：它们是 `HrClient` 的 `pub` 字段（`client.attendance`/`client.okr` 等，见 `lib.rs:80-110`），且 `test_hr_client_*_field` 测试依赖 `client.<domain>.config().app_id()` 断言。删除它们超 #327 范围，属后续"导航范式整体收敛"工作。`config` 字段被 `config()` 方法读取，无 dead_code 风险。

## 删除映射

| domain | 删 struct（inner mod.rs） | 删 facade accessor（domain/mod.rs） | 保留 |
|---|---|---|---|
| attendance | `AttendanceV1` + impl + tests | `.v1()` | `pub mod v1` |
| okr | `OkrV1` + impl + tests | `.v1()` | `pub mod v1`、`pub mod v2`、`pub type OkrV2`、`.v2()` |
| ehr | `EhrV1` + impl + tests | `.v1()` | `pub mod v1` |
| hire | `HireV1`+`HireV2` + impl + tests | `.v1()`+`.v2()` | `pub mod v1`、`pub mod v2` |
| corehr (feishu_people) | `CorehrV1`+`CorehrV2` + impl + tests | `.v1()`+`.v2()` | `pub mod v1`、`pub mod v2` |
| payroll | `PayrollV1` + impl + tests | `.v1()` | `pub mod v1` |
| performance | `PerformanceV1`+`PerformanceV2` + impl + tests | `.v1()`+`.v2()` | `pub mod v1`、`pub mod v2` |
| compensation_management | `CompensationV1` + impl + tests | `.v1()` | `pub mod v1` |

每 domain 同 commit 完成两处改动（先删 facade returning accessor，再删 inner struct），避免临时编译失败。

## Risks / Trade-offs

- **[BREAKING 公开 API]** 移除 11 个公开类型 + 11 个公开方法 → 缓解：已核实零跨 crate 引用、零 `pub use` re-export、零外部消费；v0.18 breaking 窗口正开启
- **[okr.v2 / OkrV2 alias 误删]** okr 文件改动时可能误删活的 `OkrV2` alias → 缓解：仅删 `OkrV1` struct + impl，保留 `pub type OkrV2` 与 `pub mod v2;`；build 后 `cargo build --all-features` 验证 `okr.v2()` 仍可达
- **[删除顺序致临时编译失败]** facade accessor 引用已删 struct → 缓解：同一 commit 内先改 facade、再改 inner；以最终 `cargo build --all-features` 通过为准
- **[doc example 引用错误方法]** `.group_name()` 已对照 create.rs:96 验证；若 build 阶段发现签名不符则调整示例

## Test Strategy

- 现有 8 个 `test_hr_client_*_field` 测试不调用 `.v1()`/`.v2()`（仅 `client.<domain>.config().app_id()`），删 accessor 后零破坏
- 删除的 8 个 generic serde_json tests 无 HR 断言价值，删除不丢有效覆盖率
- 验证命令（issue 验收四条）：
  - `cargo build -p openlark-hr --all-features`
  - `cargo test -p openlark-hr --all-features`
  - `cargo doc -p openlark-hr`（doctest 编译通过，不再 `rust,ignore`）
  - `cargo fmt --check` + `cargo clippy -p openlark-hr --all-features --all-targets -D warnings`
- 真实 API 路径未受影响：grep 确认 `CreateGroupRequest`/`QueryRequest` 等 Config-direct Request struct 与端点枚举未被改动
