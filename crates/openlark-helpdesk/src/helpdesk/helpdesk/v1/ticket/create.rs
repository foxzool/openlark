//! 创建工单

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
}
