//! 批量删除一次性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/lump_sum_payment/batch_remove>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量删除一次性支付记录请求
#[derive(Debug, Clone)]
pub struct BatchRemoveRequest {
    /// 配置信息
    config: Config,
}

impl BatchRemoveRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchRemoveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchRemoveResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::LumpSumPaymentBatchRemove;
        let request = ApiRequest::<BatchRemoveResponse>::post(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量删除一次性支付记录响应数据为空",
        )
        .await
    }
}

/// 批量删除一次性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResponse {
    /// 删除结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchRemoveResult>>,
}

/// 批量删除结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResult {
    /// 记录 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for BatchRemoveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_batch_remove_lump_sum_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/lump_sum_payment/batch_remove",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = BatchRemoveRequest::new(config)
            .execute()
            .await
            .expect("批量删除一次性支付应成功");

        assert!(data.results.is_none());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/lump_sum_payment/batch_remove"
        );
    }
}
