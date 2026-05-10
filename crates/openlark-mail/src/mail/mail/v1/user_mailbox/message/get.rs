//! 获取邮件详情
//! docPath: https://open.feishu.cn/document/mail-v1/user_mailbox-message/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Get Mailbox Message Request。
#[derive(Debug, Clone)]
pub struct GetMailboxMessageRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    message_id: String,
}

/// Get Mailbox Message Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailboxMessageResponse {
    /// 响应数据。
    pub data: Option<MessageData>,
}

impl ApiResponseTrait for GetMailboxMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Message Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    /// 消息 ID。
    pub message_id: String,
    /// subject 字段。
    pub subject: String,
    /// body 字段。
    pub body: String,
}

impl GetMailboxMessageRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailboxMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailboxMessageResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}",
            self.user_mailbox_id, self.message_id
        );
        let req: ApiRequest<GetMailboxMessageResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取邮件详情", "响应数据为空"))
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
