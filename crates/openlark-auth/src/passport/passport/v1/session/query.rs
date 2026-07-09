//! 批量获取脱敏的用户登录信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/authentication-management/login-state-management/query>

use crate::common::api_endpoints::PassportApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量获取脱敏用户登录信息请求体
#[derive(Debug, Serialize)]
struct QuerySessionRequestBody {
    user_ids: Vec<String>,
}

/// 批量获取脱敏用户登录信息请求
pub struct QuerySessionRequest {
    config: Config,
    user_ids: Vec<String>,
}

impl QuerySessionRequest {
    /// 创建批量查询会话请求实例
    ///
    /// # 参数
    /// - `config`: SDK 配置信息
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
        }
    }

    /// 设置用户ID列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 执行批量查询会话请求
    pub async fn execute(self) -> SDKResult<QuerySessionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行批量查询会话请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QuerySessionResponse> {
        let body = QuerySessionRequestBody {
            user_ids: self.user_ids,
        };

        let req: ApiRequest<QuerySessionResponse> =
            ApiRequest::post(PassportApiV1::SessionQuery.path()).body(serde_json::to_value(&body)?);

        let response = Transport::request(req, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("query_session", "响应数据为空"))
    }
}

/// 批量获取脱敏用户登录信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySessionResponse {
    /// 会话信息列表
    pub items: Vec<SessionInfo>,
}

impl ApiResponseTrait for QuerySessionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// 用户ID
    pub user_id: String,
    /// 会话状态
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/passport/v1/sessions/query
    #[tokio::test]
    async fn test_query_session_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/passport/v1/sessions/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "items": [] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        QuerySessionRequest::new(config)
            .user_ids(vec!["test001".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
