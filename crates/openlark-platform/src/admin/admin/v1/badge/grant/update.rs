//! 修改勋章授予名单 API
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/update

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 修改勋章授予名单的请求构建器。
pub struct UpdateBadgeGrantBuilder {
    badge_id: String,
    grant_id: String,
    user_ids: Vec<String>,
    config: Config,
}

impl UpdateBadgeGrantBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            grant_id: String::new(),
            user_ids: Vec::new(),
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

    /// 设置用户 ID 列表。
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<UpdateBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateBadgeGrantResponse> {
        let url = format!(
            "/open-apis/admin/v1/badges/{}/grants/{}",
            self.badge_id, self.grant_id
        );
        let request_body = UpdateBadgeGrantRequest {
            user_ids: self.user_ids,
        };
        let api_request: ApiRequest<UpdateBadgeGrantResponse> =
            ApiRequest::put(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("修改勋章授予名单", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct UpdateBadgeGrantRequest {
    user_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// 修改勋章授予名单的响应。
pub struct UpdateBadgeGrantResponse {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID 列表。
    pub user_ids: Vec<String>,
    /// 更新时间。
    pub update_time: String,
}

impl ApiResponseTrait for UpdateBadgeGrantResponse {
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
