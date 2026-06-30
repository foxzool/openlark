---
change: add-platform-v1-accessors
design-doc: docs/superpowers/specs/2026-06-30-platform-v1-accessors-design.md
base-ref: bfd9b5ae64bab42eb176cf3cc3b75671926613de
---

# add-platform-v1-accessors 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **事实源**：tasks.md 四组（admin/directory/apaas/全局验证）。技术决策见 design doc（D1-D6）。

**Goal**：为 `AdminV1`/`DirectoryV1`/`ApaasV1` 三个入口铺设 full-depth 链式子 API 访问器（共约 30+ service 类型），与 SparkV1 体验对齐；同时把 cleanup-dead-code-allows 临时改名的 `_config` 字段恢复为 `config`。

**Architecture**：照 SparkV1 三级蓝本复制——每级子模块建一个 `XxxService`（`#[derive(Debug, Clone)]`），持 config，暴露下一级访问器或叶子 builder 构造方法。入口/中间级 service 持 `Arc<PlatformConfig>`（访问器用 `Arc::clone` 廉价传递），叶子 service（其子是 .rs 操作、不再分层）持 owned `Config`（由父级 `arc.as_ref().clone()` 得到）。叶子访问器调已存在 builder 的 `new(self.config.clone(), ...)`，**builder 签名不动**。三入口已被 `AdminService.v1()`/`AppEngineService.v1()`/`DirectoryService.v1()` 返回，可达性已验证。

**Tech Stack**：Rust，openlark-platform crate，openlark-core 的 `Config`/`PlatformConfig`。

## Global Constraints

- **base-ref**：`bfd9b5ae64`（main HEAD，commit `bfd9b5ae6`）。
- **可达性前提**（design「可达性」节）：`AdminService.v1()` / `AppEngineService.v1()`（返回 `ApaasV1`，注意不是 `AppEngineV1`）/ `DirectoryService.v1()` 已返回三入口，default/full feature 下可达。装访问器不会制造 dead_code。
- **范式约束（D1-D6）**：
  - **D1** 每级子模块建 `XxxService`，full-depth 链到叶子 builder（不能只顶层）。
  - **D2** config 流转：入口/中间级 `Arc<PlatformConfig>`（`Arc::clone`），叶子 service 持 owned `Config`（`arc.as_ref().clone()`），叶子访问器调 builder 的 `new(self.config.clone(), ...)`。
  - **D3** 访问器返回值类型为 service（`pub fn badge(&self) -> BadgeService`），非引用；service 全部 `#[derive(Debug, Clone)]`。
  - **D4** 全手写，不引入宏。30+ service 虽重复但每叶子 builder 名字/参数不同，宏抽象收益有限；照 SparkV1 一致。
  - **D5** admin 的 `audit.rs`/`users.rs` 是 facade，**已有** `AuditApi`/`UsersApi` 类型（持 `Arc<PlatformConfig>`，runtime stub）——只装 `audit()`/`users()` 访问器返回它们，**不新建类型、不套第二层 service**。
  - **D6** 测试仿 `test_spark_v1_directory_access`：构造 config → 链式到最末级 → `let _ = ...` 证可达。
- **叶子 builder 签名不动**：纯加法，不改任何已存在的 builder `new()`/方法/endpoint/序列化。
- **path 参数下传**（apaas 深嵌套特有，区别于 SparkV1）：apaas 的叶子 builder 在 `new()` 时需要 `namespace`/`object_api_name`/`role_api_name`/`workspace_id` 等路径参数。中间级 service 的访问器**接受这些参数并下传**（如 `application(namespace).object(object_api_name).record()` 持有已绑定的 namespace/object_api_name）。详见各 Task 的 Produces 块。
- **不提交 git**：主会话负责 commit。每个 task 完成后报告主会话，由主会话勾选 tasks.md + commit（参考 comet-build 阶段专项「每个 task 验收后必须 commit」）。
- **CI 红线（全局，每个 task 的验证隐式包含）**：
  - `cargo fmt --check`（workspace）
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo clippy -p openlark-platform -- -W dead_code`（无新增 dead_code 告警；dead_code 是新 service 未被上级消费的信号）

---

## 文件结构总览

三入口对应三个模块树根（每个有 `v1/mod.rs` 持入口 struct）：

| 入口 | 模块根 | v1/mod.rs | 深度 |
|------|--------|-----------|------|
| `AdminV1` | `crates/openlark-platform/src/admin/admin/v1/` | `mod.rs` | 浅 |
| `DirectoryV1` | `crates/openlark-platform/src/directory/directory/v1/` | `mod.rs` | 浅 |
| `ApaasV1` | `crates/openlark-platform/src/app_engine/apaas/v1/` | `mod.rs` | 3-4 层深嵌套 |

**改动模式**（每个浅模块）：在 `XxxModule/mod.rs` 顶部加 `use` 与（如需）`use openlark_core::config::Config;`，加 `XxxService` struct + `impl`（持 config + 暴露叶子 builder 构造方法），然后在 `v1/mod.rs` 的入口 struct `impl` 里加 `pub fn xxx()` 访问器。`v1/mod.rs` 的 `_config` → `config` 重命名 + 删 reserved 注释（闭环 cleanup-dead-code-allows）。

**apaas 深嵌套改动模式**：与上面相同，但中间级（`ApplicationService`/`ObjectService`/`RoleService`/`WorkspaceService`）的访问器**带路径参数**并把参数存进返回的下级 service 字段；最深一级（`RecordService`/`MemberService`/`TableViewService` 等）持 owned `Config` + 已绑定的路径参数，访问器把它们与 config 一起喂给 builder。

---

## Task 1: platform admin v1（AdminV1，浅，6 操作集合 + 2 facade）

**Files:**
- Modify: `crates/openlark-platform/src/admin/admin/v1/mod.rs`（`AdminV1`：`_config` → `config`，装 8 个访问器，改测试）
- Modify: `crates/openlark-platform/src/admin/admin/v1/badge/mod.rs`（加 `BadgeService`，持 owned `Config`）
- Modify: `crates/openlark-platform/src/admin/admin/v1/badge_image/mod.rs`（加 `BadgeImageService`）
- Modify: `crates/openlark-platform/src/admin/admin/v1/password/mod.rs`（加 `PasswordService`）
- Modify: `crates/openlark-platform/src/admin/admin/v1/admin_dept_stat/mod.rs`（加 `AdminDeptStatService`）
- Modify: `crates/openlark-platform/src/admin/admin/v1/admin_user_stat/mod.rs`（加 `AdminUserStatService`）
- Modify: `crates/openlark-platform/src/admin/admin/v1/audit_info/mod.rs`（加 `AuditInfoService`）
- 不动：`audit.rs`/`users.rs`（D5，已有 `AuditApi`/`UsersApi` 类型，只加访问器）

**Interfaces:**
- Consumes: `AdminService.v1()` 已返回 `AdminV1`（`crates/openlark-platform/src/admin.rs`）。叶子 builder `new(config: Config)` 签名见各 `.rs` 文件，**不动**。
- Produces: `AdminV1::{badge(), badge_image(), password(), admin_dept_stat(), admin_user_stat(), audit_info(), audit(), users()}` 访问器。叶子访问器返回各 `XxxRequestBuilder`（如 `BadgeService::create()` → `create::CreateBadgeRequestBuilder`）。

**叶子 builder `new()` 签名速查**（已验证，均 `config: Config` 单参，无路径参数）：
- `badge/create.rs`: `CreateBadgeRequestBuilder::new(config: Config)`
- `badge/get.rs`: `GetBadgeRequestBuilder::new(config: Config, badge_id: impl Into<String>)` — **有 badge_id 参数**，访问器需接受并下传
- `badge/list.rs`: `ListBadgeRequestBuilder::new(config: Config)`
- `badge/update.rs`: `UpdateBadgeRequestBuilder::new(config: Config, badge_id: ...)`
- `badge/grant/create.rs` 等 grant 子操作：各自 `new(config: Config, badge_id: ..., ...)` — grant 是 badge 的下一级，按 D1 应在 `BadgeService` 内暴露 `grant(badge_id)` 访问器返回 `BadgeGrantService`（持 owned Config + badge_id）
- `badge_image/create.rs`: `CreateBadgeImageRequestBuilder::new(config: Config)`
- `password/reset.rs`: `ResetPasswordRequestBuilder::new(config: Config, ...)`
- `admin_dept_stat/list.rs`: `new(config: Config)`
- `admin_user_stat/list.rs`: `new(config: Config)`
- `audit_info/list.rs`: `new(config: Config)`

> **注意**：实施前先用 `grep -rn "pub fn new" crates/openlark-platform/src/admin/admin/v1/` 核对每个 builder 的确切参数（部分带 badge_id/user_id 等）。访问器签名照搬下传，不臆造。

- [x] **Step 1: 写失败测试**（仿 `test_spark_v1_directory_access`）

在 `crates/openlark-platform/src/admin/admin/v1/mod.rs` 的 `#[cfg(test)] mod tests` 内加（保留现有 `test_admin_v1_creation` 但把 `_config` 引用改成 `config`，见 Step 3）：

