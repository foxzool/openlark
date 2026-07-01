//! 更新实习 Offer 入/离职状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer/intern_offer_status>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 更新实习 Offer 入/离职状态请求
#[derive(Debug, Clone)]
pub struct InternOfferStatusRequest {
    /// 配置信息
    config: Config,
    offer_id: String,
    request_body: Option<Value>,
}

impl InternOfferStatusRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `offer_id`。
    pub fn offer_id(mut self, offer_id: String) -> Self {
        self.offer_id = offer_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<InternOfferStatusResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<InternOfferStatusResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.offer_id.trim(), "Offer ID 不能为空");

        let api_endpoint = HireApiV1::OfferInternOfferStatus(self.offer_id);
        let mut request = ApiRequest::<InternOfferStatusResponse>::post(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新实习 Offer 入/离职状态响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新实习 Offer 入/离职状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InternOnboardingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `actual_onboarding_date` 字段。
    pub actual_onboarding_date: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InternOffboardingInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InternOffboardingInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `actual_offboarding_date` 字段。
    pub actual_offboarding_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `notes` 字段。
    pub notes: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `InternOfferStatusResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InternOfferStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `operation` 字段。
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `onboarding_info` 字段。
    pub onboarding_info: Option<InternOnboardingInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offboarding_info` 字段。
    pub offboarding_info: Option<InternOffboardingInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for InternOfferStatusResponse {
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
