//! 查询租户授权状态
//! docPath: <https://open.feishu.cn/document/application-v6/scope/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询租户授权状态的请求。
#[derive(Debug, Clone)]
pub struct ListApplicationScopeRequest {
    config: Arc<Config>,
}

/// 查询租户授权状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListApplicationScopeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListApplicationScopeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListApplicationScopeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行查询租户授权状态请求。
    pub async fn execute(self) -> SDKResult<ListApplicationScopeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListApplicationScopeResponse> {
        let path = "/open-apis/application/v6/scopes";
        let req: ApiRequest<ListApplicationScopeResponse> = ApiRequest::get(path);

        Transport::request_typed(req, &self.config, Some(option), "查询租户授权状态").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../scopes → 强类型 ListApplicationScopeResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_list_application_scope_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/application/v6/scopes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "scope_id": "scope_001", "scope_name": "通讯录读取" } }
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

        let resp = ListApplicationScopeRequest::new(config)
            .execute()
            .await
            .expect("查询租户授权状态应成功");
        assert_eq!(resp.data.unwrap()["scope_id"], "scope_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/application/v6/scopes");
    }
}
