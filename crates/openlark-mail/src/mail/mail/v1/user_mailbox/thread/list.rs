//! 列出邮件会话
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 列出邮件会话请求。
#[derive(Debug, Clone)]
pub struct UserMailboxThreadListRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    /// 分页大小（必填，1-20）。
    page_size: i32,
    /// 分页标记（可选，首次请求不填）。
    page_token: Option<String>,
    /// 文件夹 id（可选，INBOX/SENT/SPAM/ARCHIVED/SCHEDULED/TRASH/DRAFT 或自定义文件夹 ID）。
    folder_id: Option<String>,
    /// 是否只查询未读会话（可选）。
    only_unread: Option<bool>,
    /// 标签 id（可选，IMPORTANT/OTHER/FLAGGED 或自定义标签 ID）。
    label_id: Option<String>,
}

impl UserMailboxThreadListRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_mailbox_id: String::new(),
            page_size: 0,
            page_token: None,
            folder_id: None,
            only_unread: None,
            label_id: None,
        }
    }

    /// 设置路径参数 `user_mailbox_id`。
    pub fn user_mailbox_id(mut self, user_mailbox_id: impl Into<String>) -> Self {
        self.user_mailbox_id = user_mailbox_id.into();
        self
    }

    /// 设置分页大小（必填，取值范围 1-20）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    /// 设置分页标记（可选，分页查询结果还有更多项时返回新的 page_token）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置文件夹 id（可选，INBOX/SENT/SPAM/ARCHIVED/SCHEDULED/TRASH/DRAFT 或自定义文件夹 ID）。
    pub fn folder_id(mut self, folder_id: impl Into<String>) -> Self {
        self.folder_id = Some(folder_id.into());
        self
    }

    /// 设置是否只查询未读会话（可选）。
    pub fn only_unread(mut self, only_unread: bool) -> Self {
        self.only_unread = Some(only_unread);
        self
    }

    /// 设置标签 id（可选，IMPORTANT/OTHER/FLAGGED 或自定义标签 ID）。
    pub fn label_id(mut self, label_id: impl Into<String>) -> Self {
        self.label_id = Some(label_id.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        if !(1..=20).contains(&self.page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 不能为空，取值范围 1-20",
            ));
        }
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/threads",
            self.user_mailbox_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(path)
            .query("page_size", self.page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("folder_id", self.folder_id)
            .query_opt("only_unread", self.only_unread.map(|v| v.to_string()))
            .query_opt("label_id", self.label_id);
        Transport::request_typed(req, &self.config, Some(option), "列出邮件会话").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserMailboxThreadListRequest::new(config);
    }

    /// 端到端：GET .../user_mailboxes/{umb}/threads?page_size=... → 官方查询参数形状（page_size 必填 + 可选过滤）。
    #[tokio::test]
    async fn test_thread_list_sends_official_query() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/mail/v1/user_mailboxes/umb_001/threads"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{ "id": "xx", "body_preview": "hello world" }],
                    "page_token": "xxx",
                    "has_more": true
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

        let resp = UserMailboxThreadListRequest::new(config)
            .user_mailbox_id("umb_001")
            .page_size(20)
            .folder_id("INBOX")
            .only_unread(true)
            .execute()
            .await
            .expect("列出邮件会话应成功");
        assert_eq!(resp["items"][0]["id"], "xx");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=20"));
        assert!(query.contains("folder_id=INBOX"));
        assert!(query.contains("only_unread=true"));
    }

    /// page_size 缺省或越界应快速失败，不发请求。
    #[tokio::test]
    async fn test_thread_list_rejects_invalid_page_size() {
        let config = Arc::new(Config::default());
        let err = UserMailboxThreadListRequest::new(config)
            .user_mailbox_id("umb_001")
            .execute()
            .await
            .expect_err("缺少 page_size 应报错");
        assert!(err.to_string().contains("page_size"));

        let config = Arc::new(Config::default());
        let err = UserMailboxThreadListRequest::new(config)
            .user_mailbox_id("umb_001")
            .page_size(21)
            .execute()
            .await
            .expect_err("page_size 越界应报错");
        assert!(err.to_string().contains("page_size"));
    }
}
