//! 批量删除邮件组权限成员

use crate::common::api_utils::{extract_response_data, serialize_params};
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

/// Batch Delete Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct BatchDeleteMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchDeleteMailGroupPermissionMemberBody,
}

/// Batch Delete Mail Group Permission Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteMailGroupPermissionMemberBody {
    /// permission_member_ids 字段。
    pub permission_member_ids: Vec<String>,
}

/// Batch Delete Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeleteMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchDeleteMailGroupPermissionMemberBody::default(),
        }
    }

    /// permission member ids。
    pub fn permission_member_ids(mut self, ids: Vec<String>) -> Self {
        self.body.permission_member_ids = ids;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupPermissionMemberResponse> {
        validate_required!(self.mailgroup_id.trim(), "mailgroup_id 不能为空");
        validate_required_list!(
            self.body.permission_member_ids,
            1000,
            "permission_member_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_delete",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchDeleteMailGroupPermissionMemberResponse> =
            ApiRequest::delete(&path).body(serialize_params(&self.body, "批量删除邮件组权限成员")?);

        let response = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(response, "批量删除邮件组权限成员")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../mailgroups/{}/permission_members/batch_delete → BatchDeleteMailGroupPermissionMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_batch_delete_mail_group_permission_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/permission_members/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
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

        let resp = BatchDeleteMailGroupPermissionMemberRequest::new(config, "group_001")
            .permission_member_ids(vec!["pm1".to_string()])
            .execute()
            .await
            .expect("批量删除邮件组权限成员应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/permission_members/batch_delete"
        );
    }
}
