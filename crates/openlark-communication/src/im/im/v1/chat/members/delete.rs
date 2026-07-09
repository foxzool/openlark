//! 将用户或机器人移出群聊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::v1::chat::members::models::{
        DeleteChatMembersBody, DeleteChatMembersResponse, MemberIdType,
    },
};

/// 将用户或机器人移出群聊请求
///
/// 用于批量移除群聊中的成员。
pub struct DeleteChatMembersRequest {
    config: Config,
    chat_id: String,
    member_id_type: Option<MemberIdType>,
}

impl DeleteChatMembersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            member_id_type: None,
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 群成员 ID 类型（查询参数，可选，默认 open_id）
    pub fn member_id_type(mut self, member_id_type: MemberIdType) -> Self {
        self.member_id_type = Some(member_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/delete>
    pub async fn execute(
        self,
        body: DeleteChatMembersBody,
    ) -> SDKResult<DeleteChatMembersResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: DeleteChatMembersBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteChatMembersResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");
        if body.id_list.is_empty() {
            return Err(openlark_core::error::validation_error(
                "id_list 不能为空".to_string(),
                "成员列表不可为空".to_string(),
            ));
        }

        // url: DELETE:/open-apis/im/v1/chats/:chat_id/members
        let mut req: ApiRequest<DeleteChatMembersResponse> =
            ApiRequest::delete(format!("{}/{}/members", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "将用户或机器人移出群聊")?);

        if let Some(member_id_type) = self.member_id_type {
            req = req.query("member_id_type", member_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "将用户或机器人移出群聊")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE /open-apis/im/v1/chats/test001/members
    #[tokio::test]
    async fn test_delete_chat_members_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/im/v1/chats/test001/members"))
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

        let body: DeleteChatMembersBody =
            serde_json::from_value(json!({ "id_list": ["test001"] })).expect("body 构造");
        DeleteChatMembersRequest::new(config)
            .chat_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
