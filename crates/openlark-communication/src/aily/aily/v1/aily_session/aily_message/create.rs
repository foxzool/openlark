//! 创建 Aily 消息
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/aily_session-aily_message/create>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_MESSAGES};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 创建 Aily 消息请求
///
/// 用于在指定会话中创建新的 Aily 消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `aily_session_id`: 会话 ID，必填
///
/// # 请求体字段
///
/// 请求体为动态 JSON，根据具体需求传递
///
/// # 示例
///
/// ```rust,ignore
/// let body = serde_json::json!({
///     "content": "你好",
///     "role": "user"
/// });
/// let request = CreateAilyMessageRequest::new(config)
///     .aily_session_id("session_xxx")
///     .execute(body).await?;
/// ```
pub struct CreateAilyMessageRequest {
    config: Config,
    aily_session_id: String,
}

impl CreateAilyMessageRequest {
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
    /// docPath: <https://open.feishu.cn/document/aily-v1/aily_session-aily_message/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.aily_session_id, "aily_session_id 不能为空");

        let url = AILY_V1_MESSAGES.replace("{aily_session_id}", &self.aily_session_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url).json_body(&body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建 Aily 消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aily_message_request_builder() {
        let config = Config::default();
        let request = CreateAilyMessageRequest::new(config).aily_session_id("session_xxx");
        assert_eq!(request.aily_session_id, "session_xxx");
    }

    #[test]
    fn test_create_aily_message_request_default_values() {
        let config = Config::default();
        let request = CreateAilyMessageRequest::new(config);
        assert_eq!(request.aily_session_id, "");
    }

    #[test]
    fn test_create_aily_message_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = CreateAilyMessageRequest::new(config.clone()).aily_session_id("session_111");
        let request2 = CreateAilyMessageRequest::new(config).aily_session_id("session_222");
        assert_eq!(request1.aily_session_id, "session_111");
        assert_eq!(request2.aily_session_id, "session_222");
    }

    #[test]
    fn test_create_aily_message_json_body() {
        let body = serde_json::json!({
            "content": "测试消息",
            "role": "user"
        });
        assert_eq!(body["content"], "测试消息");
        assert_eq!(body["role"], "user");
    }
}
