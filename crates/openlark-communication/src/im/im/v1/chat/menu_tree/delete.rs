//! 删除群菜单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::serialize_params, endpoints::IM_V1_CHATS,
    im::v1::chat::menu_tree::models::ChatMenuTopLevelIdsBody,
};

/// 删除群菜单请求
///
/// 用于删除群聊菜单树中的一级菜单项。
pub struct DeleteChatMenuTreeRequest {
    config: Config,
    chat_id: String,
}

impl DeleteChatMenuTreeRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/delete>
    pub async fn execute(self, body: ChatMenuTopLevelIdsBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: ChatMenuTopLevelIdsBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");
        if body.chat_menu_top_level_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "chat_menu_top_level_ids 不能为空".to_string(),
                "一级菜单 ID 列表不可为空".to_string(),
            ));
        }

        // url: DELETE:/open-apis/im/v1/chats/:chat_id/menu_tree
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::delete(format!("{}/{}/menu_tree", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "删除群菜单")?);

        Transport::request_typed(req, &self.config, Some(option), "删除群菜单").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE /open-apis/im/v1/chats/test001/menu_tree
    #[tokio::test]
    async fn test_delete_chat_menu_tree_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/im/v1/chats/test001/menu_tree"))
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

        let body: ChatMenuTopLevelIdsBody =
            serde_json::from_value(json!({ "chat_menu_top_level_ids": ["test001"] }))
                .expect("body 构造");
        DeleteChatMenuTreeRequest::new(config)
            .chat_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
