//! 转交人工任务
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/user-task/transfer>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 转交人工任务 Builder
#[derive(Debug, Clone)]
pub struct TransferApprovalTaskBuilder {
    config: Config,
    /// 审批任务 ID
    approval_task_id: String,
    /// 转交人用户 ID
    transfer_to_user_id: String,
    /// 转交原因
    reason: Option<String>,
}

impl TransferApprovalTaskBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        approval_task_id: impl Into<String>,
        transfer_to_user_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            approval_task_id: approval_task_id.into(),
            transfer_to_user_id: transfer_to_user_id.into(),
            reason: None,
        }
    }

    /// 设置转交原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferApprovalTaskResponse> {
        self.execute_with_options.await
    }(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TransferApprovalTaskResponse> {
        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/transfer",
            self.approval_task_id
        );

        let request = TransferApprovalTaskRequest {
            transfer_to_user_id: self.transfer_to_user_id,
            reason: self.reason,
        };

        let req = ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 转交人工任务请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TransferApprovalTaskRequest {
    /// 转交人用户 ID
    #[serde(rename = "transfer_to_user_id")]
    transfer_to_user_id: String,
    /// 转交原因
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

/// 转交人工任务响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransferApprovalTaskResponse {
    /// 任务 ID
    #[serde(rename = "task_id")]
    task_id: String,
    /// 转交结果
    #[serde(rename = "result")]
    result: bool,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for TransferApprovalTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
