//! 上传应用图标
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 上传应用图标请求。
#[derive(Debug, Clone)]
pub struct AppAvatarUploadCreateRequest {
    config: Arc<Config>,
}

impl AppAvatarUploadCreateRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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
        let path = "/open-apis/application/v7/app_avatar/upload".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("上传应用图标", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = AppAvatarUploadCreateRequest::new(config);
    }

    /// 端到端：POST .../app_avatar/upload → 原始 serde_json::Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_create_app_avatar_upload_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/application/v7/app_avatar/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "icon": "avatar_data" }
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

        let resp = AppAvatarUploadCreateRequest::new(config)
            .execute(json!({ "image_type": "icon" }))
            .await
            .expect("上传应用图标应成功");
        assert_eq!(resp["icon"], "avatar_data");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v7/app_avatar/upload"
        );
    }
}
