//! 获取邮件详情
//! docPath: <https://open.feishu.cn/document/mail-v1/user_mailbox-message/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Get Mailbox Message Request。
#[derive(Debug, Clone)]
pub struct GetMailboxMessageRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    message_id: String,
}

/// Get Mailbox Message Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailboxMessageResponse {
    /// 响应数据。
    pub data: Option<MessageData>,
}

impl ApiResponseTrait for GetMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Message Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    /// 消息 ID。
    pub message_id: String,
    /// subject 字段。
    pub subject: String,
    /// body 字段。
    pub body: String,
}

impl GetMailboxMessageRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailboxMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailboxMessageResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}",
            self.user_mailbox_id, self.message_id
        );
        let req: ApiRequest<GetMailboxMessageResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取邮件详情", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../user_mailboxes/{umb}/messages/{msg} → 强类型 GetMailboxMessageResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_mailbox_message_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/messages/msg_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "message_id": "msg_001", "subject": "测试主题", "body": "测试正文" } }
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

        let resp = GetMailboxMessageRequest::new(config, "umb_001", "msg_001")
            .execute()
            .await
            .expect("获取邮件详情应成功");
        let data = resp.data.expect("响应 data 应非空");
        assert_eq!(data.message_id, "msg_001");
        assert_eq!(data.subject, "测试主题");
        assert_eq!(data.body, "测试正文");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/messages/msg_001"
        );
    }
}
