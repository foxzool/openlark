//! 更新群置顶
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/put_top_notice>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_CHATS,
};

/// 更新群置顶请求
///
/// 用于设置或更新群聊的置顶消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `chat_id`: 群 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = PutTopNoticeRequest::new(config).chat_id("oc_xxx");
/// ```
pub struct PutTopNoticeRequest {
    config: Config,
    chat_id: String,
}

impl PutTopNoticeRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
        }
    }

    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/top_notice/put_top_notice",
            IM_V1_CHATS, self.chat_id
        ))
        .body(serialize_params(&body, "更新群置顶")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "更新群置顶")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_put_top_notice_request_builder() {
        let config = Config::default();
        let request = PutTopNoticeRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
    }

    #[test]
    fn test_put_top_notice_request_default_values() {
        let config = Config::default();
        let request = PutTopNoticeRequest::new(config);
        assert_eq!(request.chat_id, "");
    }

    #[test]
    fn test_put_top_notice_request_multiple_chats() {
        let config = Config::default();
        let request1 = PutTopNoticeRequest::new(config.clone()).chat_id("oc_111");
        let request2 = PutTopNoticeRequest::new(config).chat_id("oc_222");
        assert_eq!(request1.chat_id, "oc_111");
        assert_eq!(request2.chat_id, "oc_222");
    }
}
