//! 删除用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/acs-v1/user/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType, http::Transport,
    req_option::RequestOption, validate_required,
};

/// 删除用户请求
#[derive(Debug)]
pub struct DeleteUserRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID（路径参数，必填）。
    user_id: String,
}

impl DeleteUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, user_id: impl Into<String>) -> Self {
        Self {
            config,
            user_id: user_id.into(),
        }
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_id, "user_id 不能为空");

        let path = format!("/open-apis/acs/v1/users/{}", self.user_id);
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(&path).with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.into_result()
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
    async fn test_delete_user_rejects_empty_id() {
        let req = DeleteUserRequest::new(test_config(), "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("user_id"));
    }

    /// 端到端：DELETE .../users/{id} + 响应解析。
    #[tokio::test]
    async fn test_delete_user_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/acs/v1/users/u_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_id": "u_001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = DeleteUserRequest::new(config, "u_001")
            .execute()
            .await
            .expect("删除用户应成功");
        assert_eq!(data["user_id"], "u_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/acs/v1/users/u_001");
    }
}
