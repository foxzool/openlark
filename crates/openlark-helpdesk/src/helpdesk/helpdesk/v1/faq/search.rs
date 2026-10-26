//! 搜索知识库
//!
//! 搜索服务台知识库。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 搜索知识库查询参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFaqQuery {
    /// 搜索 query（必填）；非英文内容可用 url 编码，或 base64 编码并设置 base64=true
    pub query: String,
    /// 是否转换为 base64，"true" 表示是，不填写表示否
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64: Option<String>,
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
    query: String,
    base64: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl SearchFaqRequest {
    /// 创建新的搜索知识库请求
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            query: String::new(),
            base64: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索 query（必填）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// 设置是否转换为 base64（"true" 表示是）
    pub fn base64(mut self, base64: impl Into<String>) -> Self {
        self.base64 = Some(base64.into());
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
        openlark_core::validate_required!(self.query, "query 不能为空");

        let api_endpoint = HelpdeskApiV1::FaqSearch;
        let mut request = ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url());

        request = request.query("query", self.query.as_str());

        if let Some(ref base64) = self.base64 {
            request = request.query("base64", base64);
        }

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(request, &self.config, Some(option), "搜索知识库").await
    }
}

/// 搜索知识库请求构建器
#[derive(Debug, Clone)]
pub struct SearchFaqRequestBuilder {
    config: Arc<Config>,
    query: String,
    base64: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl SearchFaqRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            query: String::new(),
            base64: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索 query（必填）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// 设置是否转换为 base64（"true" 表示是）
    pub fn base64(mut self, base64: impl Into<String>) -> Self {
        self.base64 = Some(base64.into());
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
        openlark_core::validate_required!(self.query, "query 不能为空");

        let api_endpoint = HelpdeskApiV1::FaqSearch;
        let mut request = ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url());

        request = request.query("query", self.query.as_str());

        if let Some(ref base64) = self.base64 {
            request = request.query("base64", base64);
        }

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string().as_str());
        }

        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(request, &self.config, None, "搜索知识库").await
    }
}

/// 执行搜索知识库
pub async fn search_faqs(
    config: &Config,
    query: impl Into<String>,
) -> SDKResult<SearchFaqResponse> {
    let api_endpoint = HelpdeskApiV1::FaqSearch;
    let request =
        ApiRequest::<SearchFaqResponse>::get(api_endpoint.to_url()).query("query", query.into());

    Transport::request_typed(request, config, None, "搜索知识库").await
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

        assert!(builder.query.is_empty());
        assert!(builder.base64.is_none());
    }

    /// 端到端：GET .../faqs/search（query 必填）→ 强类型 SearchFaqResponse 解析（扁平 data 信封）。
    #[tokio::test]
    async fn test_search_faqs_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/faqs/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [ { "id": "faq_001", "title": "如何重置密码？" } ],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = SearchFaqRequest::new(config)
            .query("wifi")
            .base64("true")
            .execute()
            .await
            .expect("搜索知识库应成功");
        assert!(resp.items.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/helpdesk/v1/faqs/search");
        // 官方查询参数名为 query（非 keyword），base64 为可选参数
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("query=wifi"),
            "query 应包含 query=wifi: {query}"
        );
        assert!(
            query.contains("base64=true"),
            "query 应包含 base64=true: {query}"
        );
        assert!(
            !query.contains("keyword="),
            "不应再有 keyword 参数: {query}"
        );
    }
}
