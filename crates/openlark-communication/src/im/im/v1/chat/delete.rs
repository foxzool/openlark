//! 解散群
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::IM_V1_CHATS,
};

/// 解散群请求
///
/// 用于解散指定的群聊。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `chat_id`: 群 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::chat::DeleteChatRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = DeleteChatRequest::new(config)
///     .chat_id("oc_xxx");
/// let response = request.execute().await?;
/// ```
pub struct DeleteChatRequest {
    config: Config,
    chat_id: String,
}

impl DeleteChatRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: DELETE:/open-apis/im/v1/chats/:chat_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", IM_V1_CHATS, self.chat_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "解散群")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_chat_request_builder() {
        let config = Config::default();
        let request = DeleteChatRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
    }

    #[test]
    fn test_empty_chat_id() {
        let config = Config::default();
        let request = DeleteChatRequest::new(config);
        assert_eq!(request.chat_id, "");
    }

    #[test]
    fn test_chat_id_with_string() {
        let config = Config::default();
        let request = DeleteChatRequest::new(config).chat_id(String::from("oc_test_123"));
        assert_eq!(request.chat_id, "oc_test_123");
    }

    #[test]
    fn test_delete_chat_request_builder_chaining() {
        let config = Config::default();
        let request = DeleteChatRequest::new(config).chat_id("oc_aaaaa");
        assert_eq!(request.chat_id, "oc_aaaaa");
    }
}
