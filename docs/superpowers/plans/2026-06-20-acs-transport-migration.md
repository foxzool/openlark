# acs 子模块迁移到 Transport + core Config — 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把 `openlark-security` crate 的 acs 子模块（31 个端点 + 6 个资源 Service）改造成 SDK 唯一规范实现——走 `Arc<openlark_core::Config>` + `Transport::request` + `validate_required!`，删除绕过 Transport 的原始 `reqwest::Client::new()` Service/Builder 层，并修复 `rule_external` 的字段语义错误。

**Architecture:** Approach A——在 `SecurityServices::new` 边界做一次 `SecurityConfig → openlark_core::Config` 转换喂给 `AcsProject`；security_and_compliance 不动；公开 `SecurityClient::new(SecurityConfig)` 签名不变。每个端点恢复 `validate_required!` + `.with_supported_access_token_types([App])`（acs 是应用级接口，需要 App token），并返回真实 `resp.data`。

**Tech Stack:** Rust, openlark-core（`Transport`/`Config`/`ApiRequest`/`validate_required!`），serde，飞书 acs v1 API。

**关联 spec:** `docs/superpowers/specs/2026-06-20-acs-transport-migration-design.md`

---

## 关键背景（执行者必读）

1. **现状分类**（已 grep 确认）：31 个端点文件**全部**已挂 `Transport::request` + `Arc<Config>`，但：
   - **全部缺** `validate_required!` 和 `.with_supported_access_token_types([App])`。
   - **24 个**返回 `Ok(XxxResponse { data: None })` 丢弃响应；7 个返回 `resp.data` 但路径/方法往往是错的（如 `user/create.rs` 用 `GET` + 假 `.replace()` 链拼出 `/open-apis/security/acs/v1/user/create`，既不是 POST 也不是真实路径）。
   - **结论**：不能信任任何现有端点文件，每个都要重写。
2. **Service/Builder 层**（`users/mod.rs` 等 6 个 `mod.rs`）用原始 `reqwest::Client::new()` 手工发请求，是真正被 `AcsV1Service` 调用的路径，但**字段语义也错**（rule_external）。本计划把真实逻辑迁到端点文件，把 `mod.rs` 瘦化为门面。
3. **Token**：`ApiRequest` 默认 `supported_access_token_types = [User, Tenant]`，但 acs 是应用级接口，**必须显式设 `[AccessTokenType::App]`**，否则 Transport 解析到 Tenant/User token，飞书会拒绝。这是每个端点都要做的动作。
4. **Config 构造**：`Config::builder().app_id(..).app_secret(..).base_url(..).build()` 是规范构造链。
5. **路径参数端点**清单（需要 `validate_required!` 校验路径 ID）：所有 `/{resource}/{id}` 形式。
6. **rule_external 文档核对结论**（已用 Playwright 核对真实文档）：
   - `create`：`?rule_id=&user_id_type=` 查询参数 + body `{"rule": {devices:[...]}}` 包装
   - `delete`：`?rule_id=` 查询参数，**无 body**
   - `get`：`?device_id=&user_id_type=` 查询参数，无 body
   - `device_bind`：flat body `{"device_id": "<单个>", "rule_ids": [...]}`

## 文件结构

**修改：**
- `crates/openlark-security/src/lib.rs` — `SecurityServices::new` 加 SecurityConfig→Config 转换
- `crates/openlark-security/src/acs/acs/mod.rs` — `AcsProject`/`AcsV1Service` 改吃 `Arc<Config>`
- `crates/openlark-security/src/acs/acs/v1/{users,devices,visitors,user_faces,access_records,rule_external}/mod.rs` — 瘦化为门面 Service（删 Builder/reqwest）
- 31 个端点 `.rs` 文件 — 重写为规范模式

**不动：** `security_and_compliance/**`、`models/SecurityConfig`、`models/mod.rs`。

---

## Task 1: 边界转换 — SecurityServices 接 core Config

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/mod.rs`
- Modify: `crates/openlark-security/src/lib.rs`

- [ ] **Step 1: 改 `AcsProject`/`AcsV1Service` 吃 `Arc<Config>`**

在 `crates/openlark-security/src/acs/acs/mod.rs`，把所有 `Arc<crate::models::SecurityConfig>` 替换为 `Arc<openlark_core::config::Config>`。`AcsProject::new` 和 `AcsV1Service::new` 签名改为：

```rust
use openlark_core::config::Config;
use std::sync::Arc;

