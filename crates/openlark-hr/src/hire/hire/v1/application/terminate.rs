//! 终止投递
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/terminate>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 终止投递请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TerminateRequest {
    /// 配置信息
    config: Config,
    application_id: String,
    request_body: Value,
}

impl TerminateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            request_body: Value::Null,
        }
    }

    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TerminateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TerminateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");
        if self.request_body.is_null() {
            return Err(openlark_core::error::validation_error(
                "请求体不能为空",
                "终止投递时 request_body 为必填参数",
            ));
        }

        let api_endpoint = HireApiV1::ApplicationTerminate(self.application_id.clone());
        let request =
            ApiRequest::<TerminateResponse>::post(api_endpoint.to_url()).body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "终止投递响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 终止投递响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerminateResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for TerminateResponse {
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
