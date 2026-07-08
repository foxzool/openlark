//! 修改邮件组部分信息
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/patch>

use crate::common::api_utils::{extract_response_data, serialize_params};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
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
    /// 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// owner 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
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

    /// description。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// owner。
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.body.owner = Some(owner.into());
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
        let path = format!("/open-apis/mail/v1/mailgroups/{}", self.mailgroup_id);
        let req: ApiRequest<PatchMailGroupResponse> =
            ApiRequest::patch(&path).body(serialize_params(&self.body, "请求")?);

        let response = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(response, "修改邮件组部分信息")
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
    }
}
