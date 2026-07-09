//! 更新 Offer 状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer/offer_status>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新 Offer 状态请求
#[derive(Debug, Clone)]
pub struct OfferStatusRequest {
    /// 配置信息
    config: Config,
    offer_id: String,
    request_body: Option<Value>,
}

impl OfferStatusRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `offer_id`。
    pub fn offer_id(mut self, offer_id: String) -> Self {
        self.offer_id = offer_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OfferStatusResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OfferStatusResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.offer_id.trim(), "Offer ID 不能为空");

        let api_endpoint = HireApiV1::OfferOfferStatus(self.offer_id);
        let mut request = ApiRequest::<OfferStatusResponse>::patch(api_endpoint.to_url());
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新 Offer 状态响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新 Offer 状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OfferStatusResponse {}

impl ApiResponseTrait for OfferStatusResponse {
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

    /// 端到端：PATCH /open-apis/hire/v1/offers/test001/offer_status
    #[tokio::test]
    async fn test_offer_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/hire/v1/offers/test001/offer_status"))
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

        OfferStatusRequest::new(config)
            .offer_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