pub struct AcsProject {
    config: Arc<Config>,
    v1: AcsV1Service,
}
impl AcsProject {
    pub fn new(config: Arc<Config>) -> Self {
        Self { v1: AcsV1Service::new(config.clone()), config }
    }
    pub fn config(&self) -> &Config { &self.config }
}
pub struct AcsV1Service {
    config: Arc<Config>,
    // 6 个资源 service 字段（后续 task 瘦化）
    // ...
}
impl AcsV1Service {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            // 后续 task 填充
            config,
            users: crate::acs::acs::v1::users::UsersService::new(config.clone()),
            // ...
        }
    }
}
```

> 注：此时 6 个资源 Service 还是旧的 raw-reqwest 版（吃 SecurityConfig），会编译失败。先注释掉 `AcsV1Service` 的 6 个字段和构造，只留 `config` 字段，让 crate 编译通过。后续 task 逐个恢复。

- [ ] **Step 2: 在 `SecurityServices::new` 加转换**

`crates/openlark-security/src/lib.rs`，`SecurityServices::new` 内为 acs 构造 core Config：

```rust
impl SecurityServices {
    pub fn new(config: crate::models::SecurityConfig) -> Self {
        let sec_config = std::sync::Arc::new(config);
        // 为 acs 转换出 core Config（Approach A shim）
        let core_config = std::sync::Arc::new(
            openlark_core::config::Config::builder()
                .app_id(sec_config.app_id.clone())
                .app_secret(sec_config.app_secret.clone())
                .base_url(sec_config.base_url.clone())
                .build(),
        );
        Self {
            acs: AcsProject::new(core_config),
            security_and_compliance: SecurityAndComplianceProject::new(sec_config.clone()),
            config: sec_config,
        }
    }
}
```

- [ ] **Step 3: 编译验证**

Run: `cargo check -p openlark-security`
Expected: 编译通过（AcsV1Service 字段已注释，仅 config）。若有 `prelude`/`lib.rs` re-export 引用旧签名，一并修正。

- [ ] **Step 4: Commit**

```bash
git add crates/openlark-security/src/lib.rs crates/openlark-security/src/acs/acs/mod.rs
git commit -m "refactor(acs): 边界转换 SecurityConfig→core Config (Approach A)"
```

---

## Task 2: users 资源组重写（user/get, list, patch, create, delete + face 子组）

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/user/get.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/list.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/patch.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/create.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/delete.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/face/get.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/user/face/update.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/users/mod.rs`

**端点 → 飞书路径/方法/校验映射：**

| 文件 | 方法 | 路径 | 校验 |
|------|------|------|------|
| `user/get.rs` | GET | `/open-apis/acs/v1/users/{user_id}` | `validate_required!(user_id)` |
| `user/list.rs` | GET | `/open-apis/acs/v1/users` | 无路径 ID（分页参数可选） |
| `user/patch.rs` | PATCH | `/open-apis/acs/v1/users/{user_id}` | `validate_required!(user_id)` |
| `user/create.rs` | POST | `/open-apis/acs/v1/users` | body 必填 |
| `user/delete.rs` | DELETE | `/open-apis/acs/v1/users/{user_id}` | `validate_required!(user_id)` |
| `user/face/get.rs` | GET | `/open-apis/acs/v1/users/{user_id}/face` | `validate_required!(user_id)` |
| `user/face/update.rs` | PATCH/POST | `/open-apis/acs/v1/users/{user_id}/face` | `validate_required!(user_id)` |

- [ ] **Step 1: 写 `user/get.rs` 的失败测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use std::sync::Arc;

    fn test_config() -> Arc<Config> {
        Arc::new(Config::builder().app_id("x").app_secret("y").build())
    }

    #[tokio::test]
    async fn test_get_user_rejects_empty_id() {
        let req = GetUserRequest::new(test_config(), "  ");
        let result = req.execute_with_options(Default::default()).await;
        assert!(result.is_err(), "空 user_id 必须校验失败");
        let err = result.unwrap_err();
        assert!(err.to_string().contains("user_id"));
    }
}
```

- [ ] **Step 2: 运行测试，确认失败**

Run: `cargo test -p openlark-security user::get::tests 2>&1 | tail -5`
Expected: FAIL（当前 `get.rs` 无校验，会尝试发请求而非返回校验错误；或编译错误因为还没改实现）

- [ ] **Step 3: 重写 `user/get.rs` 实现**

```rust
//! 获取单个用户信息
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetUserRequest {
    config: Arc<Config>,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetUserResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}

