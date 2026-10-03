# OpenLark API 实现模板与检查点

> **目标**: 为新增 API 和整改现有 API 提供可直接复制、可执行的代码模板  
> **范围**: 覆盖 Request 结构体、Builder/Setter、validate_required!、execute/execute_with_options、RequestOption 透传、mod.rs 导出、最小测试矩阵  
> **状态**: 本模板为 issue #41 验收标准第 3 项产出

---

## 1. 快速导航

| 章节 | 内容 | 适用场景 |
|------|------|---------|
| §2 | 完整 API 文件模板 | 新增 API 时直接复制 |
| §3 | 两种端点风格对比 | 选择 enum 或常量风格 |
| §4 | RequestOption 透传规范 | 确保 user_access_token 支持 |
| §5 | mod.rs 导出模板 | 模块导出一致性 |
| §6 | 最小测试矩阵 | 测试覆盖要求 |
| §7 | 检查点清单 | PR 前自检 |

---

## 2. 完整 API 文件模板

### 2.1 推荐模板（enum 端点 + typed response）

**适用**: `openlark-docs`, `openlark-auth` 等使用 enum 端点的 crate

**文件路径**: `crates/{crate}/src/{bizTag}/{project}/{version}/{resource}/{name}.rs`

```rust
//! {API 中文名称}
//!
//! {API 功能一句话描述}。
//! docPath: {飞书开放平台文档路径，如 /document/server-docs/im-v1/message/create}

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::{
    api_endpoints::{ProjectApiV1},  // 根据实际 crate 调整
    api_utils::serialize_params,
};

// ============================================================================
// 请求体结构体
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Body {
    /// {字段中文描述}
    pub {field_name}: {field_type},
    /// {字段中文描述}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub {optional_field}: Option<{field_type}>,
}

// ============================================================================
// 响应体结构体
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Response {
    /// {字段中文描述}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub {field_name}: Option<{field_type}>,
}

impl ApiResponseTrait for {Name}Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data  // 或 ResponseFormat::Raw / ResponseFormat::Items
    }
}

// ============================================================================
// Request Builder
// ============================================================================

pub struct {Name}Request {
    config: Arc<Config>,
    // 路径参数（URL 中直接替换的，如 /open-apis/im/v1/messages/:message_id）
    path_param: String,
    // 查询参数（可选，用于 GET 请求）
    query_param: Option<String>,
}

impl {Name}Request {
    /// 创建新的请求构建器
    ///
    /// # 参数
    /// - `config`: SDK 配置（通过 `Config::builder()` 创建）
    /// - `path_param`: {路径参数描述}
    pub fn new(config: Arc<Config>, path_param: impl Into<String>) -> Self {
        Self {
            config,
            path_param: path_param.into(),
            query_param: None,
        }
    }

    /// 设置查询参数（示例：用于 GET 请求的可选过滤条件）
    pub fn query_param(mut self, value: impl Into<String>) -> Self {
        self.query_param = Some(value.into());
        self
    }

    // ------------------------------------------------------------------------
    // 执行方法
    // ------------------------------------------------------------------------

    /// 执行 API 请求（使用默认 RequestOption）
    ///
    /// # 参数
    /// - `body`: 请求体（对于 GET/DELETE 等无 body 的请求，可省略此参数）
    ///
    /// # 返回
    /// - `SDKResult<{Name}Response>`: 成功时返回响应体，失败时返回 CoreError
    pub async fn execute(self, body: {Name}Body) -> SDKResult<{Name}Response> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 执行 API 请求（支持自定义 RequestOption）
    ///
    /// **必须实现** - 用于支持 user_access_token、tenant_key、自定义 header
    ///
    /// # 参数
    /// - `body`: 请求体
    /// - `option`: 请求选项（包含 user_access_token、tenant_key、request_id 等）
    ///
    /// # 使用场景
    /// - 用户态 API 需要传递 `user_access_token`
    /// - 商店应用需要传递 `tenant_key` / `app_ticket`
    /// - 链路追踪需要传递 `request_id` / 自定义 header
    pub async fn execute_with_options(
        self,
        body: {Name}Body,
        option: RequestOption,
    ) -> SDKResult<{Name}Response> {
        // 1. 必填字段校验
        validate_required!(self.path_param.trim(), "{path_param_name} 不能为空");
        validate_required!(body.{required_field}, "{required_field_name} 不能为空");

        // 2. 构建端点（使用 enum 风格）
        let api_endpoint = ProjectApiV1::{EndpointVariant}(self.path_param);

        // 3. 构建请求
        let mut api_request: ApiRequest<{Name}Response> = 
            ApiRequest::post(&api_endpoint.to_url())  // 或 get/delete/patch
                .body(serialize_params(&body, "{API 中文名称}")?);

        // 4. 添加查询参数（如果有）
        if let Some(query) = self.query_param {
            api_request = api_request.query("{query_key}", query);
        }

        // 5. 发送请求并抽取 typed 响应（关键：必须透传 option；统一走 request_typed）
        Transport::request_typed(api_request, &self.config, Some(option), "{API 中文名称}").await
    }
}

// ============================================================================
// 便捷函数（可选，用于函数式调用场景）
// ============================================================================

/// 便捷函数：直接执行 API
///
/// **注意**: 此函数不支持自定义 RequestOption，仅用于简单场景。
/// 如需传递 user_access_token 等，请使用 `{Name}Request`。
pub async fn {name_snake_case}(
    config: &Config,
    path_param: impl Into<String>,
    body: {Name}Body,
) -> SDKResult<{Name}Response> {
    let request = {Name}Request::new(Arc::new(config.clone()), path_param);
    request.execute(body).await
}
```

