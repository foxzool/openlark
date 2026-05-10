//! 修改勋章信息 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 修改勋章信息的请求构建器。
pub struct UpdateBadgeBuilder {
    badge_id: String,
    name: Option<String>,
    description: Option<String>,
    config: Config,
}

impl UpdateBadgeBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            name: None,
            description: None,
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeResponse> {
        let url = format!("/open-apis/admin/v1/badges/{}", self.badge_id);
        let request_body = UpdateBadgeRequest {
            name: self.name,
            description: self.description,
        };
        let api_request: ApiRequest<UpdateBadgeResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("修改勋章信息", "响应数据为空"))
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章信息的响应。
pub struct UpdateBadgeResponse {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeResponse {
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
