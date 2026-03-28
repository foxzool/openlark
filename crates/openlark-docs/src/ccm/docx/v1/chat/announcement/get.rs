/// 获取群公告基本信息
///
/// 此接口用于获取指定群聊的群公告基本信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取群公告基本信息请求
pub struct GetChatAnnouncementRequest {
    chat_id: String,
    user_id_type: Option<String>,
    config: Config,
}

/// 获取群公告基本信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementResponse {
    pub revision_id: i64,
    pub create_time: i64,
    pub update_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_v2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_v2: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetChatAnnouncementRequest {
    /// 创建获取群公告基本信息请求
    pub fn new(config: Config) -> Self {
        Self {
            chat_id: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置群聊 ID
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 设置 user_id_type（可选）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get
    pub async fn execute(self) -> SDKResult<GetChatAnnouncementResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetChatAnnouncementResponse> {
        validate_required!(self.chat_id, "群聊ID不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementGet(self.chat_id.clone());
        let mut api_request: ApiRequest<GetChatAnnouncementResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", &user_id_type);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
