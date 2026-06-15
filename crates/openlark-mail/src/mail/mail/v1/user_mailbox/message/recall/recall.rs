//! 撤回已发送邮件
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-sent_message/recall

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

/// 撤回已发送邮件请求。
#[derive(Debug, Clone)]
pub struct RecallMessageRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 撤回已发送邮件响应。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallMessageResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<RecallMessageData>,
}

impl ApiResponseTrait for RecallMessageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 撤回邮件数据。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallMessageData {
    /// 撤回状态：unavailable（不可撤回）/ available（可撤回）。
    pub recall_status: Option<String>,
    /// 不支持撤回的原因：recall_not_enabled / migration_domain / sender_address_not_owned /
    /// already_recalled / not_delivered / exceeded_time_limit（仅 recall_status=unavailable 时返回）。
    pub recall_restriction_reason: Option<String>,
}

impl RecallMessageRequest {
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
    pub async fn execute(self) -> SDKResult<RecallMessageResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecallMessageResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/recall",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<RecallMessageResponse> = ApiRequest::post(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("撤回邮件", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": { "recall_status": "available" } }"#;
        let resp: RecallMessageResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            resp.data.unwrap().recall_status,
            Some("available".to_string())
        );
    }
}
