//! 删除消息表情回复
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_MESSAGES,
    im::im::v1::message::reaction::models::MessageReaction,
};

/// 删除消息表情回复请求
///
/// 用于删除指定消息的表情回复。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
/// - `reaction_id`: 表情回复 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeleteMessageReactionRequest::new(config)
///     .message_id("msg_xxx")
///     .reaction_id("reaction_xxx")
///     .execute().await?;
/// ```
pub struct DeleteMessageReactionRequest {
    config: Config,
    message_id: String,
    reaction_id: String,
}

impl DeleteMessageReactionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            reaction_id: String::new(),
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 表情回复 ID（路径参数）
    pub fn reaction_id(mut self, reaction_id: impl Into<String>) -> Self {
        self.reaction_id = reaction_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/delete>
    pub async fn execute(self) -> SDKResult<MessageReaction> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MessageReaction> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(self.reaction_id, "reaction_id 不能为空");

        // url: DELETE:/open-apis/im/v1/messages/:message_id/reactions/:reaction_id
        let req: ApiRequest<MessageReaction> = ApiRequest::delete(format!(
            "{}/{}/reactions/{}",
            IM_V1_MESSAGES, self.message_id, self.reaction_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除消息表情回复")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_message_reaction_request_builder() {
        let config = Config::default();
        let request = DeleteMessageReactionRequest::new(config)
            .message_id("msg_xxx")
            .reaction_id("reaction_xxx");
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.reaction_id, "reaction_xxx");
    }

    #[test]
    fn test_delete_message_reaction_request_default_values() {
        let config = Config::default();
        let request = DeleteMessageReactionRequest::new(config);
        assert_eq!(request.message_id, "");
        assert_eq!(request.reaction_id, "");
    }

    #[test]
    fn test_delete_message_reaction_request_chaining() {
        let config = Config::default();
        let request = DeleteMessageReactionRequest::new(config)
            .message_id("msg_xxx")
            .reaction_id("reaction_xxx");
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.reaction_id, "reaction_xxx");
    }
}
