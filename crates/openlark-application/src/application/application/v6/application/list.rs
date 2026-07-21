//! 获取企业安装的应用
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

/// 获取企业安装的应用的请求。
#[derive(Debug, Clone)]
pub struct ListApplicationsRequest {
    config: Arc<Config>,
}

/// 获取企业安装的应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取企业安装的应用请求。
    pub async fn execute(self) -> SDKResult<ListApplicationsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationsResponse> {
        let path = "/open-apis/application/v6/applications";
        let req: ApiRequest<ListApplicationsResponse> = ApiRequest::get(path);

        Transport::request_typed(req, &self.config, Some(option), "获取企业安装的应用").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET /open-apis/application/v6/applications → 强类型 ListApplicationsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_list_applications_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v6/applications"))
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

        let resp = ListApplicationsRequest::new(config)
            .execute()
            .await
            .expect("获取企业安装的应用应成功");
        assert!(resp.data.unwrap()["items"].is_array());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications"
        );
    }
}
