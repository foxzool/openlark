//! 获取邮件组权限成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Get Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct GetMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    permission_member_id: String,
}

/// Get Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<PermissionMemberData>,
}

impl ApiResponseTrait for GetMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Permission Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberData {
    /// 权限成员 ID。
    pub permission_member_id: String,
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    pub member_type: String,
}

impl GetMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        permission_member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            permission_member_id: permission_member_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
            self.mailgroup_id, self.permission_member_id
        );
        let req: ApiRequest<GetMailGroupPermissionMemberResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件组权限成员", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../mailgroups/{}/permission_members/{} → GetMailGroupPermissionMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_mail_group_permission_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/permission_members/pm_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "permission_member_id": "pm_001",
                        "member_id": "m1",
                        "member_type": "USER"
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

        let resp = GetMailGroupPermissionMemberRequest::new(config, "group_001", "pm_001")
            .execute()
            .await
            .expect("获取邮件组权限成员应成功");
        let data = resp.data.expect("响应 data 应非空");
        assert_eq!(data.permission_member_id, "pm_001");
        assert_eq!(data.member_id, "m1");
        assert_eq!(data.member_type, "USER");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/permission_members/pm_001"
        );
    }
}
