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
    body: AddAssigneeBody,
    config: Config,
}

impl AddAssigneeRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            body: AddAssigneeBody {
                user_id: String::new(),
                approvers: None,
                add_assignee_type: None,
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

    /// 设置审批人列表（可选，最多 20 个）。
    pub fn approvers(mut self, approvers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body.approvers = Some(
            approvers
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>(),
        );
        self
    }

    /// 设置加签类型（并加签 currentAndAddAssign / 后加签 afterAndAddAssign）。
    pub fn add_assignee_type(mut self, add_assignee_type: impl Into<String>) -> Self {
        self.body.add_assignee_type = Some(add_assignee_type.into());
        self
    }

    /// 设置加签原因。
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = Some(opinion.into());
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
        validate_required!(self.body.user_id, "操作人ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/add_assignee",
            self.approval_task_id
        );
        let api_request: ApiRequest<AddAssigneeResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "人工任务加签").await
    }
}

/// 人工任务加签请求体（官方字段：user_id 必填，其余可选）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAssigneeBody {
    /// 操作人 ID
    pub user_id: String,
    /// 审批人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<String>>,
    /// 加签类型：并加签 currentAndAddAssign / 后加签 afterAndAddAssign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_assignee_type: Option<String>,
    /// 加签原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opinion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// 人工任务加签的响应（官方成功响应 data 为空对象，无业务字段）。
pub struct AddAssigneeResponse {}

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

    /// 端到端：POST .../approval_tasks/{id}/add_assignee → 官方形状（data 为空对象）。
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

        let _resp = AddAssigneeRequestBuilder::new(config)
            .approval_task_id("task_001")
            .user_id("1783981209205788")
            .approvers(vec!["1783981209205788"])
            .add_assignee_type("currentAndAddAssign")
            .opinion("加签")
            .execute()
            .await
            .expect("人工任务加签应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/add_assignee"
        );
        // 请求体须与官方文档一致：user_id + approvers + add_assignee_type + opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "user_id": "1783981209205788",
                "approvers": ["1783981209205788"],
                "add_assignee_type": "currentAndAddAssign",
                "opinion": "加签"
            })
        );
    }
}
