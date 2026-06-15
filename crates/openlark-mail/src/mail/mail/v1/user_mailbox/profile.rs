//! 获取用户邮箱信息
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox/profile

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

/// 获取用户邮箱信息请求。
#[derive(Debug, Clone)]
pub struct GetUserMailboxProfileRequest {
    config: Arc<Config>,
    /// 用户邮箱ID（只支持 "me"）。
    user_mailbox_id: String,
}

/// 获取用户邮箱信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserMailboxProfileResponse {
    /// 错误码。
    pub code: i32,
    /// 错误描述。
    pub msg: String,
    /// 响应数据。
    pub data: Option<GetUserMailboxProfileData>,
}

impl ApiResponseTrait for GetUserMailboxProfileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱信息响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserMailboxProfileData {
    /// 用户主邮箱地址。
    pub primary_email_address: Option<String>,
}

impl GetUserMailboxProfileRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetUserMailboxProfileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetUserMailboxProfileResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/profile",
            self.user_mailbox_id
        );
        let req: ApiRequest<GetUserMailboxProfileResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户邮箱信息", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": { "primary_email_address": "abc@abc.com" }
        }"#;
        let resp: GetUserMailboxProfileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            resp.data.unwrap().primary_email_address,
            Some("abc@abc.com".to_string())
        );
    }
}
