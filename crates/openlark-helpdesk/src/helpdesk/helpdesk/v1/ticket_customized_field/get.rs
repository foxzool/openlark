//! 获取指定工单自定义字段
//!
//! 获取指定工单自定义字段的详情。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/get>

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

/// 获取指定工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketCustomizedFieldResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TicketCustomizedFieldItem>,
}

impl ApiResponseTrait for GetTicketCustomizedFieldResponse {
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

/// 获取指定工单自定义字段请求
#[derive(Debug, Clone)]
pub struct GetTicketCustomizedFieldRequest {
    config: Arc<Config>,
    id: String,
}

impl GetTicketCustomizedFieldRequest {
    /// 创建新的获取指定工单自定义字段请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行获取指定工单自定义字段请求
    pub async fn execute(self) -> SDKResult<GetTicketCustomizedFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldGet(self.id.clone());
        let request = ApiRequest::<GetTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取指定工单自定义字段")
    }
}

/// 获取指定工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct GetTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl GetTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldGet(self.id.clone());
        let request = ApiRequest::<GetTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取指定工单自定义字段")
    }
}

/// 执行获取指定工单自定义字段
pub async fn get_ticket_customized_field(
    config: &Config,
    id: String,
) -> SDKResult<GetTicketCustomizedFieldResponse> {
    let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldGet(id);
    let request = ApiRequest::<GetTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取指定工单自定义字段")
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
        let builder =
            GetTicketCustomizedFieldRequestBuilder::new(Arc::new(config), "field_123".to_string());

        assert_eq!(builder.id, "field_123");
    }

    /// 端到端：GET .../ticket_customized_fields/{id} → 强类型 GetTicketCustomizedFieldResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": { "data": { "id": "tcf_001", "name": "工单编号", "field_type": "text", "required": true } }
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

        let resp = GetTicketCustomizedFieldRequest::new(config, "tcf_001".to_string())
            .execute()
            .await
            .expect("获取指定工单自定义字段应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001"
        );
    }
}
