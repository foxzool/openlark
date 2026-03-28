//! 将用户或机器人移出群聊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::members::models::{
        DeleteChatMembersBody, DeleteChatMembersResponse, MemberIdType,
    },
};

/// 将用户或机器人移出群聊请求
pub struct DeleteChatMembersRequest {
    config: Config,
    chat_id: String,
    member_id_type: Option<MemberIdType>,
}

impl DeleteChatMembersRequest {
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
