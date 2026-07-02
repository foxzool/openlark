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
