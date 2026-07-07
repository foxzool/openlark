//! 更新邮件组
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/update>

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use crate::mail::mail::v1::mailgroup::models::{UpdateMailGroupBody, UpdateMailGroupResponse};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新邮件组请求
#[derive(Debug, Clone)]
pub struct UpdateMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
    body: UpdateMailGroupBody,
}

impl UpdateMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
            body: UpdateMailGroupBody::default(),
        }
    }

    /// description。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// only admins send。
    pub fn only_admins_send(mut self, only_admins_send: bool) -> Self {
        self.body.only_admins_send = Some(only_admins_send);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpdateMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupUpdate(self.mail_group_id.clone());
        let mut request = ApiRequest::<UpdateMailGroupResponse>::put(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新邮件组")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新邮件组")
    }
}

impl ApiResponseTrait for UpdateMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../mailgroups/{} → UpdateMailGroupResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_update_mail_group_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/mail/v1/mailgroups/group_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "mail_group_id": "group_001", "updated_at": "1700000000" }
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

        let resp = UpdateMailGroupRequest::new(config, "group_001".to_string())
            .execute()
            .await
            .expect("更新邮件组应成功");
        assert_eq!(resp.mail_group_id, "group_001");
        assert_eq!(resp.updated_at, "1700000000");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001"
        );
    }
}