### 2.2 替代模板（端点常量风格）

**适用**: `openlark-communication` 等使用常量端点的 crate

```rust
//! {API 中文名称}
//!
//! docPath: {飞书开放平台文档路径}

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    common::api_utils::serialize_params,
    endpoints::{IM_V1_MESSAGES},  // 常量端点
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Body {
    pub {field_name}: {field_type},
    #[serde(skip_serializing_if = "Option::is_none")]
    pub {optional_field}: Option<{field_type}>,
}

pub struct {Name}Request {
    config: Arc<Config>,
    path_param: String,
    query_param: Option<String>,
}

impl {Name}Request {
    pub fn new(config: Arc<Config>, path_param: impl Into<String>) -> Self {
        Self {
            config,
            path_param: path_param.into(),
            query_param: None,
        }
    }

    pub fn query_param(mut self, value: impl Into<String>) -> Self {
        self.query_param = Some(value.into());
        self
    }

    pub async fn execute(self, body: {Name}Body) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        body: {Name}Body,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.path_param.trim(), "{path_param_name} 不能为空");
        validate_required!(body.{required_field}, "{required_field_name} 不能为空");

        // 使用常量端点
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_V1_MESSAGES)
            .body(serialize_params(&body, "{API 中文名称}")?);

        if let Some(query) = self.query_param {
            req = req.query("{query_key}", query);
        }

        // 关键：透传 option；统一走 request_typed
        Transport::request_typed(req, &self.config, Some(option), "{API 中文名称}").await
    }
}
```

---

## 3. 端点定义风格对比

### 3.1 enum 风格（推荐用于复杂 API 集合）

**文件**: `crates/{crate}/src/common/api_endpoints.rs`

```rust
/// 项目 API V1 端点定义
pub enum ProjectApiV1 {
    /// GET /open-apis/project/v1/{resource_id}
    ResourceGet(String),
    /// POST /open-apis/project/v1/{resource_id}/actions
    ResourceAction(String),
    /// DELETE /open-apis/project/v1/{resource_id}
    ResourceDelete(String),
}

impl ProjectApiV1 {
    /// 生成完整 URL 路径
    pub fn to_url(&self) -> String {
        match self {
            Self::ResourceGet(id) => format!("/open-apis/project/v1/{}", id),
            Self::ResourceAction(id) => format!("/open-apis/project/v1/{}/actions", id),
            Self::ResourceDelete(id) => format!("/open-apis/project/v1/{}", id),
        }
    }
}
```

**优点**:
- 编译时检查端点类型
- 支持路径参数类型安全传递
- 易于扩展新端点

### 3.2 常量风格（推荐用于简单 API 集合）

**文件**: `crates/{crate}/src/endpoints.rs`

```rust
/// IM 消息 API V1 端点
pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";
pub const IM_V1_MESSAGES_DELETE: &str = "/open-apis/im/v1/messages/{message_id}";

/// 带路径参数的端点生成函数
pub fn im_v1_message_delete(message_id: &str) -> String {
    format!("/open-apis/im/v1/messages/{}", message_id)
}
```

