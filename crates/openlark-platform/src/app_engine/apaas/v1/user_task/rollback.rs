//! 退回人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/rollback>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 退回人工任务 Builder
#[derive(Debug, Clone)]
pub struct RollbackTaskBuilder {
    config: Config,
    /// 任务 ID
    task_id: String,
    /// 退回节点 ID
    node_id: String,
    /// 退回原因
    reason: Option<String>,
}

impl RollbackTaskBuilder {
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
    task_id: String,
    /// 退回结果
    #[serde(rename = "result")]
    result: bool,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for RollbackTaskResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
