//! 订阅邮箱事件

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Subscribe Mailbox Event Request。
#[derive(Debug, Clone)]
pub struct SubscribeMailboxEventRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

/// Subscribe Mailbox Event Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMailboxEventResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SubscribeMailboxEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SubscribeMailboxEventRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SubscribeMailboxEventResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SubscribeMailboxEventResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/event/subscribe",
            self.user_mailbox_id
        );
        let req: ApiRequest<SubscribeMailboxEventResponse> = ApiRequest::post(&path);

        Transport::request_typed(req, &self.config, Some(option), "订阅邮箱事件").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_mailboxes/{id}/event/subscribe → SubscribeMailboxEventResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_subscribe_mailbox_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/mb_001/event/subscribe",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "subscribed": true } }
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

        let resp = SubscribeMailboxEventRequest::new(config, "mb_001")
            .execute()
            .await
            .expect("订阅邮箱事件应成功");
        assert_eq!(resp.data.unwrap()["subscribed"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/mb_001/event/subscribe"
        );
    }
}
