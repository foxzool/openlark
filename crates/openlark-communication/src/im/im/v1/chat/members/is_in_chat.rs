//! 判断用户或机器人是否在群里
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/is_in_chat>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{endpoints::IM_V1_CHATS, im::v1::chat::members::models::IsInChatResponse};

/// 判断用户或机器人是否在群里请求
///
/// 用于查询当前身份是否已经加入某个群聊。
pub struct IsInChatRequest {
    config: Config,
    chat_id: String,
}

impl IsInChatRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/is_in_chat>
    pub async fn execute(self) -> SDKResult<IsInChatResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<IsInChatResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/members/is_in_chat
        let req: ApiRequest<IsInChatResponse> = ApiRequest::get(format!(
            "{}/{}/members/is_in_chat",
            IM_V1_CHATS, self.chat_id
        ));

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "判断用户或机器人是否在群里",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/im/v1/chats/test001/members/is_in_chat
    #[tokio::test]
    async fn test_is_in_chat_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats/test001/members/is_in_chat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "is_in_chat": true }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        IsInChatRequest::new(config)
            .chat_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
