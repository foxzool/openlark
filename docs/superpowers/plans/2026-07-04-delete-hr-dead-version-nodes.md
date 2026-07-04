---
change: delete-hr-dead-version-nodes
design-doc: docs/superpowers/specs/2026-07-04-delete-hr-dead-version-nodes-design.md
base-ref: f62224f33ddd91f68849410f83d8caf37fd5c868
---

# 删除 openlark-hr 死版本节点 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 删除 `openlark-hr` 的 11 个零资源 accessor 死版本节点 struct 与 11 个返回它们的 facade accessor，并把 `lib.rs` 顶层 doc example 从编译失败谎言改为可编译检查的 `no_run` doctest。

**Architecture:** 每个 domain 同 commit 内**先删 facade returning accessor、再删 inner struct**（避免临时编译失败）。保留所有 `pub mod v1;`/`pub mod v2;` 子模块声明（真实 Config-direct Request 资源在其下，必须可达），保留 `okr` 的 `pub type OkrV2 = v2::OkrV2;` 别名与 `okr.v2()` accessor（活类型）。真实 API Request struct（`CreateGroupRequest` 等）与端点枚举完全不动。

**Tech Stack:** Rust, `openlark-hr` crate，feature-gated 模块（`attendance`/`okr`/`ehr`/`hire`/`corehr`/`payroll`/`performance`/`compensation`）。

## Global Constraints

- **不删 8 个 facade struct 本身**（`Attendance`/`Okr`/`Ehr`/`Hire`/`Corehr`/`Payroll`/`Performance`/`CompensationManagement`）：它们是 `HrClient` 的 `pub` 字段，且 `lib.rs` 的 `test_hr_client_*_field` 测试依赖 `client.<domain>.config().app_id()`。
- **保留所有 `pub mod v1;` 与 `pub mod v2;` 模块声明**：真实资源（`v1/group/create.rs` 等）在其下。
- **okr 例外**：保留 `pub type OkrV2 = v2::OkrV2;` 与 `pub fn v2(&self) -> okr::OkrV2`，只删 `OkrV1` + `okr.v1()`。
- **真实 API 路径不动**：`CreateGroupRequest`/`QueryRequest` 等 Config-direct Request struct、端点枚举（`AttendanceApiV1` 等）、`v1/`/`v2/` 子模块资源代码均不改动。
- **每 domain 一个 commit**，commit message 用 `refactor: 删除 <domain> 死版本节点 <Struct> + .vN() accessor` 格式（不附 issue 号，由 PR 填写）。
- **删除顺序（同 commit 内）**：facade returning accessor 先删，inner struct 后删——这样 facade 不再引用即将消失的 inner 类型。
- **工作目录基线**：`base-ref` 为 `f62224f33ddd91f68849410f83d8caf37fd5c868`（`main` HEAD）。

---

## Task 1: 删除 attendance 死版本节点

**Files:**
- Modify: `crates/openlark-hr/src/attendance/mod.rs`（facade，删 `pub fn v1`）
- Modify: `crates/openlark-hr/src/attendance/attendance/mod.rs`（inner，删 `AttendanceV1` struct + impl + tests，保留 `pub mod v1;`）

**删除目标：**
- facade `attendance/mod.rs:29-32`：方法 `pub fn v1(&self) -> attendance::AttendanceV1 { attendance::AttendanceV1::new(self.config.clone()) }`（含上一行 doc comment `/// 获取 attendance 项目 v1 版本服务`）
- inner `attendance/attendance/mod.rs`：`AttendanceV1` struct 定义 + 整个 `impl AttendanceV1 { ... }` 块 + 文件末尾整个 `#[cfg(test)] #[allow(unused_imports)] mod tests { ... }` 块（generic serde_json 测试，与 HR 无关）

**保留：**
- inner 文件顶部的 `pub mod v1;` 声明
- inner 文件顶部的 `use openlark_core::config::Config;`——删除 struct 后此 import 变成未使用，**需一并删除**（`cargo build` 会 warning unused import，clippy `-D warnings` 会 fail）
- facade 的 `use openlark_core::config::Config;` 仍被 `Attendance` struct 与 `new`/`config` 方法使用，**保留**
- facade 的 `#[cfg(test)] mod tests` 块（与 facade struct 伴生，独立于版本节点）**保留**

- [ ] **Step 1: 编辑 facade `attendance/mod.rs`，删除 `pub fn v1` 方法**

