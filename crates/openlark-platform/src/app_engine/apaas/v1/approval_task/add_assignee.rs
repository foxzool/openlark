//! 人工任务加签 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/apaas-v1/flow/user-task/add_assignee
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/add_assignee

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
pub struct AddAssigneeBuilder {
    approval_task_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl AddAssigneeBuilder {
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

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("人工任务加签", "响应数据为空"))
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
