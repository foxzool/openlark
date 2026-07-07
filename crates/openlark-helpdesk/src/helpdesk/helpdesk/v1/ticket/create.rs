//! 创建工单
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/events/created>

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use crate::helpdesk::helpdesk::v1::ticket::models::{CreateTicketBody, CreateTicketResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建工单请求。
#[derive(Debug, Clone)]
pub struct CreateTicketRequest {
    config: Arc<Config>,
    body: CreateTicketBody,
}

impl CreateTicketRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTicketBody::default(),
        }
    }

    /// 设置标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置优先级。
    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.body.priority = Some(priority.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateTicketResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTicketResponse> {
        validate_required!(self.body.title.trim(), "工单标题不能为空");

        let api_endpoint = HelpdeskApiV1::TicketCreate;
        let mut request = ApiRequest::<CreateTicketResponse>::post(api_endpoint.to_url());

        request = request.body(serialize_params(&self.body, "创建工单")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建工单")
    }
}

impl ApiResponseTrait for CreateTicketResponse {
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
        let request = CreateTicketRequest::new(arc_config.clone())
            .title("test".to_string())
            .description("test".to_string());
        let _ = request;
    }

    /// 端到端：POST .../tickets → 强类型 CreateTicketResponse 解析（扁平响应，单层 data 信封）。
    #[tokio::test]
    async fn test_create_ticket_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/tickets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ticket_id": "tk_001",
                    "title": "无法登录",
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

        let resp = CreateTicketRequest::new(config)
            .title("无法登录")
            .execute()
            .await
            .expect("创建工单应成功");
        assert_eq!(resp.ticket_id, "tk_001");
        assert_eq!(resp.title, "无法登录");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/tickets");
    }
}
