//! 更新 Offer 申请表自定义字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/offer_custom_field/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hire::hire::common_models::OfferCustomFieldOperationResult;

/// 更新 Offer 申请表自定义字段请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    offer_custom_field_id: Option<String>,
    request_body: Option<Value>,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            offer_custom_field_id: None,
            request_body: None,
        }
    }

    /// 设置 `offer_custom_field_id`。
    pub fn offer_custom_field_id(mut self, offer_custom_field_id: impl Into<String>) -> Self {
        self.offer_custom_field_id = Some(offer_custom_field_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        let offer_custom_field_id = self.offer_custom_field_id.unwrap_or_default();
        validate_required!(
            offer_custom_field_id.trim(),
            "offer_custom_field_id 不能为空"
        );

        let mut request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/offer_custom_fields/{offer_custom_field_id}"
        ));
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新 Offer 申请表自定义字段响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新 Offer 申请表自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateResponse {
    #[serde(flatten)]
    /// `operation` 字段。
    pub operation: OfferCustomFieldOperationResult,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/hire/v1/offer_custom_fields/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/hire/v1/offer_custom_fields/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "operation": {  } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateRequest::new(config)
            .offer_custom_field_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
