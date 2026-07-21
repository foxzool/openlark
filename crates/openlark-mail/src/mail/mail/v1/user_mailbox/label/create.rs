//! 创建标签
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

/// 创建标签请求。
#[derive(Debug, Clone)]
pub struct UserMailboxLabelCreateRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

impl UserMailboxLabelCreateRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_mailbox_id: String::new(),
        }
    }

    /// 设置路径参数 `user_mailbox_id`。
    pub fn user_mailbox_id(mut self, user_mailbox_id: impl Into<String>) -> Self {
        self.user_mailbox_id = user_mailbox_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.user_mailbox_id, "user_mailbox_id 不能为空");
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/labels",
            self.user_mailbox_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "创建标签").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserMailboxLabelCreateRequest::new(config);
    }
}
