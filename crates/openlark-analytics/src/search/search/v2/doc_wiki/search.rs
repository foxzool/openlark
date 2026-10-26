//! 搜索文档
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/doc_wiki/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 时间范围（秒级时间戳，包含 start/end 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDocWikiTimeRange {
    /// 时间范围的起始时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// 时间范围的截止时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}

/// 文档过滤参数（doc_filter 与 wiki_filter 至少传一个）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDocWikiDocFilter {
    /// 文档所有者 OpenID（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_ids: Option<Vec<String>>,
    /// 文档类型（如 DOC/SHEET/BITABLE 等，最多 10 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_types: Option<Vec<String>>,
    /// 搜索文件夹内的文档（文件夹 token 列表，最多 50 个；存在时 wiki 筛选器失效）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_tokens: Option<Vec<String>>,
    /// 仅搜文档标题（默认 false）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_title: Option<bool>,
    /// 浏览文档的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_time: Option<SearchDocWikiTimeRange>,
    /// 排序方式（如 CREATE_TIME）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    /// 文档创建的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<SearchDocWikiTimeRange>,
    /// 搜索在会话内的文档（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
    /// 文档分享者 OpenID（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_ids: Option<Vec<String>>,
    /// 仅搜文档评论（默认 false）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_comment: Option<bool>,
    /// 【我编辑的文档】的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_edit_time: Option<SearchDocWikiTimeRange>,
    /// 【我评论的文档】的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_comment_time: Option<SearchDocWikiTimeRange>,
    /// 文档创建者 OpenID（注意和 creator_ids 区分，最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_creator_ids: Option<Vec<String>>,
}

/// Wiki 过滤参数（doc_filter 与 wiki_filter 至少传一个）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDocWikiWikiFilter {
    /// Wiki 所有者 OpenID（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_ids: Option<Vec<String>>,
    /// Wiki 类型（如 DOC/WIKI/DOCX 等，最多 10 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_types: Option<Vec<String>>,
    /// 搜索某个 Space 下的 Wiki（Space ID 列表，最多 50 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_ids: Option<Vec<String>>,
    /// 仅搜 Wiki 标题（默认 false）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_title: Option<bool>,
    /// 浏览文档的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_time: Option<SearchDocWikiTimeRange>,
    /// 排序方式（如 CREATE_TIME）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    /// Wiki 创建的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<SearchDocWikiTimeRange>,
    /// 搜索在会话内的文档（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
    /// 文档分享者 OpenID（最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_ids: Option<Vec<String>>,
    /// 仅搜文档评论（默认 false）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_comment: Option<bool>,
    /// 【我编辑的文档】的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_edit_time: Option<SearchDocWikiTimeRange>,
    /// 【我评论的文档】的时间范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_comment_time: Option<SearchDocWikiTimeRange>,
    /// 文档创建者 OpenID（注意和 creator_ids 区分，最多 20 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_creator_ids: Option<Vec<String>>,
}

/// 搜索文档请求体。
///
/// `query` 必填（至少搭配一种 doc/wiki 筛选器）；
/// `doc_filter`/`wiki_filter` 至少传一个；`page_token`/`page_size` 为分页字段（分页在请求体内）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDocWikiBody {
    /// 搜索关键词（必填，长度 0～30 字符）。
    pub query: String,
    /// 文档过滤参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_filter: Option<SearchDocWikiDocFilter>,
    /// Wiki 过滤参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_filter: Option<SearchDocWikiWikiFilter>,
    /// 分页标记（第一次请求不填）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 分页大小（默认 0，取值范围 0～20）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// 搜索文档请求。
#[derive(Debug, Clone)]
pub struct SearchDocWikiRequest {
    config: Arc<Config>,
    body: SearchDocWikiBody,
}