impl GetUserRequest {
    pub fn new(config: Arc<Config>, user_id: impl Into<String>) -> Self {
        Self { config, user_id: user_id.into() }
    }

    pub async fn execute(self) -> SDKResult<GetUserResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetUserResponse> {
        validate_required!(self.user_id.trim(), "user_id 不能为空");
        let path = format!("/open-apis/acs/v1/users/{}", self.user_id);
        let req = ApiRequest::get(&path)
            .with_supported_access_token_types(vec![AccessTokenType::App]);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetUserResponse { data: resp.data })
    }
}
```

> 注：这里返回 `Ok(GetUserResponse { data: resp.data })` 而非 `resp.data.ok_or_else(...)`——因为 list 类接口 data 可空是合法的。对于"必然有数据"的接口（如 get 单个）可用 `ok_or_else`。统一用前者更安全，spec §8 已说明这是增强非破坏。

- [ ] **Step 4: 运行测试，确认通过**

Run: `cargo test -p openlark-security user::get::tests 2>&1 | tail -5`
Expected: PASS

- [ ] **Step 5: 重写 `user/list.rs`、`user/patch.rs`、`user/delete.rs`、`user/create.rs`**

对每个文件套用 Step 3 的模式：
- `list.rs`：`Get` `/open-apis/acs/v1/users`，无校验。类型名保持 `ListUsersRequest`/`ListUsersResponse`（已重命名）。
- `patch.rs`：`Patch`，`validate_required!(user_id)`，路径 `/users/{user_id}`。
- `delete.rs`：`Delete`，`validate_required!(user_id)`。
- `create.rs`：`Post` `/open-apis/acs/v1/users`，body 用 `Option<serde_json::Value>` 类型 setter（`.body_value(json)`）。**本计划范围只做 Transport 迁移 + 必填校验，不细化各 POST body 的具体字段**（深层字段建模是独立的字段核对工作，不在本 spec 范围——见 spec §9）。所有 POST 端点统一用 `serde_json::Value` body，调用方自行构造 JSON。

每个文件都加 `validate_required!`（路径参数的）、`.with_supported_access_token_types(vec![AccessTokenType::App])`、`Ok(X { data: resp.data })`。

- [ ] **Step 6: 重写 `user/face/get.rs` 和 `user/face/update.rs`**

`face/get.rs` 恢复 typed `FaceData`：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserFaceResponse {
    pub data: Option<FaceData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    pub face_url: String,
}
```

执行体：`validate_required!(user_id)` + GET `/users/{user_id}/face` + App token + `Ok(GetUserFaceResponse { data: resp.data })`。

`face/update.rs`：Patch/Post，`validate_required!(user_id)`，body setter。

- [ ] **Step 7: 瘦化 `users/mod.rs` 为门面**

把 `users/mod.rs` 的 `UsersService` + 所有 `*Builder` + `get_app_token` + raw reqwest **全部删除**，替换为：

```rust
//! 门禁用户管理 API
use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UsersService {
    config: Arc<Config>,
}

impl UsersService {
    pub fn new(config: Arc<Config>) -> Self { Self { config } }
    pub fn get(&self, user_id: impl Into<String>) -> super::user::get::GetUserRequest {
        super::user::get::GetUserRequest::new(self.config.clone(), user_id)
    }
    pub fn list(&self) -> super::user::list::ListUsersRequest {
        super::user::list::ListUsersRequest::new(self.config.clone())
    }
    pub fn patch(&self, user_id: impl Into<String>) -> super::user::patch::PatchUserRequest {
        super::user::patch::PatchUserRequest::new(self.config.clone(), user_id)
    }
    pub fn create(&self) -> super::user::create::UserCreateRequest {
        super::user::create::UserCreateRequest::new(self.config.clone())
    }
    pub fn delete(&self, user_id: impl Into<String>) -> super::user::delete::DeleteUserRequest {
        super::user::delete::DeleteUserRequest::new(self.config.clone(), user_id)
    }
}
```

> 注：`users/mod.rs` 当前声明了 `pub mod user;` 等子模块。瘦化时保留模块声明，只删 Service/Builder 逻辑。`user_faces/mod.rs` 同理（见 Task 5，但 face 的端点在 `user/face/` 下，确认模块树结构后归并）。

