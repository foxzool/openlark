//! 删除邮件组
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete>

use crate::common::api_endpoints::MailApiV1;
use crate::mail::mail::v1::mailgroup::models::DeleteMailGroupResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除邮件组请求
#[derive(Debug, Clone)]
pub struct DeleteMailGroupRequest {
    config: Arc<Config>,
    mail_group_id: String,
}

impl DeleteMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mail_group_id: String) -> Self {
        Self {
            config,
            mail_group_id,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteMailGroupResponse> {
        validate_required!(self.mail_group_id.trim(), "邮件组ID不能为空");

        let api_endpoint = MailApiV1::MailGroupDelete(self.mail_group_id.clone());
        let request = ApiRequest::<DeleteMailGroupResponse>::delete(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除邮件组",
        )
        .await
    }
}

impl ApiResponseTrait for DeleteMailGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../mailgroups/{} → DeleteMailGroupResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_delete_mail_group_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/mail/v1/mailgroups/group_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "success": true }
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

        let resp = DeleteMailGroupRequest::new(config, "group_001".to_string())
            .execute()
            .await
            .expect("删除邮件组应成功");
        assert!(resp.success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001"
        );
    }
}
