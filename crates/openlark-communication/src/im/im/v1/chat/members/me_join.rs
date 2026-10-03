//! 用户或机器人主动加入群聊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/me_join>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{common::models::EmptyData, endpoints::IM_V1_CHATS};

/// 用户或机器人主动加入群聊请求
///
/// 用于让当前身份主动加入指定群聊。
pub struct MeJoinChatMembersRequest {
    config: Config,
    chat_id: String,
}

impl MeJoinChatMembersRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/me_join>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: PATCH:/open-apis/im/v1/chats/:chat_id/members/me_join
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}/members/me_join", IM_V1_CHATS, self.chat_id));

        Transport::request_typed(req, &self.config, Some(option), "用户或机器人主动加入群聊").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/im/v1/chats/test001/members/me_join
    #[tokio::test]
    async fn test_me_join_chat_members_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/im/v1/chats/test001/members/me_join"))
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

        MeJoinChatMembersRequest::new(config)
            .chat_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
