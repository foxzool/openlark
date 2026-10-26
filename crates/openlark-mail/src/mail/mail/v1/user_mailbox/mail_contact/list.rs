//! 列出邮箱联系人

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// List Mail Contact Request。
#[derive(Debug, Clone)]
pub struct ListMailContactRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    /// 分页大小（必填，取值范围 1～20）。
    page_size: u32,
    /// 分页标记，第一次请求不填。
    page_token: Option<String>,
}

/// List Mail Contact Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailContactResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListMailContactRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            page_size: 0,
            page_token: None,
        }
    }

    /// 设置分页大小（必填，取值范围 1～20）。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    /// 设置分页标记，第一次请求不填。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListMailContactResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailContactResponse> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        // page_size 必填（数值字段无 Validatable 实现，按 logout 范式手动校验）
        if !(1..=20).contains(&self.page_size) {
            return Err(error::validation_error(
                "page_size",
                "page_size 不能为空，取值范围 1～20",
            ));
        }
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts",
            self.user_mailbox_id
        );
        let req: ApiRequest<ListMailContactResponse> = ApiRequest::get(&path)
            .query("page_size", self.page_size.to_string())
            .query_opt("page_token", self.page_token);

        Transport::request_typed(req, &self.config, Some(option), "列出邮箱联系人").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../user_mailboxes/{umb}/mail_contacts?page_size=20 → ListMailContactResponse 解析。
    #[tokio::test]
    async fn test_list_mail_contact_sends_required_query_params() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/mail_contacts",
            ))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "tok_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "items": [] } }
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

        let resp = ListMailContactRequest::new(config, "umb_001")
            .page_size(20)
            .page_token("tok_001")
            .execute()
            .await
            .expect("列出邮箱联系人应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    /// 失败路径：未设置 page_size 时本地校验报错，不发起请求。
    #[tokio::test]
    async fn test_list_mail_contact_rejects_missing_page_size() {
        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .build(),
        );

        let result = ListMailContactRequest::new(config, "umb_001")
            .execute()
            .await;
        assert!(result.is_err());
    }
}
