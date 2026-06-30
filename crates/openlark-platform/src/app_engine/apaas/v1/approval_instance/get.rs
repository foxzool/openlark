//! 获取人工任务详情 API

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取审批实例详情的请求构建器。
pub struct GetInstanceRequestBuilder {
    approval_instance_id: String,
    config: Config,
}

impl GetInstanceRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_instance_id: String::new(),
            config,
        }
    }

    /// 设置审批实例 ID。
    pub fn approval_instance_id(mut self, approval_instance_id: impl Into<String>) -> Self {
        self.approval_instance_id = approval_instance_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetInstanceResponse> {
        validate_required!(self.approval_instance_id, "实例ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}",
            self.approval_instance_id
        );
        let api_request: ApiRequest<GetInstanceResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取人工任务详情", "响应数据为空")
        })
    }
}

/// 审批实例详情响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInstanceResponse {
    /// 审批实例 ID。
    pub instance_id: String,
    /// 审批实例状态。
    pub status: String,
    /// 发起人 ID。
    pub initiator_id: String,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for GetInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to GetInstanceRequestBuilder, will be removed in v1.0 (#271)")]
pub type GetInstanceBuilder = GetInstanceRequestBuilder;

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
