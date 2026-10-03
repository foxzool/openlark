//! 删除用户邮箱别名
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-alias/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mailbox Alias Request。
#[derive(Debug, Clone)]
pub struct DeleteMailboxAliasRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    alias_id: String,
}

/// Delete Mailbox Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailboxAliasResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteMailboxAliasRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            alias_id: alias_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailboxAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailboxAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/aliases/{}",
            self.user_mailbox_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailboxAliasResponse> = ApiRequest::delete(&path);

        Transport::request_typed(req, &self.config, Some(option), "删除用户邮箱别名").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../user_mailboxes/{id}/aliases/{alias_id} → DeleteMailboxAliasResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_delete_mailbox_alias_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/mb_001/aliases/alias_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "deleted": true } }
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

        let resp = DeleteMailboxAliasRequest::new(config, "mb_001", "alias_001")
            .execute()
            .await
            .expect("删除用户邮箱别名应成功");
        assert_eq!(resp.data.unwrap()["deleted"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/user_mailboxes/mb_001/aliases/alias_001"
        );
    }
}
