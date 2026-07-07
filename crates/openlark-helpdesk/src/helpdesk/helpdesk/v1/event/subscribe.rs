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
use crate::common::api_utils::extract_response_data;

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
    pub async fn execute(self) -> SDKResult<EventSubscribeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventSubscribeResponse> {
        let request =
            ApiRequest::<EventSubscribeResponse>::post(HelpdeskApiV1::EventSubscribe.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "订阅服务台事件")
    }
}

/// 订阅服务台事件请求构建器
#[derive(Debug, Clone)]
pub struct EventSubscribeRequestBuilder {
    config: Arc<Config>,
}

impl EventSubscribeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<EventSubscribeResponse> {
        let request =
            ApiRequest::<EventSubscribeResponse>::post(HelpdeskApiV1::EventSubscribe.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "订阅服务台事件")
    }
}

/// 执行订阅服务台事件
pub async fn subscribe_event(config: &Config) -> SDKResult<EventSubscribeResponse> {
    let request =
        ApiRequest::<EventSubscribeResponse>::post(HelpdeskApiV1::EventSubscribe.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "订阅服务台事件")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = EventSubscribeRequestBuilder::new(Arc::new(config));

        assert!(builder.config.app_id() == "test_app_id");
    }

    /// 端到端：POST .../events/subscribe → 空 body 响应经 extract_response_data 成功返回。
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
            .execute()
            .await
            .expect("订阅服务台事件应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/events/subscribe"
        );
    }
}
