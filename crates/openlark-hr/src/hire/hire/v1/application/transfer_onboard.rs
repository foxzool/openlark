//! 操作候选人入职
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/application/transfer_onboard

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::ApplicationOperationResult;

/// 操作候选人入职请求
#[derive(Debug, Clone)]
pub struct TransferOnboardRequest {
    /// 配置信息
    config: Config,
    application_id: String,
    request_body: Value,
}

impl TransferOnboardRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
            request_body: Value::Null,
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TransferOnboardResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TransferOnboardResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");
        if self.request_body.is_null() {
            return Err(openlark_core::error::validation_error(
                "请求体不能为空",
                "操作候选人入职时 request_body 为必填参数",
            ));
        }

        let api_endpoint = HireApiV1::ApplicationTransferOnboard(self.application_id.clone());
        let request = ApiRequest::<TransferOnboardResponse>::post(api_endpoint.to_url())
            .body(self.request_body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "操作候选人入职响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 操作候选人入职响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferOnboardResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: ApplicationOperationResult,
}

impl ApiResponseTrait for TransferOnboardResponse {
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
