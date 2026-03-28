//! 创建邮件组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/mail-v1/mail_group/create>

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::mailgroup::models::{CreateMailGroupBody, CreateMailGroupResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建邮件组请求
#[derive(Debug, Clone)]
pub struct CreateMailGroupRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateMailGroupBody,
}

impl CreateMailGroupRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateMailGroupBody::default(),
        }
    }

    /// 设置邮件组名称（邮箱地址）
    pub fn mail_group_id(mut self, mail_group_id: impl Into<String>) -> Self {
        self.body.mail_group_id = mail_group_id.into();
        self
    }

    /// 设置邮件组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置邮件组所有者
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.body.owner = Some(owner.into());
        self
    }

    /// 设置邮件组成员
    pub fn members(mut self, members: Vec<String>) -> Self {
        self.body.members = Some(members);
        self
    }

    /// 设置仅管理员发送
    pub fn only_admins_send(mut self, only_admins_send: bool) -> Self {
        self.body.only_admins_send = Some(only_admins_send);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateMailGroupResponse> {
        // 验证必填字段
        validate_required!(self.body.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupCreate;
        let mut request = ApiRequest::<CreateMailGroupResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建邮件组")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建邮件组")
    }
}

impl ApiResponseTrait for CreateMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mail_group_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateMailGroupRequest::new(config)
            .mail_group_id("team@example.com")
            .description("项目团队邮件组")
            .only_admins_send(true);

        assert_eq!(request.body.mail_group_id, "team@example.com");
        assert_eq!(request.body.description, Some("项目团队邮件组".to_string()));
        assert_eq!(request.body.only_admins_send, Some(true));
    }

    #[test]
    fn test_mail_api_v1_url() {
        let endpoint = MailApiV1::MailGroupCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/mail/v1/mailgroups");

        let endpoint = MailApiV1::MailGroupGet("group_123".to_string());
        assert!(endpoint.to_url().contains("group_123"));
    }
}