```rust
    #[test]
    fn test_admin_v1_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = AdminV1::new(std::sync::Arc::new(config));
        // 6 操作集合叶子 builder 可达
        let _ = api.badge().create();
        let _ = api.badge().list();
        let _ = api.badge_image().create();
        let _ = api.password().reset();
        let _ = api.admin_dept_stat().list();
        let _ = api.admin_user_stat().list();
        let _ = api.audit_info().list();
        // badge.grant 深一级（badge_id 路径参数）
        let _ = api.badge().grant("badge_x").list();
        // 2 facade（D5：返回已有 AuditApi/UsersApi stub 类型）
        let _ = api.audit();
        let _ = api.users();
    }
```

> 若 `password().reset()` / `badge().grant()` / `audit_info().list()` 的 builder `new()` 带额外必填参数（如 `user_id`/`badge_id`），在访问器里接受该参数（`reset(user_id)`）或在 builder 链上补占位串。实施时核对真实签名后定。

- [x] **Step 2: 跑测试确认失败**

Run: `cargo test -p openlark-platform --lib admin::admin::v1::tests::test_admin_v1_chain_access`
Expected: FAIL（`no method named badge` / `field _config is private` 之类，因为访问器/字段重命名还没做）

- [x] **Step 3: 改 `v1/mod.rs` 入口——`_config` → `config` + 装 8 访问器 + 改测试**

把 `AdminV1` struct 字段 `_config: Arc<PlatformConfig>` 改成 `config: Arc<PlatformConfig>`，删掉 `// reserved：待装访问器/execute（见 #274，不完整脚手架）` 注释，`new()` 内 `Self { _config: config }` 改 `Self { config }`。`impl AdminV1` 末尾加 8 个访问器：

```rust
    /// badge 资源
    pub fn badge(&self) -> badge::BadgeService {
        badge::BadgeService::new(self.config.as_ref().clone())
    }

    /// badge_image 资源
    pub fn badge_image(&self) -> badge_image::BadgeImageService {
        badge_image::BadgeImageService::new(self.config.as_ref().clone())
    }

    /// password 资源
    pub fn password(&self) -> password::PasswordService {
        password::PasswordService::new(self.config.as_ref().clone())
    }

    /// admin_dept_stat 资源
    pub fn admin_dept_stat(&self) -> admin_dept_stat::AdminDeptStatService {
        admin_dept_stat::AdminDeptStatService::new(self.config.as_ref().clone())
    }

    /// admin_user_stat 资源
    pub fn admin_user_stat(&self) -> admin_user_stat::AdminUserStatService {
        admin_user_stat::AdminUserStatService::new(self.config.as_ref().clone())
    }

    /// audit_info 资源
    pub fn audit_info(&self) -> audit_info::AuditInfoService {
        audit_info::AuditInfoService::new(self.config.as_ref().clone())
    }

    /// audit facade（D5：复用已有 AuditApi stub 类型）
    pub fn audit(&self) -> audit::AuditApi {
        audit::AuditApi::new(self.config.clone())
    }

    /// users facade（D5：复用已有 UsersApi stub 类型）
    pub fn users(&self) -> users::UsersApi {
        users::UsersApi::new(self.config.clone())
    }
```

同时把 `test_admin_v1_creation` 里的 `api._config.app_id()` 改成 `api.config.app_id()`。

> 注意 facade（`audit`/`users`）返回类型持 `Arc<PlatformConfig>`（已是），所以访问器用 `self.config.clone()`（`Arc::clone`），不是 `as_ref().clone()`——这区别于叶子 service。

- [x] **Step 4: 给 6 个操作集合模块加 `XxxService`**

每个 `XxxModule/mod.rs` 顶部加：

```rust
use openlark_core::config::Config;
```

然后加 service。**`badge/mod.rs`（持 owned Config + 含 grant 下一级）模板**：

```rust
//! Badge module

use openlark_core::config::Config;

pub mod create;
pub mod get;
pub mod grant;
pub mod list;
pub mod update;

/// badge 资源服务
#[derive(Debug, Clone)]
pub struct BadgeService {
    config: Config,
}

impl BadgeService {
    /// 创建新的 badge 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建勋章
    pub fn create(&self) -> create::CreateBadgeRequestBuilder {
        create::CreateBadgeRequestBuilder::new(self.config.clone())
    }

    /// 获取勋章
    pub fn get(&self, badge_id: impl Into<String>) -> get::GetBadgeRequestBuilder {
        get::GetBadgeRequestBuilder::new(self.config.clone(), badge_id)
    }

    /// 勋章列表
    pub fn list(&self) -> list::ListBadgeRequestBuilder {
        list::ListBadgeRequestBuilder::new(self.config.clone())
    }

    /// 更新勋章
    pub fn update(&self, badge_id: impl Into<String>) -> update::UpdateBadgeRequestBuilder {
        update::UpdateBadgeRequestBuilder::new(self.config.clone(), badge_id)
    }

    /// badge.grant 子资源
    pub fn grant(&self, badge_id: impl Into<String>) -> grant::BadgeGrantService {
        grant::BadgeGrantService::new(self.config.clone(), badge_id)
    }
}
```

