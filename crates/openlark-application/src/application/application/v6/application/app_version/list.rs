//! 获取应用版本列表
//! docPath: <https://open.feishu.cn/document/application-v6/admin/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用版本列表的请求。
#[derive(Debug, Clone)]
pub struct ListApplicationVersionsRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取应用版本列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationVersionsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationVersionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationVersionsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取应用版本列表请求。
    pub async fn execute(self) -> SDKResult<ListApplicationVersionsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationVersionsResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions",
            self.app_id
        );
        let req: ApiRequest<ListApplicationVersionsResponse> = ApiRequest::get(&path);

        Transport::request_typed(req, &self.config, Some(option), "获取应用版本列表").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/{app_id}/app_versions → 强类型 ListApplicationVersionsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_list_application_versions_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_versions",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "items": [] } }
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

        let resp = ListApplicationVersionsRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("获取应用版本列表应成功");
        assert!(resp.data.unwrap()["items"].is_array());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_versions"
        );
    }
}
