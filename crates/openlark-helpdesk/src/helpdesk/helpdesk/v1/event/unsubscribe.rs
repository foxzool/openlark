//! 取消订阅服务台事件
//!
//! 本接口用于取消订阅服务台事件。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/event/unsubscribe

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
    pub async fn execute(self) -> SDKResult<EventUnsubscribeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EventUnsubscribeResponse> {
        let request =
            ApiRequest::<EventUnsubscribeResponse>::post(HelpdeskApiV1::EventUnsubscribe.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "取消订阅服务台事件")
    }
}

/// 取消订阅服务台事件请求构建器
#[derive(Debug, Clone)]
pub struct EventUnsubscribeRequestBuilder {
    config: Arc<Config>,
}

impl EventUnsubscribeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<EventUnsubscribeResponse> {
        let request =
            ApiRequest::<EventUnsubscribeResponse>::post(HelpdeskApiV1::EventUnsubscribe.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "取消订阅服务台事件")
    }
}

/// 执行取消订阅服务台事件
pub async fn unsubscribe_event(config: &Config) -> SDKResult<EventUnsubscribeResponse> {
    let request =
        ApiRequest::<EventUnsubscribeResponse>::post(HelpdeskApiV1::EventUnsubscribe.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "取消订阅服务台事件")
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
        let builder = EventUnsubscribeRequestBuilder::new(Arc::new(config));

        assert!(builder.config.app_id() == "test_app_id");
    }
}
