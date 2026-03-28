//! 搜索 Wiki
//!
//! 搜索 Wiki，用户通过关键词查询 Wiki，只能查找自己可见的 Wiki。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSearchResult;
use crate::common::api_endpoints::WikiApiV1;
use crate::common::api_utils::*;

/// 搜索 Wiki 请求（流式 Builder 模式）
pub struct SearchWikiRequest {
    config: Config,
    /// 搜索关键词
    query: String,
    /// 空间ID（可选）
    space_id: Option<String>,
    /// 节点ID（可选）
    node_id: Option<String>,
    /// 每页大小
    page_size: Option<i32>,
    /// 页面标记
    page_token: Option<String>,
}

/// 搜索 Wiki 请求体（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchWikiRequestBody {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 搜索 Wiki 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiResponse {
    /// 搜索结果列表
    #[serde(default)]
    pub items: Vec<WikiSearchResult>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 页面标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchWikiRequest {
    /// 创建搜索 Wiki 请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: String::new(),
            space_id: None,
            node_id: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// 设置空间 ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = Some(space_id.into());
        self
    }

    /// 设置节点 ID
    pub fn node_id(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置页面标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchWikiResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchWikiResponse> {
        validate_required!(self.query, "搜索关键词不能为空");

        let api_endpoint = WikiApiV1::NodeSearch;

        let request_body = SearchWikiRequestBody {
            query: self.query,
            space_id: self.space_id,
            node_id: self.node_id,
            page_size: self.page_size,
            page_token: self.page_token,
        };

        let api_request: ApiRequest<SearchWikiResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request_body, "搜索Wiki")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "搜索")
    }
}

/// 搜索 Wiki 请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 SearchWikiRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiParams {
    /// 搜索关键词
    pub query: String,
    /// 空间ID（可选）
    pub space_id: Option<String>,
    /// 节点ID（可选）
    pub node_id: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
