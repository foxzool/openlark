//! 获取邮件撤回进度
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-sent_message/get_recall_detail

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

/// 获取邮件撤回进度请求。
#[derive(Debug, Clone)]
pub struct GetRecallDetailRequest {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

/// 获取邮件撤回进度响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetRecallDetailResponse {
    /// 错误码，非 0 表示失败。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<RecallDetailData>,
}

impl ApiResponseTrait for GetRecallDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 撤回进度数据。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallDetailData {
    /// 整体撤回进度：in_progress / done。
    pub recall_status: Option<String>,
    /// 撤回最终结果（仅 done 时有意义）：all_success / all_fail / some_fail / processing。
    pub recall_result: Option<String>,
    /// 撤回成功的收件人数。
    pub success_count: Option<i32>,
    /// 撤回失败的收件人数。
    pub failure_count: Option<i32>,
    /// 处理中的收件人数。
    pub processing_count: Option<i32>,
    /// 每个收件人的撤回详情列表。
    pub items: Option<Vec<RecallDetailItem>>,
}

/// 收件人撤回详情。
#[derive(Debug, Clone, Deserialize)]
pub struct RecallDetailItem {
    /// 收件人邮箱地址。
    pub recipient_address: Option<String>,
    /// 收件人显示名称。
    pub recipient_name: Option<String>,
    /// 该收件人的撤回状态：success / fail / processing。
    pub status: Option<String>,
    /// 撤回失败原因（仅 status=fail）：message_has_been_read / not_using_lark_mail /
    /// not_in_the_same_tenant / invalid_address / unknown。
    pub fail_reason: Option<String>,
    /// 是否为邮件组地址。
    pub is_mailing_list: Option<bool>,
    /// 邮件组内成功撤回人数（仅 is_mailing_list=true）。
    pub mailing_list_success_count: Option<i32>,
    /// 邮件组内撤回失败人数（仅 is_mailing_list=true）。
    pub mailing_list_failure_count: Option<i32>,
    /// 邮件组完成百分比 0-100（仅 is_mailing_list=true）。
    pub mailing_list_finish_percent: Option<i32>,
}

impl GetRecallDetailRequest {
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
    pub async fn execute(self) -> SDKResult<GetRecallDetailResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetRecallDetailResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.message_id, "message_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/messages/{}/recall",
            self.mailbox_id, self.message_id
        );
        let req: ApiRequest<GetRecallDetailResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件撤回进度", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{ "code": 0, "msg": "success", "data": { "recall_status": "done", "recall_result": "all_success", "success_count": 2 } }"#;
        let resp: GetRecallDetailResponse = serde_json::from_str(json).unwrap();
        let d = resp.data.unwrap();
        assert_eq!(d.recall_status, Some("done".to_string()));
        assert_eq!(d.success_count, Some(2));
    }
}