删除以下 4 行（doc comment + 方法签名 + body + 闭合括号）：

```rust
    /// 获取 attendance 项目 v1 版本服务
    pub fn v1(&self) -> attendance::AttendanceV1 {
        attendance::AttendanceV1::new(self.config.clone())
    }
```

删除后 `impl Attendance { ... }` 仅剩 `new` 与 `config` 两个方法。

- [ ] **Step 2: 编辑 inner `attendance/attendance/mod.rs`，删除死 struct + impl + tests + 未使用 import**

删除以下内容：
1. 文件顶部 `use openlark_core::config::Config;`（struct 删除后无人使用）
2. 整个 `pub struct AttendanceV1 { config: Config }` 定义（含其上 2 行 doc comment）
3. 整个 `impl AttendanceV1 { ... }` 块
4. 文件末尾整个 `#[cfg(test)] #[allow(unused_imports)] mod tests { ... }` 块

**保留** `pub mod v1;` 声明。删除后该 inner mod.rs 内容应为：

```rust
/// attendance 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过，零 warning（unused import 已清）。若报 `cannot find type AttendanceV1` 说明 facade accessor 未删干净；若报 unused import 说明 inner 的 `use Config` 未删。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/attendance/mod.rs crates/openlark-hr/src/attendance/attendance/mod.rs
git commit -m "refactor: 删除 attendance 死版本节点 AttendanceV1 + .v1() accessor"
```

---

## Task 2: 删除 okr 死版本节点（保留 OkrV2 别名）

**Files:**
- Modify: `crates/openlark-hr/src/okr/mod.rs`（facade，删 `pub fn v1`，**保留 `pub fn v2`**）
- Modify: `crates/openlark-hr/src/okr/okr/mod.rs`（inner，删 `OkrV1` struct + impl + tests，**保留 `pub type OkrV2`、`pub mod v1`、`pub mod v2`**）

**关键风险（Design Doc D2）：** okr inner mod.rs 同时含死的 `OkrV1` struct 与活的 `pub type OkrV2 = v2::OkrV2;` 别名。facade 的 `pub fn v2(&self) -> okr::OkrV2` 依赖此别名。**只删 `OkrV1` struct + impl，绝对不能删 `pub type OkrV2` 行**。

**删除目标：**
- facade `okr/mod.rs:29-31`：方法 `pub fn v1(&self) -> okr::OkrV1 { okr::OkrV1::new(self.config.clone()) }`（含 doc comment `/// 获取 okr 项目 v1 版本服务`）
- inner `okr/okr/mod.rs`：`OkrV1` struct + `impl OkrV1` 块 + `#[cfg(test)] mod tests` 块

**保留（切勿误删）：**
- inner 的 `pub mod v1;` 与 `pub mod v2;` 声明
- inner 的 `pub type OkrV2 = v2::OkrV2;` 别名（含其上 2 行 doc comment）
- facade 的 `pub fn v2(&self) -> okr::OkrV2 { okr::OkrV2::new(self.config.clone()) }` 方法（含 doc comment）

- [ ] **Step 1: 编辑 facade `okr/mod.rs`，仅删除 `pub fn v1`，保留 `pub fn v2`**

删除以下 3 行：

```rust
    /// 获取 okr 项目 v1 版本服务
    pub fn v1(&self) -> okr::OkrV1 {
        okr::OkrV1::new(self.config.clone())
    }
```

**保留**紧随其后的 `pub fn v2` 方法（验证：删除后 `impl Okr` 含 `new`/`config`/`v2` 三个方法）。

- [ ] **Step 2: 编辑 inner `okr/okr/mod.rs`，删除 `OkrV1` struct + impl + tests，保留 `OkrV2` 别名**

删除：
1. `OkrV1` struct 定义（含其上 doc comment `/// okr 项目 v1 版本服务` / `/// OkrV1 服务入口。`）
2. 整个 `impl OkrV1 { ... }` 块
3. 文件末尾整个 `#[cfg(test)] #[allow(unused_imports)] mod tests { ... }` 块

**切勿删除** `pub type OkrV2 = v2::OkrV2;`（含其上 doc comment）。

