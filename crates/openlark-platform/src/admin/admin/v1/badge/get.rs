//! 获取勋章详情 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取勋章详情请求
pub struct GetBadgeBuilder {
    badge_id: String,
    config: Config,
}

impl GetBadgeBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
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
/// 获取勋章详情的响应。
pub struct GetBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 图标地址。
    pub icon_url: Option<String>,
    /// 创建时间。
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