> 上述 `get`/`update`/`grant` 的确切参数（badge_id 是否必填、类型）实施时核对 `get.rs`/`update.rs`/`grant/*.rs` 的 `new()` 真实签名后照搬。若 builder `new()` 不带 badge_id（如 list），访问器就不带参数。

`grant/mod.rs` 需加 `BadgeGrantService`（持 owned `Config` + `badge_id: String`），暴露 `create()/get()/list()/update()/delete()` 各调对应 builder `new(self.config.clone(), self.badge_id.clone(), ...)`——照 `BadgeService` 模板复制，路径参数 `badge_id` 下传。

其余 5 个浅模块（`badge_image`/`password`/`admin_dept_stat`/`admin_user_stat`/`audit_info`）同样照 `BadgeService` 模板：持 owned `Config`，每个 `.rs` 子操作对应一个访问器调 `XxxRequestBuilder::new(self.config.clone(), ...)`。各模块只有 1-2 个 `.rs`（如 `audit_info/list.rs`、`admin_dept_stat/list.rs`），service 内 1-2 个访问器即可。

- [x] **Step 5: 跑测试确认通过**

Run: `cargo test -p openlark-platform --lib admin::admin::v1::tests`
Expected: PASS（`test_admin_v1_creation` + `test_admin_v1_chain_access` 均过；各 `.rs` 自带的 builder 单测不过不动）

- [x] **Step 6: admin 范围 clippy + fmt**

Run:
```bash
cargo clippy -p openlark-platform --all-targets -- -D warnings
cargo clippy -p openlark-platform -- -W dead_code
cargo fmt --check
```
Expected: 全 exit 0；`-W dead_code` 无新增告警（每个新 service 都被 `AdminV1` 访问器消费）。

- [x] **Step 7: 报告主会话勾选 + commit**

报告完成。主会话勾选 tasks.md `1.1`-`1.5` 并 commit（消息建议 `feat(platform): admin v1 链式访问器（#274）`）。

---

## Task 2: platform directory v1（DirectoryV1，浅，8 子模块）

**Files:**
- Modify: `crates/openlark-platform/src/directory/directory/v1/mod.rs`（`DirectoryV1`：`_config` → `config`，装 8 访问器，改测试）
- Modify: `crates/openlark-platform/src/directory/directory/v1/department/mod.rs`（加 `DepartmentService`）
- Modify: `crates/openlark-platform/src/directory/directory/v1/departments/mod.rs`（加 facade 处理——见下）
- Modify: `crates/openlark-platform/src/directory/directory/v1/users/mod.rs`（加 `UsersService` 或 facade）
- Modify: `crates/openlark-platform/src/directory/directory/v1/employee/mod.rs`（加 `EmployeeService`）
- Modify: `crates/openlark-platform/src/directory/directory/v1/sync/mod.rs`（加 `SyncService`）
- Modify: `crates/openlark-platform/src/directory/directory/v1/collaboration_rule/mod.rs`（加 `CollaborationRuleService`）
- Modify: `crates/openlark-platform/src/directory/directory/v1/collaboration_share_entity/mod.rs`（加 `CollaborationShareEntityService`）
- Modify: `crates/openlark-platform/src/directory/directory/v1/collaboration_tenant/mod.rs`（加 `CollaborationTenantService`）

**Interfaces:**
- Consumes: `DirectoryService.v1()` 已返回 `DirectoryV1`（`crates/openlark-platform/src/directory/mod.rs`）。叶子 builder `new(config: Config, ...)` 签名见各 `.rs`。
- Produces: `DirectoryV1::{department(), departments(), users(), employee(), sync(), collaboration_rule(), collaboration_share_entity(), collaboration_tenant()}`。

**叶子 builder `new()` 签名速查**（已验证 `department/create.rs`: `new(config: Config, name: impl Into<String>)` — **有 name 参数**；其余用 `grep -rn "pub fn new" crates/openlark-platform/src/directory/directory/v1/` 核对）。

> **`departments/mod.rs` 与 `users/mod.rs` 注意**：当前内容仅一行模块声明或空（`sync/mod.rs` 是 `//! Sync module`）。`departments` 是「部门兼容 facade」（对照 admin 的 `users.rs`），需先 Read 看是否已有 service 类型；若没有则按浅模块加 `DepartmentsService`。实施前必读这两个文件确认。

- [x] **Step 1: 写失败测试**

在 `crates/openlark-platform/src/directory/directory/v1/mod.rs` 的 `#[cfg(test)] mod tests` 内加（保留 `test_directory_v1_creation` 但 `_config` → `config`）：

```rust
    #[test]
    fn test_directory_v1_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = DirectoryV1::new(std::sync::Arc::new(config));
        // 8 子模块叶子 builder 可达（部分带路径参数，照真实 new 签名补占位串）
        let _ = api.department().create("test_dept".to_string());
        let _ = api.employee().create(/* 照 employee/create.rs new 签名 */);
        let _ = api.collaboration_rule().create(/* ... */);
        let _ = api.collaboration_share_entity().list();
        let _ = api.collaboration_tenant().list();
        let _ = api.departments();
        let _ = api.users();
        let _ = api.sync();
    }
```

> 实施时把每个 `.create(...)`/`.list()` 的参数照对应 builder 真实 `new()` 补齐（grep 后定）。无 builder 的 facade（如 `sync` 若无操作）访问器返回 service 即可，`let _ = api.sync();`。

- [x] **Step 2: 跑测试确认失败**

Run: `cargo test -p openlark-platform --lib directory::directory::v1::tests::test_directory_v1_chain_access`
Expected: FAIL（`no method named department` / 字段私有）

- [x] **Step 3: 改 `v1/mod.rs` 入口**

`DirectoryV1._config` → `config`，删 reserved 注释，`new()` 内改名，加 8 访问器（照 Task 1 Step 3 的 admin 模板；浅模块用 `self.config.as_ref().clone()` 喂 owned Config service）。`test_directory_v1_creation` 里 `api._config` → `api.config`。

- [x] **Step 4: 给 8 个子模块加 `XxxService`**

照 Task 1 Step 4 的 `BadgeService` 模板。每个模块顶部 `use openlark_core::config::Config;`，加 `XxxService { config: Config }` + `impl` 暴露各 `.rs` builder 构造方法。`department/mod.rs` 模板（含 name 参数下传）：

