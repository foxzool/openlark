//! 修改邮件组部分信息
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/patch>

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mail Group Request。
#[derive(Debug, Clone)]
pub struct PatchMailGroupRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: PatchMailGroupBody,
}

/// Patch Mail Group Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchMailGroupBody {
    /// 邮件组地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 邮件组名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮件组描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 谁可发送邮件到此邮件组（ANYONE/ALL_INTERNAL_USERS/ALL_GROUP_MEMBERS/CUSTOM_MEMBERS）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_send_mail: Option<String>,
}

/// Patch Mail Group Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailGroupResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl PatchMailGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: PatchMailGroupBody::default(),
        }
    }

    /// email。
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.body.email = Some(email.into());
        self
    }

    /// name。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = Some(name.into());
        self
    }

    /// description。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// who_can_send_mail（ANYONE/ALL_INTERNAL_USERS/ALL_GROUP_MEMBERS/CUSTOM_MEMBERS）。
    pub fn who_can_send_mail(mut self, who_can_send_mail: impl Into<String>) -> Self {
        self.body.who_can_send_mail = Some(who_can_send_mail.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailGroupResponse> {
        validate_required!(self.mailgroup_id, "mailgroup_id 不能为空");
        let path = format!("/open-apis/mail/v1/mailgroups/{}", self.mailgroup_id);
        let req: ApiRequest<PatchMailGroupResponse> =
            ApiRequest::patch(&path).body(serialize_params(&self.body, "请求")?);

        Transport::request_typed(req, &self.config, Some(option), "修改邮件组部分信息").await
    }
}

impl openlark_core::api::ApiResponseTrait for PatchMailGroupResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../mailgroups/{} → PatchMailGroupResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_mail_group_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/mail/v1/mailgroups/group_001"))
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

        let resp = PatchMailGroupRequest::new(config, "group_001")
            .email("new_group@xxx.xx")
            .name("new name")
            .description("new description")
            .who_can_send_mail("ANYONE")
            .execute()
            .await
            .expect("修改邮件组部分信息应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001"
        );
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(
            body,
            json!({
                "email": "new_group@xxx.xx",
                "name": "new name",
                "description": "new description",
                "who_can_send_mail": "ANYONE"
            })
        );
    }
}
