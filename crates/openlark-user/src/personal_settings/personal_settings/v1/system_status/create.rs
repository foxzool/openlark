//! system_status create
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusCreateRequest {
    config: Arc<Config>,
}

/// 创建系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusCreateResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusCreateRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusCreateResponse> {
        let req: ApiRequest<SystemStatusCreateResponse> =
            ApiRequest::post("/open-apis/personal_settings/v1/system_statuses");

        Transport::request_typed(req, &self.config, Some(option), "system_status create").await
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

    /// 端到端：POST .../system_statuses（无 body）→ 响应解析。
    #[tokio::test]
    async fn test_create_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/personal_settings/v1/system_statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "system_status_id": "ss_new" } }
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

        let resp = SystemStatusCreateRequest::new(config)
            .execute()
            .await
            .expect("创建系统状态应成功");
        assert_eq!(resp.data.unwrap()["system_status_id"], "ss_new");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses"
        );
    }
}
