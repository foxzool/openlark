//! 编辑消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
};

/// 编辑消息请求体（仅支持 text/post）
///
/// 表示编辑消息所需的请求参数。
///
/// # 字段说明
///
/// - `msg_type`: 消息类型，仅支持 text 或 post
/// - `content`: 消息内容，JSON 字符串格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessageBody {
    pub msg_type: String,
    pub content: String,
}

/// 编辑消息请求
///
/// 用于编辑已发送的消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待编辑的消息 ID，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{UpdateMessageRequest, UpdateMessageBody};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = UpdateMessageBody {
///     msg_type: "text".to_string(),
///     content: r#"{"text":"Updated content"}"#.to_string(),
/// };
/// let request = UpdateMessageRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute(body).await?;
/// ```
pub struct UpdateMessageRequest {
    config: Config,
    message_id: String,
}

impl UpdateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待编辑的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/update>
    pub async fn execute(self, body: UpdateMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");

        // url: PUT:/open-apis/im/v1/messages/:message_id
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::put(format!("{}/{}", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "编辑消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "编辑消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_update_message_request_builder() {
        let config = Config::default();
        let request = UpdateMessageRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
    }

    #[test]
    fn test_update_message_body() {
        let body = UpdateMessageBody {
            msg_type: "text".to_string(),
            content: r#"{"text":"Updated"}"#.to_string(),
        };
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Updated"}"#);
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = UpdateMessageRequest::new(config);
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_empty_msg_type() {
        let body = UpdateMessageBody {
            msg_type: "".to_string(),
            content: r#"{"text":"test"}"#.to_string(),
        };
        assert_eq!(body.msg_type, "");
    }

    #[test]
    fn test_empty_content() {
        let body = UpdateMessageBody {
            msg_type: "text".to_string(),
            content: "".to_string(),
        };
        assert_eq!(body.content, "");
    }

    #[test]
    fn test_post_message_type() {
        let body = UpdateMessageBody {
            msg_type: "post".to_string(),
            content: r#"{"post":{"zh_cn":{"title":"Title","content":[[{"tag":"text","text":"Content"}]]}}}"#.to_string(),
        };
        assert_eq!(body.msg_type, "post");
    }
}
