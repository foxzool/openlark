//! 会话标签页排序
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/sort_tabs>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::serialize_params, endpoints::IM_V1_CHATS,
    im::v1::chat::tab::models::TabIdsBody,
};

/// 会话标签页排序请求
///
/// 用于调整群聊标签页顺序。
pub struct SortChatTabsRequest {
    config: Config,
    chat_id: String,
}

impl SortChatTabsRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/sort_tabs>
    pub async fn execute(self, body: TabIdsBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: TabIdsBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        if body.tab_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "tab_ids 不能为空".to_string(),
                "会话标签页 ID 列表不可为空".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/chats/:chat_id/chat_tabs/sort_tabs
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(format!(
            "{}/{}/chat_tabs/sort_tabs",
            IM_V1_CHATS, self.chat_id
        ))
        .body(serialize_params(&body, "会话标签页排序")?);

        Transport::request_typed(req, &self.config, Some(option), "会话标签页排序").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/im/v1/chats/test001/chat_tabs/sort_tabs
    #[tokio::test]
    async fn test_sort_chat_tabs_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/im/v1/chats/test001/chat_tabs/sort_tabs"))
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

        let body: TabIdsBody =
            serde_json::from_value(json!({ "tab_ids": ["test001"] })).expect("body 构造");
        SortChatTabsRequest::new(config)
            .chat_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
