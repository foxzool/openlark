//! 查询单个工时制度
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/working_hours_type/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::WorkingHoursType;

/// 查询单个工时制度请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 工时制度 ID（必填）
    working_hours_type_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            working_hours_type_id: String::new(),
        }
    }

    /// 设置工时制度 ID（必填）
    pub fn working_hours_type_id(mut self, working_hours_type_id: String) -> Self {
        self.working_hours_type_id = working_hours_type_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.working_hours_type_id.trim(), "工时制度 ID 不能为空");

        let api_endpoint = CorehrApiV1::WorkingHoursTypeGet(self.working_hours_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<GetResponse>::get(endpoint_url);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询工时制度响应数据为空",
        )
        .await
    }
}

/// 查询单个工时制度响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 工时制度信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours_type: Option<WorkingHoursType>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v1/working_hours_types/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/working_hours_types/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "working_hours_type": {
                    "id": "test001",
                    "code": "1",
                    "name": [{ "lang": "zh-CN", "value": "张三" }],
                    "country_region_id_list": ["6890452208593356295"],
                    "default_for_job": true,
                    "active": true
                } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetRequest::new(config)
            .working_hours_type_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let working_hours_type = data.working_hours_type.expect("working_hours_type 应存在");
        assert_eq!(working_hours_type.id, Some("test001".to_string()));
        assert_eq!(
            working_hours_type.country_region_id_list,
            Some(vec!["6890452208593356295".to_string()])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
