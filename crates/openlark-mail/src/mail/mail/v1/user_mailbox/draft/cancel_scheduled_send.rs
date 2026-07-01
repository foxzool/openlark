//! 取消定时发送
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-draft/cancel_scheduled_send>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 取消定时发送请求。
#[derive(Debug, Clone)]
pub struct CancelScheduledSendRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 取消定时发送响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CancelScheduledSendResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for CancelScheduledSendResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CancelScheduledSendRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CancelScheduledSendResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelScheduledSendResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/cancel_scheduled_send",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<CancelScheduledSendResponse> = ApiRequest::post(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("取消定时发送", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": {} }"#;
        let resp: CancelScheduledSendResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.code, 0);
    }
}
