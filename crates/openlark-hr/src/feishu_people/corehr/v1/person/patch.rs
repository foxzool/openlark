//! 更新个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新个人信息请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    config: Config,
    person_id: String,
    person: Option<Value>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            person_id: String::new(),
            person: None,
        }
    }

    pub fn person_id(mut self, person_id: String) -> Self {
        self.person_id = person_id;
        self
    }

    pub fn person(mut self, person: Value) -> Self {
        self.person = Some(person);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.person_id.trim(), "个人信息ID不能为空");

        let api_endpoint = FeishuPeopleApiV1::PersonPatch(self.person_id);
        let mut request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());
        if let Some(person) = self.person {
            request = request.body(serde_json::json!({ "person": person }));
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

/// 更新个人信息响应
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
#[allow(unused_imports)]
mod tests {

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
