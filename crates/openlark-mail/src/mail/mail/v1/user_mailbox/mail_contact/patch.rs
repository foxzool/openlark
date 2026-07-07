//! 修改邮箱联系人

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mail Contact Request。
#[derive(Debug, Clone)]
pub struct PatchMailContactRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    mail_contact_id: String,
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
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailContactResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailContactResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts/{}",
            self.user_mailbox_id, self.mail_contact_id
        );
        let req: ApiRequest<PatchMailContactResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("修改邮箱联系人", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../user_mailboxes/{umb}/mail_contacts/{mc} → 强类型 PatchMailContactResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_mail_contact_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/mail_contacts/mc_001",
            ))
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
}
