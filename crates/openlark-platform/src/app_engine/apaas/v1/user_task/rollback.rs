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
};
use serde::{Deserialize, Serialize};

/// 退回人工任务 Builder
#[derive(Debug, Clone)]
pub struct RollbackTaskRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 退回节点 ID
    node_id: String,
    /// 退回原因
    reason: Option<String>,
}

impl RollbackTaskRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>, node_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            node_id: node_id.into(),
            reason: None,
        }
    }

    /// 设置退回原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
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
        let url = format!("/open-apis/apaas/v1/user_tasks/{}/rollback", self.task_id);

        let request = RollbackTaskRequest {
            node_id: self.node_id,
            reason: self.reason,
        };

        let req: ApiRequest<RollbackTaskResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 退回请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RollbackTaskRequest {
    /// 退回节点 ID
    #[serde(rename = "node_id")]
    node_id: String,
    /// 退回原因
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
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

        let resp = RollbackTaskRequestBuilder::new(config, "task_001", "node_1")
            .reason("信息需补充")
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
    }
}
