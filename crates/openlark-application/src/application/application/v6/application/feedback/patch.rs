//! 更新应用反馈
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

/// 更新应用反馈的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationFeedbackRequest {
    config: Arc<Config>,
    app_id: String,
    resource_id: String,
}

/// 更新应用反馈的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationFeedbackResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationFeedbackResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationFeedbackRequest {
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

    /// 执行更新应用反馈请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationFeedbackResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationFeedbackResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks/{}",
            self.app_id, self.resource_id
        );
        let req: ApiRequest<PatchApplicationFeedbackResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("更新应用反馈", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../applications/{app_id}/feedbacks/{feedback_id} → 强类型
    /// PatchApplicationFeedbackResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_feedback_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/feedbacks/fb_123",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "feedback_id": "fb_123" } }
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

        let resp = PatchApplicationFeedbackRequest::new(config, "cli_test_app", "fb_123")
            .execute()
            .await
            .expect("更新应用反馈应成功");
        assert_eq!(resp.data.unwrap()["feedback_id"], "fb_123");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/feedbacks/fb_123"
        );
    }
}
