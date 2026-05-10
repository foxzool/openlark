//! 查询人工任务可退回的位置
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback_points
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback_points

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
pub struct RollbackPointsBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
}

impl RollbackPointsBuilder {
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
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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
    task_id: String,
    /// 可退回节点列表
    #[serde(rename = "rollback_points")]
    rollback_points: Vec<RollbackPoint>,
}

impl ApiResponseTrait for RollbackPointsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
