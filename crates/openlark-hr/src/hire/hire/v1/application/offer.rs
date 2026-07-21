//! 获取 Offer 信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/offer>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::v1::offer::get::OfferDetail;

/// 获取 Offer 信息请求
#[derive(Debug, Clone)]
pub struct OfferRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl OfferRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OfferResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OfferResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let api_endpoint = HireApiV1::ApplicationOffer(self.application_id.clone());
        let request = ApiRequest::<OfferResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 Offer 信息响应数据为空",
        )
        .await
    }
}

/// 获取 Offer 信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `offer` 字段。
    pub offer: Option<OfferDetail>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for OfferResponse {
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

    /// 端到端：GET /open-apis/hire/v1/applications/test001/offer
    #[tokio::test]
    async fn test_offer_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/applications/test001/offer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        OfferRequest::new(config)
            .application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
