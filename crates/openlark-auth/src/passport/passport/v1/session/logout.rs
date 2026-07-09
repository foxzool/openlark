//! 退出登录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/logout>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 退出登录请求
pub struct LogoutRequest {
    config: Config,
    user_id: Option<String>,
}

impl LogoutRequest {
    /// 创建退出登录请求实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: None,
        }
    }

    /// 设置用户ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行退出登录请求
    pub async fn execute(self) -> SDKResult<LogoutResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行退出登录请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<LogoutResponse> {
        let req: ApiRequest<LogoutResponse> = ApiRequest::post(PassportApiV1::SessionLogout.path());

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("logout", "响应数据为空"))
    }
}

/// 退出登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 退出结果
    pub result: String,
}

impl ApiResponseTrait for LogoutResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/passport/v1/sessions/logout
    #[tokio::test]
    async fn test_logout_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/passport/v1/sessions/logout"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "result": "success" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        LogoutRequest::new(config)
            .user_id("test001")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
