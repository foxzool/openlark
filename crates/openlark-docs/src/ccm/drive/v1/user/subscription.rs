//! 订阅用户云文档事件
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

use crate::common::api_endpoints::DriveApi;

/// 订阅用户云文档事件请求。
#[derive(Debug, Clone)]
pub struct UserSubscriptionRequest {
    config: Arc<Config>,
}

impl UserSubscriptionRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
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
        let req: ApiRequest<serde_json::Value> = DriveApi::UserSubscription.to_request().body(body);
        Transport::request_typed(req, &self.config, Some(option), "订阅用户云文档事件").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = UserSubscriptionRequest::new(config);
    }
}