注意：inner 的 `use openlark_core::config::Config;` 在删除 `OkrV1` 后**是否还需保留**？`pub type OkrV2 = v2::OkrV2;` 不直接用 `Config`（只是别名），但 `v2::OkrV2` 自身定义在 `v2/mod.rs` 内。检查：若删除 `OkrV1` 后 `Config` 在本文件内零引用，则一并删除 `use Config` 行。由 Step 3 build 决定（unused import warning 即删）。

删除后 inner mod.rs 应含：

```rust
/// okr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;

/// okr 项目 v2 版本服务
/// OkrV2 服务入口。
pub type OkrV2 = v2::OkrV2;
```

（是否保留顶部 `use Config` 由 Step 3 编译结果裁定。）

- [ ] **Step 3: 构建验证（重点确认 okr.v2() 仍可达）**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过。重点：`pub type OkrV2` 与 `pub fn v2` 保留，facade `okr.v2()` 链路完整。若报 `cannot find type OkrV1` 说明 facade `v1` 未删；若报 `cannot find type OkrV2` 说明误删了 type 别名——立即恢复 `pub type OkrV2 = v2::OkrV2;`。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/okr/mod.rs crates/openlark-hr/src/okr/okr/mod.rs
git commit -m "refactor: 删除 okr 死版本节点 OkrV1 + .v1() accessor（保留 OkrV2 别名与 .v2()）"
```

---

## Task 3: 删除 ehr 死版本节点

**Files:**
- Modify: `crates/openlark-hr/src/ehr/mod.rs`（facade，删 `pub fn v1`）
- Modify: `crates/openlark-hr/src/ehr/ehr/mod.rs`（inner，删 `EhrV1` struct + impl + tests，保留 `pub mod v1;`）

**删除目标：**
- facade `ehr/mod.rs`：方法 `pub fn v1(&self) -> ehr::EhrV1 { ehr::EhrV1::new(self.config.clone()) }`（含 doc comment `/// 获取 ehr 项目 v1 版本服务`）
- inner `ehr/ehr/mod.rs`：`EhrV1` struct（含 doc comment `/// EhrV1 服务入口。`）+ `impl EhrV1` 块 + 顶部 `use openlark_core::config::Config;`（struct 删后无引用）+ 文件末尾 `#[cfg(test)] mod tests` 块

**保留：** inner 的 `pub mod v1;` 声明；facade 的 `Ehr` struct + `new`/`config` + 其 `#[cfg(test)] mod tests`。

- [ ] **Step 1: 编辑 facade `ehr/mod.rs`，删除 `pub fn v1` 方法**

删除以下 4 行：

```rust
    /// 获取 ehr 项目 v1 版本服务
    pub fn v1(&self) -> ehr::EhrV1 {
        ehr::EhrV1::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `ehr/ehr/mod.rs`，删除 struct + impl + tests + 未使用 import**

删除：`use openlark_core::config::Config;`、`EhrV1` struct 定义（含 doc comment）、`impl EhrV1` 块、`#[cfg(test)] mod tests` 块。保留 `pub mod v1;`。删除后内容应为：

```rust
/// ehr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/ehr/mod.rs crates/openlark-hr/src/ehr/ehr/mod.rs
git commit -m "refactor: 删除 ehr 死版本节点 EhrV1 + .v1() accessor"
```

---

## Task 4: 删除 hire 死版本节点（V1 + V2）

**Files:**
- Modify: `crates/openlark-hr/src/hire/mod.rs`（facade，删 `pub fn v1` 与 `pub fn v2`）
- Modify: `crates/openlark-hr/src/hire/hire/mod.rs`（inner，删 `HireV1` + `HireV2` struct + impl + tests，保留 `pub mod v1;` 与 `pub mod v2;`）

**删除目标：**
- facade `hire/mod.rs`：两个方法 `pub fn v1(&self) -> hire::HireV1 {...}` 与 `pub fn v2(&self) -> hire::HireV2 {...}`（含各自 doc comment `/// 获取 hire 项目 vN 版本服务`）
- inner `hire/hire/mod.rs`：`HireV1` struct + `HireV2` struct + `impl HireV1` + `impl HireV2` 块 + 顶部 `use Config`（无引用后删）+ `#[cfg(test)] mod tests` 块

**保留：** inner 的 `pub mod v1;` 与 `pub mod v2;`；facade 的 `Hire` struct + `new`/`config` + 其 tests。

- [ ] **Step 1: 编辑 facade `hire/mod.rs`，删除 `pub fn v1` 与 `pub fn v2`**

删除以下两段（各 3 行，含 doc comment）：

