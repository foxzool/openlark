//! 人工任务加签 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/apaas-v1/flow/user-task/add_assignee>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/add_assignee>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 人工任务加签的请求构建器。
pub struct AddAssigneeRequestBuilder {
    approval_task_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl AddAssigneeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            user_ids: Vec::new(),
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 设置用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<AddAssigneeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AddAssigneeResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");
        validate_required!(self.user_ids, "用户ID列表不能为空");

        let request_body = AddAssigneeRequest {
            user_ids: self.user_ids,
        };
        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/add_assignee",
            self.approval_task_id
        );
        let api_request: ApiRequest<AddAssigneeResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request_body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "人工任务加签").await
    }
}

#[derive(Debug, Serialize)]
struct AddAssigneeRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 人工任务加签的响应。
pub struct AddAssigneeResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for AddAssigneeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AddAssigneeRequestBuilder, will be removed in v1.0 (#271)")]
pub type AddAssigneeBuilder = AddAssigneeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_tasks/{id}/add_assignee → 强类型 AddAssigneeResponse。
    #[tokio::test]
    async fn test_add_assignee_approval_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/approval_tasks/task_001/add_assignee",
            ))
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

        let resp = AddAssigneeRequestBuilder::new(config)
            .approval_task_id("task_001")
            .user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .execute()
            .await
            .expect("人工任务加签应成功");
        assert_eq!(resp.result, "success");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/add_assignee"
        );
    }
}