- [ ] **Step 8: 在 `AcsV1Service` 恢复 users 字段**

回到 `acs/acs/mod.rs`，取消注释 `users` 字段，构造调 `UsersService::new(config.clone())`。

- [ ] **Step 9: 编译 + 全部 users 测试**

Run: `cargo test -p openlark-security user:: 2>&1 | tail -10`
Expected: PASS（每个端点的空 ID 校验测试都过）

- [ ] **Step 10: Commit**

```bash
git add crates/openlark-security/src/acs/acs/v1/user/ crates/openlark-security/src/acs/acs/v1/users/mod.rs crates/openlark-security/src/acs/acs/mod.rs
git commit -m "refactor(acs): users 资源组重写为 Transport+validate，瘦化 Service 为门面"
```

---

## Task 3: devices 资源组重写

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/device/{get,create,update,delete,list,approve,query}.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/client_device/get.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/devices/mod.rs`

**端点映射：**

| 文件 | 方法 | 路径 | 校验 |
|------|------|------|------|
| `device/get.rs` | GET | `/open-apis/acs/v1/devices/{device_id}` | `validate_required!(device_id)` |
| `device/create.rs` | POST | `/open-apis/acs/v1/devices` | body |
| `device/update.rs` | PUT/PATCH | `/open-apis/acs/v1/devices/{device_id}` | `validate_required!(device_id)` |
| `device/delete.rs` | DELETE | `/open-apis/acs/v1/devices/{device_id}` | `validate_required!(device_id)` |
| `device/list.rs` | GET | `/open-apis/acs/v1/devices` | 无 |
| `device/approve.rs` | POST | `/open-apis/acs/v1/devices/{device_id}/approve` | `validate_required!(device_id)` |
| `device/query.rs` | POST | `/open-apis/acs/v1/devices/query` | 无 |
| `client_device/get.rs` | GET | `/open-apis/acs/v1/client_devices/{device_id}` | `validate_required!(device_id)` |

- [ ] **Step 1: 为 `device/get.rs` 写空 device_id 校验测试**（模式同 Task 2 Step 1）

```rust
#[tokio::test]
async fn test_get_device_rejects_empty_id() {
    let req = GetDeviceRequest::new(test_config(), "  ");
    let result = req.execute_with_options(Default::default()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("device_id"));
}
```

- [ ] **Step 2: 运行测试确认失败**

Run: `cargo test -p openlark-security device::get::tests 2>&1 | tail -5`
Expected: FAIL

- [ ] **Step 3: 重写全部 8 个 device 端点**

每个文件套 Task 2 Step 3 的模式：删无意义 `.replace()` 链、改对路径、`validate_required!`（路径参数的）、`.with_supported_access_token_types(vec![AccessTokenType::App])`、`Ok(X { data: resp.data })`、删占位序列化测试。

- [ ] **Step 4: 瘦化 `devices/mod.rs` 为门面**（模式同 Task 2 Step 7）

```rust
pub struct DevicesService { config: Arc<Config> }
impl DevicesService {
    pub fn new(config: Arc<Config>) -> Self { Self { config } }
    pub fn get(&self, device_id: impl Into<String>) -> super::device::get::GetDeviceRequest { ... }
    pub fn list(&self) -> super::device::list::ListDevicesRequest { ... }
    pub fn create(&self) -> super::device::create::DeviceCreateRequest { ... }
    pub fn update(&self, device_id: impl Into<String>) -> super::device::update::UpdateDeviceRequest { ... }
    pub fn delete(&self, device_id: impl Into<String>) -> super::device::delete::DeleteDeviceRequest { ... }
    pub fn approve(&self, device_id: impl Into<String>) -> super::device::approve::ApproveDeviceRequest { ... }
    pub fn query(&self) -> super::device::query::QueryDeviceRequest { ... }
}
```

- [ ] **Step 5: 在 `AcsV1Service` 恢复 devices 字段**

- [ ] **Step 6: 编译 + 测试**

Run: `cargo test -p openlark-security device:: 2>&1 | tail -10`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/openlark-security/src/acs/acs/v1/device/ crates/openlark-security/src/acs/acs/v1/client_device/ crates/openlark-security/src/acs/acs/v1/devices/mod.rs crates/openlark-security/src/acs/acs/mod.rs
git commit -m "refactor(acs): devices 资源组重写为 Transport+validate，瘦化 Service"
```

