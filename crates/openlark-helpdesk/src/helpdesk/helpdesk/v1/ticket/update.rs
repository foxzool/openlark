//! 更新工单
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket/update>

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
use crate::helpdesk::helpdesk::v1::ticket::models::{UpdateTicketBody, UpdateTicketResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新工单请求。
#[derive(Debug, Clone)]
pub struct UpdateTicketRequest {
    config: Arc<Config>,
    ticket_id: String,
    body: UpdateTicketBody,
}

impl UpdateTicketRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self {
            config,
            ticket_id,
            body: UpdateTicketBody::default(),
        }
    }

    /// 设置标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = Some(title.into());
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// status（0=未处理，1=处理中，2=已解决）。
    pub fn status(mut self, status: i32) -> Self {
        self.body.status = Some(status);
        self
    }

    /// 设置标签名列表。
    pub fn tag_names(mut self, tag_names: Vec<String>) -> Self {
        self.body.tag_names = Some(tag_names);
        self
    }

    /// 设置评论。
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置自定义字段（JSON）。
    pub fn customized_fields(mut self, customized_fields: serde_json::Value) -> Self {
        self.body.customized_fields = Some(customized_fields);
        self
    }

    /// 设置工单类型。
    pub fn ticket_type(mut self, ticket_type: i32) -> Self {
        self.body.ticket_type = Some(ticket_type);
        self
    }

    /// 设置是否解决（0=未解决，1=已解决）。
    pub fn solved(mut self, solved: i32) -> Self {
        self.body.solved = Some(solved);
        self
    }

    /// 设置渠道。
    pub fn channel(mut self, channel: i32) -> Self {
        self.body.channel = Some(channel);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpdateTicketResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTicketResponse> {
        validate_required!(self.ticket_id.trim(), "工单ID不能为空");

        let api_endpoint = HelpdeskApiV1::TicketUpdate(self.ticket_id.clone());
        let mut request = ApiRequest::<UpdateTicketResponse>::put(api_endpoint.to_url());

        request = request.body(serialize_params(&self.body, "更新工单")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新工单")
    }
}

impl ApiResponseTrait for UpdateTicketResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../tickets/{id} → 强类型 UpdateTicketResponse 解析（扁平响应，单层 data 信封）。
    #[tokio::test]
    async fn test_update_ticket_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/helpdesk/v1/tickets/tk_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ticket_id": "tk_001",
                    "updated_at": "2024-01-02T00:00:00Z"
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

        let resp = UpdateTicketRequest::new(config, "tk_001".to_string())
            .title("更新后的标题")
            .execute()
            .await
            .expect("更新工单应成功");
        assert_eq!(resp.ticket_id, "tk_001");
        assert_eq!(resp.updated_at, "2024-01-02T00:00:00Z");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/tickets/tk_001"
        );
    }
}
