//! 更新收信规则

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新收信规则的请求。
#[derive(Debug, Clone)]
pub struct UpdateMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    rule_id: String,
}

/// 更新收信规则的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMailboxRuleResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateMailboxRuleRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            rule_id: rule_id.into(),
        }
    }

    /// 执行更新收信规则请求。
    pub async fn execute(self) -> SDKResult<UpdateMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateMailboxRuleResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/rules/{}",
            self.user_mailbox_id, self.rule_id
        );
        let req: ApiRequest<UpdateMailboxRuleResponse> = ApiRequest::put(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("更新收信规则", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PUT .../user_mailboxes/{umb}/rules/{rule} → 强类型 UpdateMailboxRuleResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_update_mailbox_rule_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/rules/r_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "rule_id": "r_001" } }
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

        let resp = UpdateMailboxRuleRequest::new(config, "umb_001", "r_001")
            .execute()
            .await
            .expect("更新收信规则应成功");
        assert_eq!(resp.data.unwrap()["rule_id"], "r_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/umb_001/rules/r_001"
        );
    }
}
