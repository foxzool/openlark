//! system_status get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusGetRequest {
    config: Arc<Config>,
}

/// 获取系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusGetResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusGetRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusGetResponse> {
        let path = "/open-apis/personal-settings/personal-settings/v1/system-status/get";
        let req: ApiRequest<SystemStatusGetResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("system_status get", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/personal-settings/personal-settings/v1/system-status/get
    #[tokio::test]
    async fn test_get_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/personal-settings/personal-settings/v1/system-status/get",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
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

        let resp = SystemStatusGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        resp.data.expect("响应 data 不应为空");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
