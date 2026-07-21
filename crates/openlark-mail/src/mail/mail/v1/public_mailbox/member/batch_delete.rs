//! 批量删除公共邮箱成员

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除公共邮箱成员的请求。
#[derive(Debug, Clone)]
pub struct BatchDeletePublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

/// 批量删除公共邮箱成员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeletePublicMailboxMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeletePublicMailboxMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeletePublicMailboxMemberRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    /// 执行批量删除公共邮箱成员请求。
    pub async fn execute(self) -> SDKResult<BatchDeletePublicMailboxMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeletePublicMailboxMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/batch_delete",
            self.public_mailbox_id
        );
        let req: ApiRequest<BatchDeletePublicMailboxMemberResponse> = ApiRequest::delete(&path);

        Transport::request_typed(req, &self.config, Some(option), "批量删除公共邮箱成员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../public_mailboxes/{id}/members/batch_delete → BatchDeletePublicMailboxMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_batch_delete_public_mailbox_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/public_mailboxes/mb_001/members/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "success": true } }
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

        let resp = BatchDeletePublicMailboxMemberRequest::new(config, "mb_001")
            .execute()
            .await
            .expect("批量删除公共邮箱成员应成功");
        assert_eq!(resp.data.unwrap()["success"].as_bool(), Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/public_mailboxes/mb_001/members/batch_delete"
        );
    }
}
