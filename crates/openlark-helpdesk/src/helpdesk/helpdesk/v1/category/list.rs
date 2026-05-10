//! 获取知识库分类列表
//!
//! 获取服务台知识库所有分类。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/list-categories

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

/// 获取知识库分类列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCategoryResponse {
    /// 分类列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CategoryItem>>,
}

impl ApiResponseTrait for ListCategoryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 知识库分类项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryItem {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// 获取知识库分类列表请求
#[derive(Debug, Clone)]
pub struct ListCategoryRequest {
    config: Arc<Config>,
}

impl ListCategoryRequest {
    /// 创建新的获取知识库分类列表请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取知识库分类列表请求
    pub async fn execute(self) -> SDKResult<ListCategoryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCategoryResponse> {
        let api_endpoint = HelpdeskApiV1::CategoryList;
        let request = ApiRequest::<ListCategoryResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识库分类列表")
    }
}

/// 获取知识库分类列表请求构建器
#[derive(Debug, Clone)]
pub struct ListCategoryRequestBuilder {
    config: Arc<Config>,
}

impl ListCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ListCategoryResponse> {
        let api_endpoint = HelpdeskApiV1::CategoryList;
        let request = ApiRequest::<ListCategoryResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取知识库分类列表")
    }
}

/// 执行获取知识库分类列表
pub async fn list_categories(config: &Config) -> SDKResult<ListCategoryResponse> {
    let api_endpoint = HelpdeskApiV1::CategoryList;
    let request = ApiRequest::<ListCategoryResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取知识库分类列表")
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
        let _builder = ListCategoryRequestBuilder::new(Arc::new(config));
    }
}
