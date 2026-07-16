//! 搜索妙记
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

use crate::common::api_endpoints::minutes::MinutesExtraApiV1;

/// 搜索妙记请求。
#[derive(Debug, Clone)]
pub struct MinuteSearchRequest {
    config: Arc<Config>,
}

impl MinuteSearchRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = MinutesExtraApiV1::Search.to_request().body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("搜索妙记", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = MinuteSearchRequest::new(config);
    }

    #[tokio::test]
    async fn search_uses_catalog_request_semantics() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/minutes/v1/minutes/search"))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "items": [], "has_more": false }
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
        let option = RequestOption::builder()
            .tenant_access_token("test-tenant-token")
            .build();

        let response = MinuteSearchRequest::new(config)
            .execute_with_options(json!({ "query": "周会", "page_size": 20 }), option)
            .await
            .expect("搜索妙记应成功");
        assert_eq!(response["has_more"], false);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(body["query"], "周会");
        assert_eq!(body["page_size"], 20);
    }
}
