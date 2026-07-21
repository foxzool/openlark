//! 获取工单列表
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/list>

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::helpdesk::helpdesk::v1::ticket::models::TicketListResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use std::sync::Arc;

/// 工单列表请求。
#[derive(Debug, Clone)]
pub struct TicketListRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl TicketListRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页游标。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TicketListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TicketListResponse> {
        let api_endpoint = HelpdeskApiV1::TicketList;
        let mut request = ApiRequest::<TicketListResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取工单列表",
        )
        .await
    }
}

impl ApiResponseTrait for TicketListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let arc_config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let _config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = TicketListRequest::new(arc_config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }

    /// 端到端：GET .../tickets → 强类型 TicketListResponse 解析（扁平响应，单层 data 信封）。
    #[tokio::test]
    async fn test_list_tickets_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/tickets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "tickets": [
                        { "ticket_id": "tk_001", "title": "无法登录", "status": "open" }
                    ],
                    "page_token": "next_page"
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

        let resp = TicketListRequest::new(config)
            .execute()
            .await
            .expect("获取工单列表应成功");
        assert_eq!(resp.tickets.len(), 1);
        assert_eq!(resp.tickets[0].ticket_id, "tk_001");
        assert_eq!(resp.page_token.as_deref(), Some("next_page"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/tickets");
    }
}
