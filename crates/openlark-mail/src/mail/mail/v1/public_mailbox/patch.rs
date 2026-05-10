//! 部分更新公共邮箱
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/public-mailbox/public_mailbox/patch

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::models::{
    PatchPublicMailboxBody, PatchPublicMailboxResponse,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use std::sync::Arc;

/// 部分更新公共邮箱请求
#[derive(Debug, Clone)]
pub struct PatchPublicMailboxRequest {
    config: Arc<Config>,
    mailbox_id: String,
    body: PatchPublicMailboxBody,
}

impl PatchPublicMailboxRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            body: PatchPublicMailboxBody::default(),
        }
    }

    /// 设置公共邮箱名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// 设置公共邮箱描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchPublicMailboxResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchPublicMailboxResponse> {
        let api_endpoint = MailApiV1::PublicMailboxPatch(self.mailbox_id.clone());
        let mut request = ApiRequest::<PatchPublicMailboxResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "部分更新公共邮箱")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "部分更新公共邮箱")
    }
}

impl ApiResponseTrait for PatchPublicMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_public_mailbox_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = PatchPublicMailboxRequest::new(config, "mailbox_123".to_string())
            .description("更新的描述");

        assert_eq!(request.mailbox_id, "mailbox_123");
        assert_eq!(request.body.description, Some("更新的描述".to_string()));
    }
}
