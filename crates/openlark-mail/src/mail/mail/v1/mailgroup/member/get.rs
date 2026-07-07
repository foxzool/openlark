//! 查询指定邮件组成员
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

/// 查询指定邮件组成员的请求。
#[derive(Debug, Clone)]
pub struct GetMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

/// 查询指定邮件组成员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMailGroupMemberRequest {
    /// 创建请求实例。
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

    /// 执行查询指定邮件组成员请求。
    pub async fn execute(self) -> SDKResult<GetMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<GetMailGroupMemberResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询指定邮件组成员", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../mailgroups/{}/members/{} → GetMailGroupMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_mail_group_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
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

        let resp = GetMailGroupMemberRequest::new(config, "group_001", "member_001")
            .execute()
            .await
            .expect("查询指定邮件组成员应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/members/member_001"
        );
    }
}
