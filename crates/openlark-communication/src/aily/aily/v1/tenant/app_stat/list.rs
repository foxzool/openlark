//! 查询应用统计数据
//!
//! docPath:

use std::collections::HashMap;

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_APP_STATS};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 查询应用统计数据请求。
#[derive(Debug, Clone)]
pub struct ListAppStatsRequest {
    config: Config,
    query: HashMap<String, String>,
}

impl ListAppStatsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 添加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(AILY_V1_APP_STATS);
        for (key, value) in self.query {
            req = req.query(key, value);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询应用统计数据")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_stats_issue_194_builder_access() {
        let request = ListAppStatsRequest::new(Config::default())
            .query_param("app_id", "app_123")
            .query_param("start_time", "1780273529");

        assert_eq!(request.query.get("app_id"), Some(&"app_123".to_string()));
        assert_eq!(
            request.query.get("start_time"),
            Some(&"1780273529".to_string())
        );
    }

    #[test]
    fn test_list_app_stats_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_APP_STATS;
        let url = AILY_V1_APP_STATS.to_string();
        assert_eq!(url, "/open-apis/aily/v1/app_stats");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