```rust
//! 部门相关 API

use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod search;

/// department 资源服务
#[derive(Debug, Clone)]
pub struct DepartmentService {
    config: Config,
}

impl DepartmentService {
    /// 创建新的 department 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建部门
    pub fn create(&self, name: impl Into<String>) -> create::DepartmentCreateRequestBuilder {
        create::DepartmentCreateRequestBuilder::new(self.config.clone(), name)
    }

    // delete/filter/mget/patch/search 同理，照各自 new() 签名下传参数
}
```

- [x] **Step 5: 跑测试确认通过**

Run: `cargo test -p openlark-platform --lib directory::directory::v1::tests`
Expected: PASS

- [x] **Step 6: directory 范围 clippy + fmt**

Run:
```bash
cargo clippy -p openlark-platform --all-targets -- -D warnings
cargo clippy -p openlark-platform -- -W dead_code
cargo fmt --check
```
Expected: 全 exit 0，无新 dead_code。

- [x] **Step 7: 报告主会话勾选 + commit**

主会话勾选 `2.1`-`2.4`，commit（`feat(platform): directory v1 链式访问器（#274）`）。

---

## Task 3: platform apaas v1 顶层 8 service + `ApaasV1` 入口

> apaas 拆三步（Task 3 顶层 / Task 4 application 深嵌套 / Task 5 workspace 嵌套），各自独立验收。Task 3 先把 `ApaasV1` 入口和 8 个**顶层** service（无深嵌套的）装好，application/workspace 先装到「返回中间级 service」为止，深嵌套在 Task 4/5 补。

**Files:**
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/mod.rs`（`ApaasV1`：`_config` → `config`，装 8 访问器，改测试）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/app/mod.rs`（加 `AppService`，浅）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/approval_task/mod.rs`（加 `ApprovalTaskService`，浅，部分带 task_id）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/approval_instance/mod.rs`（加 `ApprovalInstanceService`，浅）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/user_task/mod.rs`（加 `UserTaskService`，浅，部分带 task_id）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/seat_activity/mod.rs`（加 `SeatActivityService`，浅）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/seat_assignment/mod.rs`（加 `SeatAssignmentService`，浅）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/mod.rs`（加 `ApplicationService` **中间级**，持 `Arc<PlatformConfig>` + `namespace`，见 Task 4）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/workspace/mod.rs`（加 `WorkspaceService` **中间级**，持 `Arc<PlatformConfig>` + `workspace_id`，见 Task 5）

**Interfaces:**
- Consumes: `AppEngineService.v1()` 已返回 `ApaasV1`（`crates/openlark-platform/src/app_engine.rs`，注意返回类型是 `ApaasV1` 不是 `AppEngineV1`）。
- Produces: `ApaasV1::{app(), approval_task(), approval_instance(), user_task(), seat_activity(), seat_assignment(), application(namespace), workspace(workspace_id)}`。`application`/`workspace` 返回中间级 service（Task 4/5 接力）。

**顶层浅 builder `new()` 签名速查**（已验证）：
- `app/list.rs`: `new(config: Config)`
- `approval_instance/{list,cancel,get}.rs`: `new(config: Config)`
- `approval_task/{agree,reject,cancel,add_assignee}.rs`: `new(config: Config)`；`transfer.rs`: `new(config, approval_task_id, transfer_to_user_id)` — transfer 访问器需两参
- `user_task/query.rs`: `new(config)`；`cc/expediting/chat_group/rollback_points/rollback.rs`: `new(config, task_id)`（rollback 还要 node_id）
- `seat_activity/list.rs` / `seat_assignment/list.rs`: `new(config)`

- [ ] **Step 1: 写失败测试**

在 `crates/openlark-platform/src/app_engine/apaas/v1/mod.rs` 的 `#[cfg(test)] mod tests` 内加（保留 `test_apaas_v1_creation`，`_config` → `config`）：

```rust
    #[test]
    fn test_apaas_v1_top_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = ApaasV1::new(std::sync::Arc::new(config));
        // 顶层 6 个浅 service 叶子可达
        let _ = api.app().list();
        let _ = api.approval_instance().list();
        let _ = api.approval_task().agree();
        let _ = api.approval_task().transfer("task_1", "user_2");
        let _ = api.user_task().query();
        let _ = api.user_task().cc("task_1");
        let _ = api.seat_activity().list();
        let _ = api.seat_assignment().list();
        // application/workspace 中间级可达（深链在 Task 4/5 补）
        let _ = api.application("ns_x");
        let _ = api.workspace("ws_x");
    }
```

> transfer 的两参、cc 的 task_id 照 `transfer.rs`/`cc.rs` 真实 `new()` 签名（已查证）。

- [ ] **Step 2: 跑测试确认失败**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests::test_apaas_v1_top_chain_access`
Expected: FAIL

- [ ] **Step 3: 改 `v1/mod.rs` 入口**

`ApaasV1._config` → `config`，删 reserved 注释，`new()` 改名，加 8 访问器。注意 `application`/`workspace` 是**中间级**（持 `Arc<PlatformConfig>` + 路径参数），访问器签名：

```rust
    /// app 资源
    pub fn app(&self) -> app::AppService {
        app::AppService::new(self.config.as_ref().clone())
    }

    /// approval_task 资源
    pub fn approval_task(&self) -> approval_task::ApprovalTaskService {
        approval_task::ApprovalTaskService::new(self.config.as_ref().clone())
    }

    /// approval_instance 资源
    pub fn approval_instance(&self) -> approval_instance::ApprovalInstanceService {
        approval_instance::ApprovalInstanceService::new(self.config.as_ref().clone())
    }

    /// user_task 资源
    pub fn user_task(&self) -> user_task::UserTaskService {
        user_task::UserTaskService::new(self.config.as_ref().clone())
    }

    /// seat_activity 资源
    pub fn seat_activity(&self) -> seat_activity::SeatActivityService {
        seat_activity::SeatActivityService::new(self.config.as_ref().clone())
    }

    /// seat_assignment 资源
    pub fn seat_assignment(&self) -> seat_assignment::SeatAssignmentService {
        seat_assignment::SeatAssignmentService::new(self.config.as_ref().clone())
    }

    /// application 资源（中间级，持 namespace 路径参数）
    pub fn application(&self, namespace: impl Into<String>) -> application::ApplicationService {
        application::ApplicationService::new(self.config.clone(), namespace)
    }

    /// workspace 资源（中间级，持 workspace_id 路径参数）
    pub fn workspace(&self, workspace_id: impl Into<String>) -> workspace::WorkspaceService {
        workspace::WorkspaceService::new(self.config.clone(), workspace_id)
    }
