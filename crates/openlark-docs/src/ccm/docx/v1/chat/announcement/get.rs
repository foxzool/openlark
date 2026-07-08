/// 获取群公告基本信息
///
/// 此接口用于获取指定群聊的群公告基本信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement/get>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取群公告基本信息请求
///
/// 用于获取群公告元信息。
pub struct GetChatAnnouncementRequest {
    chat_id: String,
    user_id_type: Option<String>,
    config: Config,
}

/// 获取群公告基本信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementResponse {
    /// 修订版本号。
    pub revision_id: i64,
    /// 创建时间。
    pub create_time: i64,
    /// 更新时间。
    pub update_time: i64,
    /// 创建者 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 创建者 ID 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id_type: Option<String>,
    /// 更新者 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_id: Option<String>,
    /// 更新者 ID 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_id_type: Option<String>,
    /// 公告类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub announcement_type: Option<String>,
    /// 创建时间 v2 格式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_v2: Option<String>,
    /// 更新时间 v2 格式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_v2: Option<String>,
    /// 未建模扩展字段。
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

    /// 使用指定请求选项执行请求。
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
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../chats/{chat_id}/announcement → GetChatAnnouncementResponse（revision_id）。
    #[tokio::test]
    async fn test_get_chat_announcement_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/docx/v1/chats/chat001/announcement"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "revision_id": 1, "create_time": 100, "update_time": 200 }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetChatAnnouncementRequest::new(config)
            .chat_id("chat001")
            .execute()
            .await
            .expect("获取群公告应成功");
        assert_eq!(resp.revision_id, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement"
        );
    }
}
