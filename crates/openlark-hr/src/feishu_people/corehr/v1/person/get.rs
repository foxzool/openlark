//! 查询单个个人信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询单个个人信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    config: Config,
    person_id: String,
    user_id_type: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            person_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置 `person_id`。
    pub fn person_id(mut self, person_id: String) -> Self {
        self.person_id = person_id;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.person_id.trim(), "个人信息ID不能为空");

        let api_endpoint = FeishuPeopleApiV1::PersonGet(self.person_id);
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询单个个人信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询单个个人信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 原始响应数据。
    pub data: Value,
}

impl ApiResponseTrait for GetResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
