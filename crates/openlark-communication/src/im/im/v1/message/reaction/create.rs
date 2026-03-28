//! 添加消息表情回复
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::reaction::models::{CreateMessageReactionBody, MessageReaction},
};

/// 添加消息表情回复请求
///
/// 用于为指定消息添加表情回复（emoji reaction）。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateMessageReactionBody::new(...);
/// let request = CreateMessageReactionRequest::new(config)
///     .message_id("msg_xxx")
///     .execute(body).await?;
/// ```
pub struct CreateMessageReactionRequest {
    config: Config,
    message_id: String,
}

impl CreateMessageReactionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create>
    pub async fn execute(self, body: CreateMessageReactionBody) -> SDKResult<MessageReaction> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateMessageReactionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MessageReaction> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(
            body.reaction_type.emoji_type,
            "reaction_type.emoji_type 不能为空"
        );

        // url: POST:/open-apis/im/v1/messages/:message_id/reactions
        let req: ApiRequest<MessageReaction> =
            ApiRequest::post(format!("{}/{}/reactions", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "添加消息表情回复")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "添加消息表情回复")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message_reaction_request_builder() {
        let config = Config::default();
        let request = CreateMessageReactionRequest::new(config).message_id("msg_xxx");
        assert_eq!(request.message_id, "msg_xxx");
    }

    #[test]
    fn test_create_message_reaction_request_default_values() {
        let config = Config::default();
        let request = CreateMessageReactionRequest::new(config);
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_create_message_reaction_request_with_different_ids() {
        let config = Config::default();
        let request1 = CreateMessageReactionRequest::new(config.clone()).message_id("msg_111");
        let request2 = CreateMessageReactionRequest::new(config).message_id("msg_222");
        assert_eq!(request1.message_id, "msg_111");
        assert_eq!(request2.message_id, "msg_222");
    }
}
