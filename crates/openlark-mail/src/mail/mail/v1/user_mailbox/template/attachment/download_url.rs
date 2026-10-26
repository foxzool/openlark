//! 获取模板附件下载链接
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required, validate_required_list,
};
use std::sync::Arc;

/// 获取模板附件下载链接请求。
#[derive(Debug, Clone)]
pub struct UserMailboxTemplateAttachmentDownloadUrlRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    template_id: String,
    /// 待获取下载链接的附件 ID 列表（必填，1-20 个）。
    attachment_ids: Vec<String>,
}

impl UserMailboxTemplateAttachmentDownloadUrlRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_mailbox_id: String::new(),
            template_id: String::new(),
            attachment_ids: Vec::new(),
        }
    }

    /// 设置路径参数 `user_mailbox_id`。
    pub fn user_mailbox_id(mut self, user_mailbox_id: impl Into<String>) -> Self {
        self.user_mailbox_id = user_mailbox_id.into();
        self
    }

    /// 设置路径参数 `template_id`。
    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.template_id = template_id.into();
        self
    }

    /// 设置附件 ID 列表（必填，1-20 个，可通过获取个人邮件模板详情接口的 attachments.id 获取）。
    pub fn attachment_ids(mut self, attachment_ids: Vec<String>) -> Self {
        self.attachment_ids = attachment_ids;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.template_id, "template_id 不能为空");
        validate_required_list!(
            self.attachment_ids,
            20,
            "attachment_ids 不能为空且不能超过 20 个"
        );
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/templates/{}/attachments/download_url",
            self.user_mailbox_id, self.template_id
        );
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(path).query("attachment_ids", self.attachment_ids.join(","));
        Transport::request_typed(req, &self.config, Some(option), "获取模板附件下载链接").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserMailboxTemplateAttachmentDownloadUrlRequest::new(config);
    }

    /// 端到端：GET .../templates/{tpl}/attachments/download_url?attachment_ids=... → 官方查询参数形状。
    #[tokio::test]
    async fn test_template_download_url_sends_official_query() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/mail/v1/user_mailboxes/umb_001/templates/tpl_001/attachments/download_url",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "download_urls": [
                        { "attachment_id": "att_001", "download_url": "https://example.com/file.zip" }
                    ],
                    "failed_reasons": []
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

        let resp = UserMailboxTemplateAttachmentDownloadUrlRequest::new(config)
            .user_mailbox_id("umb_001")
            .template_id("tpl_001")
            .attachment_ids(vec!["att_001".to_string()])
            .execute()
            .await
            .expect("获取模板附件下载链接应成功");
        assert_eq!(resp["download_urls"][0]["attachment_id"], "att_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 官方查询参数为 attachment_ids（复数，数组 1-20 个）
        assert_eq!(received[0].url.query(), Some("attachment_ids=att_001"));
    }

    /// attachment_ids 为空应快速失败，不发请求。
    #[tokio::test]
    async fn test_template_download_url_rejects_empty_attachment_ids() {
        let config = Arc::new(Config::default());
        let err = UserMailboxTemplateAttachmentDownloadUrlRequest::new(config)
            .user_mailbox_id("umb_001")
            .template_id("tpl_001")
            .execute()
            .await
            .expect_err("attachment_ids 为空应报错");
        assert!(err.to_string().contains("attachment_ids"));
    }
}
