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

8 个 facade struct 删除 `.v1()`/`.v2()` 后仅剩 `new()` + `config()`，看似也成 Potemkin config 持有者。**但保留**：它们是 `HrClient` 的 `pub` 字段（`client.attendance` / `client.okr` 等，见 `lib.rs:80-110`），且 `test_hr_client_*_field` 测试依赖 `client.<domain>.config().app_id()` 断言。删除它们超出 #327 范围，属后续"导航范式整体收敛"工作。

## Risks / Trade-offs

- **[BREAKING 公开 API]** 移除 11 个公开类型 + 11 个公开方法 → **缓解**：已核实零跨 crate 引用、零外部消费；v0.18 breaking 窗口正开启（与 #325/#326/#329 同批 breaking 删除）
- **[doc example 引用错误 builder 方法]** `.group_name()` 方法名可能与源码不符 → **缓解**：build 阶段对照 `attendance/attendance/v1/group/create.rs` 验证 builder 方法签名后再写入；若 `group_name` 不存在则调整示例
- **[okr.v2 / OkrV2 alias 误删]** okr 文件改动时可能误删活的 `OkrV2` alias → **缓解**：仅删 `OkrV1` struct + impl，保留 `pub type OkrV2` 与 `pub mod v2;`；build 后 `cargo build --all-features` 验证 `okr.v2()` 仍可达
- **[删除顺序致临时编译失败]** facade accessor 引用已删 struct → **缓解**：同一 task 内先删 facade returning accessor、再删 struct，或同 commit 内一并改；以最终 `cargo build --all-features` 通过为准