---

## Task 4: rule_external 资源组重写（含字段语义修复）

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/rule_external/{create,delete,get,device_bind}.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/rule_external/mod.rs`

> 这是 spec §5 的核心修复，字段语义按文档核对结论（背景第 6 条）。**注意**：rule_external 的 4 个端点用 typed body struct（字段是评审发现的 bug，必须修），其它 POST 端点（user/device/face/visitor create）用 `serde_json::Value` body（不在本计划范围）。

- [ ] **Step 1: 写 `rule_external/delete.rs` 测试（无 body，query 参数）**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use std::sync::Arc;
    fn cfg() -> Arc<Config> { Arc::new(Config::builder().app_id("x").app_secret("y").build()) }

    #[tokio::test]
    async fn test_delete_rule_rejects_empty_id() {
        let req = DeleteRuleExternalRequest::new(cfg(), "  ");
        let result = req.execute_with_options(Default::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("rule_id"));
    }
}
```

- [ ] **Step 2: 运行确认失败**

Run: `cargo test -p openlark-security rule_external::delete::tests 2>&1 | tail -5`
Expected: FAIL

- [ ] **Step 3: 重写 `rule_external/delete.rs`（无 body，纯 query）**

```rust
//! 删除权限组
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteRuleExternalRequest {
    config: Arc<Config>,
    rule_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRuleExternalResponse {
    pub data: Option<serde_json::Value>,
}
impl ApiResponseTrait for DeleteRuleExternalResponse {
    fn data_format() -> ResponseFormat { ResponseFormat::Data }
}
impl DeleteRuleExternalRequest {
    pub fn new(config: Arc<Config>, rule_id: impl Into<String>) -> Self {
        Self { config, rule_id: rule_id.into() }
    }
    pub async fn execute(self) -> SDKResult<DeleteRuleExternalResponse> {
        self.execute_with_options(RequestOption::default()).await
    }
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteRuleExternalResponse> {
        validate_required!(self.rule_id.trim(), "rule_id 不能为空");
        let req = ApiRequest::delete("/open-apis/acs/v1/rule_external")
            .query("rule_id", &self.rule_id)
            .with_supported_access_token_types(vec![AccessTokenType::App]);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteRuleExternalResponse { data: resp.data })
    }
}
```

> **关键**：无 `.json_body`、无 body 字段——当前代码发的 `{rule_id}` body 是错的，删除。

- [ ] **Step 4: 运行测试确认通过**

Run: `cargo test -p openlark-security rule_external::delete::tests 2>&1 | tail -5`
Expected: PASS

- [ ] **Step 5: 重写 `rule_external/get.rs`（query: device_id + user_id_type）**

```rust
// execute_with_options 内：
validate_required!(self.device_id.trim(), "device_id 不能为空");
let req = ApiRequest::get("/open-apis/acs/v1/rule_external")
    .query("device_id", &self.device_id)
    .query_opt("user_id_type", self.user_id_type.as_ref().map(|t| t.as_str()))
    .with_supported_access_token_types(vec![AccessTokenType::App]);
let resp = Transport::request(req, &self.config, Some(option)).await?;
Ok(GetRuleExternalResponse { data: resp.data })
```

结构体字段：`config`、`device_id`、`user_id_type: Option<String>`（加 builder setter）。

- [ ] **Step 6: 重写 `rule_external/create.rs`（query: rule_id+user_id_type + 包装 body）**

```rust
// execute_with_options 内：
validate_required!(self.rule_id.trim(), "rule_id 不能为空");
let rule_body = serde_json::to_value(&self.body)
    .map_err(|e| openlark_core::error::validation_error("创建权限组", format!("序列化失败: {e}")))?;
let wrapped = serde_json::json!({ "rule": rule_body });
let req = ApiRequest::post("/open-apis/acs/v1/rule_external")
    .query("rule_id", &self.rule_id)
    .query_opt("user_id_type", self.user_id_type.as_ref())
    .json_body(&wrapped)
    .with_supported_access_token_types(vec![AccessTokenType::App]);
let resp = Transport::request(req, &self.config, Some(option)).await?;
Ok(CreateRuleExternalResponse { data: resp.data })
```

> `12cc9fe09` 的 `{"rule": ...}` 包装保留（文档确认正确），但补上漏掉的 `rule_id`/`user_id_type` query 参数。

