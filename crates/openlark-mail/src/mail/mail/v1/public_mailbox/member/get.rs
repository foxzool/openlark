//! 获取公共邮箱成员详情
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/get

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::member::models::GetPublicMailboxMemberResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 获取公共邮箱成员请求
#[derive(Debug, Clone)]
pub struct GetPublicMailboxMemberRequest {
    config: Arc<Config>,
    mailbox_id: String,
    member_id: String,
}

impl GetPublicMailboxMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: String, member_id: String) -> Self {
        Self {
            config,
            mailbox_id,
            member_id,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetPublicMailboxMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetPublicMailboxMemberResponse> {
        validate_required!(self.mailbox_id.trim(), "公共邮箱ID不能为空");
        validate_required!(self.member_id.trim(), "成员ID不能为空");

        let api_endpoint =
            MailApiV1::PublicMailboxMemberGet(self.mailbox_id.clone(), self.member_id.clone());
        let request = ApiRequest::<GetPublicMailboxMemberResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取公共邮箱成员")
    }
}

impl ApiResponseTrait for GetPublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_mailbox_member_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = GetPublicMailboxMemberRequest::new(
            config,
            "mailbox_123".to_string(),
            "member_456".to_string(),
        );
        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.member_id, "member_456");
    }
}
