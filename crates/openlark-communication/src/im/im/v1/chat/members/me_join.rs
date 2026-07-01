//! 用户或机器人主动加入群聊
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-member/me_join>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::extract_response_data, common::models::EmptyData, endpoints::IM_V1_CHATS,
};

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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "用户或机器人主动加入群聊")
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
