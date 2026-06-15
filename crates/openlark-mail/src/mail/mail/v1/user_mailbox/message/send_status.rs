//! 查询邮件发送状态
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-message/send_status

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

/// 查询邮件发送状态请求。
#[derive(Debug, Clone)]
pub struct GetMailSendStatusRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 查询邮件发送状态响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetMailSendStatusResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<MailSendStatusData>,
}

impl ApiResponseTrait for GetMailSendStatusResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送状态数据。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSendStatusData {
    /// 邮件业务标识 ID。
    pub message_id: Option<String>,
    /// 收件人投递状态列表。
    pub details: Option<Vec<MailSendStatusDetail>>,
}

/// 收件人投递状态详情。
#[derive(Debug, Clone, Deserialize)]
pub struct MailSendStatusDetail {
    /// 收件人信息。
    pub recipient: Option<MailRecipient>,
    /// 投递状态（1-6）。
    pub status: Option<i32>,
    /// 最后更新时间（Unix 时间戳，秒）。
    pub last_updated_time: Option<i64>,
}

/// 收件人信息。
#[derive(Debug, Clone, Deserialize)]
pub struct MailRecipient {
    /// 邮件地址。
    pub mail_address: Option<String>,
    /// 名称。
    pub name: Option<String>,
}

impl GetMailSendStatusRequest {
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
    pub async fn execute(self) -> SDKResult<GetMailSendStatusResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailSendStatusResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/send_status",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<GetMailSendStatusResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询邮件发送状态", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0, "msg": "success",
            "data": { "message_id": "197c5d72e22e1d78", "details": [{"recipient":{"mail_address":"m@o.com","name":"Mike"},"status":1}] }
        }"#;
        let resp: GetMailSendStatusResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            resp.data.unwrap().message_id,
            Some("197c5d72e22e1d78".to_string())
        );
    }
}