/// 搜索文档响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocWikiResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SearchDocWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchDocWikiRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SearchDocWikiBody::default(),
        }
    }

    /// 设置搜索关键词（必填，至少搭配一种 doc/wiki 筛选器）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置文档过滤参数（doc_filter 与 wiki_filter 至少传一个）。
    pub fn doc_filter(mut self, doc_filter: SearchDocWikiDocFilter) -> Self {
        self.body.doc_filter = Some(doc_filter);
        self
    }

    /// 设置 Wiki 过滤参数（doc_filter 与 wiki_filter 至少传一个）。
    pub fn wiki_filter(mut self, wiki_filter: SearchDocWikiWikiFilter) -> Self {
        self.body.wiki_filter = Some(wiki_filter);
        self
    }

    /// 设置分页标记（第一次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.body.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小（默认 0，取值范围 0～20）。
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.body.page_size = Some(page_size);
        self
    }

    /// 执行搜索文档请求。
    pub async fn execute(self) -> SDKResult<SearchDocWikiResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行搜索文档请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchDocWikiResponse> {
        validate_required!(self.body.query, "query 不能为空");
        let path = "/open-apis/search/v2/doc_wiki/search".to_string();
        let req: ApiRequest<SearchDocWikiResponse> =
            ApiRequest::post(&path).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(req, &self.config, Some(option), "搜索文档").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/search/v2/doc_wiki/search + 官方形状 body（含嵌套 filter）→ 响应解析。
    #[tokio::test]
    async fn test_search_doc_wiki_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/doc_wiki/search"))
            .and(wiremock::matchers::body_partial_json(json!({
                "query": "飞书文档使用指南",
                "doc_filter": {
                    "creator_ids": ["ou_789012"],
                    "doc_types": ["SHORTCUT"],
                    "folder_tokens": ["fld_123456"],
                    "only_title": false,
                    "open_time": { "start": 1742348544, "end": 1742348544 },
                    "sort_type": "CREATE_TIME"
                },
                "wiki_filter": {
                    "space_ids": ["space_1234567890fedcba"]
                },
                "page_token": "token_1234567890fedcba",
                "page_size": 15
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = SearchDocWikiRequest::new(config)
            .query("飞书文档使用指南")
            .doc_filter(SearchDocWikiDocFilter {
                creator_ids: Some(vec!["ou_789012".to_string()]),
                doc_types: Some(vec!["SHORTCUT".to_string()]),
                folder_tokens: Some(vec!["fld_123456".to_string()]),
                only_title: Some(false),
                open_time: Some(SearchDocWikiTimeRange {
                    start: Some(1742348544),
                    end: Some(1742348544),
                }),
                sort_type: Some("CREATE_TIME".to_string()),
                ..Default::default()
            })
            .wiki_filter(SearchDocWikiWikiFilter {
                space_ids: Some(vec!["space_1234567890fedcba".to_string()]),
                ..Default::default()
            })
            .page_token("token_1234567890fedcba")
            .page_size(15)
            .execute()
            .await
            .expect("搜索知识库应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/doc_wiki/search"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["query"], "飞书文档使用指南");
        assert_eq!(sent["doc_filter"]["folder_tokens"], json!(["fld_123456"]));
        assert_eq!(
            sent["doc_filter"]["open_time"],
            json!({ "start": 1742348544, "end": 1742348544 })
        );
        assert_eq!(
            sent["wiki_filter"]["space_ids"],
            json!(["space_1234567890fedcba"])
        );
        assert_eq!(sent["page_token"], "token_1234567890fedcba");
        assert_eq!(sent["page_size"], 15);
        // 未设置的嵌套可选字段不序列化
        assert!(sent["wiki_filter"].get("creator_ids").is_none());
        assert!(sent["doc_filter"].get("chat_ids").is_none());
    }

    /// 只设置必填 query 时 body 仅含 query。
    #[test]
    fn test_search_doc_wiki_body_requires_query_only() {
        let body = SearchDocWikiBody {
            query: "飞书文档使用指南".to_string(),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({ "query": "飞书文档使用指南" })
        );
    }

    /// query 为空时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_search_doc_wiki_rejects_blank_query() {
        let server = MockServer::start().await;
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let result = SearchDocWikiRequest::new(config).execute().await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