```

`test_apaas_v1_creation` 里 `api._config` → `api.config`。

- [ ] **Step 4: 给 6 个顶层浅 service 加类型**

照 Task 1 `BadgeService` 模板（持 owned `Config`）。`app/mod.rs` 模板：

```rust
//! 应用列表与基础信息接口。

use openlark_core::config::Config;

pub mod list;

/// app 资源服务
#[derive(Debug, Clone)]
pub struct AppService {
    config: Config,
}

impl AppService {
    /// 创建新的 app 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 应用列表
    pub fn list(&self) -> list::AppListRequestBuilder {
        list::AppListRequestBuilder::new(self.config.clone())
    }
}
```

`approval_task`/`user_task` 内部多操作且部分带 `task_id`，照 `BadgeService.grant(badge_id)` 范式：访问器接受 task_id 下传（如 `pub fn cc(&self, task_id: impl Into<String>) -> cc::...Builder`，调 `cc::...Builder::new(self.config.clone(), task_id)`）。`transfer` 两参 `transfer(task_id, transfer_to_user_id)`。

- [ ] **Step 5: 给 `application/mod.rs` 与 `workspace/mod.rs` 加中间级 service**（仅类型 + 构造，深嵌套访问器留 Task 4/5）

`application/mod.rs` 顶部加：

```rust
use crate::PlatformConfig;
use std::sync::Arc;

/// application 资源服务（中间级，绑定 namespace）
#[derive(Debug, Clone)]
pub struct ApplicationService {
    config: Arc<PlatformConfig>,
    namespace: String,
}

impl ApplicationService {
    /// 创建新的 application 服务
    pub fn new(config: Arc<PlatformConfig>, namespace: impl Into<String>) -> Self {
        Self { config, namespace: namespace.into() }
    }
}
```

保留现有 `pub mod audit_log; pub mod environment_variable; ...` 声明不动。`workspace/mod.rs` 同理（`WorkspaceService { config: Arc<PlatformConfig>, workspace_id: String }`），保留现有 `pub use` 不动。

- [ ] **Step 6: 跑测试确认通过**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests`
Expected: PASS（`test_apaas_v1_creation` + `test_apaas_v1_top_chain_access`）

- [ ] **Step 7: clippy + fmt**

Run:
```bash
cargo clippy -p openlark-platform --all-targets -- -D warnings
cargo clippy -p openlark-platform -- -W dead_code
cargo fmt --check
```
Expected: 全 exit 0。
> **预期 dead_code 告警**：`ApplicationService`/`WorkspaceService` 字段（namespace/workspace_id）此刻未消费——若 clippy `-W dead_code` 报这俩字段，**临时**在字段上加 `#[allow(dead_code)]` 并注释 `// Task 4/5 将消费`，Task 4/5 完成后删除 allow。其余新 service 必须零 dead_code（都被 ApaasV1 消费）。

- [ ] **Step 8: 报告主会话勾选 + commit**

主会话勾选 `3.1`（顶层 8 service）+ `3.4`（ApaasV1 入口），commit（`feat(platform): apaas v1 顶层链式访问器（#274）`）。`3.2`/`3.3` 留 Task 4/5。

---

## Task 4: apaas application 深嵌套（object→record、role→member、record_permission→member、env_var/function/flow/audit_log）

> 这是全 change 最复杂的部分。`ApplicationService` 持 `Arc<PlatformConfig>` + `namespace`，下传给 `ObjectService`（再加 `object_api_name`）→ `RecordService`（持 owned Config + namespace + object_api_name）→ 叶子 builder。`role`/`record_permission` 同构（`RoleService` + role_api_name → `MemberService`）。`environment_variable`/`function`/`flow`/`audit_log` 是 application 直接子（持 namespace 已够，无需第三级）。

**Files:**
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/mod.rs`（`ApplicationService` 加子访问器，删 Step 5 临时 allow）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/object/mod.rs`（加 `ObjectService` 中间级 + record/search/oql_query 访问器；现有 `pub use` 保留）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/object/record/mod.rs`（加 `RecordService` 叶子级，持 owned Config + namespace + object_api_name）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/role/mod.rs`（加 `RoleService` 中间级 + member 访问器）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/role/member/mod.rs`（加 `RoleMemberService` 叶子级）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/record_permission/mod.rs`（加 `RecordPermissionService` 中间级）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/record_permission/member/mod.rs`（加 `RecordPermissionMemberService` 叶子级）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/environment_variable/mod.rs`（加 `EnvironmentVariableService`，application 直接子，持 owned Config + namespace）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/function/mod.rs`（加 `FunctionService`，同上 + function_api_name 中间级）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/flow/mod.rs`（加 `FlowService`，同上 + flow_id）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/application/audit_log/mod.rs`（加 `AuditLogService`，application 直接子，持 owned Config + namespace）

**Interfaces:**
- Consumes: `ApplicationService`（Task 3 产出，持 `Arc<PlatformConfig>` + `namespace`）。
- Produces（深链）：
  - `ApplicationService::object(object_api_name) -> ObjectService`（持 Arc + namespace + object_api_name）
  - `ObjectService::record() -> RecordService`（持 owned Config + namespace + object_api_name）
  - `RecordService::create()/batch_create()/query()/...`（喂 `RecordCreateRequestBuilder::new(config, namespace, object_api_name)`）
  - `ObjectService::search()/oql_query()`（application 直接喂，持 Arc 解引用）
  - `ApplicationService::role(role_api_name) -> RoleService` → `RoleService::member() -> RoleMemberService` → 叶子
  - `ApplicationService::record_permission(api_name) -> RecordPermissionService` → `member() -> ...MemberService`
  - `ApplicationService::environment_variable() -> EnvironmentVariableService` → `get(env_var_api_name)/query()` 叶子
  - `ApplicationService::function(function_api_name) -> FunctionService` → `invoke()` 叶子
  - `ApplicationService::flow(flow_id) -> FlowService` → `execute()` 叶子
  - `ApplicationService::audit_log() -> AuditLogService` → `list/get(log_id)/data_change_logs_list/data_change_log_detail(log_id)` 叶子

**深嵌套 builder `new()` 签名速查**（已全部核对，路径参数下传照此）：
- `object/record/{create,batch_create,query,patch,delete,batch_delete,batch_update,batch_query}.rs`: `new(config, namespace, object_api_name)` — 三参
- `object/search.rs`: `new(config, namespace, search)`；`object/oql_query.rs`: `new(config, namespace, oql)` — search/oql 是用户输入查询串
- `role/member/{batch_create_authorization,batch_remove_authorization,get}.rs`: `new(config, namespace, role_api_name)`
- `record_permission/member/{batch_create_authorization,batch_remove_authorization}.rs`: `new(config, namespace, record_permission_api_name)`
- `environment_variable/query.rs`: `new(config, namespace)`；`get.rs`: `new(config, namespace, env_var_api_name)`
- `function/invoke.rs`: `new(config, namespace, function_api_name)`
- `flow/execute.rs`: `new(config, namespace, flow_id)`
- `audit_log/audit_log_list.rs` + `data_change_logs_list.rs`: `new(config, namespace)`；`get.rs` + `data_change_log_detail.rs`: `new(config, namespace, log_id)`

- [ ] **Step 1: 写失败测试（深链至少走到叶子）**

在 `crates/openlark-platform/src/app_engine/apaas/v1/mod.rs` 的 tests 内加：

```rust
    #[test]
    fn test_apaas_v1_application_deep_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = ApaasV1::new(std::sync::Arc::new(config));
        let app = api.application("ns_x");
        // object → record 深链到叶子
        let _ = app.object("obj_y").record().create();
        let _ = app.object("obj_y").record().batch_create();
        let _ = app.object("obj_y").record().query();
        // object 直接子
        let _ = app.object("obj_y").search("q");
        let _ = app.object("obj_y").oql_query("select *");
        // role → member
        let _ = app.role("role_a").member().get();
        let _ = app.role("role_a").member().batch_create_authorization();
        // record_permission → member
        let _ = app.record_permission("rp_b").member().batch_create_authorization();
        // application 直接子
        let _ = app.environment_variable().query();
        let _ = app.environment_variable().get("var_k");
        let _ = app.function("fn_a").invoke();
        let _ = app.flow("flow_1").execute();
        let _ = app.audit_log().list();
        let _ = app.audit_log().get("log_9");
    }
