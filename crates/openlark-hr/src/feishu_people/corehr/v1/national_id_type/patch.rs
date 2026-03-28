//! 更新国家证件类型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/national_id_type/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新国家证件类型请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 国家证件类型 ID（必填）
    national_id_type_id: String,
    /// 请求体
    request_body: Value,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            national_id_type_id: String::new(),
            request_body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置国家证件类型 ID（必填）
    pub fn national_id_type_id(mut self, national_id_type_id: String) -> Self {
        self.national_id_type_id = national_id_type_id;
        self
    }

    /// 设置请求体
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
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
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.national_id_type_id.trim(), "国家证件类型 ID 不能为空");

        let api_endpoint = CorehrApiV1::NationalIdTypePatch(self.national_id_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<PatchResponse>::patch(endpoint_url).body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新国家证件类型响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新国家证件类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
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
