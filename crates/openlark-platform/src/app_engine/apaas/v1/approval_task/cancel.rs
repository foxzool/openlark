//! 撤销人工任务 API

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 撤销人工任务的请求构建器。
pub struct CancelTaskRequestBuilder {
    approval_task_id: String,
    config: Config,
}

impl CancelTaskRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CancelTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/cancel",
            self.approval_task_id
        );
        let api_request: ApiRequest<CancelTaskResponse> = ApiRequest::post(url);

        Transport::request_typed(api_request, &self.config, Some(option), "撤销人工任务").await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 撤销人工任务的响应。
pub struct CancelTaskResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for CancelTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CancelTaskRequestBuilder, will be removed in v1.0 (#271)")]
pub type CancelTaskBuilder = CancelTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_tasks/{id}/cancel → 强类型 CancelTaskResponse。
    #[tokio::test]
    async fn test_cancel_approval_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/approval_tasks/task_001/cancel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "result": "success" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CancelTaskRequestBuilder::new(config)
            .approval_task_id("task_001")
            .execute()
            .await
            .expect("撤销人工任务应成功");
        assert_eq!(resp.result, "success");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/cancel"
        );
    }
}
