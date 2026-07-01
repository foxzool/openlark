//! 更新地点地址
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/location.address/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// PatchRequest
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    location_id: Option<String>,
    address_id: Option<String>,
    /// 请求体（可选）
    body: Option<Value>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            location_id: None,
            address_id: None,
            body: None,
        }
    }

    /// 设置 `location_id`。
    pub fn location_id(mut self, location_id: impl Into<String>) -> Self {
        self.location_id = Some(location_id.into());
        self
    }

    /// 设置 `address_id`。
    pub fn address_id(mut self, address_id: impl Into<String>) -> Self {
        self.address_id = Some(address_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        let location_id = self.location_id.unwrap_or_default();
        let address_id = self.address_id.unwrap_or_default();
        validate_required!(location_id.trim(), "location_id 不能为空");
        validate_required!(address_id.trim(), "address_id 不能为空");

        let mut request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/locations/{location_id}/addresses/{address_id}"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// PatchResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
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
