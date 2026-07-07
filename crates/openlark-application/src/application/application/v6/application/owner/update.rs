//! 转移应用所有者

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 转移应用所有者的请求。
#[derive(Debug, Clone)]
pub struct UpdateAppOwnerRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 转移应用所有者的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppOwnerResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateAppOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateAppOwnerRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行转移应用所有者请求。
    pub async fn execute(self) -> SDKResult<UpdateAppOwnerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateAppOwnerResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/owner",
            self.app_id
        );
        let req: ApiRequest<UpdateAppOwnerResponse> = ApiRequest::put(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("转移应用所有者", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../applications/{app_id}/owner → 强类型
    /// UpdateAppOwnerResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_update_owner_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/owner",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "new_owner": "ou_test_owner" } }
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

        let resp = UpdateAppOwnerRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("转移应用所有者应成功");
        assert_eq!(resp.data.unwrap()["new_owner"], "ou_test_owner");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/owner"
        );
    }
}
