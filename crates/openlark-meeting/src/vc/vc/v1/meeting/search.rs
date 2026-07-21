//! 搜索会议记录
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 搜索会议记录请求。
#[derive(Debug, Clone)]
pub struct MeetingSearchRequest {
    config: Arc<Config>,
}

impl MeetingSearchRequest {
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
        let path = "/open-apis/vc/v1/meetings/search".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "搜索会议记录").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = MeetingSearchRequest::new(config);
    }
}
