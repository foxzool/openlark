//! 批量获取用户迁移状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v1/user_migration/search>
//!
//! 过滤条件在 body，调用方按飞书文档自行构造 JSON 透传。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 批量获取用户迁移状态请求。
///
/// 通过 [`Self::body`] 传入过滤条件（如迁移任务 ID 列表、分页等）。
#[derive(Debug)]
pub struct SearchUserMigrationsRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body（过滤 + 分页条件）。
    body: serde_json::Value,
}

impl SearchUserMigrationsRequest {
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
            ApiRequest::post("/open-apis/security_and_compliance/v1/user_migrations/search")
                .body(self.body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("批量获取用户迁移状态", "响应数据为空"))
    }
}
