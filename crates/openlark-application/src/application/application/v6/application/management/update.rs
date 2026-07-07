//! 启停用应用

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 启停用应用的请求。
#[derive(Debug, Clone)]
pub struct UpdateApplicationManagementRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 启停用应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApplicationManagementResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateApplicationManagementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateApplicationManagementRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行启停用应用请求。
    pub async fn execute(self) -> SDKResult<UpdateApplicationManagementResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateApplicationManagementResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/management",
            self.app_id
        );
        let req: ApiRequest<UpdateApplicationManagementResponse> = ApiRequest::put(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("启停用应用", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../applications/{app_id}/management → 强类型
    /// UpdateApplicationManagementResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_update_management_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/management",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "status": "enabled" } }
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

        let resp = UpdateApplicationManagementRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("启停用应用应成功");
        assert_eq!(resp.data.unwrap()["status"], "enabled");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/management"
        );
    }
}
