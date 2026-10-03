//! 列出用户邮箱别名

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// List Mailbox Alias Request。
#[derive(Debug, Clone)]
pub struct ListMailboxAliasRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
}

/// List Mailbox Alias Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMailboxAliasResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListMailboxAliasResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListMailboxAliasRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListMailboxAliasResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListMailboxAliasResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/aliases",
            self.user_mailbox_id
        );
        let req: ApiRequest<ListMailboxAliasResponse> = ApiRequest::get(&path);

        Transport::request_typed(req, &self.config, Some(option), "列出用户邮箱别名").await
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
        let _config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListMailboxAliasRequest::new(arc_config.clone(), "test".to_string());
        let _ = request;
    }
}