```

> 各叶子访问器的参数（如 `search("q")`、`oql_query("select *")`、`get("var_k")`）照 builder `new()` 的用户输入参数照搬；路径参数（namespace/object_api_name/role_api_name）已由上级 service 持有，叶子访问器不再要。

- [ ] **Step 2: 跑测试确认失败**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests::test_apaas_v1_application_deep_chain_access`
Expected: FAIL

- [ ] **Step 3: `ApplicationService` 加子访问器**（application/mod.rs）

在 Task 3 建的 `ApplicationService` impl 里加（并删掉 Task 3 Step 5 的临时 `#[allow(dead_code)]`）：

```rust
    /// application.object 子资源
    pub fn object(&self, object_api_name: impl Into<String>) -> object::ObjectService {
        object::ObjectService::new(self.config.clone(), self.namespace.clone(), object_api_name)
    }

    /// application.role 子资源
    pub fn role(&self, role_api_name: impl Into<String>) -> role::RoleService {
        role::RoleService::new(self.config.clone(), self.namespace.clone(), role_api_name)
    }

    /// application.record_permission 子资源
    pub fn record_permission(
        &self,
        record_permission_api_name: impl Into<String>,
    ) -> record_permission::RecordPermissionService {
        record_permission::RecordPermissionService::new(
            self.config.clone(),
            self.namespace.clone(),
            record_permission_api_name,
        )
    }

    /// application.environment_variable 子资源（持 namespace，叶子级）
    pub fn environment_variable(&self) -> environment_variable::EnvironmentVariableService {
        environment_variable::EnvironmentVariableService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
        )
    }

    /// application.function 子资源
    pub fn function(&self, function_api_name: impl Into<String>) -> function::FunctionService {
        function::FunctionService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            function_api_name,
        )
    }

    /// application.flow 子资源
    pub fn flow(&self, flow_id: impl Into<String>) -> flow::FlowService {
        flow::FlowService::new(self.config.as_ref().clone(), self.namespace.clone(), flow_id)
    }

    /// application.audit_log 子资源
    pub fn audit_log(&self) -> audit_log::AuditLogService {
        audit_log::AuditLogService::new(self.config.as_ref().clone(), self.namespace.clone())
    }
```

> 注意 config 流转（D2）：`object`/`role`/`record_permission` 是中间级（下还有 member/record 一级），持 `Arc<PlatformConfig>`（`self.config.clone()`）；`environment_variable`/`function`/`flow`/`audit_log` 的子是叶子 builder，这级 service 直接持 owned `Config`（`self.config.as_ref().clone()`）。

- [ ] **Step 4: `ObjectService` 中间级**（object/mod.rs）

```rust
use crate::PlatformConfig;
use std::sync::Arc;

// 保留现有 pub mod / pub use 不动

/// application.object 资源服务（中间级，绑 namespace + object_api_name）
#[derive(Debug, Clone)]
pub struct ObjectService {
    config: Arc<PlatformConfig>,
    namespace: String,
    object_api_name: String,
}

impl ObjectService {
    pub fn new(
        config: Arc<PlatformConfig>,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
        }
    }

    /// object.record 子资源（叶子级）
    pub fn record(&self) -> record::RecordService {
        record::RecordService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }

    /// 记录搜索
    pub fn search(&self, search: impl Into<String>) -> search::RecordSearchRequestBuilder {
        search::RecordSearchRequestBuilder::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            search,
        )
    }

    /// OQL 查询
    pub fn oql_query(&self, oql: impl Into<String>) -> oql_query::OqlQueryRequestBuilder {
        oql_query::OqlQueryRequestBuilder::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            oql,
        )
    }
}
```

- [ ] **Step 5: `RecordService` 叶子级**（object/record/mod.rs）

```rust
use openlark_core::config::Config;

// 保留现有 pub mod 声明不动

/// object.record 资源服务（叶子级，持 owned Config + 路径参数）
#[derive(Debug, Clone)]
pub struct RecordService {
    config: Config,
    namespace: String,
    object_api_name: String,
}

impl RecordService {
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
        }
    }

    pub fn create(&self) -> create::RecordCreateRequestBuilder {
        create::RecordCreateRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }

    // batch_create / query / patch / delete / batch_delete / batch_update / batch_query 同模板
}
```

- [ ] **Step 6: `RoleService` + `RoleMemberService`、`RecordPermissionService` + `RecordPermissionMemberService`**

照 Step 4/5 同构。`role/mod.rs` 加 `RoleService { config: Arc, namespace, role_api_name }` + `member()` 返回 `member::RoleMemberService`；`role/member/mod.rs` 加 `RoleMemberService { config: Config, namespace, role_api_name }` + `get()/batch_create_authorization()/batch_remove_authorization()` 各喂对应 builder `new(config, namespace, role_api_name)`。record_permission 同构（`record_permission_api_name` 替换 `role_api_name`）。

- [ ] **Step 7: 4 个 application 直接子 service**（environment_variable / function / flow / audit_log）

照 Step 5 叶子级模板（持 owned `Config` + namespace [+ 用户输入参数]）。例如 `environment_variable/mod.rs`：

