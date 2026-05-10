//! 搜索知识库
//!
//! 搜索服务台知识库。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/search

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

/// 搜索知识库查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFaqQuery {
    /// 搜索关键词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索知识库响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFaqResponse {
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 知识库列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<FaqItem>>,
}

impl ApiResponseTrait for SearchFaqResponse {
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
}

/// 搜索知识库请求
#[derive(Debug, Clone)]
pub struct SearchFaqRequest {
    config: Arc<Config>,
    keyword: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl SearchFaqRequest {
    /// 创建新的搜索知识库请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            keyword: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索知识库请求
    pub async fn execute(self) -> SDKResult<SearchFaqResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqSearch;
        let mut request = ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url());

        if let Some(ref keyword) = self.keyword {
            request = request.query("keyword", keyword);
        }

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "搜索知识库")
    }
}

/// 搜索知识库请求构建器
#[derive(Debug, Clone)]
pub struct SearchFaqRequestBuilder {
    config: Arc<Config>,
    keyword: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl SearchFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            keyword: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键词
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<SearchFaqResponse> {
        let api_endpoint = HelpdeskApiV1::FaqSearch;
        let mut request = ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url());

        if let Some(ref keyword) = self.keyword {
            request = request.query("keyword", keyword);
        }

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "搜索知识库")
    }
}

/// 执行搜索知识库
pub async fn search_faqs(config: &Config) -> SDKResult<SearchFaqResponse> {
    let api_endpoint = HelpdeskApiV1::FaqSearch;
    let request = ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "搜索知识库")
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
        let builder = SearchFaqRequestBuilder::new(Arc::new(config));

        assert!(builder.keyword.is_none());
    }
}
