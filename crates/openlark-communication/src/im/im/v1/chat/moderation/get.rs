//! 获取群成员发言权限
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    endpoints::IM_V1_CHATS,
    im::v1::{chat::moderation::models::GetChatModerationResponse, message::models::UserIdType},
};

/// 获取群成员发言权限请求
///
/// 用于分页查询指定群聊的发言权限配置和成员列表。
pub struct GetChatModerationRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetChatModerationRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
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

    /// 分页大小（查询参数，可选，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/get>
    pub async fn execute(self) -> SDKResult<GetChatModerationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetChatModerationResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/moderation
        let mut req: ApiRequest<GetChatModerationResponse> =
            ApiRequest::get(format!("{}/{}/moderation", IM_V1_CHATS, self.chat_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        Transport::request_typed(req, &self.config, Some(option), "获取群成员发言权限").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/im/v1/chats/test001/moderation
    #[tokio::test]
    async fn test_get_chat_moderation_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats/test001/moderation"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "moderation_setting": "all" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetChatModerationRequest::new(config)
            .chat_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
