//! 创建公共邮箱别名
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/create

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::alias::models::{
    CreatePublicMailboxAliasBody, CreatePublicMailboxAliasResponse,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 创建公共邮箱别名请求
#[derive(Debug, Clone)]
pub struct CreatePublicMailboxAliasRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: CreatePublicMailboxAliasBody,
}

impl CreatePublicMailboxAliasRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: CreatePublicMailboxAliasBody::default(),
        }
    }

    /// 设置别名邮箱地址
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = email.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreatePublicMailboxAliasResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePublicMailboxAliasResponse> {
        validate_required!(self.body.email.trim(), "别名邮箱地址不能为空");

        let api_endpoint = MailApiV1::PublicMailboxAliasCreate(self.mailbox_id.clone());
        let mut request =
            ApiRequest::<CreatePublicMailboxAliasResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建公共邮箱别名")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建公共邮箱别名")
    }
}

impl ApiResponseTrait for CreatePublicMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_public_mailbox_alias_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreatePublicMailboxAliasRequest::new(config, "mailbox_123".to_string())
            .email("alias@example.com");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.email, "alias@example.com");
    }
}
