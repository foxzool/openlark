//! 创建用户迁移
//!
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/create
//!
//! 请求 body 字段较多（源/目标用户、范围等），调用方按飞书文档自行构造 JSON 透传。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 创建用户迁移请求。
///
/// 通过 [`Self::body`] 传入完整请求体（按飞书文档构造）。
#[derive(Debug)]
pub struct CreateUserMigrationRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body。
    body: serde_json::Value,
}

impl CreateUserMigrationRequest {
    /// 创建新的请求构建器（空 body）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: serde_json::json!({}),
        }
    }

    /// 设置请求 body（覆盖已有内容）。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = body;
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations")
                .body(self.body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("创建用户迁移", "响应数据为空"))
    }
}
