//! 创建会话
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session/create>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};
use serde::{Deserialize, Serialize};

use crate::endpoints::AILY_V1_SESSIONS;

/// 创建会话请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSessionBody {
    /// 渠道上下文，可自行构造的 Context 上下文变量，长度范围 0~255 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_context: Option<String>,
    /// 其他透传信息，会话的自定义变量内容，长度范围 0~255 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl CreateSessionBody {
    /// 创建新的会话请求体。
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

/// 创建会话请求
///
/// 用于创建新的 Aily AI 会话。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 请求体字段
///
/// - `channel_context`: 渠道上下文（可选，0~255 字符）
/// - `metadata`: 其他透传信息（可选，0~255 字符）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateSessionBody::new()
///     .channel_context("{}")
///     .metadata("{}");
/// let request = CreateSessionRequest::new(config)
///     .execute(body).await?;
/// ```
pub struct CreateSessionRequest {
    config: Config,
}

impl CreateSessionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session/create>
    pub async fn execute(self, body: CreateSessionBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateSessionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<CreateSessionBody> =
            ApiRequest::post(AILY_V1_SESSIONS).json_body(&body);

        Transport::request_typed(req, &self.config, Some(option), "创建会话").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session_body_builder() {
        let body = CreateSessionBody::new().channel_context("{}");
        assert_eq!(body.channel_context, Some("{}".to_string()));
        assert_eq!(body.metadata, None);
    }

    #[test]
    fn test_create_session_body_with_metadata() {
        let body = CreateSessionBody::new().metadata("{}");
        assert_eq!(body.metadata, Some("{}".to_string()));
    }

    #[test]
    fn test_create_session_request_builder() {
        let config = Config::default();
        let request = CreateSessionRequest::new(config);
        assert_eq!(request.config.app_id(), "");
    }

    #[test]
    fn test_create_session_body_default() {
        let body = CreateSessionBody::new();
        assert!(body.channel_context.is_none());
        assert!(body.metadata.is_none());
    }

    #[test]
    fn test_create_session_body_with_all_fields() {
        let body = CreateSessionBody::new()
            .channel_context("{}")
            .metadata("{}");
        assert_eq!(body.channel_context, Some("{}".to_string()));
        assert_eq!(body.metadata, Some("{}".to_string()));
    }

    #[test]
    fn test_create_session_body_serialization() {
        // 两个官方字段均为可选：未设置时请求体序列化为空对象
        let body = CreateSessionBody::new();
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn test_create_session_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_SESSIONS;
        let url = AILY_V1_SESSIONS.to_string();
        assert_eq!(url, "/open-apis/aily/v1/sessions");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