**优点**:
- 简单直观
- 零运行时开销
- 适合端点数量较少的模块

### 3.3 选择指南

| 场景 | 推荐风格 | 示例 crate |
|------|---------|-----------|
| API 数量 > 20，路径参数复杂 | enum | `openlark-docs` |
| API 数量 < 20，路径简单 | 常量 | `openlark-communication` |
| 需要编译时类型检查 | enum | `openlark-auth` |
| 追求最小复杂度 | 常量 | `openlark-webhook` |

---

## 4. RequestOption 透传规范

### 4.1 为什么必须支持 RequestOption

**使用场景**:
1. **用户态 API**: 需要传递 `user_access_token`
2. **商店应用**: 需要传递 `tenant_key` / `app_ticket`
3. **链路追踪**: 需要传递 `request_id` / 自定义 header

### 4.2 正确实现示例

```rust
pub async fn execute_with_options(
    self,
    body: CreateMessageBody,
    option: RequestOption,  // 接收 RequestOption
) -> SDKResult<CreateMessageResponse> {
    // ... 校验代码 ...

    let req: ApiRequest<CreateMessageResponse> = ApiRequest::post(IM_V1_MESSAGES)
        .body(serialize_params(&body, "发送消息")?);

    // ✅ 正确：透传 option 到 Transport；统一走 request_typed
    Transport::request_typed(req, &self.config, Some(option), "发送消息").await
}
```

### 4.3 常见错误

```rust
// ❌ 错误 1：完全不支持 RequestOption
pub async fn execute(self, body: Body) -> SDKResult<Response> {
    let resp = Transport::request(req, &self.config, None).await?;  // 硬编码 None
    // ...
}

// ❌ 错误 2：只使用 ApiRequest::request_option（不完整）
pub async fn execute(self, body: Body) -> SDKResult<Response> {
    let req = ApiRequest::post(endpoint)
        .body(body)
        .request_option(option);  // 仅合并 header，token 推断需要走 Transport
    let resp = Transport::request(req, &self.config, None).await?;  // 仍然传 None
    // ...
}
```

### 4.4 RequestOption 创建方式

```rust
use openlark_core::req_option::RequestOption;

// 方式 1：默认（无特殊选项）
let option = RequestOption::default();

// 方式 2：指定 user_access_token
let option = RequestOption::builder()
    .user_access_token("u-xxx".to_string())
    .build();

// 方式 3：指定 tenant_key（商店应用）
let option = RequestOption::builder()
    .tenant_key("tenant-xxx".to_string())
    .build();

// 方式 4：自定义 header
let option = RequestOption::builder()
    .header("X-Request-Id".to_string(), "req-123".to_string())
    .build();
```

---

## 5. mod.rs 导出模板

### 5.1 标准 mod.rs 结构

**文件**: `crates/{crate}/src/{bizTag}/{project}/{version}/{resource}/mod.rs`

```rust
//! {Resource} API 模块
//!
//! 包含以下 API：
//! - create: 创建{资源}
//! - get: 获取{资源}
//! - update: 更新{资源}
//! - delete: 删除{资源}
//! - list: 列出{资源}

// ============================================================================
// 子模块导出
// ============================================================================

pub mod create;
pub mod get;
pub mod update;
pub mod delete;
pub mod list;

// ============================================================================
// 模型导出（如果模型在 models.rs）
// ============================================================================

pub mod models;

// ============================================================================
// 显式 re-export（推荐）
// ============================================================================

pub use create::{CreateRequest, CreateBody, CreateResponse};
pub use get::{GetRequest, GetResponse};
pub use update::{UpdateRequest, UpdateBody, UpdateResponse};
pub use delete::{DeleteRequest, DeleteResponse};
pub use list::{ListRequest, ListResponse, ListItem};
pub use models::*;

// ============================================================================
// Service 层链式调用（给 openlark-client 使用）
// ============================================================================

use std::sync::Arc;
use openlark_core::config::Config;

/// {Resource}Service 提供链式调用入口
#[derive(Debug, Clone)]
pub struct {Resource}Service {
    config: Arc<Config>,
}

impl {Resource}Service {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建{资源}
    pub fn create(&self, path_param: impl Into<String>) -> CreateRequest {
        CreateRequest::new(self.config.clone(), path_param)
    }

    /// 获取{资源}
    pub fn get(&self, path_param: impl Into<String>) -> GetRequest {
        GetRequest::new(self.config.clone(), path_param)
    }

    /// 更新{资源}
    pub fn update(&self, path_param: impl Into<String>) -> UpdateRequest {
        UpdateRequest::new(self.config.clone(), path_param)
    }

    /// 删除{资源}
    pub fn delete(&self, path_param: impl Into<String>) -> DeleteRequest {
        DeleteRequest::new(self.config.clone(), path_param)
    }

    /// 列出{资源}
    pub fn list(&self) -> ListRequest {
        ListRequest::new(self.config.clone())
    }
}
```

