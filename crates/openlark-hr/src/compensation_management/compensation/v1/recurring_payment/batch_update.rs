//! 批量更正经常性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/recurring_payment/batch_update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量更正经常性支付记录请求
#[derive(Debug, Clone)]
pub struct BatchUpdateRequest {
    /// 配置信息
    config: Config,
}

impl BatchUpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchUpdateResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::RecurringPaymentBatchUpdate;
        let request = ApiRequest::<BatchUpdateResponse>::post(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量更正经常性支付记录响应数据为空",
        )
        .await
    }
}

/// 批量更正经常性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateResponse {
    /// 每条记录的操作结果列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_results: Option<Vec<BatchUpdateResult>>,
}

/// 批量更正操作结果（官方 recurring_payment_operate_result）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateResult {
    /// 操作记录的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 操作的记录的 unique_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 操作结果状态码（0 表示成功，21280001 等为具体错误）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 操作结果描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ApiResponseTrait for BatchUpdateResponse {
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
    async fn test_batch_update_recurring_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.operate_results[]（含 code/unique_id/message）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"operate_results": [{"id": "7390583861280556588", "unique_id": "7390583861280556588", "code": 21270202, "message": "uqniue id conflict"}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_update",
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

        let data = BatchUpdateRequest::new(config)
            .execute()
            .await
            .expect("批量更新经常性支付应成功");

        let results = data.operate_results.expect("operate_results 应存在");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_deref(), Some("7390583861280556588"));
        assert_eq!(results[0].unique_id.as_deref(), Some("7390583861280556588"));
        assert_eq!(results[0].code, Some(21270202));
        assert_eq!(results[0].message.as_deref(), Some("uqniue id conflict"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/recurring_payment/batch_update"
        );
    }
}
