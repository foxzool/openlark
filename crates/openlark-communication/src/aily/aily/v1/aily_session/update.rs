//! 更新会话
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session/update>

use crate::endpoints::AILY_V1_SESSION;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新会话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSessionBody {
    /// 渠道上下文，可自行构造的 Context 上下文变量，长度范围 0~255 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<String>,
    /// 其他透传信息，会话的自定义变量内容，长度范围 0~255 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl UpdateSessionBody {
    /// 创建空的更新请求体。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置渠道上下文。
    pub fn channel_context(mut self, channel_context: impl Into<String>) -> Self {
        self.channel_context = Some(channel_context.into());
        self
    }

    /// 设置其他透传信息。
    pub fn metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
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
/// - `channel_context`: 渠道上下文（可选，0~255 字符）
/// - `metadata`: 其他透传信息（可选，0~255 字符）
///
/// # 示例
///
/// ```rust,ignore
/// let body = UpdateSessionBody::new()
///     .channel_context("{}")
///     .metadata("{}");
/// let request = UpdateSessionRequest::new(config)
///     .aily_session_id("session_xxx")
///     .execute(body).await?;
/// ```
pub struct UpdateSessionRequest {
    config: Config,
    aily_session_id: String,
}

impl UpdateSessionRequest {
    /// 创建新的请求构建器。
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

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_SESSION.replace("{session_id}", &self.aily_session_id);
        let req: ApiRequest<UpdateSessionBody> = ApiRequest::put(&url).json_body(&body);

        Transport::request_typed(req, &self.config, Some(option), "更新会话").await
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
            .channel_context("{}")
            .metadata("{}");
        assert_eq!(body.channel_context, Some("{}".to_string()));
        assert_eq!(body.metadata, Some("{}".to_string()));
    }

    #[test]
    fn test_update_session_body_default() {
        let body = UpdateSessionBody::new();
        assert_eq!(body.channel_context, None);
        assert_eq!(body.metadata, None);
    }

    #[test]
    fn test_update_session_request_default_values() {
        let config = Config::default();
        let request = UpdateSessionRequest::new(config);
        assert_eq!(request.aily_session_id, "");
    }

    #[test]
    fn test_update_session_body_with_channel_context_only() {
        let body = UpdateSessionBody::new().channel_context("{}");
        assert_eq!(body.channel_context, Some("{}".to_string()));
        assert_eq!(body.metadata, None);
    }

    #[test]
    fn test_update_session_body_serialization() {
        let body = UpdateSessionBody::new().metadata("{}");
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["metadata"], "{}");
        assert!(json.get("channel_context").is_none());
    }

    #[test]
    fn test_update_session_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_SESSION;
        let url = AILY_V1_SESSION.replace("{session_id}", "sess_1");
        assert_eq!(url, "/open-apis/aily/v1/sessions/sess_1");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
