//! 查询人工任务可退回的位置
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback_points>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback_points>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询可退回位置 Builder
#[derive(Debug, Clone)]
pub struct RollbackPointsRequestBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
}

impl RollbackPointsRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RollbackPointsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RollbackPointsResponse> {
        let url = format!(
            "/open-apis/apaas/v1/user_tasks/{}/rollback_points",
            self.task_id
        );

        let req: ApiRequest<RollbackPointsResponse> = ApiRequest::post(&url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 可退回节点信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackPoint {
    /// 节点 ID
    #[serde(rename = "node_id")]
    node_id: String,
    /// 节点名称
    #[serde(rename = "node_name")]
    node_name: String,
    /// 节点类型
    #[serde(rename = "node_type")]
    node_type: String,
    /// 是否可退回
    #[serde(rename = "can_rollback")]
    can_rollback: bool,
}

/// 可退回位置响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RollbackPointsResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    pub task_id: String,
    /// 可退回节点列表
    #[serde(rename = "rollback_points")]
    pub rollback_points: Vec<RollbackPoint>,
}

impl ApiResponseTrait for RollbackPointsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RollbackPointsRequestBuilder, will be removed in v1.0 (#271)")]
pub type RollbackPointsBuilder = RollbackPointsRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../user_tasks/{id}/rollback_points → 强类型 RollbackPointsResponse。
    #[tokio::test]
    async fn test_rollback_points_user_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/user_tasks/task_001/rollback_points",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "task_id": "task_001",
                    "rollback_points": [
                        {
                            "node_id": "node_1",
                            "node_name": "部门审批",
                            "node_type": "APPROVAL",
                            "can_rollback": true
                        },
                        {
                            "node_id": "node_2",
                            "node_name": "发起人",
                            "node_type": "START",
                            "can_rollback": false
                        }
                    ]
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

        let resp = RollbackPointsRequestBuilder::new(config, "task_001")
            .execute()
            .await
            .expect("查询可退回位置应成功");
        assert_eq!(resp.task_id, "task_001");
        assert_eq!(resp.rollback_points.len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/user_tasks/task_001/rollback_points"
        );
    }
}
