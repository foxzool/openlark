# Comet Design Handoff

- Change: delete-hr-dead-version-nodes
- Phase: design
- Mode: compact
- Context hash: a1249ae53a88ba2373ee073266d50f31750fe61c62cd5a37cc78bb110cae0095

Generated-by: comet-handoff.sh

OpenSpec remains the canonical capability spec. This handoff is a deterministic, source-traceable context pack, not an agent-authored summary.

## openspec/changes/delete-hr-dead-version-nodes/proposal.md

- Source: openspec/changes/delete-hr-dead-version-nodes/proposal.md
- Lines: 1-34
- SHA256: 7bb24afb37fd2caa7e79e0a2a271ab649aa5eef21457212b2ae6d18c1cb04b83

```md
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
```

## openspec/changes/delete-hr-dead-version-nodes/design.md

- Source: openspec/changes/delete-hr-dead-version-nodes/design.md
- Lines: 1-88
- SHA256: 7c3906b4df78f2dda9d26944b3896c9b2d020bb08135733d8941477c0acb0fda

[TRUNCATED]

```md
## Context

`openlark-hr` 的导航层有两级 struct：

```
HrClient (lib.rs)
└── client.attendance : Attendance        ← facade struct（new + config + .v1()）
    └── .v1() → AttendanceV1              ← 版本节点 struct（new + config，零 accessor）
                 ❌ 无 .group() / .shift() / 任何资源访问器
```

真实资源**不在这条链上**。它们是 `attendance/attendance/v1/<resource>/<action>.rs` 里的自包含 Request struct：

```
CreateGroupRequest::new(config: Config)   ← 直接吃 Config
    .group_name(...)                       ← builder
    .execute() -> SDKResult<Resp>          ← 自带 HTTP 执行
```

全 crate grep 证实：无任何 `pub fn group()` / `.shift()` / `.employee()` 资源 accessor 挂在 facade 或版本节点上。因此 `Attendance → .v1() → AttendanceV1` 是转发到空的 Potemkin 死链，11 个版本节点 struct 零跨 crate 引用、零测试引用。

`lib.rs:15-27` 的 facade doc example `client.attendance.v1().group().create()` 靠 `rust,ignore` 跳过编译——实际 `AttendanceV1` 无 `.group()`，是编译失败谎言。本 change 删除死层并让 doc 指向真实路径。

## Goals / Non-Goals

**Goals:**
- 删除 11 个零资源 accessor 的死版本节点 struct + impl + 11 个 returning accessor
- 修正 `lib.rs` facade doc example 为可编译检查形式，指向 Config-direct Request 真实路径
- 保持真实 API（`CreateGroupRequest` 等）路径与行为完全不变

**Non-Goals:**
- 不删 8 个 facade struct（`Attendance` / `Okr` / `Ehr` / `Hire` / `Corehr` / `Payroll` / `Performance` / `CompensationManagement`）——它们是 `HrClient` 字段 + `test_*_field` 测试依赖的 config 持有者，删除会破坏字段模式和测试
- 不动 `okr.v2()` / `OkrV2` type alias——`OkrV2 = v2::OkrV2` 是有真实资源 accessor 的活类型
- 不重构"资源直达 Request"的整体导航范式（更大的后续工作，超出 #327 范围）
- 不动真实 API Request/Response 模型、端点枚举、`v1/` `v2/` 子模块资源代码

## Decisions

### Decision 1: 文件级删除粒度——struct + impl + 同文件 generic tests，保留 `pub mod v1/v2`

每个 `<domain>/<domain>/mod.rs` 当前包含：`pub mod v1;` (+ `pub mod v2;`) + 死 struct + impl + `#[cfg(test)] mod tests`（generic serde_json 序列化测试，断言与 HR 无关）。

**决策**：删除死 struct + impl + 同文件 generic tests 块，**保留** `pub mod v1;` / `pub mod v2;` 模块声明（真实资源代码在其下，必须可达）。

**为什么**：generic tests 是 `serde_json::from_str::<Value>` 模板，零 HR 特定断言价值，且名义上是该 struct 的伴生测试；struct 删除后保留它们只增加噪音。模块声明必须保留以暴露真实资源。

**备选**：保留 generic tests → 否决（无价值噪音，与死代码清理意图相悖）。

### Decision 2: okr 例外——保留 `pub type OkrV2 = v2::OkrV2;` 别名

`okr/okr/mod.rs` 同时含 `OkrV1`（死 struct，删）和 `pub type OkrV2 = v2::OkrV2;`（活类型别名，**保留**）。facade `okr/mod.rs` 的 `pub fn v2(&self) -> okr::OkrV2` 依赖此别名。

**决策**：仅删 `OkrV1` struct + impl，保留 `OkrV2` type alias + `pub mod v2;`。build 阶段验证 `v2::OkrV2` 确有真实资源 accessor（确认其非死节点，issue 审计结论）。

### Decision 3: accessor 删除范围——11 个 returning 方法，保留 okr.v2

8 个 facade struct 删除 11 个 returning accessor：`attendance.v1` / `okr.v1` / `ehr.v1` / `hire.v1` / `hire.v2` / `corehr.v1` / `corehr.v2` / `payroll.v1` / `performance.v1` / `performance.v2` / `compensation_management.v1`。**保留** `okr.v2()`。

### Decision 4: doc example 改为 Config-direct Request + `no_run` doctest

`lib.rs:15-27` 改为展示真实可达路径——直接构造 Config-direct Request：

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

**为什么 `no_run`**：`no_run` 编译检查（验证类型/方法/路径真实可达）但不运行——避免需要真实 config 与 tokio executor，满足 issue "doc example 编译通过" 验收，且不撒谎（每行都是真实 API）。`# hide` 行隐藏 config 占位。`.group_name()` builder 方法名 build 阶段对照 `create.rs` 源码确认。

