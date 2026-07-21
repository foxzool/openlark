//! 批量创建邮件组管理员

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

/// 批量创建邮件组管理员的请求。
#[derive(Debug, Clone)]
pub struct BatchCreateMailGroupManagerRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    body: BatchCreateMailGroupManagerBody,
}

/// 批量创建邮件组管理员请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateMailGroupManagerBody {
    /// 管理员 ID 列表。
    pub manager_ids: Vec<String>,
}

/// 批量创建邮件组管理员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateMailGroupManagerResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchCreateMailGroupManagerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchCreateMailGroupManagerRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            body: BatchCreateMailGroupManagerBody::default(),
        }
    }

    /// 设置管理员 ID 列表。
    pub fn manager_ids(mut self, ids: Vec<String>) -> Self {
        self.body.manager_ids = ids;
        self
    }

    /// 执行批量创建邮件组管理员请求。
    pub async fn execute(self) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchCreateMailGroupManagerResponse> {
        validate_required!(self.mailgroup_id.trim(), "mailgroup_id 不能为空");
        validate_required_list!(
            self.body.manager_ids,
            1000,
            "manager_ids 不能为空且不能超过 1000 个"
        );

        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/managers/batch_create",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchCreateMailGroupManagerResponse> =
            ApiRequest::post(&path).body(serialize_params(&self.body, "批量创建邮件组管理员")?);

        Transport::request_typed(req, &self.config, Some(option), "批量创建邮件组管理员").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../mailgroups/{}/managers/batch_create → BatchCreateMailGroupManagerResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_batch_create_mail_group_manager_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/mail/v1/mailgroups/group_001/managers/batch_create",
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

        let resp = BatchCreateMailGroupManagerRequest::new(config, "group_001")
            .manager_ids(vec!["m1".to_string()])
            .execute()
            .await
            .expect("批量创建邮件组管理员应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mail/v1/mailgroups/group_001/managers/batch_create"
        );
    }
}
