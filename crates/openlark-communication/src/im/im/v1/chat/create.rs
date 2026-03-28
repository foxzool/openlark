//! 创建群
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/create>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 创建群请求
///
/// 用于创建一个新的群聊。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
/// - `set_bot_manager`: 是否同时设置创建群的机器人为管理员（可选，默认 false）
/// - `uuid`: 幂等 uuid（可选）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::chat::CreateChatRequest;
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = json!({
///     "name": "群聊名称",
///     "user_id_list": ["ou_xxx", "ou_yyy"]
/// });
/// let request = CreateChatRequest::new(config)
///     .user_id_type(UserIdType::OpenId);
/// let response = request.execute(body).await?;
/// ```
pub struct CreateChatRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    set_bot_manager: Option<bool>,
    uuid: Option<String>,
}

impl CreateChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            set_bot_manager: None,
            uuid: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 是否同时设置创建群的机器人为管理员（查询参数，可选，默认 false）
    pub fn set_bot_manager(mut self, set_bot_manager: bool) -> Self {
        self.set_bot_manager = Some(set_bot_manager);
        self
    }

    /// 幂等 uuid（查询参数，可选）
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 执行请求
    ///
    /// 说明：创建群请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/create>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/im/v1/chats
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(IM_V1_CHATS).body(serialize_params(&body, "创建群")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(set_bot_manager) = self.set_bot_manager {
            req = req.query("set_bot_manager", set_bot_manager.to_string());
        }
        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建群")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chat_request_builder() {
        let config = Config::default();
        let request = CreateChatRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_chat_request_with_bot_manager() {
        let config = Config::default();
        let request = CreateChatRequest::new(config).set_bot_manager(true);
        assert_eq!(request.set_bot_manager, Some(true));
    }

    #[test]
    fn test_create_chat_request_with_uuid() {
        let config = Config::default();
        let request = CreateChatRequest::new(config).uuid("uuid-123");
        assert_eq!(request.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_create_chat_request_with_user_id() {
        let config = Config::default();
        let request = CreateChatRequest::new(config).user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_create_chat_request_with_all_options() {
        let config = Config::default();
        let request = CreateChatRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .set_bot_manager(false)
            .uuid("test-uuid");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(request.set_bot_manager, Some(false));
        assert_eq!(request.uuid, Some("test-uuid".to_string()));
    }

    #[test]
    fn test_create_chat_request_default_values() {
        let config = Config::default();
        let request = CreateChatRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.set_bot_manager, None);
        assert_eq!(request.uuid, None);
    }
}
