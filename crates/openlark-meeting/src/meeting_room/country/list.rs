//! 获取国家地区列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-country/region-list>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::meeting_room::responses::ListCountryResponse;
use crate::{common::api_utils::extract_response_data, endpoints::MEETING_ROOM};

/// 获取国家地区列表请求
pub struct ListCountryRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListCountryRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/api-reference/obtain-country/region-list>
    pub async fn execute(self) -> SDKResult<ListCountryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListCountryResponse> {
        // url: GET:/open-apis/meeting_room/country/list
        let mut req: ApiRequest<ListCountryResponse> =
            ApiRequest::get(format!("{MEETING_ROOM}/country/list"));
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

    /// 端到端：GET .../meeting_room/country/list → ListCountryResponse。
    #[tokio::test]
    async fn test_list_country_returns_typed_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/meeting_room/country/list"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "countries": [
                        { "country_id": "1814991", "name": "中国" }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListCountryRequest::new(config)
            .execute()
            .await
            .expect("获取国家地区列表应成功");
        assert_eq!(resp.countries[0].country_id, "1814991");
        assert_eq!(resp.countries[0].name.as_deref(), Some("中国"));
    }
}