```rust
    /// 获取 hire 项目 v1 版本服务
    pub fn v1(&self) -> hire::HireV1 {
        hire::HireV1::new(self.config.clone())
    }
    /// 获取 hire 项目 v2 版本服务
    pub fn v2(&self) -> hire::HireV2 {
        hire::HireV2::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `hire/hire/mod.rs`，删除两个死 struct + impl + tests**

删除：`HireV1` struct（含 doc comment）、`HireV2` struct（含 doc comment）、`impl HireV1`、`impl HireV2`、`#[cfg(test)] mod tests` 块；若 struct 删除后 `use openlark_core::config::Config;` 无引用则一并删除。保留 `pub mod v1;` 与 `pub mod v2;`。删除后内容应为：

```rust
/// hire 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/hire/mod.rs crates/openlark-hr/src/hire/hire/mod.rs
git commit -m "refactor: 删除 hire 死版本节点 HireV1/HireV2 + .v1()/.v2() accessor"
```

---

## Task 5: 删除 corehr 死版本节点（feishu_people，V1 + V2）

**Files:**
- Modify: `crates/openlark-hr/src/feishu_people/mod.rs`（facade，删 `pub fn v1` 与 `pub fn v2`）
- Modify: `crates/openlark-hr/src/feishu_people/corehr/mod.rs`（inner，删 `CorehrV1` + `CorehrV2` struct + impl + tests，保留 `pub mod v1;` 与 `pub mod v2;`）

**注意路径：** facade 在 `feishu_people/mod.rs`（struct 名 `Corehr`），inner 在 `feishu_people/corehr/mod.rs`（子目录名是 `corehr`）。facade accessor 返回类型路径是 `corehr::CorehrV1`（不是 `feishu_people::CorehrV1`）。

**删除目标：**
- facade `feishu_people/mod.rs`：`pub fn v1(&self) -> corehr::CorehrV1 {...}` 与 `pub fn v2(&self) -> corehr::CorehrV2 {...}`（含 doc comment）
- inner `feishu_people/corehr/mod.rs`：`CorehrV1` struct + `CorehrV2` struct + `impl CorehrV1` + `impl CorehrV2` + `use Config`（无引用后删）+ `#[cfg(test)] mod tests` 块

**保留：** inner 的 `pub mod v1;` 与 `pub mod v2;`；facade 的 `Corehr` struct + `new`/`config` + 其 tests。

- [ ] **Step 1: 编辑 facade `feishu_people/mod.rs`，删除 `pub fn v1` 与 `pub fn v2`**

删除以下两段：

```rust
    /// 获取 corehr 项目 v1 版本服务
    pub fn v1(&self) -> corehr::CorehrV1 {
        corehr::CorehrV1::new(self.config.clone())
    }

    /// 获取 corehr 项目 v2 版本服务
    pub fn v2(&self) -> corehr::CorehrV2 {
        corehr::CorehrV2::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `feishu_people/corehr/mod.rs`，删除两个死 struct + impl + tests**

删除：`CorehrV1` struct（含 doc comment `/// CorehrV1 服务入口。`）、`CorehrV2` struct（含 doc comment `/// CorehrV2 服务入口。`）、`impl CorehrV1`、`impl CorehrV2`、`#[cfg(test)] mod tests` 块；若 `use Config` 无引用则一并删除。保留 `pub mod v1;` 与 `pub mod v2;`。删除后内容应为：

```rust
/// corehr 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/feishu_people/mod.rs crates/openlark-hr/src/feishu_people/corehr/mod.rs
git commit -m "refactor: 删除 corehr 死版本节点 CorehrV1/CorehrV2 + .v1()/.v2() accessor"
```

---

## Task 6: 删除 payroll 死版本节点

**Files:**
- Modify: `crates/openlark-hr/src/payroll/mod.rs`（facade，删 `pub fn v1`）
- Modify: `crates/openlark-hr/src/payroll/payroll/mod.rs`（inner，删 `PayrollV1` struct + impl + tests，保留 `pub mod v1;`）

**删除目标：**
- facade `payroll/mod.rs`：方法 `pub fn v1(&self) -> payroll::PayrollV1 {...}`（含 doc comment `/// 获取 payroll 项目 v1 版本服务`）
- inner `payroll/payroll/mod.rs`：`PayrollV1` struct（含 doc comment）+ `impl PayrollV1` 块 + `use Config`（无引用后删）+ `#[cfg(test)] mod tests` 块

