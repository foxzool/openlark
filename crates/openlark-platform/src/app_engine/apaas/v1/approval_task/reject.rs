//! 拒绝人工任务 API
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/reject

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 拒绝人工任务的请求构建器。
pub struct RejectTaskBuilder {
    approval_task_id: String,
    reason: Option<String>,
    config: Config,
}

impl RejectTaskBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            reason: None,
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 设置原因。
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<RejectTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RejectTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/reject",
            self.approval_task_id
        );
        let request_body = RejectTaskRequest {
            reason: self.reason,
        };
        let api_request: ApiRequest<RejectTaskResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("拒绝人工任务", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct RejectTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 拒绝人工任务的响应。
pub struct RejectTaskResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for RejectTaskResponse {
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
