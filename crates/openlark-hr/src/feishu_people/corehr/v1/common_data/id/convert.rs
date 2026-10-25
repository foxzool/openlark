//! ID 转换
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/common_data.id/convert>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// ID 转换请求
#[derive(Debug, Clone)]
pub struct ConvertRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    request_body: Value,
}

impl ConvertRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ConvertResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ConvertResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        let api_endpoint = CorehrApiV1::CommonDataIdConvert;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<ConvertResponse>::post(endpoint_url).body(self.request_body);
        Transport::request_typed(request, &self.config, Some(option), "ID 转换响应数据为空").await
    }
}

/// ID 转换响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvertResponse {
    /// ID 信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IdInfo>>,
}

/// ID 转换信息条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdInfo {
    /// 传入的 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 目标 ID 值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

impl ApiResponseTrait for ConvertResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/common_data/id/convert
    #[tokio::test]
    async fn test_convert_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/common_data/id/convert"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {"id": "7224321696097404460", "target_id": "7224321696097404461"}
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

        let resp = ConvertRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.as_ref().expect("items 应被解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("7224321696097404460"));
        assert_eq!(items[0].target_id.as_deref(), Some("7224321696097404461"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
