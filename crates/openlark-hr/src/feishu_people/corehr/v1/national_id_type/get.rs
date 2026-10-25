//! 查询单个国家证件类型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/national_id_type/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::NationalIdType;

/// 查询单个国家证件类型请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 国家证件类型 ID（必填）
    national_id_type_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            national_id_type_id: String::new(),
        }
    }

    /// 设置国家证件类型 ID（必填）
    pub fn national_id_type_id(mut self, national_id_type_id: String) -> Self {
        self.national_id_type_id = national_id_type_id;
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

        validate_required!(self.national_id_type_id.trim(), "国家证件类型 ID 不能为空");

        let api_endpoint = CorehrApiV1::NationalIdTypeGet(self.national_id_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<GetResponse>::get(endpoint_url);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询国家证件类型响应数据为空",
        )
        .await
    }
}

/// 查询单个国家证件类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 证件类型信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_type: Option<NationalIdType>,
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

    /// 端到端：GET /open-apis/corehr/v1/national_id_types/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/national_id_types/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "national_id_type": {
                    "id": "test001",
                    "country_region_id": "6862995747139225096",
                    "name": [{ "lang": "zh-CN", "value": "张三" }],
                    "active": true,
                    "validation_rule": "^\\d{9}$",
                    "code": "AUS-TFN",
                    "identification_type": { "enum_name": "type_1" }
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
            .national_id_type_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let national_id_type = data.national_id_type.expect("national_id_type 应存在");
        assert_eq!(national_id_type.id, Some("test001".to_string()));
        assert_eq!(
            national_id_type.country_region_id,
            Some("6862995747139225096".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