```rust
use openlark_core::config::Config;

// 保留 pub mod

/// application.environment_variable 服务（叶子级）
#[derive(Debug, Clone)]
pub struct EnvironmentVariableService {
    config: Config,
    namespace: String,
}

impl EnvironmentVariableService {
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self { config, namespace: namespace.into() }
    }

    pub fn query(&self) -> query::EnvironmentVariableQueryRequestBuilder {
        query::EnvironmentVariableQueryRequestBuilder::new(self.config.clone(), self.namespace.clone())
    }

    pub fn get(&self, env_var_api_name: impl Into<String>) -> get::EnvironmentVariableGetRequestBuilder {
        get::EnvironmentVariableGetRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            env_var_api_name,
        )
    }
}
```

`function`（持 namespace + function_api_name，`invoke()`）、`flow`（namespace + flow_id，`execute()`）、`audit_log`（持 namespace，`list()/get(log_id)/data_change_logs_list()/data_change_log_detail(log_id)`）同理。audit_log 的 list 与 get/data_change_log_detail 的参数差异照 builder 真实 `new()`。

- [ ] **Step 8: 跑测试确认通过**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests`
Expected: PASS（含 `test_apaas_v1_application_deep_chain_access`）

- [ ] **Step 9: clippy + fmt（含深嵌套无 dead_code）**

Run:
```bash
cargo clippy -p openlark-platform --all-targets -- -D warnings
cargo clippy -p openlark-platform -- -W dead_code
cargo fmt --check
```
Expected: 全 exit 0，**零新 dead_code**（Task 3 的临时 allow 已删，所有字段都被下级访问器消费）。

- [ ] **Step 10: 报告主会话勾选 + commit**

主会话勾选 `3.2`，commit（`feat(platform): apaas application 深嵌套访问器（#274）`）。

---

## Task 5: apaas workspace 嵌套（table/view/enum_mod/sql_commands）

**Files:**
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/workspace/mod.rs`（`WorkspaceService` 加子访问器，删 Task 3 临时 allow）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/workspace/table/mod.rs`（加 `TableService`，持 owned Config + workspace_id；现有 `pub use` 保留）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/workspace/view/mod.rs`（加 `ViewService` 或直接访问器）
- Modify: `crates/openlark-platform/src/app_engine/apaas/v1/workspace/enum_mod/mod.rs`（加 `EnumModService` 或直接访问器）

**Interfaces:**
- Consumes: `WorkspaceService`（Task 3 产出，持 `Arc<PlatformConfig>` + `workspace_id`）。
- Produces:
  - `WorkspaceService::sql_commands(sql)` → 叶子 builder（`new(config, workspace_id, sql)`）
  - `WorkspaceService::table(table_name) -> TableService`（持 owned Config + workspace_id + table_name）→ `list()/table_get()/records_post()/records_get()/records_patch()/records_delete()/records_batch_update()` 叶子
  - `WorkspaceService::view(view_name)` → `views_get` 叶子（`new(config, workspace_id, view_name)`）
  - `WorkspaceService::enum_mod() -> EnumModService` → `list()/enum_get()` 叶子

**workspace builder `new()` 签名速查**（已核对）：
- `workspace/sql_commands.rs`: `new(config, workspace_id, sql)`
- `workspace/table/list.rs`: `new(config, workspace_id)`；`table_get.rs`/`records_post.rs`/`records_get.rs`/`records_patch.rs`/`records_delete.rs`/`records_batch_update.rs`: `new(config, workspace_id, table_name)` — table_name 在 table 级绑定
- `workspace/view/views_get.rs`: `new(config, workspace_id, view_name)`
- `workspace/enum_mod/list.rs` + `enum_get.rs`: 用 grep 核对（预期 `new(config, workspace_id)`）

- [ ] **Step 1: 写失败测试**

在 `apaas/v1/mod.rs` tests 内加：

```rust
    #[test]
    fn test_apaas_v1_workspace_chain_access() {
        let config = PlatformConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let api = ApaasV1::new(std::sync::Arc::new(config));
        let ws = api.workspace("ws_x");
        // 直接叶子
        let _ = ws.sql_commands("select 1");
        // table 子资源
        let _ = ws.table("tbl_y").list();
        let _ = ws.table("tbl_y").table_get();
        let _ = ws.table("tbl_y").records_post();
        // view 叶子
        let _ = ws.view("vw_z").views_get();
        // enum_mod
        let _ = ws.enum_mod().list();
    }
```

> table 级持 table_name，`list()` 走 `new(config, workspace_id)` 不带 table_name——核对 `table/list.rs` 真实签名：若 list 不需要 table_name，则 list 放在 `WorkspaceService` 直接访问器 `ws.list()` 而非 table 级。实施前 grep 确认，调整链结构。

- [ ] **Step 2: 跑测试确认失败**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests::test_apaas_v1_workspace_chain_access`
Expected: FAIL

- [ ] **Step 3: `WorkspaceService` 加子访问器**（workspace/mod.rs，删 Task 3 临时 allow）

```rust
    /// SQL 执行
    pub fn sql_commands(&self, sql: impl Into<String>) -> sql_commands::SqlCommandsRequestBuilder {
        sql_commands::SqlCommandsRequestBuilder::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            sql,
        )
    }

    /// table 子资源
    pub fn table(&self, table_name: impl Into<String>) -> table::TableService {
        table::TableService::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            table_name,
        )
    }

    /// view 子资源（直接返回 builder，view 只有 views_get 一个操作）
    pub fn view(&self, view_name: impl Into<String>) -> view::views_get::ViewsGetRequestBuilder {
        view::views_get::ViewsGetRequestBuilder::new(
            self.config.as_ref().clone(),
            self.workspace_id.clone(),
            view_name,
        )
    }

    /// enum_mod 子资源
    pub fn enum_mod(&self) -> enum_mod::EnumModService {
        enum_mod::EnumModService::new(self.config.as_ref().clone(), self.workspace_id.clone())
    }
```

> 若 `view` 模块将来会扩多操作，则建 `ViewService`；当前只 `views_get`，直接返回 builder 更简（D4 不 premature 抽象）。实施时若发现 view 有多 `.rs`，则升为 service。

- [ ] **Step 4: `TableService` 叶子级 + `EnumModService`**

`workspace/table/mod.rs`：

```rust
use openlark_core::config::Config;

// 保留现有 pub mod / pub use

/// workspace.table 资源服务（叶子级）
#[derive(Debug, Clone)]
pub struct TableService {
    config: Config,
    workspace_id: String,
    table_name: String,
}

impl TableService {
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
        }
    }

    pub fn table_get(&self) -> table_get::TableGetRequestBuilder {
        table_get::TableGetRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }

    pub fn records_post(&self) -> records_post::RecordsPostRequestBuilder {
        records_post::RecordsPostRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }

    // records_get / records_patch / records_delete / records_batch_update 同模板
    // list 若 new(config, workspace_id) 不带 table_name → 移到 WorkspaceService.list()
}
```

`workspace/enum_mod/mod.rs` 加 `EnumModService { config: Config, workspace_id: String }` + `list()/enum_get()`。

