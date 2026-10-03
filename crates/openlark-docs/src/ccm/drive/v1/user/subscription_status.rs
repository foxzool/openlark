//! 查询用户云文档事件订阅状态
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

use crate::common::api_endpoints::DriveApi;

/// 查询用户云文档事件订阅状态请求。
#[derive(Debug, Clone)]
pub struct UserSubscriptionStatusRequest {
    config: Arc<Config>,
}

impl UserSubscriptionStatusRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> = DriveApi::UserSubscriptionStatus.to_request();
        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "查询用户云文档事件订阅状态",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserSubscriptionStatusRequest::new(config);
    }
}
