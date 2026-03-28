//! 创建公共邮箱
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/mail-v1/public_mailbox/create>

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::public_mailbox::models::{
    CreatePublicMailboxBody, CreatePublicMailboxResponse,
};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建公共邮箱请求
#[derive(Debug, Clone)]
pub struct CreatePublicMailboxRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreatePublicMailboxBody,
}

impl CreatePublicMailboxRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreatePublicMailboxBody::default(),
        }
    }

    /// 设置公共邮箱名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置公共邮箱描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置公共邮箱所有者的用户 ID
    pub fn owner_user_id(mut self, owner_user_id: impl Into<String>) -> Self {
        self.body.owner_user_id = Some(owner_user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreatePublicMailboxResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePublicMailboxResponse> {
        // 验证必填字段
        validate_required!(self.body.name.trim(), "公共邮箱名称不能为空");

        let api_endpoint = MailApiV1::PublicMailboxCreate;
        let mut request = ApiRequest::<CreatePublicMailboxResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建公共邮箱")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建公共邮箱")
    }
}

impl ApiResponseTrait for CreatePublicMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_public_mailbox_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreatePublicMailboxRequest::new(config)
            .name("support@example.com")
            .description("技术支持公共邮箱");

        assert_eq!(request.body.name, "support@example.com");
        assert_eq!(
            request.body.description,
            Some("技术支持公共邮箱".to_string())
        );
    }
}
