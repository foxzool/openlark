//! 批量创建邮件组权限成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-permission_member/batch_create>

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

/// Batch Create Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupPermissionMemberBody,
}

/// Batch Create Mail Group Permission Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupPermissionMemberBody {
    /// 本次添加的邮件组权限成员列表（1～200 个）。
    pub items: Vec<PermissionMemberItem>,
}

/// Permission Member Item（mailgroup.permission_member）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionMemberItem {
    /// 权限组内成员唯一标识（请求体中不用填，仅响应返回）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_member_id: Option<String>,
    /// 租户内用户的唯一标识（当成员类型是 USER 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 租户内部门的唯一标识（当成员类型是 DEPARTMENT 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 成员邮箱地址（当成员类型是 MAIL_GROUP/PUBLIC_MAILBOX 时有值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 成员类型（USER/DEPARTMENT/MAIL_GROUP/PUBLIC_MAILBOX）。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Batch Create Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<BatchCreatePermissionMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Batch Create Permission Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreatePermissionMemberData {
    /// 结果列表。
    pub results: Vec<PermissionMemberResult>,
}

/// Permission Member Result。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberResult {
    /// 成员 ID。
    pub member_id: String,
    /// 状态。
    pub status: String,
}

impl BatchCreateMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupPermissionMemberBody::default(),
        }
    }

    /// items。
    pub fn items(mut self, items: Vec<PermissionMemberItem>) -> Self {
        self.body.items = items;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupPermissionMemberResponse> {
        validate_required!(self.mailgroup_id, "mailgroup_id 不能为空");
        validate_required_list!(self.body.items, 200, "items 不能为空且不能超过 200 个");
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupPermissionMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "请求")?);

        Transport::request_typed(req, &self.config, Some(option), "批量创建邮件组权限成员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../mailgroups/{}/permission_members/batch_create → BatchCreateMailGroupPermissionMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_batch_create_mail_group_permission_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/permission_members/batch_create",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "results": [
                            { "member_id": "m1", "status": "success" }
                        ]
                    }
                }
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

        let resp = BatchCreateMailGroupPermissionMemberRequest::new(config, "group_001")
            .items(vec![PermissionMemberItem {
                user_id: Some("ou_123".to_string()),
                member_type: Some("USER".to_string()),
                ..Default::default()
            }])
            .execute()
            .await
            .expect("批量创建邮件组权限成员应成功");
        let data = resp.data.expect("响应 data 应非空");
        assert_eq!(data.results.len(), 1);
        assert_eq!(data.results[0].member_id, "m1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/permission_members/batch_create"
        );
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(
            body,
            json!({ "items": [ { "user_id": "ou_123", "type": "USER" } ] })
        );
    }
}
