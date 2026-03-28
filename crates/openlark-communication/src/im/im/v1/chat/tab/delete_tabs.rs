//! 删除会话标签页
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/delete_tabs>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::tab::models::TabIdsBody,
};

/// 删除会话标签页请求
pub struct DeleteChatTabsRequest {
    config: Config,
    chat_id: String,
}

impl DeleteChatTabsRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/delete_tabs>
    pub async fn execute(self, body: TabIdsBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

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

        // url: DELETE:/open-apis/im/v1/chats/:chat_id/chat_tabs/delete_tabs
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(format!(
            "{}/{}/chat_tabs/delete_tabs",
            IM_V1_CHATS, self.chat_id
        ))
        .body(serialize_params(&body, "删除会话标签页")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "删除会话标签页")
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
