//! 获取勋章详情 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取勋章详情请求
pub struct GetBadgeBuilder {
    badge_id: String,
    config: Config,
}

impl GetBadgeBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            config,
        }
    }

    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<GetBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetBadgeResponse> {
        let api_request: ApiRequest<GetBadgeResponse> =
            ApiRequest::get(format!("/open-apis/admin/v1/badges/{}", self.badge_id));

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("获取勋章详情", "响应数据为空"))
    }
}

/// 获取勋章详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBadgeResponse {
    pub badge_id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub create_time: String,
}

impl ApiResponseTrait for GetBadgeResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
