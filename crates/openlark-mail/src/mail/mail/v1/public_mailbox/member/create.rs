//! 创建公共邮箱成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/create

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::member::models::{
    CreatePublicMailboxMemberBody, CreatePublicMailboxMemberResponse,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建公共邮箱成员请求
#[derive(Debug, Clone)]
pub struct CreatePublicMailboxMemberRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreatePublicMailboxMemberBody,
}

impl CreatePublicMailboxMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreatePublicMailboxMemberBody::default(),
        }
    }

    /// 设置成员邮箱地址
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = email.into();
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: i32) -> Self {
        self.body.member_type = Some(member_type);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreatePublicMailboxMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePublicMailboxMemberResponse> {
        validate_required!(self.body.email.trim(), "成员邮箱地址不能为空");

        let api_endpoint = MailApiV1::PublicMailboxMemberCreate(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<CreatePublicMailboxMemberResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建公共邮箱成员")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建公共邮箱成员")
    }
}

impl ApiResponseTrait for CreatePublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_public_mailbox_member_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreatePublicMailboxMemberRequest::new(config, "mailbox_123".to_string())
            .email("member@example.com")
            .member_type(1);

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.email, "member@example.com");
        assert_eq!(request.body.member_type, Some(1));
    }
}