**保留：** inner 的 `pub mod v1;`；facade 的 `Payroll` struct + `new`/`config` + 其 tests。

- [ ] **Step 1: 编辑 facade `payroll/mod.rs`，删除 `pub fn v1` 方法**

删除以下 4 行：

```rust
    /// 获取 payroll 项目 v1 版本服务
    pub fn v1(&self) -> payroll::PayrollV1 {
        payroll::PayrollV1::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `payroll/payroll/mod.rs`，删除 struct + impl + tests**

删除：`PayrollV1` struct（含 doc comment）、`impl PayrollV1`、`#[cfg(test)] mod tests` 块、`use Config`（若无引用）。保留 `pub mod v1;`。删除后内容应为：

```rust
/// payroll 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/payroll/mod.rs crates/openlark-hr/src/payroll/payroll/mod.rs
git commit -m "refactor: 删除 payroll 死版本节点 PayrollV1 + .v1() accessor"
```

---

## Task 7: 删除 performance 死版本节点（V1 + V2）

**Files:**
- Modify: `crates/openlark-hr/src/performance/mod.rs`（facade，删 `pub fn v1` 与 `pub fn v2`）
- Modify: `crates/openlark-hr/src/performance/performance/mod.rs`（inner，删 `PerformanceV1` + `PerformanceV2` struct + impl + tests，保留 `pub mod v1;` 与 `pub mod v2;`）

**删除目标：**
- facade `performance/mod.rs`：`pub fn v1(&self) -> performance::PerformanceV1 {...}` 与 `pub fn v2(&self) -> performance::PerformanceV2 {...}`（含 doc comment）
- inner `performance/performance/mod.rs`：`PerformanceV1` struct（doc comment `/// PerformanceV1 服务入口。`）+ `PerformanceV2` struct（doc comment `/// PerformanceV2 服务入口。`）+ `impl PerformanceV1` + `impl PerformanceV2` + `use Config`（无引用后删）+ `#[cfg(test)] mod tests` 块（注意此文件 tests 块用 `use serde_json;` 而非 `#[allow(unused_imports)]`，整块删）

**保留：** inner 的 `pub mod v1;` 与 `pub mod v2;`；facade 的 `Performance` struct + `new`/`config` + 其 tests。

- [ ] **Step 1: 编辑 facade `performance/mod.rs`，删除 `pub fn v1` 与 `pub fn v2`**

删除以下两段：

```rust
    /// 获取 performance 项目 v1 版本服务
    pub fn v1(&self) -> performance::PerformanceV1 {
        performance::PerformanceV1::new(self.config.clone())
    }
    /// 获取 performance 项目 v2 版本服务
    pub fn v2(&self) -> performance::PerformanceV2 {
        performance::PerformanceV2::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `performance/performance/mod.rs`，删除两个死 struct + impl + tests**

删除：`PerformanceV1` struct、`PerformanceV2` struct、`impl PerformanceV1`、`impl PerformanceV2`、`#[cfg(test)] mod tests { use serde_json; ... }` 整块；若 `use Config` 无引用则一并删除。保留 `pub mod v1;` 与 `pub mod v2;`。删除后内容应为：

```rust
/// performance 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
/// v2 子模块。
pub mod v2;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/performance/mod.rs crates/openlark-hr/src/performance/performance/mod.rs
git commit -m "refactor: 删除 performance 死版本节点 PerformanceV1/PerformanceV2 + .v1()/.v2() accessor"
```

---

## Task 8: 删除 compensation_management 死版本节点

**Files:**
- Modify: `crates/openlark-hr/src/compensation_management/mod.rs`（facade，删 `pub fn v1`）
- Modify: `crates/openlark-hr/src/compensation_management/compensation/mod.rs`（inner，删 `CompensationV1` struct + impl + tests，保留 `pub mod v1;`）

**注意路径：** facade 在 `compensation_management/mod.rs`，但 inner 子目录名是 `compensation`（不是 `compensation_management`），故 inner 文件路径是 `compensation_management/compensation/mod.rs`。facade accessor 返回类型路径是 `compensation::CompensationV1`。

