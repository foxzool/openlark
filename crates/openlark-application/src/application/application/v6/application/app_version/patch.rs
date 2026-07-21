//! 更新应用审核状态
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用审核状态的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationAppVersionRequest {
    config: Arc<Config>,
    app_id: String,
    resource_id: String,
}

/// 更新应用审核状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationAppVersionResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationAppVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationAppVersionRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        app_id: impl Into<String>,
        resource_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            resource_id: resource_id.into(),
        }
    }

    /// 执行更新应用审核状态请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationAppVersionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationAppVersionResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            self.app_id, self.resource_id
        );
        let req: ApiRequest<PatchApplicationAppVersionResponse> = ApiRequest::patch(&path);

        Transport::request_typed(req, &self.config, Some(option), "更新应用审核状态").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../applications/{app_id}/app_versions/{version_id} → 强类型 PatchApplicationAppVersionResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_application_app_version_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/app_versions/ver_1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app", "version_id": "ver_1" } }
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

        let resp = PatchApplicationAppVersionRequest::new(config, "cli_test_app", "ver_1")
            .execute()
            .await
            .expect("更新应用审核状态应成功");
        let data = resp.data.unwrap();
        assert_eq!(data["app_id"], "cli_test_app");
        assert_eq!(data["version_id"], "ver_1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/app_versions/ver_1"
        );
    }
}
