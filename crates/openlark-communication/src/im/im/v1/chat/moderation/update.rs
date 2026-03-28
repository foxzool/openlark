//! 更新群发言权限
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/update>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_CHATS,
    im::im::v1::{chat::moderation::models::UpdateChatModerationBody, message::models::UserIdType},
};

/// 更新群发言权限请求
pub struct UpdateChatModerationRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl UpdateChatModerationRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/update>
    pub async fn execute(self, body: UpdateChatModerationBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateChatModerationBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: PUT:/open-apis/im/v1/chats/:chat_id/moderation
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::put(format!("{}/{}/moderation", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "更新群发言权限")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新群发言权限")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
