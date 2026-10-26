//! 订阅服务台事件
//!
//! 本接口用于订阅服务台事件。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/event/subscribe>

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

/// 订阅服务台事件请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscribeBody {
    /// 可订阅的事件列表
    pub events: Vec<EventSubscribeEvent>,
}

/// 可订阅的服务台事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscribeEvent {
    /// 事件类型
    pub r#type: String,
    /// 事件子类型
    pub subtype: String,
}

impl EventSubscribeBody {
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

/// 订阅服务台事件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscribeResponse {}

impl ApiResponseTrait for EventSubscribeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 订阅服务台事件请求
#[derive(Debug, Clone)]
pub struct EventSubscribeRequest {
    config: Arc<Config>,
}

impl EventSubscribeRequest {
    /// 创建新的订阅服务台事件请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行订阅服务台事件请求
    pub async fn execute(self, body: EventSubscribeBody) -> SDKResult<EventSubscribeResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        body: EventSubscribeBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventSubscribeResponse> {
        body.validate()?;

        let request =
            ApiRequest::<EventSubscribeResponse>::post(HelpdeskApiV1::EventSubscribe.to_url())
                .body(serialize_params(&body, "订阅服务台事件")?);

        Transport::request_typed(request, &self.config, Some(option), "订阅服务台事件").await
    }
}

/// 订阅服务台事件请求构建器
#[derive(Debug, Clone)]
pub struct EventSubscribeRequestBuilder {
    config: Arc<Config>,
    body: EventSubscribeBody,
}

impl EventSubscribeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: EventSubscribeBody { events: Vec::new() },
        }
    }

    /// 设置可订阅的事件列表
    pub fn events(mut self, events: Vec<EventSubscribeEvent>) -> Self {
        self.body.events = events;
        self
    }

    /// 追加单个可订阅事件
    pub fn event(mut self, event: EventSubscribeEvent) -> Self {
        self.body.events.push(event);
        self
    }

    /// 构建请求体
    pub fn body(&self) -> EventSubscribeBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<EventSubscribeResponse> {
        EventSubscribeRequest::new(self.config.clone())
            .execute(self.body())
            .await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventSubscribeResponse> {
        EventSubscribeRequest::new(self.config.clone())
            .execute_with_options(self.body(), option)
            .await
    }
}

/// 执行订阅服务台事件
pub async fn subscribe_event(
    config: &Config,
    body: EventSubscribeBody,
) -> SDKResult<EventSubscribeResponse> {
    subscribe_event_with_options(
        config,
        body,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

/// 执行订阅服务台事件（支持自定义选项）
pub async fn subscribe_event_with_options(
    config: &Config,
    body: EventSubscribeBody,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<EventSubscribeResponse> {
    body.validate()?;

    let request =
        ApiRequest::<EventSubscribeResponse>::post(HelpdeskApiV1::EventSubscribe.to_url())
            .body(serialize_params(&body, "订阅服务台事件")?);

    Transport::request_typed(request, config, Some(option), "订阅服务台事件").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_rejects_empty_events() {
        let body = EventSubscribeBody { events: Vec::new() };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_rejects_blank_type() {
        let body = EventSubscribeBody {
            events: vec![EventSubscribeEvent {
                r#type: "  ".to_string(),
                subtype: "ticket_message.created_v1".to_string(),
            }],
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serializes_official_shape() {
        let body = EventSubscribeBody {
            events: vec![EventSubscribeEvent {
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
        let builder = EventSubscribeRequestBuilder::new(Arc::new(config));

        assert!(builder.config.app_id() == "test_app_id");
        assert!(builder.body().events.is_empty());
    }

    /// 端到端：POST .../events/subscribe（body: events 数组）→ 空 body 响应成功返回。
    #[tokio::test]
    async fn test_subscribe_event_returns_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/events/subscribe"))
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

        EventSubscribeRequest::new(config)
            .execute(EventSubscribeBody {
                events: vec![EventSubscribeEvent {
                    r#type: "helpdesk.ticket_message".to_string(),
                    subtype: "ticket_message.created_v1".to_string(),
                }],
            })
            .await
            .expect("订阅服务台事件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/events/subscribe"
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
