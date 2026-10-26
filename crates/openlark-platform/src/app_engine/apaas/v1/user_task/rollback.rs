//! 退回人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 退回人工任务 Builder
#[derive(Debug, Clone)]
pub struct RollbackTaskRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 请求体
    body: RollbackTaskBody,
}

impl RollbackTaskRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: RollbackTaskBody {
                operator_user_id: String::new(),
                to_task_id: String::new(),
                opinion: String::new(),
            },
        }
    }

    /// 设置操作人 kunlunUserID
    pub fn operator_user_id(mut self, operator_user_id: impl Into<String>) -> Self {
        self.body.operator_user_id = operator_user_id.into();
        self
    }

    /// 设置退回到的任务 ID（可通过「查询人工任务可退回的位置」获取）
    pub fn to_task_id(mut self, to_task_id: impl Into<String>) -> Self {
        self.body.to_task_id = to_task_id.into();
        self
    }

    /// 设置退回原因
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = opinion.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RollbackTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RollbackTaskResponse> {
        validate_required!(self.body.operator_user_id, "操作人ID不能为空");
        validate_required!(self.body.to_task_id, "退回到的任务ID不能为空");
        validate_required!(self.body.opinion, "退回原因不能为空");

        let url = format!("/open-apis/apaas/v1/user_tasks/{}/rollback", self.task_id);

        let req: ApiRequest<RollbackTaskResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 退回请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackTaskBody {
    /// 操作人 kunlunUserID，可通过 aPaaS 用户管理页面获取
    #[serde(rename = "operator_user_id")]
    pub operator_user_id: String,
    /// 退回到的任务 ID，可通过「查询人工任务可退回的位置」获取
    #[serde(rename = "to_task_id")]
    pub to_task_id: String,
    /// 退回原因
    #[serde(rename = "opinion")]
    pub opinion: String,
}

/// 退回响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackTaskResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// 退回结果
    #[serde(rename = "result")]
    pub result: bool,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for RollbackTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RollbackTaskRequestBuilder, will be removed in v1.0 (#271)")]
pub type RollbackTaskBuilder = RollbackTaskRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_tasks/{id}/rollback → 强类型 RollbackTaskResponse。
    #[tokio::test]
    async fn test_rollback_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/user_tasks/task_001/rollback"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "task_id": "task_001",
                    "result": true,
                    "message": "退回成功"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RollbackTaskRequestBuilder::new(config, "task_001")
            .operator_user_id("u_001")
            .to_task_id("task_000")
            .opinion("信息需补充")
            .execute()
            .await
            .expect("退回人工任务应成功");
        assert_eq!(resp.task_id, "task_001");
        assert!(resp.result);
        assert_eq!(resp.message, "退回成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_tasks/task_001/rollback"
        );
        // 请求体须与官方文档一致：operator_user_id + to_task_id + opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "operator_user_id": "u_001",
                "to_task_id": "task_000",
                "opinion": "信息需补充"
            })
        );
    }
}