- [ ] **Step 7: 重写 `rule_external/device_bind.rs`（flat body，字段名修正）**

```rust
// body 结构（按文档）：
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBindBody {
    pub device_id: String,          // 单个，非数组
    pub rule_ids: Vec<String>,      // 数组
}
impl BindDeviceToRuleRequest {
    pub fn new(config: Arc<Config>) -> Self { ... }
    pub fn device_id(mut self, v: impl Into<String>) -> Self { self.body.device_id = v.into(); self }
    pub fn rule_ids(mut self, v: Vec<String>) -> Self { self.body.rule_ids = v; self }
    pub async fn execute_with_options(...) -> ... {
        validate_required!(self.body.device_id.trim(), "device_id 不能为空");
        validate_required_list!(self.body.rule_ids, 10000, "rule_ids 不能为空且不能超过 10000 个");
        let req = ApiRequest::post("/open-apis/acs/v1/rule_external/device_bind")
            .json_body(&self.body)
            .with_supported_access_token_types(vec![AccessTokenType::App]);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(BindDeviceToRuleResponse { data: resp.data })
    }
}
```

> **关键修复**：当前代码发 `{rule_id, device_ids[], overwrite}`，文档要求 `{device_id(单), rule_ids[]}`。删 `overwrite`（文档无此字段）。`rule_ids` 上限 10000 来自文档（背景核对时抓到 "Length range: 0 ～ 10000"）。

- [ ] **Step 8: 瘦化 `rule_external/mod.rs` 为门面**

删 `RuleExternalService` 的 4 个 `*Builder`、`get_app_token`、所有 raw reqwest、`models::acs::PermissionRuleRequest`/`DeviceBindRuleRequest` 的引用。替换为门面：

```rust
pub struct RuleExternalService { config: Arc<Config> }
impl RuleExternalService {
    pub fn new(config: Arc<Config>) -> Self { Self { config } }
    pub fn create(&self, rule_id: impl Into<String>) -> super::create::CreateRuleExternalRequest { ... }
    pub fn get(&self, device_id: impl Into<String>) -> super::get::GetRuleExternalRequest { ... }
    pub fn delete(&self, rule_id: impl Into<String>) -> super::delete::DeleteRuleExternalRequest { ... }
    pub fn device_bind(&self) -> super::device_bind::BindDeviceToRuleRequest { ... }
}
```

- [ ] **Step 9: 在 `AcsV1Service` 恢复 rule_external 字段**

- [ ] **Step 10: 编译 + 全部 rule_external 测试**

Run: `cargo test -p openlark-security rule_external:: 2>&1 | tail -10`
Expected: PASS（delete/get 空 ID 校验、device_bind 空 device_id/rule_ids 校验都过）

- [ ] **Step 11: Commit**

```bash
git add crates/openlark-security/src/acs/acs/v1/rule_external/ crates/openlark-security/src/acs/acs/mod.rs
git commit -m "fix(acs): rule_external 字段语义修复（delete 去 body，get 用 device_id，device_bind 改字段名）+ Transport 重写"
```

---

