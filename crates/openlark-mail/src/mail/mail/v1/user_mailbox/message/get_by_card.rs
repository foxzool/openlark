//! 获取邮件卡片的邮件列表

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Get Mailbox Message By Card Request。
#[derive(Debug, Clone)]
pub struct GetMailboxMessageByCardRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

/// Get Mailbox Message By Card Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailboxMessageByCardResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMailboxMessageByCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMailboxMessageByCardRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailboxMessageByCardResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailboxMessageByCardResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/get_by_card",
            self.user_mailbox_id
        );
        let req: ApiRequest<GetMailboxMessageByCardResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件卡片的邮件列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../user_mailboxes/{umb}/messages/get_by_card → 强类型 GetMailboxMessageByCardResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_by_card_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/messages/get_by_card",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "message_id": "msg_001" } }
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

        let resp = GetMailboxMessageByCardRequest::new(config, "umb_001")
            .execute()
            .await
            .expect("获取邮件卡片的邮件列表应成功");
        assert_eq!(resp.data.unwrap()["message_id"], "msg_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/messages/get_by_card"
        );
    }
}
