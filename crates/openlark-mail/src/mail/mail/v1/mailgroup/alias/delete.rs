//! 删除邮件组别名
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mail Group Alias Request。
#[derive(Debug, Clone)]
pub struct DeleteMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    alias_id: String,
}

/// Delete Mail Group Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupAliasResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupAliasResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

impl DeleteMailGroupAliasRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        alias_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            alias_id: alias_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases/{}",
            self.mailgroup_id, self.alias_id
        );
        let req: ApiRequest<DeleteMailGroupAliasResponse> = ApiRequest::delete(&path);

        Transport::request_typed(req, &self.config, Some(option), "删除邮件组别名").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../mailgroups/{}/aliases/{} → DeleteMailGroupAliasResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_delete_mail_group_alias_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/aliases/alias_001",
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

        let resp = DeleteMailGroupAliasRequest::new(config, "group_001", "alias_001")
            .execute()
            .await
            .expect("删除邮件组别名应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/aliases/alias_001"
        );
    }
}
