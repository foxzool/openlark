//! 更新应用协作者

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用协作者的请求。
#[derive(Debug, Clone)]
pub struct UpdateApplicationCollaboratorsRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 更新应用协作者的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApplicationCollaboratorsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateApplicationCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateApplicationCollaboratorsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行更新应用协作者请求。
    pub async fn execute(self) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            self.app_id
        );
        let req: ApiRequest<UpdateApplicationCollaboratorsResponse> = ApiRequest::put(&path);

        Transport::request_typed(req, &self.config, Some(option), "更新应用协作者").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../applications/{app_id}/collaborators → 强类型 UpdateApplicationCollaboratorsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_update_application_collaborators_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/collaborators",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app" } }
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

        let resp = UpdateApplicationCollaboratorsRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("更新应用协作者应成功");
        assert_eq!(resp.data.unwrap()["app_id"], "cli_test_app");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/collaborators"
        );
    }
}
