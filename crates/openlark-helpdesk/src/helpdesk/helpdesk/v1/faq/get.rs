//! 获取指定知识库
//!
//! 获取指定知识库的详情。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/get

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

/// 获取指定知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFaqResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FaqItem>,
}

impl ApiResponseTrait for GetFaqResponse {
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

/// 获取指定知识库请求
#[derive(Debug, Clone)]
pub struct GetFaqRequest {
    config: Arc<Config>,
    id: String,
}

impl GetFaqRequest {
    /// 创建新的获取指定知识库请求
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行获取指定知识库请求
    pub async fn execute(self) -> SDKResult<GetFaqResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqGet(self.id.clone());
        let request = ApiRequest::<GetFaqResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取指定知识库")
    }
}

/// 获取指定知识库请求构建器
#[derive(Debug, Clone)]
pub struct GetFaqRequestBuilder {
    config: Arc<Config>,
    id: String,
}

impl GetFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String) -> Self {
        Self { config, id }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqGet(self.id.clone());
        let request = ApiRequest::<GetFaqResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取指定知识库")
    }
}

/// 执行获取指定知识库
pub async fn get_faq(config: &Config, id: String) -> SDKResult<GetFaqResponse> {
    let api_endpoint = HelpdeskApiV1::FaqGet(id);
    let request = ApiRequest::<GetFaqResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取指定知识库")
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
        let builder = GetFaqRequestBuilder::new(Arc::new(config), "faq_123".to_string());

        assert_eq!(builder.id, "faq_123");
    }
}
