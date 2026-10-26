//! 转交人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/transfer>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/transfer>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 转交人工任务 Builder
#[derive(Debug, Clone)]
pub struct TransferApprovalTaskRequestBuilder {
    config: Config,
    /// 审批任务 ID
    approval_task_id: String,
    /// 请求体
    body: TransferApprovalTaskBody,
}

impl TransferApprovalTaskRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, approval_task_id: impl Into<String>) -> Self {
        Self {
            config,
            approval_task_id: approval_task_id.into(),
            body: TransferApprovalTaskBody {
                user_id: String::new(),
                from_user_ids: None,
                to_user_ids: None,
                opinion: None,
            },
        }
    }

    /// 设置操作人 ID（必填）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置原审批人 ID 列表（可选，最多 20 个）
    pub fn from_user_ids(
        mut self,
        from_user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.body.from_user_ids = Some(
            from_user_ids
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>(),
        );
        self
    }

    /// 设置新审批人 ID 列表（可选，最多 20 个）
    pub fn to_user_ids(mut self, to_user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body.to_user_ids = Some(
            to_user_ids
                .into_iter()
                .map(Into::into)
                .collect::<Vec<String>>(),
        );
        self
    }

    /// 设置转交原因
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = Some(opinion.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferApprovalTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TransferApprovalTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");
        validate_required!(self.body.user_id, "操作人ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/transfer",
            self.approval_task_id
        );

        let req = ApiRequest::<TransferApprovalTaskResponse>::post(&url)
            .body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "转交人工任务").await
    }
}

/// 转交人工任务请求体（官方字段：user_id 必填，其余可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferApprovalTaskBody {
    /// 操作人 ID
    pub user_id: String,
    /// 原审批人 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_user_ids: Option<Vec<String>>,
    /// 新审批人 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_ids: Option<Vec<String>>,
    /// 转交原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opinion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// 转交人工任务的响应（官方成功响应 data 为空对象，无业务字段）。
pub struct TransferApprovalTaskResponse {}

impl ApiResponseTrait for TransferApprovalTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to TransferApprovalTaskRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type TransferApprovalTaskBuilder = TransferApprovalTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_tasks/{id}/transfer → 官方形状（data 为空对象）。
    #[tokio::test]
    async fn test_transfer_approval_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/approval_tasks/task_001/transfer"))
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

        let _resp = TransferApprovalTaskRequestBuilder::new(config, "task_001")
            .user_id("1783981209205788")
            .from_user_ids(vec!["1783981209205788"])
            .to_user_ids(vec!["1783981209205789"])
            .opinion("转交")
            .execute()
            .await
            .expect("转交人工任务应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_tasks/task_001/transfer"
        );
        // 请求体须与官方文档一致：user_id + from_user_ids + to_user_ids + opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "user_id": "1783981209205788",
                "from_user_ids": ["1783981209205788"],
                "to_user_ids": ["1783981209205789"],
                "opinion": "转交"
            })
        );
    }
}
