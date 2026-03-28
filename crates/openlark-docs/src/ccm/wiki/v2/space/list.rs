//! 获取知识空间列表
//!
//! 此接口用于获取有权限访问的知识空间列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::WikiSpace;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 获取知识空间列表请求（流式 Builder 模式）
pub struct ListWikiSpacesRequest {
    config: Config,
    /// 每页大小（最大 50）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

/// 获取知识空间列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesResponse {
    /// 知识空间列表
    #[serde(default)]
    pub items: Vec<WikiSpace>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 页面标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListWikiSpacesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWikiSpacesRequest {
    /// 创建获取知识空间列表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置每页大小（最大 50）
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
    pub async fn execute(self) -> SDKResult<ListWikiSpacesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListWikiSpacesResponse> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if page_size < 1 || page_size > 50 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须在 1~50 之间",
                ));
            }
        }

        // ===== 构建请求 =====
        let api_endpoint = WikiApiV2::SpaceList;

        let mut api_request: ApiRequest<ListWikiSpacesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            api_request = api_request.query("page_token", &page_token);
        }

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识空间列表")
    }
}

/// 获取知识空间列表请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 ListWikiSpacesRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpacesParams {
    /// 每页大小（最大 50）
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_list_wiki_spaces_builder() {
        let config = Config::default();
        let request = ListWikiSpacesRequest::new(config)
            .page_size(20)
            .page_token("token123");

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    /// 测试不带参数的请求
    #[test]
    fn test_list_wiki_spaces_no_params() {
        let config = Config::default();
        let request = ListWikiSpacesRequest::new(config);

        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
    }

    /// 测试响应数据结构
    #[test]
    fn test_list_wiki_spaces_response() {
        let response = ListWikiSpacesResponse {
            items: vec![],
            has_more: Some(true),
            page_token: Some("next_token".to_string()),
        };

        assert!(response.items.is_empty());
        assert_eq!(response.has_more, Some(true));
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(ListWikiSpacesResponse::data_format(), ResponseFormat::Data);
    }

    /// 测试边界page_size
    #[test]
    fn test_page_size_boundaries() {
        let config = Config::default();
        let min_request = ListWikiSpacesRequest::new(config.clone()).page_size(1);
        assert_eq!(min_request.page_size, Some(1));

        let max_request = ListWikiSpacesRequest::new(config).page_size(50);
        assert_eq!(max_request.page_size, Some(50));
    }

    /// 测试空列表响应
    #[test]
    fn test_empty_response() {
        let response = ListWikiSpacesResponse {
            items: vec![],
            has_more: Some(false),
            page_token: None,
        };

        assert!(response.items.is_empty());
        assert_eq!(response.has_more, Some(false));
    }

    /// 测试分页token设置
    #[test]
    fn test_page_token_setting() {
        let config = Config::default();
        let request = ListWikiSpacesRequest::new(config).page_token("custom_token_456");

        assert_eq!(request.page_token, Some("custom_token_456".to_string()));
    }

    /// 测试已弃用的参数结构（保留以测试向后兼容性）
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_params() {
        let params = ListWikiSpacesParams {
            page_size: Some(30),
            page_token: Some("old_token".to_string()),
        };

        assert_eq!(params.page_size, Some(30));
        assert_eq!(params.page_token, Some("old_token".to_string()));
    }
}
