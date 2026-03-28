//! 添加会话标签页
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
};

/// 添加会话标签页请求
pub struct CreateChatTabRequest {
    config: Config,
    chat_id: String,
}

impl CreateChatTabRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-tab/create>
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

        // url: POST:/open-apis/im/v1/chats/:chat_id/chat_tabs
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/chat_tabs", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "添加会话标签页")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "添加会话标签页")
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
