//! 创建用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 创建用户请求
///
/// 通过 body 传入用户信息。本计划范围只做 Transport 迁移，body 用 `serde_json::Value`
/// 透传（字段细化见 spec §9，不在本次范围）。
#[derive(Debug)]
pub struct CreateUserRequest {
    /// 配置信息。
    config: Config,
    /// 请求体（必填，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl CreateUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let body = self
            .body
            .ok_or_else(|| validation_error("创建用户", "body 不能为空"))?;

        let req: ApiRequest<serde_json::Value> = ApiRequest::post("/open-apis/acs/v1/users")
            .body(body)
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("创建用户", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_create_user_rejects_empty_body() {
        // body 未设置必须在发请求前校验失败。
        let req = CreateUserRequest::new(test_config());
        let result = req.execute_with_options(RequestOption::default()).await;
        assert!(result.is_err(), "空 body 必须校验失败");
        assert!(result.unwrap_err().to_string().contains("body"));
    }
}
