//! 获取群信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/get-2>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 获取群信息请求
///
/// 用于获取指定群聊的详细信息。
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
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::chat::GetChatRequest;
/// use openlark_communication::im::im::v1::message::models::UserIdType;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = GetChatRequest::new(config)
///     .chat_id("oc_xxx")
///     .user_id_type(UserIdType::OpenId);
/// let response = request.execute().await?;
/// ```
pub struct GetChatRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl GetChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            user_id_type: None,
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/get-2>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}", IM_V1_CHATS, self.chat_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取群信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chat_request_builder() {
        let config = Config::default();
        let request = GetChatRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
    }

    #[test]
    fn test_get_chat_request_with_user_id_type() {
        let config = Config::default();
        let request = GetChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_get_chat_request_with_union_id() {
        let config = Config::default();
        let request = GetChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_get_chat_request_with_user_id() {
        let config = Config::default();
        let request = GetChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_get_chat_request_default_values() {
        let config = Config::default();
        let request = GetChatRequest::new(config);
        assert_eq!(request.chat_id, "");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_get_chat_request_with_all_options() {
        let config = Config::default();
        let request = GetChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.chat_id, "oc_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }
}
