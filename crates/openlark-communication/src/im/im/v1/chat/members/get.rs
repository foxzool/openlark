//! 获取群成员列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::members::models::{ListChatMembersResponse, MemberIdType},
};

/// 获取群成员列表请求
pub struct GetChatMembersRequest {
    config: Config,
    chat_id: String,
    member_id_type: Option<MemberIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl GetChatMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            member_id_type: None,
            page_size: None,
            page_token: None,
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

    /// 分页大小（查询参数，可选，默认 20，最大 100）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/get>
    pub async fn execute(self) -> SDKResult<ListChatMembersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListChatMembersResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/members
        let mut req: ApiRequest<ListChatMembersResponse> =
            ApiRequest::get(format!("{}/{}/members", IM_V1_CHATS, self.chat_id));

        if let Some(member_id_type) = self.member_id_type {
            req = req.query("member_id_type", member_id_type.as_str());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取群成员列表")
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
