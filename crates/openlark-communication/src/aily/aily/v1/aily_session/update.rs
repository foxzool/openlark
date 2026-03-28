//! 更新会话
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session/update>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_SESSION};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新会话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateSessionBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// 更新会话请求
///
/// 用于更新指定 Aily AI 会话的信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
///
/// # 请求体字段
///
/// - `name`: 会话名称（可选）
/// - `description`: 会话描述（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = UpdateSessionBody::new()
///     .name("新名称")
///     .description("新描述");
/// let request = UpdateSessionRequest::new(config)
///     .aily_session_id("session_xxx")
///     .execute(body).await?;
/// ```
pub struct UpdateSessionRequest {
    config: Config,
    aily_session_id: String,
}

impl UpdateSessionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
        }
    }

    /// 会话 ID（路径参数）
    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session/update>
    pub async fn execute(self, body: UpdateSessionBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_SESSION.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<UpdateSessionBody> = ApiRequest::put(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新会话")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_update_session_request_builder() {
        let config = Config::default();
        let request = UpdateSessionRequest::new(config).aily_session_id("session_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
    }

    #[test]
    fn test_update_session_body_builder() {
        let body = UpdateSessionBody::new()
            .name("新名称")
            .description("新描述");
        assert_eq!(body.name, Some("新名称".to_string()));
        assert_eq!(body.description, Some("新描述".to_string()));
    }

    #[test]
    fn test_update_session_body_default() {
        let body = UpdateSessionBody::new();
        assert_eq!(body.name, None);
        assert_eq!(body.description, None);
    }

    #[test]
    fn test_update_session_request_default_values() {
        let config = Config::default();
        let request = UpdateSessionRequest::new(config);
        assert_eq!(request.aily_session_id, "");
    }

    #[test]
    fn test_update_session_body_with_name_only() {
        let body = UpdateSessionBody::new().name("仅名称");
        assert_eq!(body.name, Some("仅名称".to_string()));
        assert_eq!(body.description, None);
    }
}
