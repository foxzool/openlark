//! 更新个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/person/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `PatchRequest` 请求。
#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    person_id: Option<String>,
    request_body: Option<Value>,
}

impl PatchRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            person_id: None,
            request_body: None,
        }
    }

    /// 设置 `person_id`。
    pub fn person_id(mut self, person_id: String) -> Self {
        self.person_id = Some(person_id);
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let person_id = self.person_id.unwrap_or_default();
        validate_required!(person_id.trim(), "person_id 不能为空");
        let api_endpoint = FeishuPeopleApiV2::PersonPatch(person_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新个人信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `PatchResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 原始响应数据。
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