**备选 A** `rust,ignore`（现状）→ 否决（issue 明确要求编译通过，ignore 等于不验证）。
**备选 B** 完整可运行 doctest（带 tokio runtime + mock config）→ 否决（过度工程，lib.rs 顶层 doc 不是端到端测试位置）。

### Decision 5: facade struct 保留

```

Full source: openspec/changes/delete-hr-dead-version-nodes/design.md

## openspec/changes/delete-hr-dead-version-nodes/tasks.md

- Source: openspec/changes/delete-hr-dead-version-nodes/tasks.md
- Lines: 1-26
- SHA256: 765d4d042ef5d8ad05574c54bf973d7d858ecef7c036a823bd0d5f919c69079a

```md
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
```

## openspec/changes/delete-hr-dead-version-nodes/specs/v1-sub-api-accessors/spec.md

- Source: openspec/changes/delete-hr-dead-version-nodes/specs/v1-sub-api-accessors/spec.md
- Lines: 1-39
- SHA256: 991df062163234f379b4acf4a680aa26e302dcf48f9f9b3c672e5ed5f6e95ecc

```md
## ADDED Requirements

### Requirement: openlark-hr 零资源 accessor 死版本节点 SHALL 删除

当业务 crate 的真实资源为自包含 Config-direct Request struct（如 `CreateGroupRequest::new(config)`，直接持有 `Config`、自带 builder 与 `execute()`，不经版本节点链）时，零资源 accessor 的版本节点 struct SHALL 连同其 returning facade accessor 一并删除，而非保留为 Potemkin 入口。openlark-hr 的 11 个版本节点 struct（`AttendanceV1` / `OkrV1` / `EhrV1` / `HireV1` / `HireV2` / `CorehrV1` / `CorehrV2` / `PayrollV1` / `PerformanceV1` / `PerformanceV2` / `CompensationV1`）每个仅含 `new()` + `config()`、零资源访问器，且零跨 crate 引用、零测试引用，SHALL 删除；8 个 facade struct（`Attendance` / `Okr` / `Ehr` / `Hire` / `Corehr` / `Payroll` / `Performance` / `CompensationManagement`）上返回这 11 个死节点的 `.v1()` / `.v2()` accessor（共 11 个）SHALL 同步删除。本 requirement 构成对 `v1-sub-api-accessors` 现有「非破坏性补全」requirement 的 HR crate 专属例外：HR 因资源导航范式不同（Config-direct Request，非链式 accessor），需要 breaking 删除零引用死节点，而非 platform/ai 的纯加法补全。`okr.v2()` 与 `OkrV2`（`pub type OkrV2 = v2::OkrV2`，有真实资源 accessor 的活类型）不在删除范围。

#### Scenario: 11 个死版本节点 struct 与 returning accessor 移除

- **WHEN** 变更后在 `crates/openlark-hr/src/` 中 grep `struct AttendanceV1\|struct OkrV1\|struct EhrV1\|struct HireV1\|struct HireV2\|struct CorehrV1\|struct CorehrV2\|struct PayrollV1\|struct PerformanceV1\|struct PerformanceV2\|struct CompensationV1`
- **THEN** 命中数为 0；8 个 facade struct 上对应的 11 个 `.v1()`/`.v2()` returning accessor 同步移除

#### Scenario: okr.v2 与 OkrV2 alias 保留

- **WHEN** 变更后检查 `okr/okr/mod.rs` 与 `okr/mod.rs`
- **THEN** 仍存在 `pub type OkrV2 = v2::OkrV2;` 与 `pub fn v2(&self) -> okr::OkrV2`，`okr.v2()` 链可达（`OkrV2` 为有资源 accessor 的活类型，非死节点）

#### Scenario: 真实资源路径不受影响

- **WHEN** 变更后以原有模块路径构造 Config-direct Request，如 `openlark_hr::attendance::attendance::v1::group::CreateGroupRequest::new(config)`
- **THEN** 构造方式与签名不变，编译通过；真实 API 行为（请求/响应/端点）前后一致

#### Scenario: HR crate 编译与测试通过

- **WHEN** 运行 `cargo build -p openlark-hr --all-features` 与 `cargo test -p openlark-hr --all-features`
- **THEN** 均通过；facade struct 的 config 持有者角色与 `HrClient` 字段模式不被破坏

### Requirement: openlark-hr facade doc 指向真实可达路径

openlark-hr 的 facade 文档（`lib.rs` 顶层 doc example）SHALL 指向真实可达的 Config-direct Request 构造路径，不得展示编译失败的链式调用。doc example SHALL 以可编译检查的 doctest 形式（`no_run` 或更强）呈现，确保 advertised 的 API 路径类型/方法真实可达。原有 `client.attendance.v1().group().create()`（`AttendanceV1` 无 `.group()`，靠 `rust,ignore` 跳过编译的谎言）SHALL 改为 Config-direct 构造，如 `CreateGroupRequest::new(client.config().clone()).group_name(...)`。

#### Scenario: doc example 编译检查通过

- **WHEN** 运行 `cargo doc -p openlark-hr` 的 doctest（或 `cargo test --doc -p openlark-hr`）
- **THEN** facade doc example 不再是 `rust,ignore`，以 `no_run`（或更强）形式编译通过，展示的 Config-direct Request 路径真实可达

#### Scenario: doc example 不展示死链调用

- **WHEN** 变更后检查 `crates/openlark-hr/src/lib.rs` 顶层 doc
- **THEN** 不存在 `.v1().group()` 等引用已删除版本节点 accessor 的链式调用，example 仅展示真实可达 API
```

