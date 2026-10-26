//! 订阅邮箱事件

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Subscribe Mailbox Event Request。
#[derive(Debug, Clone)]
pub struct SubscribeMailboxEventRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    body: SubscribeMailboxEventBody,
}

/// 订阅邮箱事件的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeMailboxEventBody {
    /// 事件类型（当前仅支持 1：邮件相关事件）。
    pub event_type: i32,
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
            body: SubscribeMailboxEventBody::default(),
        }
    }

    /// 设置事件类型（必填，当前仅支持 1：邮件相关事件）。
    pub fn event_type(mut self, event_type: i32) -> Self {
        self.body.event_type = event_type;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SubscribeMailboxEventResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SubscribeMailboxEventResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        // event_type 必填且取值范围 1～1（数值字段无 Validatable 实现，按 logout 范式手动校验）
        if self.body.event_type != 1 {
            return Err(error::validation_error(
                "event_type",
                "event_type 不能为空，当前仅支持 1（邮件相关事件）",
            ));
        }
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/event/subscribe",
            self.user_mailbox_id
        );
        let req: ApiRequest<SubscribeMailboxEventResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "订阅邮箱事件")?);

        Transport::request_typed(req, &self.config, Some(option), "订阅邮箱事件").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_mailboxes/{id}/event/subscribe → SubscribeMailboxEventResponse 解析，请求体对齐官方示例。
    #[tokio::test]
    async fn test_subscribe_mailbox_event_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/mb_001/event/subscribe",
            ))
            .and(body_json(json!({ "event_type": 1 })))
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
            .event_type(1)
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

    /// 失败路径：未设置 event_type 时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_subscribe_mailbox_event_rejects_missing_event_type() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = SubscribeMailboxEventRequest::new(config, "mb_001")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
