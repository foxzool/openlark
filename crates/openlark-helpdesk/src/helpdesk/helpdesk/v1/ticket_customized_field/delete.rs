//! 删除指定工单自定义字段
//!
//! 删除指定的工单自定义字段。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 删除工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTicketCustomizedFieldResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteTicketCustomizedFieldResult>,
}

impl openlark_core::api::ApiResponseTrait for DeleteTicketCustomizedFieldResponse {}

/// 删除工单自定义字段结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTicketCustomizedFieldResult {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 删除工单自定义字段请求
#[derive(Debug, Clone)]
pub struct DeleteTicketCustomizedFieldRequest {
    config: Arc<Config>,
    id: String,
}

impl DeleteTicketCustomizedFieldRequest {
    /// 创建新的删除工单自定义字段请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行删除工单自定义字段请求
    pub async fn execute(self) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行删除工单自定义字段请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
        let req: ApiRequest<DeleteTicketCustomizedFieldResponse> = ApiRequest::delete(
            HelpdeskApiV1::TicketCustomizedFieldDelete(self.id.clone()).to_url(),
        );

        Transport::request_typed(req, &self.config, Some(option), "删除工单自定义字段").await
    }
}

/// 删除工单自定义字段请求构建器
#[derive(Debug, Clone)]
pub struct DeleteTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl DeleteTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
        let request = DeleteTicketCustomizedFieldRequest::new(self.config.clone(), self.id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
        let request = DeleteTicketCustomizedFieldRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行删除工单自定义字段
pub async fn delete_ticket_customized_field(
    config: &Config,
    id: String,
) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
    delete_ticket_customized_field_with_options(config, id, RequestOption::default()).await
}

/// 执行删除工单自定义字段（支持自定义选项）
pub async fn delete_ticket_customized_field_with_options(
    config: &Config,
    id: String,
    option: RequestOption,
) -> SDKResult<DeleteTicketCustomizedFieldResponse> {
    let req: ApiRequest<DeleteTicketCustomizedFieldResponse> =
        ApiRequest::delete(HelpdeskApiV1::TicketCustomizedFieldDelete(id).to_url());

    Transport::request_typed(req, config, Some(option), "删除工单自定义字段").await
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
        let builder = DeleteTicketCustomizedFieldRequestBuilder::new(
            Arc::new(config),
            "field_123".to_string(),
        );

        assert_eq!(builder.id, "field_123");
    }

    /// 端到端：DELETE .../ticket_customized_fields/{id} → 强类型 DeleteTicketCustomizedFieldResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "success": true } }
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

        let resp = DeleteTicketCustomizedFieldRequest::new(config, "tcf_001".to_string())
            .execute()
            .await
            .expect("删除工单自定义字段应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/ticket_customized_fields/tcf_001"
        );
    }
}
