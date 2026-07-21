//! 获取工单详情
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/get>

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::helpdesk::helpdesk::v1::ticket::models::GetTicketResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 获取工单详情请求。
#[derive(Debug, Clone)]
pub struct GetTicketRequest {
    config: Arc<Config>,
    ticket_id: String,
}

impl GetTicketRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self { config, ticket_id }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetTicketResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTicketResponse> {
        validate_required!(self.ticket_id.trim(), "工单ID不能为空");

        let api_endpoint = HelpdeskApiV1::TicketGet(self.ticket_id.clone());
        let request = ApiRequest::<GetTicketResponse>::get(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工单",
        )
        .await
    }
}

impl ApiResponseTrait for GetTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../tickets/{id} → 强类型 GetTicketResponse 解析（扁平响应，单层 data 信封）。
    #[tokio::test]
    async fn test_get_ticket_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/tickets/tk_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ticket_id": "tk_001",
                    "title": "无法登录",
                    "status": "open",
                    "created_at": "2024-01-01T00:00:00Z"
                }
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

        let resp = GetTicketRequest::new(config, "tk_001".to_string())
            .execute()
            .await
            .expect("获取工单详情应成功");
        assert_eq!(resp.ticket_id, "tk_001");
        assert_eq!(resp.status, "open");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/tickets/tk_001"
        );
    }
}
