//! 批量查询成本中心版本信息
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 批量查询成本中心版本信息请求。
#[derive(Debug, Clone)]
pub struct CostCenterQueryMultiTimelineRequest {
    config: Arc<Config>,
}

impl CostCenterQueryMultiTimelineRequest {
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
        let path = "/open-apis/corehr/v2/cost_centers/query_multi_timeline".to_string();
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "批量查询成本中心版本信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CostCenterQueryMultiTimelineRequest::new(config);
    }
}
