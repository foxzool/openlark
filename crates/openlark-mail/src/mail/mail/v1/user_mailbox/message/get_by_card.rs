//! 获取邮件卡片的邮件列表

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

/// Get Mailbox Message By Card Request。
#[derive(Debug, Clone)]
pub struct GetMailboxMessageByCardRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    /// 邮件卡片 ID（必填）。
    card_id: String,
    /// 邮件卡片 Owner ID（必填，与 user_id_type 无关）。
    owner_id: String,
    /// 用户 ID 类型（可选）：open_id / union_id / user_id，默认 open_id。
    user_id_type: Option<String>,
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
            card_id: String::new(),
            owner_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置邮件卡片 ID（必填，可通过接收消息事件的推送获取）。
    pub fn card_id(mut self, card_id: impl Into<String>) -> Self {
        self.card_id = card_id.into();
        self
    }

    /// 设置邮件卡片 Owner ID（必填，可通过接收消息事件的推送获取）。
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.owner_id = owner_id.into();
        self
    }

    /// 设置用户 ID 类型（可选）：open_id / union_id / user_id，默认 open_id。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
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
        validate_required!(self.card_id, "card_id 不能为空");
        validate_required!(self.owner_id, "owner_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/get_by_card",
            self.user_mailbox_id
        );
        let req: ApiRequest<GetMailboxMessageByCardResponse> = ApiRequest::get(&path)
            .query("card_id", self.card_id)
            .query("owner_id", self.owner_id)
            .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "获取邮件卡片的邮件列表").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../user_mailboxes/{umb}/messages/get_by_card?card_id=...&owner_id=... → 强类型 GetMailboxMessageByCardResponse 解析（双层 data 信封）。
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
            .card_id("512ca581-6059-4449-8150-5522e6641d32")
            .owner_id("1234567890")
            .user_id_type("open_id")
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
        // 官方必填查询参数 card_id / owner_id（+ 可选 user_id_type）
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("card_id=512ca581-6059-4449-8150-5522e6641d32"));
        assert!(query.contains("owner_id=1234567890"));
        assert!(query.contains("user_id_type=open_id"));
    }

    /// 缺必填查询参数应快速失败，不发请求。
    #[tokio::test]
    async fn test_get_by_card_rejects_missing_required_query() {
        let config = Arc::new(Config::default());
        let err = GetMailboxMessageByCardRequest::new(config, "umb_001")
            .card_id("512ca581-6059-4449-8150-5522e6641d32")
            .execute()
            .await
            .expect_err("缺少 owner_id 应报错");
        assert!(err.to_string().contains("owner_id"));
    }
}
