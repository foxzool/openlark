//! 获取工单自定义字段列表
//!
//! 获取工单自定义字段列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取工单自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldResponse {
    /// 工单自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TicketCustomizedFieldItem>>,
}

impl ApiResponseTrait for ListTicketCustomizedFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工单自定义字段项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCustomizedFieldItem {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// 获取工单自定义字段列表请求
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequest {
    config: Arc<Config>,
}

impl ListTicketCustomizedFieldRequest {
    /// 创建新的获取工单自定义字段列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取工单自定义字段列表请求
    pub async fn execute(self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
        let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工单自定义字段列表",
        )
        .await
    }
}

/// 获取工单自定义字段列表请求构建器
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
}

impl ListTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
        let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, None, "获取工单自定义字段列表").await
    }
}

/// 执行获取工单自定义字段列表
pub async fn list_ticket_customized_fields(
    config: &Config,
) -> SDKResult<ListTicketCustomizedFieldResponse> {
    let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
    let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

    Transport::request_typed(request, config, None, "获取工单自定义字段列表").await
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
        let _builder = ListTicketCustomizedFieldRequestBuilder::new(Arc::new(config));
    }

    /// 端到端：GET .../ticket_customized_fields → 强类型 ListTicketCustomizedFieldResponse 解析（外层 data 信封 + items）。
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/ticket_customized_fields"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": { "items": [ { "id": "tcf_001", "name": "工单编号", "field_type": "text", "required": true } ] }
                })),
            )
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

        let resp = ListTicketCustomizedFieldRequest::new(config)
            .execute()
            .await
            .expect("获取工单自定义字段列表应成功");
        assert!(resp.items.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields"
        );
    }
}
