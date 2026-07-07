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

/// 批量获取数据源请求。
#[derive(Debug, Clone)]
pub struct ListDataSourceRequest {
    config: Arc<Config>,
}

/// 批量获取数据源响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDataSourceResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListDataSourceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListDataSourceRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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
        let req: ApiRequest<ListDataSourceResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("批量获取数据源", "响应数据为空"))
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

    /// 端到端：GET /open-apis/search/v2/data_sources → 响应解析。
    #[tokio::test]
    async fn test_list_data_sources_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/search/v2/data_sources"))
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

        let resp = ListDataSourceRequest::new(config)
            .execute()
            .await
            .expect("获取数据源列表应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/data_sources");
    }
}
