//! 更新应用能力配置
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 更新应用能力配置请求。
#[derive(Debug, Clone)]
pub struct ApplicationAbilityPatchRequest {
    config: Arc<Config>,
    app_id: String,
}

impl ApplicationAbilityPatchRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 设置路径参数 `app_id`。
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
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
        validate_required!(self.app_id, "app_id 不能为空");
        let path = format!(
            "/open-apis/application/v7/applications/{}/ability",
            self.app_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("更新应用能力配置", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = ApplicationAbilityPatchRequest::new(config);
    }

    /// 端到端：PATCH .../applications/{app_id}/ability → 原始 serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_patch_application_ability_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v7/applications/cli_test_app/ability",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "app_id": "cli_test_app" }
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

        let resp = ApplicationAbilityPatchRequest::new(config)
            .app_id("cli_test_app")
            .execute(json!({ "abilities": [] }))
            .await
            .expect("更新应用能力配置应成功");
        assert_eq!(resp["app_id"], "cli_test_app");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v7/applications/cli_test_app/ability"
        );
    }
}
