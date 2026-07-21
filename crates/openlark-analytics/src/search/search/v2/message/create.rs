//! 搜索消息
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/suite-search/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 搜索消息请求。
#[derive(Debug, Clone)]
pub struct SearchMessageRequest {
    config: Arc<Config>,
}

/// 搜索消息响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessageResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SearchMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchMessageRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行搜索消息请求。
    pub async fn execute(self) -> SDKResult<SearchMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行搜索消息请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchMessageResponse> {
        let path = "/open-apis/search/v2/message".to_string();
        let req: ApiRequest<SearchMessageResponse> = ApiRequest::post(&path);

        Transport::request_typed(req, &self.config, Some(option), "搜索消息").await
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

    /// 端到端：POST /open-apis/search/v2/message → 响应解析。
    #[tokio::test]
    async fn test_create_search_message_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/message"))
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

        let resp = SearchMessageRequest::new(config)
            .execute()
            .await
            .expect("创建搜索消息应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/search/v2/message");
    }
}
