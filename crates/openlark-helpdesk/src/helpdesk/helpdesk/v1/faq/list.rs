//! 获取知识库列表
//!
//! 获取服务台知识库列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/list>

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

/// 获取知识库列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFaqResponse {
    /// 知识库列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<FaqItem>>,
}

impl ApiResponseTrait for ListFaqResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识库项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    /// 知识库ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 获取知识库列表请求
#[derive(Debug, Clone)]
pub struct ListFaqRequest {
    config: Arc<Config>,
}

impl ListFaqRequest {
    /// 创建新的获取知识库列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取知识库列表请求
    pub async fn execute(self) -> SDKResult<ListFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqList;
        let request = ApiRequest::<ListFaqResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取知识库列表")
    }
}

/// 获取知识库列表请求构建器
#[derive(Debug, Clone)]
pub struct ListFaqRequestBuilder {
    config: Arc<Config>,
}

impl ListFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqList;
        let request = ApiRequest::<ListFaqResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取知识库列表")
    }
}

/// 执行获取知识库列表
pub async fn list_faqs(config: &Config) -> SDKResult<ListFaqResponse> {
    let api_endpoint = HelpdeskApiV1::FaqList;
    let request = ApiRequest::<ListFaqResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取知识库列表")
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
        let _builder = ListFaqRequestBuilder::new(Arc::new(config));
    }
}
