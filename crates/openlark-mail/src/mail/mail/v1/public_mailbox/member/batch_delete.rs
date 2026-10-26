//! 批量删除公共邮箱成员

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::models::BatchDeletePublicMailboxMemberBody;

/// 批量删除公共邮箱成员的请求。
#[derive(Debug, Clone)]
pub struct BatchDeletePublicMailboxMemberRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
    body: BatchDeletePublicMailboxMemberBody,
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
            body: BatchDeletePublicMailboxMemberBody::default(),
        }
    }

    /// 设置本次调用删除的公共邮箱成员 ID 列表（1～200 个）。
    pub fn member_id_list(mut self, member_id_list: Vec<String>) -> Self {
        self.body.member_id_list = member_id_list;
        self
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
        validate_required!(self.public_mailbox_id, "public_mailbox_id 不能为空");
        validate_required_list!(
            self.body.member_id_list,
            200,
            "member_id_list 不能为空且不能超过 200 个"
        );
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/members/batch_delete",
            self.public_mailbox_id
        );
        let req: ApiRequest<BatchDeletePublicMailboxMemberResponse> =
            ApiRequest::delete(&path).body(serialize_params(&self.body, "批量删除公共邮箱成员")?);

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
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/public_mailboxes/mb_001/members/batch_delete",
            ))
            .and(body_json(json!({ "member_id_list": ["mem_001"] })))
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
            .member_id_list(vec!["mem_001".to_string()])
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

    /// 失败路径：member_id_list 为空时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_batch_delete_public_mailbox_member_rejects_empty_list() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = BatchDeletePublicMailboxMemberRequest::new(config, "mb_001")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
