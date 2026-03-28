//! 排序群菜单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/sort>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::menu_tree::models::ChatMenuTopLevelIdsBody,
};

/// 排序群菜单请求
pub struct SortChatMenuTreeRequest {
    config: Config,
    chat_id: String,
}

impl SortChatMenuTreeRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/sort>
    pub async fn execute(self, body: ChatMenuTopLevelIdsBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

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

        // url: POST:/open-apis/im/v1/chats/:chat_id/menu_tree/sort
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/menu_tree/sort", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "排序群菜单")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "排序群菜单")
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
