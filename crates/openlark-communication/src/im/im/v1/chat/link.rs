//! 获取群分享链接
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/link>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::models::{ChatLinkValidityPeriod, GetChatLinkResponse},
};

/// 获取群分享链接请求
///
/// 用于获取群聊的分享链接，方便用户邀请其他人加入群聊。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `chat_id`: 群 ID，必填
/// - `validity_period`: 群分享链接有效期（可选，默认 week）
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetChatLinkRequest::new(config)
///     .chat_id("oc_xxx")
///     .validity_period(ChatLinkValidityPeriod::Day);
/// ```
pub struct GetChatLinkRequest {
    config: Config,
    chat_id: String,
    validity_period: Option<ChatLinkValidityPeriod>,
}

impl GetChatLinkRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            validity_period: None,
        }
    }

    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn validity_period(mut self, validity_period: ChatLinkValidityPeriod) -> Self {
        self.validity_period = Some(validity_period);
        self
    }

    pub async fn execute(self) -> SDKResult<GetChatLinkResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetChatLinkResponse> {
        // === 必填字段验证 ===
        validate_required!(self.chat_id, "chat_id 不能为空");

        let body = GetChatLinkBody {
            validity_period: self.validity_period.map(|v| v.as_str().to_string()),
        };

        let req: ApiRequest<GetChatLinkResponse> =
            ApiRequest::post(format!("{}/{}/link", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "获取群分享链接")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取群分享链接")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatLinkBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chat_link_request_builder() {
        let config = Config::default();
        let request = GetChatLinkRequest::new(config).chat_id("oc_xxx");
        assert_eq!(request.chat_id, "oc_xxx");
    }

    #[test]
    fn test_get_chat_link_request_with_validity_period() {
        let config = Config::default();
        let request = GetChatLinkRequest::new(config)
            .chat_id("oc_xxx")
            .validity_period(ChatLinkValidityPeriod::Week);
        assert_eq!(request.validity_period, Some(ChatLinkValidityPeriod::Week));
    }

    #[test]
    fn test_get_chat_link_request_default_values() {
        let config = Config::default();
        let request = GetChatLinkRequest::new(config);
        assert_eq!(request.chat_id, "");
        assert_eq!(request.validity_period, None);
    }
}
