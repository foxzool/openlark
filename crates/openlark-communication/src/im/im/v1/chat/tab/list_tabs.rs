//! 拉取会话标签页
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/list_tabs>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::endpoints::IM_V1_CHATS;

/// 拉取会话标签页请求
///
/// 用于读取指定群聊的全部标签页。
pub struct ListChatTabsRequest {
    config: Config,
    chat_id: String,
}

impl ListChatTabsRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/list_tabs>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/chat_tabs/list_tabs
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/chat_tabs/list_tabs",
            IM_V1_CHATS, self.chat_id
        ));

        Transport::request_typed(req, &self.config, Some(option), "拉取会话标签页").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/im/v1/chats/test001/chat_tabs/list_tabs
    #[tokio::test]
    async fn test_list_chat_tabs_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/im/v1/chats/test001/chat_tabs/list_tabs"))
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

        ListChatTabsRequest::new(config)
            .chat_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
