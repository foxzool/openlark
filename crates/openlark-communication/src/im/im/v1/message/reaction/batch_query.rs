//! 批量获取消息表情回复
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 批量获取消息表情回复请求。
#[derive(Debug, Clone)]
pub struct MessageReactionBatchQueryRequest {
    config: Arc<Config>,
}

impl MessageReactionBatchQueryRequest {
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
        let path = "/open-apis/im/v1/messages/reactions/batch_query".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "批量获取消息表情回复").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = MessageReactionBatchQueryRequest::new(config);
    }
}
