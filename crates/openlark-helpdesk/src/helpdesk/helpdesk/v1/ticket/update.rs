//! 更新工单

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

    /// status。
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.body.status = Some(status.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpdateTicketResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
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

        let response = openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
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

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
