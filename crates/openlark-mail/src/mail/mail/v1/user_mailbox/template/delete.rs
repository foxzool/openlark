//! 删除邮件模板
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 删除邮件模板请求。
#[derive(Debug, Clone)]
pub struct UserMailboxTemplateDeleteRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    template_id: String,
}

impl UserMailboxTemplateDeleteRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_mailbox_id: String::new(),
            template_id: String::new(),
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

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        validate_required!(self.template_id, "template_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/templates/{}",
            self.user_mailbox_id, self.template_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(path);
        Transport::request_typed(req, &self.config, Some(option), "删除邮件模板").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserMailboxTemplateDeleteRequest::new(config);
    }
}
