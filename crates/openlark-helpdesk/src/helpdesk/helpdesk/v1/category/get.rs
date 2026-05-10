//! 获取指定知识库分类
//!
//! 获取指定知识库分类的详情。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/get

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

/// 获取指定知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCategoryResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CategoryItem>,
}

impl ApiResponseTrait for GetCategoryResponse {
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

/// 获取指定知识库分类请求
#[derive(Debug, Clone)]
pub struct GetCategoryRequest {
    config: Arc<Config>,
    id: String,
}

impl GetCategoryRequest {
    /// 创建新的获取指定知识库分类请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行获取指定知识库分类请求
    pub async fn execute(self) -> SDKResult<GetCategoryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetCategoryResponse> {
        let api_endpoint = HelpdeskApiV1::CategoryGet(self.id.clone());
        let request = ApiRequest::<GetCategoryResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取指定知识库分类")
    }
}

/// 获取指定知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct GetCategoryRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl GetCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetCategoryResponse> {
        let api_endpoint = HelpdeskApiV1::CategoryGet(self.id.clone());
        let request = ApiRequest::<GetCategoryResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取指定知识库分类")
    }
}

/// 执行获取指定知识库分类
pub async fn get_category(config: &Config, id: String) -> SDKResult<GetCategoryResponse> {
    let api_endpoint = HelpdeskApiV1::CategoryGet(id);
    let request = ApiRequest::<GetCategoryResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取指定知识库分类")
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
        let builder = GetCategoryRequestBuilder::new(Arc::new(config), "category_123".to_string());

        assert_eq!(builder.id, "category_123");
    }
}
