//! 查询邮箱地址状态
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/user/query>

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询邮箱地址状态的请求。
#[derive(Debug, Clone)]
pub struct QueryMailUserRequest {
    config: Arc<Config>,
    body: QueryMailUserBody,
}

/// 查询邮箱地址状态的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryMailUserBody {
    /// 需要查询的邮箱地址列表。
    pub email_list: Vec<String>,
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
        Self {
            config,
            body: QueryMailUserBody::default(),
        }
    }

    /// 设置需要查询的邮箱地址列表。
    pub fn email_list(mut self, email_list: Vec<String>) -> Self {
        self.body.email_list = email_list;
        self
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
        validate_required!(self.body.email_list, "email_list 不能为空");
        let path = "/open-apis/mail/v1/users/query".to_string();
        let req: ApiRequest<QueryMailUserResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "查询邮箱地址状态")?);

        Transport::request_typed(req, &self.config, Some(option), "查询邮箱地址状态").await
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
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mail/v1/users/query"))
            .and(body_json(json!({ "email_list": ["test@a.com"] })))
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
            .email_list(vec!["test@a.com".to_string()])
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

    /// 失败路径：email_list 为空时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_query_mail_user_rejects_empty_email_list() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = QueryMailUserRequest::new(config).execute().await;
        assert!(result.is_err());
    }
}
