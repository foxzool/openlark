//! 删除邮件组成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete>

use crate::common::api_utils::extract_response_data;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mail Group Member Request。
#[derive(Debug, Clone)]
pub struct DeleteMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

/// Delete Mail Group Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupMemberRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            member_id: member_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<DeleteMailGroupMemberResponse> = ApiRequest::delete(&path);

        let response = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(response, "删除邮件组成员")
    }
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../mailgroups/{}/members/{} → DeleteMailGroupMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_delete_mail_group_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/members/member_001",
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

        let resp = DeleteMailGroupMemberRequest::new(config, "group_001", "member_001")
            .execute()
            .await
            .expect("删除邮件组成员应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/members/member_001"
        );
    }
}
