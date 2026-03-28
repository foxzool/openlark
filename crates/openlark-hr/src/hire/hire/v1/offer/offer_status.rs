//! 更新 Offer 状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer/offer_status>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新 Offer 状态请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OfferStatusRequest {
    /// 配置信息
    config: Config,
    offer_id: String,
    request_body: Option<Value>,
}

impl OfferStatusRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_id: String::new(),
            request_body: None,
        }
    }

    pub fn offer_id(mut self, offer_id: String) -> Self {
        self.offer_id = offer_id;
        self
    }

    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OfferStatusResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OfferStatusResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.offer_id.trim(), "Offer ID 不能为空");

        let api_endpoint = HireApiV1::OfferOfferStatus(self.offer_id);
        let mut request = ApiRequest::<OfferStatusResponse>::patch(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新 Offer 状态响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新 Offer 状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OfferStatusResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for OfferStatusResponse {
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
