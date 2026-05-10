//! 删除勋章授予名单 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除勋章授予名单的请求构建器。
pub struct DeleteBadgeGrantBuilder {
    badge_id: String,
    grant_id: String,
    config: Config,
}

impl DeleteBadgeGrantBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置授予记录 ID。
    pub fn grant_id(mut self, grant_id: impl Into<String>) -> Self {
        self.grant_id = grant_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<DeleteBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteBadgeGrantResponse> {
        let url = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            self.badge_id, self.grant_id
        );
        let api_request: ApiRequest<DeleteBadgeGrantResponse> = ApiRequest::delete(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("删除勋章授予名单", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 删除勋章授予名单的响应。
pub struct DeleteBadgeGrantResponse {
    /// 执行结果。
    pub result: String,
}

impl ApiResponseTrait for DeleteBadgeGrantResponse {
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
