//! 删除指定工单自定义字段
//!
//! 删除指定的工单自定义字段。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/delete>

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 删除工单自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTicketCustomizedFieldResponse {
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
        let req: ApiRequest<DeleteTicketCustomizedFieldResponse> =
            ApiRequest::delete(HelpdeskApiV1::TicketCustomizedFieldDelete(self.id.clone()).to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除工单自定义字段")
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

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "删除工单自定义字段")
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
        let builder = DeleteTicketCustomizedFieldRequestBuilder::new(Arc::new(config), "field_123".to_string());

        assert_eq!(builder.id, "field_123");
    }
}
