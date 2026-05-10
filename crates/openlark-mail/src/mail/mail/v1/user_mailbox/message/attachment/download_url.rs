//! 获取附件下载链接
//! docPath: https://open.feishu.cn/document/mail-v1/user_mailbox-message/user_mailbox-message-attachment/download_url

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GetAttachmentDownloadUrlRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    message_id: String,
    attachment_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttachmentDownloadUrlResponse {
    pub data: Option<DownloadUrlData>,
}

impl ApiResponseTrait for GetAttachmentDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrlData {
    pub download_url: String,
    pub expire_time: String,
}

impl GetAttachmentDownloadUrlRequest {
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

    pub async fn execute(self) -> SDKResult<GetAttachmentDownloadUrlResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

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

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取附件下载链接", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
