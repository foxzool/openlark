//! 同意人工任务 API
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/agree>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 同意人工任务的请求构建器。
pub struct AgreeTaskRequestBuilder {
    approval_task_id: String,
    body: AgreeTaskBody,
    config: Config,
}

impl AgreeTaskRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            body: AgreeTaskBody {
                user_id: String::new(),
                opinion: None,
            },
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 设置操作人 ID（必填）。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置审批意见。
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = Some(opinion.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<AgreeTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<AgreeTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");
        validate_required!(self.body.user_id, "操作人ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/agree",
            self.approval_task_id
        );
        let api_request: ApiRequest<AgreeTaskResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "同意人工任务").await
    }
}

/// 同意人工任务请求体（官方字段：user_id 必填，opinion 可选）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreeTaskBody {
    /// 操作人 ID
    pub user_id: String,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opinion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// 同意人工任务的响应（官方成功响应 data 为空对象，无业务字段）。
pub struct AgreeTaskResponse {}

impl ApiResponseTrait for AgreeTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AgreeTaskRequestBuilder, will be removed in v1.0 (#271)")]
pub type AgreeTaskBuilder = AgreeTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_tasks/{id}/agree → 官方形状（data 为空对象）。
    #[tokio::test]
    async fn test_agree_approval_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/approval_tasks/task_001/agree"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _resp = AgreeTaskRequestBuilder::new(config)
            .approval_task_id("task_001")
            .user_id("1783981209205788")
            .opinion("同意")
            .execute()
            .await
            .expect("同意人工任务应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/agree"
        );
        // 请求体须与官方文档一致：user_id + opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "user_id": "1783981209205788",
                "opinion": "同意"
            })
        );
    }
}
