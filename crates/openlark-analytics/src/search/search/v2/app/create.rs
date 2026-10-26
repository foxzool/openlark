//! 搜索应用
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/suite-search/create-2>

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

/// 搜索应用请求体。
///
/// `query` 必填（官方文档请求体仅此一个字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchAppBody {
    /// 搜索关键词（必填）。
    pub query: String,
}

/// 搜索应用请求。
#[derive(Debug, Clone)]
pub struct SearchAppRequest {
    config: Arc<Config>,
    body: SearchAppBody,
    /// 用户 ID 类型（查询参数，默认 open_id）。
    user_id_type: Option<String>,
    /// 分页大小（查询参数，默认 20，取值范围 1～100）。
    page_size: Option<i32>,
    /// 分页标记（查询参数，第一次请求不填）。
    page_token: Option<String>,
}

/// 搜索应用响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAppResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SearchAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchAppRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SearchAppBody::default(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置搜索关键词（必填）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置用户 ID 类型（默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小（默认 20，取值范围 1～100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（第一次请求不填）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行搜索应用请求。
    pub async fn execute(self) -> SDKResult<SearchAppResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行搜索应用请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<SearchAppResponse> {
        validate_required!(self.body.query, "query 不能为空");
        let path = "/open-apis/search/v2/app".to_string();
        let req: ApiRequest<SearchAppResponse> = ApiRequest::post(&path)
            .body(serde_json::to_value(&self.body)?)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .query_opt("page_token", self.page_token);

        Transport::request_typed(req, &self.config, Some(option), "搜索应用").await
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

    /// 端到端：POST /open-apis/search/v2/app + body {query} + 分页查询参数 → 响应解析。
    #[tokio::test]
    async fn test_create_search_app_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/app"))
            .and(wiremock::matchers::body_json(json!({
                "query": "测试应用"
            })))
            .and(wiremock::matchers::query_param("user_id_type", "open_id"))
            .and(wiremock::matchers::query_param("page_size", "20"))
            .and(wiremock::matchers::query_param("page_token", "tok_001"))
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

        let resp = SearchAppRequest::new(config)
            .query("测试应用")
            .user_id_type("open_id")
            .page_size(20)
            .page_token("tok_001")
            .execute()
            .await
            .expect("创建搜索应用应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/app");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent, json!({ "query": "测试应用" }));
    }

    /// query 为空时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_create_search_app_rejects_blank_query() {
        let server = MockServer::start().await;
        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let result = SearchAppRequest::new(config).execute().await;
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
