//! 获取应用协作者列表
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用协作者列表的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationCollaboratorsRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取应用协作者列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationCollaboratorsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationCollaboratorsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取应用协作者列表请求。
    pub async fn execute(self) -> SDKResult<GetApplicationCollaboratorsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationCollaboratorsResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            self.app_id
        );
        let req: ApiRequest<GetApplicationCollaboratorsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取应用协作者列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../applications/{app_id}/collaborators → 强类型 GetApplicationCollaboratorsResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_application_collaborators_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/collaborators",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "app_id": "cli_test_app", "collaborators": [] } }
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

        let resp = GetApplicationCollaboratorsRequest::new(config, "cli_test_app")
            .execute()
            .await
            .expect("获取应用协作者列表应成功");
        let data = resp.data.unwrap();
        assert_eq!(data["app_id"], "cli_test_app");
        assert!(data["collaborators"].is_array());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/collaborators"
        );
    }
}