- [ ] **Step 5: 跑测试确认通过**

Run: `cargo test -p openlark-platform --lib app_engine::apaas::v1::tests`
Expected: PASS（含 workspace 链）

- [ ] **Step 6: clippy + fmt**

Run:
```bash
cargo clippy -p openlark-platform --all-targets -- -D warnings
cargo clippy -p openlark-platform -- -W dead_code
cargo fmt --check
```
Expected: 全 exit 0，零新 dead_code。

- [ ] **Step 7: 报告主会话勾选 + commit**

主会话勾选 `3.3` + `3.5` + `3.6`，commit（`feat(platform): apaas workspace 嵌套访问器（#274）`）。

---

## Task 6: 全局验证 + 闭环（cleanup-dead-code-allows 收尾）

> 所有源码改完后，跑全局 CI 红线 + 确认无 `_config` 遗留 + 移除 reserved 注释。这是 change 的最终门控。

**Files:**
- 全 workspace（只读验证）
- 若发现遗漏：回到对应 Task 修

- [ ] **Step 1: `cargo fmt --check`（workspace）**

Run: `cargo fmt --check`
Expected: exit 0（无 diff 输出）。
> **CI 红线**：CI lint job 第一步就是 `cargo fmt --check`，clippy 过 ≠ fmt 过。

- [ ] **Step 2: `cargo clippy --workspace --all-targets --all-features -- -D warnings`**

Run: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
Expected: exit 0。

- [ ] **Step 3: `cargo clippy -p openlark-platform -- -W dead_code`**

Run: `cargo clippy -p openlark-platform -- -W dead_code`
Expected: 无新增 dead_code 告警（设计上每个新 service 都被上级访问器消费）。
> 若有告警：说明某 service 字段/方法未被消费，回到对应 Task 补访问器或修签名。**不要用 `#[allow(dead_code)]` 掩盖**（除 Task 3 Step 7 已说明的中间级临时 allow，且 Task 4/5 已删）。

- [ ] **Step 4: grep 确认 3 入口无 `_config` 遗留**

Run:
```bash
grep -rn '_config' crates/openlark-platform/src/admin/admin/v1/mod.rs \
                crates/openlark-platform/src/directory/directory/v1/mod.rs \
                crates/openlark-platform/src/app_engine/apaas/v1/mod.rs
```
Expected: 0 匹配（三入口 struct 字段已全 `config`）。
> 注意：admin 的 `audit.rs`/`users.rs` facade 内的 `QueryAuditLogsRequest._config` 等是**另一回事**（runtime stub 的请求体字段，非入口 struct 字段），不在本 grep 范围（那俩文件不在上面列表里）。它们的 `_config` 是 cleanup-dead-code-allows 留的 stub 脚手架，本 change 不动（D5：不改 stub 行为）。

- [ ] **Step 5: grep 确认 3 入口 reserved 注释已移除**

Run:
```bash
grep -rn 'reserved：待装访问器' crates/openlark-platform/src/admin/admin/v1/mod.rs \
                                  crates/openlark-platform/src/directory/directory/v1/mod.rs \
                                  crates/openlark-platform/src/app_engine/apaas/v1/mod.rs
```
Expected: 0 匹配（闭环 cleanup-dead-code-allows：3 个入口的「待装访问器」reserved 注释随 `_config` → `config` 一起删了，Task 1/2/3 Step 3 已做）。

- [ ] **Step 6: 完整测试**

Run: `cargo test -p openlark-platform`
Expected: 全 PASS（含三入口的 chain access 测试 + 各 builder 原有单测）。

- [ ] **Step 7: 报告主会话勾选 + commit**

主会话勾选 `4.1`-`4.5`，commit（`chore(platform): v1 访问器闭环验证（#274）`）。change 进入 verify 阶段。

---

## Self-Review

**1. Spec 覆盖**（tasks.md 四组 → Task）：
- `1.1`-`1.5`（admin 6 操作集合 + 2 facade + 测试 + clippy）→ Task 1 ✓
- `2.1`-`2.4`（directory 8 子模块 + 测试 + clippy）→ Task 2 ✓
- `3.1`（顶层 8 service）→ Task 3 ✓
- `3.2`（application 深嵌套逐级）→ Task 4 ✓
- `3.3`（workspace 嵌套）→ Task 5 ✓
- `3.4`（ApaasV1 入口）→ Task 3 ✓
- `3.5`（深链 access 测试）→ Task 4 Step 1 ✓
- `3.6`（apaas clippy + fmt）→ Task 3/4/5 各 Step 末尾 + Task 6 全局 ✓
- `4.1`-`4.5`（全局验证 + 闭环）→ Task 6 ✓

**2. D1-D6 范式覆盖**：
- D1 full-depth → 所有 Task 的 service 链都到叶子 builder ✓
- D2 config 流转 → Task 1 Step 3 注释 facade vs 叶子区别；Task 3 Step 5 中间级 Arc；Task 4 Step 3 注释中间级 vs 叶子 ✓
- D3 返回值类型 service + `#[derive(Debug, Clone)]` → 所有 service 模板 ✓
- D4 手写无宏 → 全部模板复制 ✓
- D5 facade 复用 → Task 1 Step 3 `audit()`/`users()` 返回已有 `AuditApi`/`UsersApi`，不新建类型 ✓
- D6 测试范式 → Task 1/2/3/4/5 各 Step 1 的 `test_*_chain_access` 仿 `test_spark_v1_directory_access` ✓

**3. Placeholder 扫描**：
- 部分 builder `new()` 参数（badge_id/task_id/name 等）标注「实施时 grep 核对」——这是有意的：计划已给出已验证的主要签名速查表，但个别 builder（如 admin badge/get、user_task/rollback）的次要参数实施时需照搬，不是 placeholder 而是明确的核对指令。
- 所有 service 模板含完整代码（struct + impl + 访问器），非「Similar to Task N」。

**4. 类型一致性**：
- `XxxService` 命名统一（`BadgeService`/`DepartmentService`/`ApplicationService`/`ObjectService`/`RecordService`/`RoleService`/`RoleMemberService`/`RecordPermissionService`/`RecordPermissionMemberService`/`EnvironmentVariableService`/`FunctionService`/`FlowService`/`AuditLogService`/`TableService`/`EnumModService`/`AppService`/`ApprovalTaskService`/`ApprovalInstanceService`/`UserTaskService`/`SeatActivityService`/`SeatAssignmentService`）。
- config 字段：入口/中间级 `Arc<PlatformConfig>`，叶子 `Config`——全计划一致。
- 路径参数命名：`namespace`/`object_api_name`/`role_api_name`/`record_permission_api_name`/`function_api_name`/`flow_id`/`workspace_id`/`table_name`/`env_var_api_name`/`log_id`——与 builder `new()` 参数名对齐（已核对）。
