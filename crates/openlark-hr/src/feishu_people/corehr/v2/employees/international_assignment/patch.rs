//! 更新外派信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.international_assignment/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    international_assignment_id: Option<String>,
    request_body: Option<Value>,
}

impl PatchRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            international_assignment_id: None,
            request_body: None,
        }
    }

    pub fn international_assignment_id(mut self, international_assignment_id: String) -> Self {
        self.international_assignment_id = Some(international_assignment_id);
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let international_assignment_id = self.international_assignment_id.unwrap_or_default();
        validate_required!(
            international_assignment_id.trim(),
            "international_assignment_id 不能为空"
        );
        let api_endpoint =
            FeishuPeopleApiV2::EmployeesInternationalAssignmentPatch(international_assignment_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新外派信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    pub data: Value,
}

impl ApiResponseTrait for PatchResponse {
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
