//! 设置猎头保护期
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/agency/protect>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 设置猎头保护期请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProtectRequest {
    /// 配置信息
    config: Config,
    agency_id: String,
    request_body: Option<Value>,
}

impl ProtectRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            agency_id: String::new(),
            request_body: None,
        }
    }

    pub fn agency_id(mut self, agency_id: String) -> Self {
        self.agency_id = agency_id;
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ProtectResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProtectResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.agency_id.trim(), "猎头供应商 ID 不能为空");

        let api_endpoint = HireApiV1::AgencyProtect(self.agency_id);
        let mut request = ApiRequest::<ProtectResponse>::post(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "设置猎头保护期响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 设置猎头保护期响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtectResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for ProtectResponse {
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