### 5.2 父级 mod.rs 聚合导出

**文件**: `crates/{crate}/src/{bizTag}/{project}/{version}/mod.rs`

```rust
//! {Project} V{Version} API 模块

pub mod {resource1};
pub mod {resource2};

// 聚合导出
pub use {resource1}::{Resource1Service};
pub use {resource2}::{Resource2Service};
```

---

## 6. 最小测试矩阵

### 6.1 单元测试模板

**文件**: `crates/{crate}/tests/{name}_tests.rs` 或内联测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    // =========================================================================
    // 构造测试
    // =========================================================================

    #[test]
    fn test_request_builder() {
        let config = Arc::new(Config::builder().app_id("test").app_secret("test").build());
        let request = CreateRequest::new(config, "path-param")
            .query_param("filter");

        // 验证构造成功
        assert_eq!(request.path_param, "path-param");
        assert_eq!(request.query_param, Some("filter".to_string()));
    }

    // =========================================================================
    // 校验测试
    // =========================================================================

    #[test]
    fn test_validation_required_fields() {
        let config = Arc::new(Config::builder().app_id("test").app_secret("test").build());
        let request = CreateRequest::new(config, "");  // 空路径参数

        let body = CreateBody {
            field: "".to_string(),  // 空必填字段
        };

        // 在异步上下文中测试校验
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(request.execute(body));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("不能为空"));
    }

    // =========================================================================
    // 序列化测试
    // =========================================================================

    #[test]
    fn test_body_serialization() {
        let body = CreateBody {
            field: "value".to_string(),
            optional_field: Some("optional".to_string()),
        };

        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("value"));
        assert!(json.contains("optional"));
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{"field": "value"}"#;
        let response: CreateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.field, Some("value".to_string()));
    }

    // =========================================================================
    // RequestOption 测试（可选，用于验证透传）
    // =========================================================================

    #[test]
    fn test_request_option_builder() {
        let option = RequestOption::builder()
            .user_access_token("token-123".to_string())
            .tenant_key("tenant-456".to_string())
            .build();

        // 验证 option 构造成功
        assert!(option.user_access_token().is_some());
        assert_eq!(option.user_access_token().unwrap(), "token-123");
    }
}
```

### 6.2 集成测试模板

**文件**: `tests/integration_{crate}_{api}.rs`

```rust
//! {API 名称} 集成测试
//!
//! 运行前需要设置环境变量：
//! - OPENLARK_APP_ID
//! - OPENLARK_APP_SECRET
//! - OPENLARK_USER_ACCESS_TOKEN（可选，用于测试用户态 API）

use openlark_core::config::Config;
use openlark_{crate}::{ModuleClient, api_name};

