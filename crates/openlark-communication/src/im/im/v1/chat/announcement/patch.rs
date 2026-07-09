//! 更新群公告信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat-announcement/patch>

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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat-announcement/patch>
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
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/im/v1/chats/test001/announcement
    #[tokio::test]
    async fn test_patch_chat_announcement_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/im/v1/chats/test001/announcement"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: PatchChatAnnouncementBody =
            serde_json::from_value(json!({ "revision": "test001", "requests": ["test001"] }))
                .expect("body 构造");
        PatchChatAnnouncementRequest::new(config)
            .chat_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
