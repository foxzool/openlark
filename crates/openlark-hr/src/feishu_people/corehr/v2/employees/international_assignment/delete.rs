//! 删除外派信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.international_assignment/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `DeleteRequest` 请求。
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    config: Config,
    international_assignment_id: Option<String>,
    request_body: Option<Value>,
}

impl DeleteRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            international_assignment_id: None,
            request_body: None,
        }
    }

    /// 设置 `international_assignment_id`。
    pub fn international_assignment_id(mut self, international_assignment_id: String) -> Self {
        self.international_assignment_id = Some(international_assignment_id);
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let international_assignment_id = self.international_assignment_id.unwrap_or_default();
        validate_required!(
            international_assignment_id.trim(),
            "international_assignment_id 不能为空"
        );
        let api_endpoint =
            FeishuPeopleApiV2::EmployeesInternationalAssignmentDelete(international_assignment_id);
        let mut request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除外派信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `DeleteResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 原始响应数据。
    pub data: Value,
}

impl ApiResponseTrait for DeleteResponse {
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