## Task 5: visitors + user_faces + access_records 资源组重写

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/visitor/{create,delete}.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/face/{create,delete,get}.rs`
- Modify: `crates/openlark-security/src/acs/acs/v1/access_record/{list.rs, access_photo/get.rs}`
- Modify: `crates/openlark-security/src/acs/acs/v1/{visitors,user_faces,access_records}/mod.rs`

**端点映射：**

| 文件 | 方法 | 路径 | 校验 |
|------|------|------|------|
| `visitor/create.rs` | POST | `/open-apis/acs/v1/visitors` | body |
| `visitor/delete.rs` | DELETE | `/open-apis/acs/v1/visitors/{visitor_id}` | `validate_required!(visitor_id)` |
| `face/create.rs` | POST | `/open-apis/acs/v1/faces` | body |
| `face/delete.rs` | DELETE | `/open-apis/acs/v1/faces/{face_id}` | `validate_required!(face_id)` |
| `face/get.rs` | GET | `/open-apis/acs/v1/faces/{face_id}` | `validate_required!(face_id)` |
| `access_record/list.rs` | GET | `/open-apis/acs/v1/access_records` | 无 |
| `access_record/access_photo/get.rs` | GET | `/open-apis/acs/v1/access_records/{access_record_id}/access_photo` | `validate_required!(access_record_id)` |

- [ ] **Step 1: 为每个路径参数端点写空 ID 校验测试**（visitor/delete、face/delete、face/get、access_photo/get）

每个测试模式同 Task 2 Step 1。例：

```rust
#[tokio::test]
async fn test_delete_visitor_rejects_empty_id() {
    let req = DeleteVisitorRequest::new(test_config(), "  ");
    let result = req.execute_with_options(Default::default()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("visitor_id"));
}
```

- [ ] **Step 2: 运行确认全部失败**

Run: `cargo test -p openlark-security visitor:: face:: access_record:: 2>&1 | tail -10`
Expected: FAIL

- [ ] **Step 3: 重写全部 7 个端点**

套 Task 2 Step 3 模式：正确路径、`validate_required!`（路径参数）、App token、返回 `resp.data`、删占位测试。

- [ ] **Step 4: 瘦化 3 个 `mod.rs` 为门面**（`visitors/mod.rs`、`user_faces/mod.rs`、`access_records/mod.rs`）

模式同前。`user_faces/mod.rs` 的 Service 方法指向 `face/` 下的端点（注意模块树：face 端点在 `v1/face/`，user_faces Service 是门面）。

- [ ] **Step 5: 在 `AcsV1Service` 恢复 visitors/user_faces/access_records 字段**

- [ ] **Step 6: 编译 + 测试**

Run: `cargo test -p openlark-security visitor:: face:: access_record:: 2>&1 | tail -10`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add crates/openlark-security/src/acs/acs/v1/visitor/ crates/openlark-security/src/acs/acs/v1/face/ crates/openlark-security/src/acs/acs/v1/access_record/ crates/openlark-security/src/acs/acs/v1/{visitors,user_faces,access_records}/mod.rs crates/openlark-security/src/acs/acs/mod.rs
git commit -m "refactor(acs): visitors/face/access_record 资源组重写为 Transport+validate"
```

---

