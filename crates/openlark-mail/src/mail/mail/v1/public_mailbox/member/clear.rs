//! 删除公共邮箱所有成员

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 删除公共邮箱所有成员的请求。
#[derive(Debug, Clone)]
pub struct ClearPublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

/// 删除公共邮箱所有成员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearPublicMailboxMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ClearPublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ClearPublicMailboxMemberRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    /// 执行删除公共邮箱所有成员请求。
    pub async fn execute(self) -> SDKResult<ClearPublicMailboxMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ClearPublicMailboxMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/clear",
            self.public_mailbox_id
        );
        let req: ApiRequest<ClearPublicMailboxMemberResponse> = ApiRequest::post(&path);

        Transport::request_typed(req, &self.config, Some(option), "清空公共邮箱成员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../public_mailboxes/{id}/members/clear → ClearPublicMailboxMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_clear_public_mailbox_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/public_mailboxes/mb_001/members/clear",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "success": true } }
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

        let resp = ClearPublicMailboxMemberRequest::new(config, "mb_001")
            .execute()
            .await
            .expect("清空公共邮箱成员应成功");
        assert_eq!(resp.data.unwrap()["success"].as_bool(), Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/public_mailboxes/mb_001/members/clear"
        );
    }
}
