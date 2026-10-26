//! 修改邮箱联系人

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mail Contact Request。
#[derive(Debug, Clone)]
pub struct PatchMailContactRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    mail_contact_id: String,
    body: PatchMailContactBody,
}

/// 修改邮箱联系人的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMailContactBody {
    /// 联系人姓名（1～64 字符）。
    pub name: String,
    /// 联系人公司（最大 64 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 联系人手机号（最大 40 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 联系人邮箱（最大 319 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_address: Option<String>,
    /// 联系人标签（最大 64 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 联系人备注（最大 1000 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 联系人职位（最大 64 字符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// Patch Mail Contact Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailContactResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchMailContactRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        mail_contact_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            mail_contact_id: mail_contact_id.into(),
            body: PatchMailContactBody::default(),
        }
    }

    /// 设置联系人姓名（必填，1～64 字符）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置联系人公司。
    pub fn company(mut self, company: impl Into<String>) -> Self {
        self.body.company = Some(company.into());
        self
    }

    /// 设置联系人手机号。
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.body.phone = Some(phone.into());
        self
    }

    /// 设置联系人邮箱。
    pub fn mail_address(mut self, mail_address: impl Into<String>) -> Self {
        self.body.mail_address = Some(mail_address.into());
        self
    }

    /// 设置联系人标签。
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.body.tag = Some(tag.into());
        self
    }

    /// 设置联系人备注。
    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.body.remark = Some(remark.into());
        self
    }

    /// 设置联系人职位。
    pub fn position(mut self, position: impl Into<String>) -> Self {
        self.body.position = Some(position.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailContactResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailContactResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.mail_contact_id, "mail_contact_id 不能为空");
        validate_required!(self.body.name, "name 不能为空");
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts/{}",
            self.user_mailbox_id, self.mail_contact_id
        );
        let req: ApiRequest<PatchMailContactResponse> =
            ApiRequest::patch(&path).body(serialize_params(&self.body, "修改邮箱联系人")?);

        Transport::request_typed(req, &self.config, Some(option), "修改邮箱联系人").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../user_mailboxes/{umb}/mail_contacts/{mc} → 强类型 PatchMailContactResponse 解析，请求体对齐官方示例。
    #[tokio::test]
    async fn test_patch_mail_contact_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/mail_contacts/mc_001",
            ))
            .and(body_json(json!({
                "name": "张三",
                "position": "CEO"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "mail_contact_id": "mc_001" } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = PatchMailContactRequest::new(config, "umb_001", "mc_001")
            .name("张三")
            .position("CEO")
            .execute()
            .await
            .expect("修改邮箱联系人应成功");
        assert_eq!(resp.data.unwrap()["mail_contact_id"], "mc_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/mail_contacts/mc_001"
        );
    }

    /// 失败路径：未设置 name 时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_patch_mail_contact_rejects_missing_name() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = PatchMailContactRequest::new(config, "umb_001", "mc_001")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
