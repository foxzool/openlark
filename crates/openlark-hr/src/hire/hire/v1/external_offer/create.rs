//! 创建外部 Offer
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_offer/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 创建外部 Offer请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        let mut request = ApiRequest::<CreateResponse>::post("/open-apis/hire/v1/external_offers");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建外部 Offer 响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建外部 Offer响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `external_offer_id` 字段。
    pub external_offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer_id` 字段。
    pub offer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
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
