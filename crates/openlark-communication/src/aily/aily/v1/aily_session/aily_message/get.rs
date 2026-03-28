//! 获取 Aily 消息
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session-aily_message/get>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_MESSAGES};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 获取 Aily 消息请求
///
/// 用于获取指定 Aily 消息的详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
/// - `aily_message_id`: 消息 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetMessageRequest::new(config)
///     .aily_session_id("session_xxx")
///     .aily_message_id("message_xxx")
///     .execute().await?;
/// ```
pub struct GetMessageRequest {
    config: Config,
    aily_session_id: String,
    aily_message_id: String,
}

impl GetMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            aily_session_id: String::new(),
            aily_message_id: String::new(),
        }
    }

    /// 会话 ID（路径参数）
    pub fn aily_session_id(mut self, aily_session_id: impl Into<String>) -> Self {
        self.aily_session_id = aily_session_id.into();
        self
    }

    /// 消息 ID（路径参数）
    pub fn aily_message_id(mut self, aily_message_id: impl Into<String>) -> Self {
        self.aily_message_id = aily_message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session-aily_message/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");
        validate_required!(self.aily_message_id, "aily_message_id 不能为空");

        let url = AILY_V1_MESSAGES.replace("{aily_session_id}", &self.aily_session_id)
            + "/"
            + &self.aily_message_id;
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(&url);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取 Aily 消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_request_builder() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .aily_session_id("session_xxx")
            .aily_message_id("message_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
        assert_eq!(request.aily_message_id, "message_xxx");
    }

    #[test]
    fn test_get_message_request_default_values() {
        let config = Config::default();
        let request = GetMessageRequest::new(config);
        assert_eq!(request.aily_session_id, "");
        assert_eq!(request.aily_message_id, "");
    }

    #[test]
    fn test_get_message_request_with_session_id_only() {
        let config = Config::default();
        let request = GetMessageRequest::new(config).aily_session_id("session_123");
        assert_eq!(request.aily_session_id, "session_123");
        assert_eq!(request.aily_message_id, "");
    }

    #[test]
    fn test_get_message_request_url_construction() {
        let request = GetMessageRequest::new(Config::default())
            .aily_session_id("sess_1")
            .aily_message_id("msg_1");
        assert_eq!(request.aily_session_id, "sess_1");
        assert_eq!(request.aily_message_id, "msg_1");
    }

    #[test]
    fn test_get_message_request_chaining() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .aily_session_id("session_xxx")
            .aily_message_id("message_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
        assert_eq!(request.aily_message_id, "message_xxx");
    }
}
