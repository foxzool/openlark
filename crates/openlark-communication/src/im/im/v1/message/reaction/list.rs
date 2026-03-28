//! 获取消息表情回复
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_MESSAGES,
    im::im::v1::{
        message::models::UserIdType, message::reaction::models::ListMessageReactionsResponse,
    },
};

/// 获取消息表情回复请求
///
/// 用于获取指定消息的表情回复列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
/// - `reaction_type`: 待查询的表情类型，可选
/// - `page_token`: 分页标记，可选
/// - `page_size`: 分页大小，可选，默认 20，最大 50
/// - `user_id_type`: 用户 ID 类型，可选，默认 open_id
///
/// # 示例
///
/// ```rust,ignore
/// let request = ListMessageReactionsRequest::new(config)
///     .message_id("msg_xxx")
///     .page_size(50)
///     .execute().await?;
/// ```
pub struct ListMessageReactionsRequest {
    config: Config,
    message_id: String,
    reaction_type: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
    user_id_type: Option<UserIdType>,
}

impl ListMessageReactionsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            reaction_type: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 待查询的表情类型（查询参数，可选，对应 emoji_type）
    pub fn reaction_type(mut self, reaction_type: impl Into<String>) -> Self {
        self.reaction_type = Some(reaction_type.into());
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，最大 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-reaction/list>
    pub async fn execute(self) -> SDKResult<ListMessageReactionsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListMessageReactionsResponse> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");

        // url: GET:/open-apis/im/v1/messages/:message_id/reactions
        let mut req: ApiRequest<ListMessageReactionsResponse> =
            ApiRequest::get(format!("{}/{}/reactions", IM_V1_MESSAGES, self.message_id));

        if let Some(reaction_type) = self.reaction_type {
            req = req.query("reaction_type", reaction_type);
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取消息表情回复")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_message_reactions_request_builder() {
        let config = Config::default();
        let request = ListMessageReactionsRequest::new(config).message_id("msg_xxx");
        assert_eq!(request.message_id, "msg_xxx");
    }

    #[test]
    fn test_list_message_reactions_request_default_values() {
        let config = Config::default();
        let request = ListMessageReactionsRequest::new(config);
        assert_eq!(request.message_id, "");
        assert!(request.reaction_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_list_message_reactions_request_with_optional_params() {
        let config = Config::default();
        let request = ListMessageReactionsRequest::new(config)
            .message_id("msg_xxx")
            .reaction_type("smile")
            .page_size(50)
            .page_token("token123")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.reaction_type, Some("smile".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_list_message_reactions_request_chaining() {
        let config = Config::default();
        let request = ListMessageReactionsRequest::new(config)
            .message_id("msg_xxx")
            .page_size(30);
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.page_size, Some(30));
    }
}
