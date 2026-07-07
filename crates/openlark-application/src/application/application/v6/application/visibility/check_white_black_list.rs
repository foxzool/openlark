//! 查询用户或部门是否在应用的可用或禁用名单

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询用户或部门是否在应用的可用或禁用名单的请求。
#[derive(Debug, Clone)]
pub struct CheckApplicationVisibilityWhiteBlackListRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 查询用户或部门是否在应用的可用或禁用名单的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckApplicationVisibilityWhiteBlackListResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CheckApplicationVisibilityWhiteBlackListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CheckApplicationVisibilityWhiteBlackListRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行查询用户或部门是否在应用的可用或禁用名单请求。
    pub async fn execute(self) -> SDKResult<CheckApplicationVisibilityWhiteBlackListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CheckApplicationVisibilityWhiteBlackListResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/visibility/check_white_black_list",
            self.app_id
        );
        let req: ApiRequest<CheckApplicationVisibilityWhiteBlackListResponse> =
            ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询用户或部门是否在应用的可用或禁用名单",
                "响应数据为空",
            )
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{app_id}/visibility/check_white_black_list → 强类型响应解析（双层 data 信封）。
    #[tokio::test]
    async fn test_check_application_visibility_white_black_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/visibility/check_white_black_list",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "in_white_list": true, "in_black_list": false } }
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

        let resp = CheckApplicationVisibilityWhiteBlackListRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("查询用户或部门是否在应用的可用或禁用名单应成功");
        let data = resp.data.expect("响应数据不应为空");
        assert_eq!(data["in_white_list"], true);
        assert_eq!(data["in_black_list"], false);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/visibility/check_white_black_list"
        );
    }
}
