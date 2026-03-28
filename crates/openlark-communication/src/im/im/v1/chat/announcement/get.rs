//! 获取群公告信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-announcement/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::{
        chat::announcement::models::GetChatAnnouncementResponse, message::models::UserIdType,
    },
};

/// 获取群公告信息请求
///
/// 用于获取指定群聊的公告信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `chat_id`: 群 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetChatAnnouncementRequest::new(config)
///     .chat_id("oc_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct GetChatAnnouncementRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl GetChatAnnouncementRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<GetChatAnnouncementResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetChatAnnouncementResponse> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        let mut req: ApiRequest<GetChatAnnouncementResponse> =
            ApiRequest::get(format!("{}/{}/announcement", IM_V1_CHATS, self.chat_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取群公告信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chat_announcement_request_builder() {
        let config = Config::default();
        let request = GetChatAnnouncementRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
    }

    #[test]
    fn test_get_chat_announcement_request_with_user_id_type() {
        let config = Config::default();
        let request = GetChatAnnouncementRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_get_chat_announcement_request_default_values() {
        let config = Config::default();
        let request = GetChatAnnouncementRequest::new(config);
        assert_eq!(request.chat_id, "");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_chat_announcement_request_with_all_options() {
        let config = Config::default();
        let request = GetChatAnnouncementRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.chat_id, "oc_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}
