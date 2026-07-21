//! 获取附件下载链接
//! docPath: <https://open.feishu.cn/document/mail-v1/user_mailbox-message/user_mailbox-message-attachment/download_url>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取附件下载链接的请求。
#[derive(Debug, Clone)]
pub struct GetAttachmentDownloadUrlRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    message_id: String,
    attachment_id: String,
}

/// 获取附件下载链接的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttachmentDownloadUrlResponse {
    /// 响应数据。
    pub data: Option<DownloadUrlData>,
}

impl ApiResponseTrait for GetAttachmentDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 附件下载链接数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrlData {
    /// 下载地址。
    pub download_url: String,
    /// 过期时间。
    pub expire_time: String,
}

impl GetAttachmentDownloadUrlRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
        attachment_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            message_id: message_id.into(),
            attachment_id: attachment_id.into(),
        }
    }

    /// 执行获取附件下载链接请求。
    pub async fn execute(self) -> SDKResult<GetAttachmentDownloadUrlResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAttachmentDownloadUrlResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/attachments/download_url",
            self.user_mailbox_id, self.message_id
        );
        let req: ApiRequest<GetAttachmentDownloadUrlResponse> =
            ApiRequest::get(&path).query("attachment_id", self.attachment_id);

        Transport::request_typed(req, &self.config, Some(option), "获取附件下载链接").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../user_mailboxes/{umb}/messages/{msg}/attachments/download_url → 强类型 GetAttachmentDownloadUrlResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_download_url_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/messages/msg_001/attachments/download_url",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "download_url": "https://example.com/file.zip", "expire_time": "2026-07-08T00:00:00Z" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = GetAttachmentDownloadUrlRequest::new(config, "umb_001", "msg_001", "att_001")
            .execute()
            .await
            .expect("获取附件下载链接应成功");
        let data = resp.data.expect("响应 data 应非空");
        assert_eq!(data.download_url, "https://example.com/file.zip");
        assert_eq!(data.expire_time, "2026-07-08T00:00:00Z");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/messages/msg_001/attachments/download_url"
        );
        assert_eq!(received[0].url.query(), Some("attachment_id=att_001"));
    }
}
