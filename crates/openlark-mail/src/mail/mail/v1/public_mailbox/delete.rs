//! 删除公共邮箱
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/delete

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::models::DeletePublicMailboxResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除公共邮箱请求
#[derive(Debug, Clone)]
pub struct DeletePublicMailboxRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl DeletePublicMailboxRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeletePublicMailboxResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeletePublicMailboxResponse> {
        validate_required!(self.mailbox_id.trim(), "公共邮箱ID不能为空");

        let api_endpoint = MailApiV1::PublicMailboxDelete(self.mailbox_id.clone());
        let request = ApiRequest::<DeletePublicMailboxResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除公共邮箱")
    }
}

impl ApiResponseTrait for DeletePublicMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_public_mailbox_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = DeletePublicMailboxRequest::new(config, "mailbox_123".to_string());
        assert_eq!(request.mailbox_id, "mailbox_123");
    }
}