**删除目标：**
- facade `compensation_management/mod.rs`：方法 `pub fn v1(&self) -> compensation::CompensationV1 {...}`（含 doc comment `/// 获取 compensation 项目 v1 版本服务`，约第 28 行）
- inner `compensation_management/compensation/mod.rs`：`CompensationV1` struct（含 doc comment）+ `impl CompensationV1` 块 + `use Config`（无引用后删）+ `#[cfg(test)] mod tests` 块

**保留：** inner 的 `pub mod v1;`；facade 的 `CompensationManagement` struct + `new`/`config` + 其 tests。

- [ ] **Step 1: 编辑 facade `compensation_management/mod.rs`，删除 `pub fn v1` 方法**

删除以下 4 行：

```rust
    /// 获取 compensation 项目 v1 版本服务
    pub fn v1(&self) -> compensation::CompensationV1 {
        compensation::CompensationV1::new(self.config.clone())
    }
```

- [ ] **Step 2: 编辑 inner `compensation_management/compensation/mod.rs`，删除 struct + impl + tests**

删除：`CompensationV1` struct（含 doc comment）、`impl CompensationV1`、`#[cfg(test)] mod tests` 块、`use Config`（若无引用）。保留 `pub mod v1;`。删除后内容应为：

```rust
/// compensation 项目模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织

/// v1 子模块。
pub mod v1;
```

- [ ] **Step 3: 构建验证**

Run: `cargo build -p openlark-hr --all-features --all-targets`
Expected: 编译通过零 warning。

- [ ] **Step 4: 提交**

```bash
git add crates/openlark-hr/src/compensation_management/mod.rs crates/openlark-hr/src/compensation_management/compensation/mod.rs
git commit -m "refactor: 删除 compensation_management 死版本节点 CompensationV1 + .v1() accessor"
```

---

## Task 9: 修正 `lib.rs` facade doc example（`rust,ignore` → `no_run` 真实路径）

**Files:**
- Modify: `crates/openlark-hr/src/lib.rs:15-27`（顶层 module doc 的 `## 使用示例` 代码块）

**Interfaces:**
- Consumes: Task 1 完成后 `attendance/attendance/v1/group::CreateGroupRequest` 仍可达（真实资源，未删）
- Produces: 一个编译通过的 `no_run` doctest，issue 验收"doc example 编译通过"满足

**已验证事实（Design Doc D4 + 本计划作者已核对）：**
- `attendance/attendance/v1/group/mod.rs` 第 11 行 `pub use create::CreateGroupRequest;` → 路径 `openlark_hr::attendance::attendance::v1::group::CreateGroupRequest` 可达 ✓
- `CreateGroupRequest::new(config: Config) -> Self`（`create.rs:65`）✓
- `.group_name(String) -> Self` builder（`create.rs:96`）✓
- `HrClient::new(config: Config) -> Self`（`lib.rs:115`）、`HrClient::config(&self) -> &Config`（`lib.rs:139`），`Config: Clone` 故 `client.config().clone() -> Config` ✓

- [ ] **Step 1: 替换 `lib.rs:15-27` 的 `rust,ignore` 代码块**

将以下旧内容（编译失败谎言——`AttendanceV1` 无 `.group()`）：

```rust
//! ```rust,ignore
//! use openlark_hr::prelude::*;
//! use openlark_hr::HrClient;
//!
//! let client = HrClient::new(config);
//! // 推荐：字段式 meta 入口
//! client.attendance.v1().group().create()
//!     .group_name("技术部".to_string())
//!     .execute()
//!     .await?;
//!
//! // 字段访问是当前推荐方式；旧方法式入口只保留兼容职责。
//! ```
```

替换为以下 `no_run` Config-direct Request 真实路径：

```rust
//! ```no_run
//! use openlark_hr::HrClient;
//! use openlark_hr::attendance::attendance::v1::group::CreateGroupRequest;
//! # let config: openlark_core::config::Config = unimplemented!();
//! let client = HrClient::new(config);
//! // 真实资源直达：Config-direct Request 自带 builder + execute()
//! let _request = CreateGroupRequest::new(client.config().clone())
//!     .group_name("技术部".to_string());
//! ```
```

说明：
- `no_run` 编译检查但不运行，不需真实 tokio executor / 凭证。
- `# let config: ... = unimplemented!();` 是 doctest 标准隐藏行，编译期 `unimplemented!()` 类型推断为 `Config`，`no_run` 不运行故不会 panic。
- `_request` 前缀下划线避免 unused warning。

- [ ] **Step 2: doctest 编译验证**

