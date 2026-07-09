//! 获取单个工作城市信息
//!
//! docPath: <https://open.feishu.cn/document/contact-v3/work_city/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::work_city::models::WorkCityResponse, endpoints::CONTACT_V3_WORK_CITIES,
};

/// 获取单个工作城市信息请求
pub struct GetWorkCityRequest {
    /// 配置信息。
    config: Config,
    /// 工作城市 ID。
    work_city_id: String,
}

impl GetWorkCityRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            work_city_id: String::new(),
        }
    }

    /// 工作城市 ID（路径参数）
    pub fn work_city_id(mut self, work_city_id: impl Into<String>) -> Self {
        self.work_city_id = work_city_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/contact-v3/work_city/get>
    pub async fn execute(self) -> SDKResult<WorkCityResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WorkCityResponse> {
        validate_required!(self.work_city_id, "work_city_id 不能为空");

        // url: GET:/open-apis/contact/v3/work_cities/:work_city_id
        let req: ApiRequest<WorkCityResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_WORK_CITIES, self.work_city_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单个工作城市信息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/contact/v3/work_cities/test001
    #[tokio::test]
    async fn test_get_work_city_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/contact/v3/work_cities/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "work_city": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetWorkCityRequest::new(config)
            .work_city_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
