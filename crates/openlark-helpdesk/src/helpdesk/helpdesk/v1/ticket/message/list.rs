//! 获取工单消息列表
//!
//! 获取服务台工单消息详情。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket-management/ticket-message/list

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

/// 获取工单消息列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketMessageResponse {
    /// 工单消息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TicketMessageItem>>,
}

impl ApiResponseTrait for ListTicketMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 工单消息项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessageItem {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 发送者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 获取工单消息列表请求
#[derive(Debug, Clone)]
pub struct ListTicketMessageRequest {
    config: Arc<Config>,
    ticket_id: String,
}

impl ListTicketMessageRequest {
    /// 创建新的获取工单消息列表请求
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self { config, ticket_id }
    }

    /// 执行获取工单消息列表请求
    pub async fn execute(self) -> SDKResult<ListTicketMessageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTicketMessageResponse> {
        let api_endpoint = HelpdeskApiV1::TicketMessageList(self.ticket_id.clone());
        let request = ApiRequest::<ListTicketMessageResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取工单消息列表")
    }
}

/// 获取工单消息列表请求构建器
#[derive(Debug, Clone)]
pub struct ListTicketMessageRequestBuilder {
    config: Arc<Config>,
    ticket_id: String,
}

impl ListTicketMessageRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, ticket_id: String) -> Self {
        Self { config, ticket_id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListTicketMessageResponse> {
        let api_endpoint = HelpdeskApiV1::TicketMessageList(self.ticket_id.clone());
        let request = ApiRequest::<ListTicketMessageResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取工单消息列表")
    }
}

/// 执行获取工单消息列表
pub async fn list_ticket_messages(
    config: &Config,
    ticket_id: String,
) -> SDKResult<ListTicketMessageResponse> {
    let api_endpoint = HelpdeskApiV1::TicketMessageList(ticket_id);
    let request = ApiRequest::<ListTicketMessageResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取工单消息列表")
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
            ListTicketMessageRequestBuilder::new(Arc::new(config), "ticket_123".to_string());

        assert_eq!(builder.ticket_id, "ticket_123");
    }
}
