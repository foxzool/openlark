//! 回复消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/reply>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

/// 回复消息请求体
///
/// 表示回复消息所需的请求参数。
///
/// # 字段说明
///
/// - `content`: 消息内容（JSON 字符串）
/// - `msg_type`: 消息类型
/// - `reply_in_thread`: 是否以话题形式回复（可选，默认 false）
/// - `uuid`: 幂等 uuid（可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMessageBody {
    /// 消息内容（JSON 字符串）
    pub content: String,
    /// 消息类型
    pub msg_type: String,
    /// 是否以话题形式回复（可选，默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_in_thread: Option<bool>,
    /// 幂等 uuid（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 回复消息请求
///
/// 用于回复指定消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待回复的消息 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{ReplyMessageRequest, ReplyMessageBody};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = ReplyMessageBody {
///     content: r#"{"text":"Reply"}"#.to_string(),
///     msg_type: "text".to_string(),
///     reply_in_thread: Some(false),
///     uuid: None,
/// };
/// let request = ReplyMessageRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute(body).await?;
/// ```
pub struct ReplyMessageRequest {
    config: Config,
    message_id: String,
}

impl ReplyMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待回复的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/reply>
    pub async fn execute(self, body: ReplyMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ReplyMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");

        // url: POST:/open-apis/im/v1/messages/:message_id/reply
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/reply", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "回复消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "回复消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_reply_message_request_builder() {
        let config = Config::default();
        let request = ReplyMessageRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
    }

    #[test]
    fn test_reply_message_body() {
        let body = ReplyMessageBody {
            content: r#"{"text":"Reply"}"#.to_string(),
            msg_type: "text".to_string(),
            reply_in_thread: Some(false),
            uuid: Some("uuid-123".to_string()),
        };
        assert_eq!(body.content, r#"{"text":"Reply"}"#);
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.reply_in_thread, Some(false));
        assert_eq!(body.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_reply_message_body_without_optional_fields() {
        let body = ReplyMessageBody {
            content: r#"{"text":"Reply"}"#.to_string(),
            msg_type: "text".to_string(),
            reply_in_thread: None,
            uuid: None,
        };
        assert_eq!(body.reply_in_thread, None);
        assert_eq!(body.uuid, None);
    }

    #[test]
    fn test_reply_message_body_with_thread() {
        let body = ReplyMessageBody {
            content: r#"{"text":"Thread reply"}"#.to_string(),
            msg_type: "text".to_string(),
            reply_in_thread: Some(true),
            uuid: None,
        };
        assert_eq!(body.reply_in_thread, Some(true));
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = ReplyMessageRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_empty_msg_type() {
        let body = ReplyMessageBody {
            content: r#"{"text":"Reply"}"#.to_string(),
            msg_type: "".to_string(),
            reply_in_thread: None,
            uuid: None,
        };
        assert_eq!(body.msg_type, "");
    }
}
