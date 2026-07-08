//! 重置用户密码 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/password/reset>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 重置用户密码的请求构建器。
pub struct ResetPasswordRequestBuilder {
    user_id: String,
    new_password: String,
    config: Config,
}

impl ResetPasswordRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            user_id: String::new(),
            new_password: String::new(),
            config,
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置新密码。
    pub fn new_password(mut self, new_password: impl Into<String>) -> Self {
        self.new_password = new_password.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ResetPasswordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ResetPasswordResponse> {
        validate_required!(self.user_id, "用户ID不能为空");
        validate_required!(self.new_password, "新密码不能为空");

        let request_body = ResetPasswordRequest {
            user_id: self.user_id,
            new_password: self.new_password,
        };

        let api_request: ApiRequest<ResetPasswordResponse> =
            ApiRequest::post("/open-apis/admin/v1/password/reset")
                .body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("重置用户密码", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct ResetPasswordRequest {
    user_id: String,
    new_password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 重置用户密码的响应。
pub struct ResetPasswordResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for ResetPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ResetPasswordRequestBuilder, will be removed in v1.0 (#271)")]
pub type ResetPasswordBuilder = ResetPasswordRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../admin/v1/password/reset → 强类型 ResetPasswordResponse。
    #[tokio::test]
    async fn test_reset_password_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/admin/v1/password/reset"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "result": "success" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ResetPasswordRequestBuilder::new(config)
            .user_id("u_001")
            .new_password("NewP@ssw0rd!")
            .execute()
            .await
            .expect("重置用户密码应成功");
        assert_eq!(resp.result, "success");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/admin/v1/password/reset");
    }
}
