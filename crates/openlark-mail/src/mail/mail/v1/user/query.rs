//! 查询邮箱地址状态
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/user/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询邮箱地址状态的请求。
#[derive(Debug, Clone)]
pub struct QueryMailUserRequest {
    config: Arc<Config>,
}

/// 查询邮箱地址状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMailUserResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryMailUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryMailUserRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行查询邮箱地址状态请求。
    pub async fn execute(self) -> SDKResult<QueryMailUserResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QueryMailUserResponse> {
        let path = "/open-apis/mail/v1/users/query".to_string();
        let req: ApiRequest<QueryMailUserResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询邮箱地址状态", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../mail/v1/users/query → QueryMailUserResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_query_mail_user_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mail/v1/users/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "email": "user@example.com" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = QueryMailUserRequest::new(config)
            .execute()
            .await
            .expect("查询邮箱地址状态应成功");
        assert_eq!(
            resp.data.unwrap()["email"].as_str(),
            Some("user@example.com")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/mail/v1/users/query");
    }
}
