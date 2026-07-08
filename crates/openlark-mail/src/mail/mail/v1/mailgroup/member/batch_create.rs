//! 批量创建邮件组成员
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-member/batch_create>

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Batch Create Mail Group Member Request。
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupMemberBody,
}

/// Batch Create Mail Group Member Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupMemberBody {
    /// 成员列表。
    pub members: Vec<MailGroupMemberItem>,
}

/// Mail Group Member Item。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberItem {
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Batch Create Mail Group Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<BatchCreateMailGroupMemberData>,
}

impl ApiResponseTrait for BatchCreateMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Batch Create Mail Group Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupMemberData {
    /// 结果列表。
    pub results: Vec<MailGroupMemberResult>,
}

/// Mail Group Member Result。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMemberResult {
    /// 成员 ID。
    pub member_id: String,
    /// 状态。
    pub status: String,
}

impl BatchCreateMailGroupMemberRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupMemberBody::default(),
        }
    }

    /// members。
    pub fn members(mut self, members: Vec<MailGroupMemberItem>) -> Self {
        self.body.members = members;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupMemberResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组成员")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量创建邮件组成员", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../mailgroups/{}/members/batch_create → BatchCreateMailGroupMemberResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_batch_create_mail_group_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/members/batch_create",
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

        let resp = BatchCreateMailGroupMemberRequest::new(config, "group_001")
            .members(vec![MailGroupMemberItem {
                member_id: "m1".to_string(),
                member_type: None,
            }])
            .execute()
            .await
            .expect("批量创建邮件组成员应成功");
        let data = resp.data.expect("响应 data 应非空");
        assert_eq!(data.results.len(), 1);
        assert_eq!(data.results[0].member_id, "m1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/members/batch_create"
        );
    }
}
