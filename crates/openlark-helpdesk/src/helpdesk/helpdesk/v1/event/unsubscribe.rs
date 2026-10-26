//! 取消订阅服务台事件
//!
//! 本接口用于取消订阅服务台事件。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/event/unsubscribe>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 取消订阅服务台事件请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventUnsubscribeBody {
    /// 要取消订阅的事件列表
    pub events: Vec<EventUnsubscribeEvent>,
}

/// 要取消订阅的服务台事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventUnsubscribeEvent {
    /// 事件类型
    pub r#type: String,
    /// 事件子类型
    pub subtype: String,
}

impl EventUnsubscribeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        openlark_core::validate_required!(self.events, "events 不能为空");
        for event in &self.events {
            openlark_core::validate_required!(event.r#type, "type 不能为空");
            openlark_core::validate_required!(event.subtype, "subtype 不能为空");
        }
        Ok(())
    }
}

/// 取消订阅服务台事件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventUnsubscribeResponse {}

impl ApiResponseTrait for EventUnsubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消订阅服务台事件请求
#[derive(Debug, Clone)]
pub struct EventUnsubscribeRequest {
    config: Arc<Config>,
}

impl EventUnsubscribeRequest {
    /// 创建新的取消订阅服务台事件请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行取消订阅服务台事件请求
    pub async fn execute(self, body: EventUnsubscribeBody) -> SDKResult<EventUnsubscribeResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        body: EventUnsubscribeBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventUnsubscribeResponse> {
        body.validate()?;

        let request =
            ApiRequest::<EventUnsubscribeResponse>::post(HelpdeskApiV1::EventUnsubscribe.to_url())
                .body(serialize_params(&body, "取消订阅服务台事件")?);

        Transport::request_typed(request, &self.config, Some(option), "取消订阅服务台事件").await
    }
}

/// 取消订阅服务台事件请求构建器
#[derive(Debug, Clone)]
pub struct EventUnsubscribeRequestBuilder {
    config: Arc<Config>,
    body: EventUnsubscribeBody,
}

impl EventUnsubscribeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: EventUnsubscribeBody { events: Vec::new() },
        }
    }

    /// 设置要取消订阅的事件列表
    pub fn events(mut self, events: Vec<EventUnsubscribeEvent>) -> Self {
        self.body.events = events;
        self
    }

    /// 追加单个要取消订阅的事件
    pub fn event(mut self, event: EventUnsubscribeEvent) -> Self {
        self.body.events.push(event);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> EventUnsubscribeBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<EventUnsubscribeResponse> {
        EventUnsubscribeRequest::new(self.config.clone())
            .execute(self.body())
            .await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventUnsubscribeResponse> {
        EventUnsubscribeRequest::new(self.config.clone())
            .execute_with_options(self.body(), option)
            .await
    }
}

/// 执行取消订阅服务台事件
pub async fn unsubscribe_event(
    config: &Config,
    body: EventUnsubscribeBody,
) -> SDKResult<EventUnsubscribeResponse> {
    unsubscribe_event_with_options(
        config,
        body,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

/// 执行取消订阅服务台事件（支持自定义选项）
pub async fn unsubscribe_event_with_options(
    config: &Config,
    body: EventUnsubscribeBody,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<EventUnsubscribeResponse> {
    body.validate()?;

    let request =
        ApiRequest::<EventUnsubscribeResponse>::post(HelpdeskApiV1::EventUnsubscribe.to_url())
            .body(serialize_params(&body, "取消订阅服务台事件")?);

    Transport::request_typed(request, config, Some(option), "取消订阅服务台事件").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_rejects_empty_events() {
        let body = EventUnsubscribeBody { events: Vec::new() };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_rejects_blank_subtype() {
        let body = EventUnsubscribeBody {
            events: vec![EventUnsubscribeEvent {
                r#type: "helpdesk.ticket_message".to_string(),
                subtype: "".to_string(),
            }],
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serializes_official_shape() {
        let body = EventUnsubscribeBody {
            events: vec![EventUnsubscribeEvent {
                r#type: "helpdesk.ticket_message".to_string(),
                subtype: "ticket_message.created_v1".to_string(),
            }],
        };
        let value = serde_json::to_value(&body).expect("请求体应可序列化");

        assert_eq!(value["events"][0]["type"], "helpdesk.ticket_message");
        assert_eq!(value["events"][0]["subtype"], "ticket_message.created_v1");
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = EventUnsubscribeRequestBuilder::new(Arc::new(config));

        assert!(builder.config.app_id() == "test_app_id");
        assert!(builder.body().events.is_empty());
    }

    /// 端到端：POST .../events/unsubscribe（body: events 数组）→ 空 body 响应成功返回。
    #[tokio::test]
    async fn test_unsubscribe_event_returns_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/events/unsubscribe"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
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

        EventUnsubscribeRequest::new(config)
            .execute(EventUnsubscribeBody {
                events: vec![EventUnsubscribeEvent {
                    r#type: "helpdesk.ticket_message".to_string(),
                    subtype: "ticket_message.created_v1".to_string(),
                }],
            })
            .await
            .expect("取消订阅服务台事件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/events/unsubscribe"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["events"][0]["type"], "helpdesk.ticket_message");
        assert_eq!(
            request_body["events"][0]["subtype"],
            "ticket_message.created_v1"
        );
    }
}
