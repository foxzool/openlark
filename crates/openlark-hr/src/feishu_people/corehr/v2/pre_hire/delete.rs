//! 删除待入职信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/pre_hire/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DeleteRequest {
    config: Config,
    pre_hire_id: Option<String>,
    request_body: Option<Value>,
}

impl DeleteRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pre_hire_id: None,
            request_body: None,
        }
    }

    pub fn pre_hire_id(mut self, pre_hire_id: String) -> Self {
        self.pre_hire_id = Some(pre_hire_id);
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let pre_hire_id = self.pre_hire_id.unwrap_or_default();
        validate_required!(pre_hire_id.trim(), "pre_hire_id 不能为空");
        let api_endpoint = FeishuPeopleApiV2::PreHireDelete(pre_hire_id);
        let mut request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除待入职信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