Run: `cargo doc -p openlark-hr --all-features`
Expected: 文档构建无 doctest 编译错误。重点确认：`CreateGroupRequest` 路径可达、`.group_name(String)` 方法签名匹配。

若报 `cannot find type CreateGroupRequest` → 检查 `group/mod.rs` 的 `pub use` 是否存在（已验证存在）；若报 `.group_name` not found → 核对 `create.rs:96` 签名是否为 `pub fn group_name(mut self, group_name: String) -> Self`。

- [ ] **Step 3: 提交**

```bash
git add crates/openlark-hr/src/lib.rs
git commit -m "refactor: lib.rs facade doc example 改为 no_run Config-direct Request 真实路径"
```

---

## Task 10: 最终验证（issue 验收四条 + 真实 API 未受影响）

**Files:** 无修改，仅运行验证命令。

**目标：** 跑完 Design Doc「Test Strategy」与 issue 验收的全部命令，确认全绿且真实 API 路径未改。

- [ ] **Step 1: 全量 build**

Run: `cargo build -p openlark-hr --all-features`
Expected: 编译通过。

- [ ] **Step 2: 全量 test**

Run: `cargo test -p openlark-hr --all-features`
Expected: 全部测试通过。重点：8 个 `test_hr_client_*_field` 测试（仅依赖 `client.<domain>.config().app_id()`）零破坏。

- [ ] **Step 3: doctest 编译**

Run: `cargo doc -p openlark-hr --all-features`
Expected: 文档构建通过，Task 9 的 `no_run` doctest 编译通过（不再 `rust,ignore`）。

- [ ] **Step 4: fmt + clippy**

Run: `cargo fmt --check && cargo clippy -p openlark-hr --all-features --all-targets -D warnings`
Expected: fmt 无 diff、clippy 零 warning。

- [ ] **Step 5: 真实 API 路径未受影响验证（grep 确认）**

Run:
```bash
git diff --stat f62224f33ddd91f68849410f83d8caf37fd5c868 -- \
  'crates/openlark-hr/src/attendance/attendance/v1/**' \
  'crates/openlark-hr/src/okr/okr/v1/**' \
  'crates/openlark-hr/src/okr/okr/v2/**' \
  'crates/openlark-hr/src/ehr/ehr/v1/**' \
  'crates/openlark-hr/src/hire/hire/v1/**' \
  'crates/openlark-hr/src/hire/hire/v2/**' \
  'crates/openlark-hr/src/feishu_people/corehr/v1/**' \
  'crates/openlark-hr/src/feishu_people/corehr/v2/**' \
  'crates/openlark-hr/src/payroll/payroll/v1/**' \
  'crates/openlark-hr/src/performance/performance/v1/**' \
  'crates/openlark-hr/src/performance/performance/v2/**' \
  'crates/openlark-hr/src/compensation_management/compensation/v1/**'
```
Expected: 空输出（真实资源代码零改动）。

补充确认 Config-direct Request struct 与端点枚举未改：

```bash
git diff f62224f33ddd91f68849410f83d8caf37fd5c868 -- \
  'crates/openlark-hr/src/**/create.rs' \
  'crates/openlark-hr/src/**/query.rs' \
  'crates/openlark-hr/src/common/api_endpoints.rs' \
  | grep -E '^\+|^-' | grep -v '^[+-]{3}'
```
Expected: 空输出（`CreateGroupRequest`/`QueryRequest` 等 Request struct 与端点枚举零改动）。

- [ ] **Step 6: 跨 crate 引用回归确认**

Run:
```bash
grep -rn "AttendanceV1\|OkrV1\|EhrV1\|HireV1\|HireV2\|CorehrV1\|CorehrV2\|PayrollV1\|PerformanceV1\|PerformanceV2\|CompensationV1" \
  --include="*.rs" \
  crates/ src/ examples/ tests/
```
Expected: 仅 `OkrV2`（活别名，保留）相关命中；11 个已删 struct 名零命中（确认无残留引用）。`OkrV2` 别名本身仍存在于 `okr/okr/mod.rs`，属预期。

若 Step 1-5 全绿且 Step 6 grep 无残留 → 验证通过，可进入 verify 阶段。任一步骤失败：停止，按 `superpowers:systematic-debugging` 定位根因（最常见：某 inner mod.rs 的 `use Config` 未清致 unused import warning → clippy fail；或 okr 误删 `OkrV2` 别名 → build fail）。
