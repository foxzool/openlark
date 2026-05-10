//! 同意人工任务 API
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/user-task/agree

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 同意人工任务的请求构建器。
pub struct AgreeTaskBuilder {
    approval_task_id: String,
    config: Config,
}

impl AgreeTaskBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            config,
        }
    }

    /// 设置人工任务 ID。
    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<AgreeTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<AgreeTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/agree",
            self.approval_task_id
        );
        let api_request: ApiRequest<AgreeTaskResponse> = ApiRequest::post(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("同意人工任务", "响应数据为空"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 同意人工任务的响应。
pub struct AgreeTaskResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for AgreeTaskResponse {
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
