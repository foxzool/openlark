//! 创建邮件组别名
//! docPath: <https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup-alias/create>

use crate::common::api_utils::serialize_params;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Create Mail Group Alias Request。
#[derive(Debug, Clone)]
pub struct CreateMailGroupAliasRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: CreateMailGroupAliasBody,
}

/// Create Mail Group Alias Body。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateMailGroupAliasBody {
    /// 邮箱别名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

/// Create Mail Group Alias Response（响应 `data` 内容：mailgroup_alias 信封）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailGroupAliasResponse {
    /// 邮件组别名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailgroup_alias: Option<MailGroupAliasData>,
}

impl ApiResponseTrait for CreateMailGroupAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Mail Group Alias Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAliasData {
    /// 主邮箱地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    /// 邮箱别名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alias: Option<String>,
}

impl CreateMailGroupAliasRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: CreateMailGroupAliasBody::default(),
        }
    }

    /// 邮箱别名。
    pub fn email_alias(mut self, email_alias: impl Into<String>) -> Self {
        self.body.email_alias = Some(email_alias.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateMailGroupAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateMailGroupAliasResponse> {
        validate_required!(self.mailgroup_id, "mailgroup_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/aliases",
            self.mailgroup_id
        );
        let req: ApiRequest<CreateMailGroupAliasResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "创建邮件组别名")?);

        Transport::request_typed(req, &self.config, Some(option), "创建邮件组别名").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_builder_basic() {
        let arc_config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateMailGroupAliasRequest::new(arc_config.clone(), "test".to_string())
            .email_alias("alias@xx.xxx");
        assert_eq!(request.body.email_alias.as_deref(), Some("alias@xx.xxx"));
        let value = serde_json::to_value(&request.body).expect("序列化 body");
        assert_eq!(value, serde_json::json!({ "email_alias": "alias@xx.xxx" }));
    }

    /// 端到端：POST .../mailgroups/{}/aliases → 请求体 email_alias + 响应 data.mailgroup_alias。
    #[tokio::test]
    async fn test_create_mail_group_alias_wire_format() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mail/v1/mailgroups/group_001/aliases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "mailgroup_alias": {
                        "primary_email": "primary@xx.xxx",
                        "email_alias": "alias@xx.xxx"
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

        let resp = CreateMailGroupAliasRequest::new(config, "group_001")
            .email_alias("alias@xx.xxx")
            .execute()
            .await
            .expect("创建邮件组别名应成功");
        let alias = resp.mailgroup_alias.expect("mailgroup_alias 应非空");
        assert_eq!(alias.primary_email.as_deref(), Some("primary@xx.xxx"));
        assert_eq!(alias.email_alias.as_deref(), Some("alias@xx.xxx"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(body, json!({ "email_alias": "alias@xx.xxx" }));
    }
}
