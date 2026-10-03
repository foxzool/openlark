# 标准示例（以仓库现有风格为准）

下面给出两种“仓库里真实存在”的风格：
- A) 使用端点常量（典型：`openlark-communication`）
- B) 使用 enum 端点（典型：`openlark-docs` / `openlark-auth`）

实现新 API 时优先模仿目标 crate 的现有文件风格。

## A) 端点常量 + Builder + execute（推荐默认）

参考现有文件：`crates/openlark-communication/src/im/im/v1/message/create.rs`

```rust
//! 示例：发送消息（精简结构示例）
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageBody {
    pub receive_id: String,
    pub msg_type: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

pub struct CreateMessageRequest {
    config: Config,
    receive_id_type: Option<String>, // 示例：真实代码里通常是 enum
}

impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self { config, receive_id_type: None }
    }

    pub fn receive_id_type(mut self, v: impl Into<String>) -> Self {
        self.receive_id_type = Some(v.into());
        self
    }

    /// 执行请求（转调 execute_with_options，传 RequestOption::default()）
    pub async fn execute(self, body: CreateMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行（option 透传到 Transport::request 的 Some(option)）
    pub async fn execute_with_options(
        self,
        body: CreateMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(body.receive_id, "receive_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            openlark_core::error::validation_error("receive_id_type 不能为空", "需要指定接收者 ID 类型")
        })?;

        // url: POST:/open-apis/im/v1/messages
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_V1_MESSAGES)
            .query("receive_id_type", receive_id_type)
            .body(serialize_params(&body, "发送消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "发送消息")
    }
}
```

补充约定（新增/重构时统一）：
- **每个 Request 必须提供 `execute_with_options(..., RequestOption)` 并把 option 透传到 `Transport::request(..., Some(option))`**；`execute` 只是无 option 版本的转调（`execute_with_options(.., RequestOption::default())`）。见核心契约 5。
- 不要只调用 `ApiRequest::request_option(...)`：当前实现仅合并 header，token 推断/注入需要走 `Transport` 的 `option` 参数

## B) enum 端点 + typed response + ApiResponseTrait

参考现有文件：`crates/openlark-docs/src/base/base/v2/app/role/create.rs`

```rust
//! 示例：新增自定义角色（精简结构示例）
//!
//! docPath: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::common::api_endpoints::BaseApiV2;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
    pub role_name: String,
    pub table_roles: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<serde_json::Value>,
}

impl ApiResponseTrait for CreateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub struct Create {
    config: Config,
    app_token: String,
    req: CreateReq,
}

impl Create {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CreateReq { role_name: String::new(), table_roles: vec![] },
        }
    }

    pub fn app_token(mut self, v: impl Into<String>) -> Self { self.app_token = v.into(); self }
    pub fn role_name(mut self, v: impl Into<String>) -> Self { self.req.role_name = v.into(); self }
    pub fn table_roles(mut self, v: Vec<serde_json::Value>) -> Self { self.req.table_roles = v; self }

    /// 使用默认请求选项执行（转调 execute_with_options）
    pub async fn execute(self) -> SDKResult<CreateResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行（option 透传到 Transport::request 的 Some(option)）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");

        let api_endpoint = BaseApiV2::RoleCreate(self.app_token);
        let api_request: ApiRequest<CreateResp> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.req, "新增自定义角色")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新增自定义角色")
    }
}
```

补充约定（新增/重构时统一）：
- 与 A 范式一致：每个 Request 必须提供 `execute_with_options(..., RequestOption)` 并透传 `Some(option)`；`execute()` 只是无 option 版本的转调。

## 最小导出约定（mod.rs）

在同级 `mod.rs` 中导出模块（模仿目标 crate 现有结构）：

```rust
pub mod create;
pub mod get;
pub mod models;
```

## DocsClient / 链式入口（openlark-docs 真实形态）

> ⚠️ **历史纠错**：旧版文档曾给出 `client.docs.service().base().v2().app().role().create()`
> 这样的深层链式调用——**这些 `service()`/`base()`/`v2()` 方法与 `DocsService`/`BaseService`
> 类型在仓库中均不存在**。核实 `crates/openlark-docs/src/common/chain.rs:609-651`：`DocsClient`
> 只暴露 `ccm`/`base`/`baike`/`minutes` 公开字段（按 feature 裁剪）和 `config()` 方法，
> 没有 `service()` 方法，也没有逐层 `.base().v2().app().role()` 的 Service 链。

`openlark-docs` 的真实调用形态是**两段式**：

1. 用 `DocsClient::new(config)` 拿到 client，再取 `docs.base.config().clone()`（或对应子 client 的 `.config()`）得到 `Config`；
2. 用户自己 `CreateRole::new(config)...execute()` 直接构造 Builder 调用，**不存在统一的深层 Service 链**。

```rust
use openlark_docs::DocsClient;
use openlark_docs::base::base::v2::app::role::create::Create;

// 1) 拿到 Config（DocsClient 内部 Arc<Config>，clone 廉价）
let docs = DocsClient::new(config);
let config = docs.base.config().clone();   // crates/openlark-docs/src/common/chain.rs:1080-1091

// 2) 用户直接构造 Builder 调用 API
let resp = Create::new(config)
    .app_token("bascn...")
    .role_name("角色名")
    .table_roles(vec![...])
    .execute()
    .await?;
```

### 对新 API 的要求

`openlark-docs` crate 新增 API 时：
- 在对应 `mod.rs` 导出 Builder（如 `Create`）即可，**不需要**补任何 `service.rs` / 深层 Service 链；
- Builder 必须实现 `execute` + `execute_with_options`（见上方 A/B 范式），不依赖外层 Service。

其他 crate（非 docs）若已有 `src/service.rs` 链式入口，按该 crate 现有结构补链路即可；event/webhook 类 P2 模块不进统一 `service.rs` 链路。
