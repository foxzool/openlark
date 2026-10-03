//! 拒绝人工任务 API
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/reject>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 拒绝人工任务的请求构建器。
pub struct RejectTaskRequestBuilder {
    approval_task_id: String,
    reason: Option<String>,
    config: Config,
}

impl RejectTaskRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            reason: None,
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 设置原因。
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<RejectTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RejectTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/reject",
            self.approval_task_id
        );
        let request_body = RejectTaskRequest {
            reason: self.reason,
        };
        let api_request: ApiRequest<RejectTaskResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "拒绝人工任务").await
    }
}

#[derive(Debug, Serialize)]
struct RejectTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 拒绝人工任务的响应。
pub struct RejectTaskResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for RejectTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RejectTaskRequestBuilder, will be removed in v1.0 (#271)")]
pub type RejectTaskBuilder = RejectTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_tasks/{id}/reject → 强类型 RejectTaskResponse。
    #[tokio::test]
    async fn test_reject_approval_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/approval_tasks/task_001/reject"))
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

        let resp = RejectTaskRequestBuilder::new(config)
            .approval_task_id("task_001")
            .reason("内容不符合要求")
            .execute()
            .await
            .expect("拒绝人工任务应成功");
        assert_eq!(resp.result, "success");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/reject"
        );
    }
}
