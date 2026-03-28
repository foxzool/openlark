//! 获取国家地区列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-country/region-list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::MEETING_ROOM};

/// 获取国家地区列表请求
pub struct ListCountryRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListCountryRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-country/region-list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/meeting_room/country/list
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/country/list", MEETING_ROOM));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取国家地区列表")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListCountryRequest::new(config.clone())
            .query_param("key1".to_string(), "value1".to_string());
        let _ = request;
    }
}
