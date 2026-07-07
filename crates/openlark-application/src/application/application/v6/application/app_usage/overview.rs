//! 获取应用使用概览

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用使用概览的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationUsageOverviewRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取应用使用概览的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationUsageOverviewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationUsageOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationUsageOverviewRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取应用使用概览请求。
    pub async fn execute(self) -> SDKResult<GetApplicationUsageOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationUsageOverviewResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/overview",
            self.app_id
        );
        let req: ApiRequest<GetApplicationUsageOverviewResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取应用使用概览", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{app_id}/app_usage/overview → 强类型 GetApplicationUsageOverviewResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_application_usage_overview_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_usage/overview",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "active_users": 256, "total_users": 1024 } }
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

        let resp = GetApplicationUsageOverviewRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("获取应用使用概览应成功");
        let data = resp.data.expect("响应数据不应为空");
        assert_eq!(data["active_users"], 256);
        assert_eq!(data["total_users"], 1024);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_usage/overview"
        );
    }
}
