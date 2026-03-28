//! 更新群信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/update-2>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 更新群信息请求
///
/// 用于更新群聊的基本信息，如群名称、群描述等。
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
/// use openlark_communication::im::im::v1::chat::UpdateChatRequest;
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = json!({
///     "name": "新的群名称",
///     "description": "新的群描述"
/// });
/// let request = UpdateChatRequest::new(config)
///     .chat_id("oc_xxx")
///     .user_id_type(UserIdType::OpenId);
/// let response = request.execute(body).await?;
/// ```
pub struct UpdateChatRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl UpdateChatRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/update-2>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: PUT:/open-apis/im/v1/chats/:chat_id
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::put(format!("{}/{}", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "更新群信息")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新群信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_update_chat_request_builder() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.chat_id, "oc_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_update_chat_request_without_user_id_type() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_empty_chat_id() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config);
        assert_eq!(request.chat_id, "");
    }

    #[test]
    fn test_update_chat_request_with_user_id() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_update_chat_request_with_union_id() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config)
            .chat_id("oc_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_update_chat_request_builder_chaining() {
        let config = Config::default();
        let request = UpdateChatRequest::new(config)
            .chat_id("oc_test_123")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.chat_id, "oc_test_123");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }
}