## Task 6: rule 资源组重写（rule/get, list, create, delete）

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/rule/{get,list,create,delete}.rs`

> `rule` 子组（不同于 rule_external）当前没有独立 Service，端点文件直接被使用。核对路径后重写。

**端点映射：**

| 文件 | 方法 | 路径 | 校验 |
|------|------|------|------|
| `rule/get.rs` | GET | `/open-apis/acs/v1/rules/{rule_id}` | `validate_required!(rule_id)` |
| `rule/list.rs` | GET | `/open-apis/acs/v1/rules` | 无 |
| `rule/create.rs` | POST | `/open-apis/acs/v1/rules` | body |
| `rule/delete.rs` | DELETE | `/open-apis/acs/v1/rules/{rule_id}` | `validate_required!(rule_id)` |

> 注：`rule/get.rs` 当前有错误的 `.replace()` 链拼出 `/open-apis/security/acs/v1/rule/get`（非真实路径），必须删除换成真实路径。路径以飞书文档为准（实现时用 `openlark-api-field-verify` 技能核对 rule 子组的真实路径，与 rule_external 区分）。

- [ ] **Step 1: 核对 rule 子组真实路径**（用 field-verify 技能渲染 `acs-v1/rule/{get,list,create,delete}` 文档）

Run: `node .agents/skills/openlark-api-field-verify/scripts/fetch_doc.js --batch acs-v1/rule/get acs-v1/rule/list acs-v1/rule/create acs-v1/rule/delete --base "https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/" --out-dir /tmp/rule_check`

确认每个端点的真实路径和方法。若 `acs-v1/rule/*` 不存在（可能 rule 只有 rule_external），则 rule 子组可能是历史误建——记录发现，按实际文档调整。

- [ ] **Step 2: 写空 ID 校验测试（rule/get, rule/delete）**

- [ ] **Step 3: 重写 4 个 rule 端点**（模式同前，路径用 Step 1 核对结果）

- [ ] **Step 4: 编译 + 测试**

Run: `cargo test -p openlark-security rule:: 2>&1 | tail -10`（注意与 rule_external 区分测试过滤）
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/openlark-security/src/acs/acs/v1/rule/
git commit -m "refactor(acs): rule 资源组重写为 Transport+validate（路径按文档核对）"
```

---

## Task 7: openapi_audit 端点 + 收尾清理

**Files:**
- Modify: `crates/openlark-security/src/acs/acs/v1/openapi_audit/get.rs`
- Modify: `crates/openlark-security/src/acs/acs/mod.rs`（AcsV1Service 最终化）
- Modify: `crates/openlark-security/src/models/acs.rs`（删仅 Builder 用的请求结构）

- [ ] **Step 1: 核对并重写 `openapi_audit/get.rs`**

用 field-verify 核对路径（可能是 `/open-apis/acs/v1/openapi_audits/{audit_id}` 或类似）。重写：App token、`validate_required!`（若有路径 ID）、返回 `resp.data`。

- [ ] **Step 2: 删除 `models/acs.rs` 中仅被旧 Builder 使用的请求结构**

grep 确认 `PermissionRuleRequest`、`DeviceBindRuleRequest` 等是否还被引用：

Run: `rg "PermissionRuleRequest|DeviceBindRuleRequest" crates/openlark-security/src/`
若仅被已删除的 `rule_external/mod.rs` 引用，删除这些结构。保留响应模型（`UserInfo`、`PermissionRuleInfo` 等）若被 typed Response 使用；否则记录但不删（避免误删公共类型）。

- [ ] **Step 3: 确认 `AcsV1Service` 6 个字段全部恢复**（users/devices/visitors/user_faces/rule_external/access_records）

`acs/acs/mod.rs` 的 `AcsV1Service::new` 构造所有 6 个 Service。

- [ ] **Step 4: 全 crate 编译 + 全测试**

Run: `cargo test -p openlark-security 2>&1 | tail -15`
Expected: 全 PASS

- [ ] **Step 5: fmt + lint**

Run: `just fmt && just lint`
Expected: 无错误

- [ ] **Step 6: 确认无残留 raw reqwest / get_app_token**

Run: `rg "reqwest::Client::new|get_app_token" crates/openlark-security/src/acs/`
Expected: 无输出（全部清除）

- [ ] **Step 7: 确认 validate_apis.py 覆盖率不回归**

Run: `python3 tools/validate_apis.py --filter-tags acs 2>&1 | tail -10`
Expected: acs 覆盖率与重构前一致（31 个端点文件路径未变）

- [ ] **Step 8: Commit**

```bash
git add -A crates/openlark-security/
git commit -m "refactor(acs): openapi_audit 重写 + 清理 models 死代码 + 收尾"
```

---

## Task 8: 端到端验证 + GitHub issue

- [ ] **Step 1: 完整 check-all**

Run: `just check-all`
Expected: 全绿（fmt + lint + test + coverage + audit）

- [ ] **Step 2: 验证 client 调用链可用**

确认 `SecurityClient::new(SecurityConfig)` 签名不变，`client.security().acs().v1().users().get(id)` 链路类型正确（写一个 doc-test 或 unit test 构造 Config 走到 `.execute_with_options` 前的校验）。

- [ ] **Step 3: 开 GitHub issue 跟踪 security_and_compliance**

用 `gh` CLI 开 issue：标题 "security_and_compliance 子模块：迁移到 Transport+core Config"，描述同样的 raw reqwest 债务（9 处），引用本计划作为模板。

Run: `gh issue create --title "security_and_compliance 子模块：迁移到 Transport+core Config" --body "..."`
Expected: issue 创建成功，记录编号到 spec 文档。

- [ ] **Step 4: 最终 commit（若有 spec 更新）**

```bash
git add docs/superpowers/specs/2026-06-20-acs-transport-migration-design.md
git commit -m "docs: 补充 acs 迁移 issue 链接"
```

---

## 验收标准（全部满足才算完成）

- [ ] acs 31 个端点全部走 `Transport::request`，无 `reqwest::Client::new()`（grep 验证）
- [ ] acs 无 `*Builder` 类型，无 `get_app_token` 辅助
- [ ] 所有路径参数端点恢复 `validate_required!`
- [ ] 所有端点设 `.with_supported_access_token_types(vec![AccessTokenType::App])`
- [ ] 24 个 DROPS-RESP 端点改为返回真实 `resp.data`
- [ ] `user/face/get` 恢复 `FaceData` typed 字段
- [ ] rule_external 的 create/delete/get/device_bind 字段语义与文档一致
- [ ] `SecurityClient::new(SecurityConfig)` 签名不变，调用链可用
- [ ] `just check-all` 全绿
- [ ] security_and_compliance 的 GitHub issue 已开
