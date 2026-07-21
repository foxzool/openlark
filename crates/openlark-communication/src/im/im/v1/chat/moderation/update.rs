//! 更新群发言权限
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::{api_utils::serialize_params, models::EmptyData},
    endpoints::IM_V1_CHATS,
    im::v1::{chat::moderation::models::UpdateChatModerationBody, message::models::UserIdType},
};

/// 更新群发言权限请求
///
/// 用于调整群聊的禁言模式及可发言成员列表。
pub struct UpdateChatModerationRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl UpdateChatModerationRequest {
    /// 创建新的请求构建器。
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

    /// 使用指定请求选项执行请求。
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

        Transport::request_typed(req, &self.config, Some(option), "更新群发言权限").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PUT /open-apis/im/v1/chats/test001/moderation
    #[tokio::test]
    async fn test_update_chat_moderation_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/im/v1/chats/test001/moderation"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: UpdateChatModerationBody = serde_json::from_value(json!({})).expect("body 构造");
        UpdateChatModerationRequest::new(config)
            .chat_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
