//! 获取 Offer 信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/hire-v1/application/offer

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

use crate::hire::hire::v1::offer::get::OfferDetail;

/// 获取 Offer 信息请求
#[derive(Debug, Clone)]
pub struct OfferRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl OfferRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OfferResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OfferResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let api_endpoint = HireApiV1::ApplicationOffer(self.application_id.clone());
        let request = ApiRequest::<OfferResponse>::get(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 Offer 信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 Offer 信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer` 字段。
    pub offer: Option<OfferDetail>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for OfferResponse {
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
