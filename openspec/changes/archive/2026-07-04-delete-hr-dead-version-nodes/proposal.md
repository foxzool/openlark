## Why

`openlark-hr` 有 11 个版本节点 struct（`AttendanceV1` / `OkrV1` / `EhrV1` / `HireV1` / `HireV2` / `CorehrV1` / `CorehrV2` / `PayrollV1` / `PerformanceV1` / `PerformanceV2` / `CompensationV1`）每个仅有 `new()` + `config()`，**零资源访问器**——它们持有的 `config` 既不向下传递也不暴露任何子 API。这是比 platform P1 场景（转发到真实资源）更糟的 Potemkin 死链：转发到空。

实地核查证实根因：HR 的真实资源是**自包含 Request struct**（如 `CreateGroupRequest::new(config)` / `QueryRequest::new(config)`），直接吃 `Config`、自带 builder + `execute()`，根本不经过版本节点链。因此 `Attendance::new(cfg).v1()` 返回的 `AttendanceV1` 无任何 `.group()` / `.shift()` 可调用——全 crate grep 不到任何资源 accessor。`lib.rs:15-27` 的 facade doc example `client.attendance.v1().group().create()` 是靠 `rust,ignore` 跳过编译的**谎言**。

11 个 struct 经核实**零跨 crate 引用、零测试引用**（仅被各自 facade 的 `.v1()`/`.v2()` 返回 + 自身定义引用）。v0.18 breaking 窗口正开启，是删除死导航层、让 doc 指向真实可达路径的时机。本 change 是补审 #313 的 confirmed 跟进。

## What Changes

- 删除 11 个死版本节点 struct + impl 块：`AttendanceV1` / `OkrV1` / `EhrV1` / `HireV1` / `HireV2` / `CorehrV1` / `CorehrV2` / `PayrollV1` / `PerformanceV1` / `PerformanceV2` / `CompensationV1`
- 删除 8 个 facade struct 上返回这些死节点的 11 个 accessor 方法：`attendance.v1()` / `okr.v1()` / `ehr.v1()` / `hire.v1()` / `hire.v2()` / `corehr.v1()` / `corehr.v2()` / `payroll.v1()` / `performance.v1()` / `performance.v2()` / `compensation_management.v1()`
- **BREAKING**：移除上述 11 个公开版本节点类型及其 returning accessor（零外部引用，删除无外部消费方成本）
- 修正 `lib.rs:15-27` facade doc example：从编译失败的 `client.attendance.v1().group().create()` 改为指向真实可达路径（直接构造 Config-direct Request），并改为可编译检查的 doctest 形式
- 保留 `okr.v2()`：`OkrV2 = type alias → v2::OkrV2` 是有真实资源 accessor 的活类型，非死节点
- 保留 8 个 facade struct 本身（`Attendance` / `Okr` / `Ehr` / `Hire` / `Corehr` / `Payroll` / `Performance` / `CompensationManagement`）：它们是 `HrClient` 字段 + 现有测试依赖的 config 持有者

## Capabilities

### New Capabilities
（无）

### Modified Capabilities
- `v1-sub-api-accessors`：新增 HR crate 专属 requirement。现有 requirement 要求 platform/ai 的 v1 入口**补全**链式 accessor 到叶子 builder；HR 是其逆命题——当真实资源为自包含 Config-direct Request struct（不经版本节点链）时，零资源 accessor 的版本节点 struct SHALL 连同其 returning accessor 一并删除，而非保留为 Potemkin 入口；facade doc SHALL 指向真实可达的 Request 构造路径，不得展示编译失败的链式调用。本 requirement 构成对 `v1-sub-api-accessors`「非破坏性补全」requirement 的 HR crate 专属例外：HR 因资源导航范式不同需要 breaking 删除零引用死节点，而非 platform/ai 的纯加法补全。

## Impact

- **crates/openlark-hr**（全部改动集中于此，不跨 crate）：
  - 11 个 `<domain>/<domain>/mod.rs`：删除死版本节点 struct + impl（`attendance/attendance/mod.rs`、`okr/okr/mod.rs`、`ehr/ehr/mod.rs`、`hire/hire/mod.rs`、`feishu_people/corehr/mod.rs`、`payroll/payroll/mod.rs`、`performance/performance/mod.rs`、`compensation_management/compensation/mod.rs`）
  - 8 个 `<domain>/mod.rs`：删除 returning accessor 方法（`attendance/mod.rs`、`okr/mod.rs`、`ehr/mod.rs`、`hire/mod.rs`、`feishu_people/mod.rs`、`payroll/mod.rs`、`performance/mod.rs`、`compensation_management/mod.rs`）
  - `src/lib.rs:15-27`：修正 facade doc example 指向真实可达路径
- **公开 API（v0.18 breaking）**：移除 11 个公开版本节点类型 + 11 个 facade accessor 方法
- **真实 API 路径**：不受影响——`CreateGroupRequest` / `QueryRequest` 等 Config-direct Request struct 删除前后均可达且行为不变
- **依赖**：无新增；无 endpoint/模型改动
