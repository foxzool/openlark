//! 修改群菜单元信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
};

/// 修改群菜单元信息请求
pub struct PatchChatMenuItemRequest {
    config: Config,
    chat_id: String,
    menu_item_id: String,
}

impl PatchChatMenuItemRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            menu_item_id: String::new(),
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 菜单项 ID（路径参数）
    pub fn menu_item_id(mut self, menu_item_id: impl Into<String>) -> Self {
        self.menu_item_id = menu_item_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-menu_tree/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");
        validate_required!(self.menu_item_id, "menu_item_id 不能为空");

        // url: PATCH:/open-apis/im/v1/chats/:chat_id/menu_items/:menu_item_id
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(format!(
            "{}/{}/menu_items/{}",
            IM_V1_CHATS, self.chat_id, self.menu_item_id
        ))
        .body(serialize_params(&body, "修改群菜单元信息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "修改群菜单元信息")
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