#[tokio::test]
async fn test_api_success() {
    // 从环境变量加载配置
    let config = Config::from_env().expect("Failed to load config from env");
    let client = ModuleClient::new(config);

    // 执行 API 调用
    let result = client
        .chain()
        .api()
        .execute(body)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_api_with_user_access_token() {
    let config = Config::from_env().unwrap();
    let client = ModuleClient::new(config);

    // 创建带 user_access_token 的 RequestOption
    let option = RequestOption::builder()
        .user_access_token(std::env::var("OPENLARK_USER_ACCESS_TOKEN").unwrap())
        .build();

    let result = client
        .chain()
        .api()
        .execute_with_options(body, option)
        .await;

    assert!(result.is_ok());
}
```

### 6.3 测试检查点

| 测试类型 | 必须 | 检查项 |
|---------|------|-------|
| 构造测试 | ✅ | Request 能正确构建，setter 链式调用有效 |
| 校验测试 | ✅ | `validate_required!` 对空字符串/空白字符串生效 |
| 序列化测试 | ✅ | Body 能正确序列化为 JSON，字段名符合预期 |
| 反序列化测试 | ✅ | Response 能正确从 JSON 反序列化，可选字段处理正确 |
| 端点测试 | 推荐 | 生成的 URL 路径正确 |
| 集成测试 | 推荐 | 真实 API 调用成功（需要有效凭证） |

---

## 7. 检查点清单（PR 前自检）

### 7.1 代码实现检查点

- [ ] **落盘路径正确**: `crates/{crate}/src/{bizTag}/{project}/{version}/{resource}/{name}.rs`
- [ ] **文档注释**: 文件顶部包含 `//! docPath: {url}`
- [ ] **导入顺序**: 标准库 → 第三方库 → crate 内部模块
- [ ] **命名规范**: 
  - Request 结构体: `{Name}Request`
  - Body 结构体: `{Name}Body`
  - Response 结构体: `{Name}Response`
  - 方法名: `snake_case`
- [ ] **必填校验**: 所有必填字段使用 `validate_required!` 宏
- [ ] **空白处理**: 字符串校验前调用 `.trim()`
- [ ] **端点使用**: 使用常量或 enum，禁止硬编码 URL
- [ ] **execute 方法**: 提供 `execute()` 和 `execute_with_options()` 两个方法
- [ ] **RequestOption 透传**: `Transport::request(..., Some(option))`
- [ ] **错误处理**: 统一走 `Transport::request_typed(.., "中文名")` 抽取 typed 响应

### 7.2 导出检查点

- [ ] **mod.rs 导出**: 同级 `mod.rs` 包含 `pub mod {name};`
- [ ] **显式 re-export**: 使用 `pub use` 导出公开类型
- [ ] **Service 层**: 提供 `*Service` 结构体用于链式调用
- [ ] **父级聚合**: 父级 `mod.rs` 正确聚合导出子模块

### 7.3 质量检查点

- [ ] **格式化**: `just fmt` 通过
- [ ] **Lint**: `just lint` 零警告
- [ ] **测试**: `just test` 通过
- [ ] **文档**: `cargo doc` 生成无错误
- [ ] **Clippy**: `cargo clippy --all-targets --all-features` 零警告

### 7.4 兼容性检查点

- [ ] **向后兼容**: 不破坏现有公开 API
- [ ] **Feature gating**: 使用 `#[cfg(feature = "...")]` 控制编译
- [ ] **MSRV**: 使用 Rust 1.88+ 兼容语法

---

## 8. 快速参考卡片

### 8.1 常用导入

```rust
// 核心导入
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 工具导入
use crate::common::{
    api_endpoints::{ProjectApiV1},
    api_utils::serialize_params,
};
```

### 8.2 常用宏

```rust
// 必填字段校验
validate_required!(field, "字段描述不能为空");
validate_required!(field.trim(), "字段描述不能为空（自动 trim）");

// 列表字段校验（如果有）
validate_required_list!(list, "列表不能为空");
```

### 8.3 HTTP 方法映射

| API 方法 | ApiRequest 方法 | 示例 |
|---------|----------------|------|
| GET | `ApiRequest::get(url)` | `ApiRequest::get(&endpoint.to_url())` |
| POST | `ApiRequest::post(url)` | `ApiRequest::post(url).body(body)` |
| PUT | `ApiRequest::put(url)` | `ApiRequest::put(url).body(body)` |
| PATCH | `ApiRequest::patch(url)` | `ApiRequest::patch(url).body(body)` |
| DELETE | `ApiRequest::delete(url)` | `ApiRequest::delete(url)` |

### 8.4 ResponseFormat 选择

| 场景 | ResponseFormat |
|------|---------------|
| 标准 data 包装响应 | `ResponseFormat::Data` |
| 原始响应（无包装） | `ResponseFormat::Raw` |
| items 数组响应 | `ResponseFormat::Items` |

---

## 9. 相关资源

- **整改清单**: `.worktrees/issue-41-plan-a/docs/issue-41-backlog.md`
- **标准示例**: `.agents/skills/openlark-api/references/standard-example.md`
- **项目规范**: `AGENTS.md`
- **父 issue**: #41

---

## 10. 变更日志

| 日期 | 版本 | 变更 |
|------|------|------|
| 2025-04-08 | v1.0 | 初始版本，覆盖完整 API 实现模板 |
