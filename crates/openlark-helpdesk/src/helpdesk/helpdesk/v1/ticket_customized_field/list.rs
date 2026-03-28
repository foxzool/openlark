//! 获取工单自定义字段列表
//!
//! 获取工单自定义字段列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/ticket_customized_field/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取工单自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTicketCustomizedFieldResponse {
    /// 工单自定义字段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TicketCustomizedFieldItem>>,
}

impl ApiResponseTrait for ListTicketCustomizedFieldResponse {
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

/// 获取工单自定义字段列表请求
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequest {
    config: Arc<Config>,
}

impl ListTicketCustomizedFieldRequest {
    /// 创建新的获取工单自定义字段列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取工单自定义字段列表请求
    pub async fn execute(self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
        let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取工单自定义字段列表")
    }
}

/// 获取工单自定义字段列表请求构建器
#[derive(Debug, Clone)]
pub struct ListTicketCustomizedFieldRequestBuilder {
    config: Arc<Config>,
}

impl ListTicketCustomizedFieldRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListTicketCustomizedFieldResponse> {
        let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
        let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取工单自定义字段列表")
    }
}

/// 执行获取工单自定义字段列表
pub async fn list_ticket_customized_fields(config: &Config) -> SDKResult<ListTicketCustomizedFieldResponse> {
    let api_endpoint = HelpdeskApiV1::TicketCustomizedFieldList;
    let request = ApiRequest::<ListTicketCustomizedFieldResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取工单自定义字段列表")
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
        let _builder = ListTicketCustomizedFieldRequestBuilder::new(Arc::new(config));
    }
}
