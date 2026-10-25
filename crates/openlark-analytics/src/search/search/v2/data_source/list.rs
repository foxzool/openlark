//! 批量获取数据源
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::get::DataSource;

/// 批量获取数据源请求。
#[derive(Debug, Clone)]
pub struct ListDataSourceRequest {
    config: Arc<Config>,
    /// 回包数据格式：0-全量数据，1-摘要数据（摘要仅含 id/name/state）。
    view: Option<i32>,
    /// 分页大小，最大值 50。
    page_size: Option<i32>,
    /// 分页标记，第一次请求不填。
    page_token: Option<String>,
}

/// 批量获取数据源响应。
///
/// 官方响应 `data` 内容形如 `{ has_more, page_token, items: [...] }`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDataSourceResponse {
    /// 是否还有更多项。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记，has_more 为 true 时返回新的 page_token。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 数据源中的数据记录。
    #[serde(default)]
    pub items: Option<Vec<DataSource>>,
}

impl ApiResponseTrait for ListDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            view: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置回包数据格式：0-全量数据，1-摘要数据。
    pub fn view(mut self, view: i32) -> Self {
        self.view = Some(view);
        self
    }

    /// 设置分页大小（最大值 50）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（第一次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行批量获取数据源请求。
    pub async fn execute(self) -> SDKResult<ListDataSourceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行批量获取数据源请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListDataSourceResponse> {
        let path = "/open-apis/search/v2/data_sources".to_string();
        let req: ApiRequest<ListDataSourceResponse> = ApiRequest::get(&path)
            .query_opt("view", self.view.map(|v| v.to_string()))
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token);

        Transport::request_typed(req, &self.config, Some(option), "批量获取数据源").await
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

    /// 端到端：GET /open-apis/search/v2/data_sources?view&page_size&page_token → 响应解析。
    #[tokio::test]
    async fn test_list_data_sources_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/data_sources"))
            .and(wiremock::matchers::query_param("view", "1"))
            .and(wiremock::matchers::query_param("page_size", "10"))
            .and(wiremock::matchers::query_param("page_token", "tok_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "items": [
                        { "id": "6953903108179099667", "name": "客服工单", "state": 0 }
                    ]
                }
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

        let resp = ListDataSourceRequest::new(config)
            .view(1)
            .page_size(10)
            .page_token("tok_001")
            .execute()
            .await
            .expect("获取数据源列表应成功");
        assert_eq!(resp.has_more, Some(false));
        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("6953903108179099667"));
        assert_eq!(items[0].name.as_deref(), Some("客服工单"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/data_sources");
        // 查询参数按官方文档拼入 URL
        let pairs: Vec<(String, String)> = received[0]
            .url
            .query_pairs()
            .map(|p| (p.0.to_string(), p.1.to_string()))
            .collect();
        assert!(pairs.contains(&("view".to_string(), "1".to_string())));
        assert!(pairs.contains(&("page_size".to_string(), "10".to_string())));
        assert!(pairs.contains(&("page_token".to_string(), "tok_001".to_string())));
    }

    /// 不设置查询参数时不拼任何 query。
    #[tokio::test]
    async fn test_list_data_sources_without_query_params() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/data_sources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "has_more": false, "items": [] }
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

        let resp = ListDataSourceRequest::new(config)
            .execute()
            .await
            .expect("获取数据源列表应成功");
        assert_eq!(resp.has_more, Some(false));
        assert!(resp.items.as_ref().is_some_and(|i| i.is_empty()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(received[0].url.query().is_none());
    }
}
