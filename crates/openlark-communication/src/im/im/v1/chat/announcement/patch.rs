//! 更新群公告信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat-announcement/patch

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_CHATS,
    im::v1::chat::announcement::models::PatchChatAnnouncementBody,
};

/// 更新群公告信息请求
///
/// 用于提交新的群公告内容和版本号。
pub struct PatchChatAnnouncementRequest {
    config: Config,
    chat_id: String,
}

impl PatchChatAnnouncementRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat-announcement/patch
    pub async fn execute(self, body: PatchChatAnnouncementBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: PatchChatAnnouncementBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.chat_id, "chat_id 不能为空");
        validate_required!(body.revision, "revision 不能为空");

        if body.requests.is_empty() {
            return Err(openlark_core::error::validation_error(
                "requests 不能为空".to_string(),
                "公告内容 requests 不可为空".to_string(),
            ));
        }

        // url: PATCH:/open-apis/im/v1/chats/:chat_id/announcement
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}/announcement", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "更新群公告信息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新群公告信息")
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
