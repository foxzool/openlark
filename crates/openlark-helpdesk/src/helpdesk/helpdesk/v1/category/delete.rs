//! 删除指定知识库分类
//!
//! 删除指定的知识库分类。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/category/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 删除知识库分类响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCategoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DeleteCategoryResult>,
}

impl openlark_core::api::ApiResponseTrait for DeleteCategoryResponse {}

/// 删除知识库分类结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCategoryResult {
    /// 是否删除成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 删除知识库分类请求
#[derive(Debug, Clone)]
pub struct DeleteCategoryRequest {
    config: Arc<Config>,
    id: String,
}

impl DeleteCategoryRequest {
    /// 创建新的删除知识库分类请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行删除知识库分类请求
    pub async fn execute(self) -> SDKResult<DeleteCategoryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行删除知识库分类请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteCategoryResponse> {
        let req: ApiRequest<DeleteCategoryResponse> =
            ApiRequest::delete(HelpdeskApiV1::CategoryDelete(self.id.clone()).to_url());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除知识库分类")
    }
}

/// 删除知识库分类请求构建器
#[derive(Debug, Clone)]
pub struct DeleteCategoryRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl DeleteCategoryRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<DeleteCategoryResponse> {
        let request = DeleteCategoryRequest::new(self.config.clone(), self.id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<DeleteCategoryResponse> {
        let request = DeleteCategoryRequest::new(self.config.clone(), self.id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行删除知识库分类
pub async fn delete_category(config: &Config, id: String) -> SDKResult<DeleteCategoryResponse> {
    delete_category_with_options(config, id, RequestOption::default()).await
}

/// 执行删除知识库分类（支持自定义选项）
pub async fn delete_category_with_options(
    config: &Config,
    id: String,
    option: RequestOption,
) -> SDKResult<DeleteCategoryResponse> {
    let req: ApiRequest<DeleteCategoryResponse> =
        ApiRequest::delete(HelpdeskApiV1::CategoryDelete(id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "删除知识库分类")
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
            DeleteCategoryRequestBuilder::new(Arc::new(config), "category_123".to_string());

        assert_eq!(builder.id, "category_123");
    }
}
