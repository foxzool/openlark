//! 批量删除经常性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/recurring_payment/batch_remove>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量删除经常性支付记录请求
#[derive(Debug, Clone)]
pub struct BatchRemoveRequest {
    /// 需要删除的记录 ID 列表（必填，最多 500 个）
    record_ids: Vec<String>,
    /// 删除原因（可选）
    reason: Option<String>,
    /// 配置信息
    config: Config,
}

impl BatchRemoveRequest {
    /// 创建请求
    pub fn new(config: Config, record_ids: Vec<String>) -> Self {
        Self {
            record_ids,
            reason: None,
            config,
        }
    }

    /// 设置删除原因（可选）
    pub fn reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
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

        // 1. 验证必填字段
        validate_required_list!(self.record_ids, 500, "record_ids 不能为空且不能超过 500 个");

        // 2. 构建端点与请求体
        let api_endpoint = CompensationApiV1::RecurringPaymentBatchRemove;
        let request = ApiRequest::<BatchRemoveResponse>::post(api_endpoint.to_url());
        let request_body = BatchRemoveRequestBody {
            record_ids: self.record_ids,
            reason: self.reason,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量删除经常性支付记录响应数据为空",
        )
        .await
    }
}

/// 批量删除经常性支付记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRemoveRequestBody {
    /// 需要删除的记录 ID 列表（最多 500 个）
    pub record_ids: Vec<String>,
    /// 原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 批量删除经常性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResponse {
    /// 每条记录的操作结果列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_results: Option<Vec<BatchRemoveResult>>,
}

/// 批量删除操作结果（官方 recurring_payment_operate_result）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveResult {
    /// 操作记录的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 操作的记录的 unique_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 操作结果状态码（0 表示成功，21270304 等为具体错误）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 操作结果描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
    async fn test_batch_remove_recurring_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.operate_results[]（含 code/unique_id/message）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"operate_results": [{"id": "7390583861280556588", "unique_id": "7390583861280556588", "code": 21270202, "message": "unique id conflict"}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_remove",
            ))
            .and(body_json(json!({
                "record_ids": ["7397033607132351532"],
                "reason": "删除错误创建的数据"
            })))
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

        let data = BatchRemoveRequest::new(config, vec!["7397033607132351532".to_string()])
            .reason("删除错误创建的数据".to_string())
            .execute()
            .await
            .expect("批量删除经常性支付应成功");

        let results = data.operate_results.expect("operate_results 应存在");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_deref(), Some("7390583861280556588"));
        assert_eq!(results[0].unique_id.as_deref(), Some("7390583861280556588"));
        assert_eq!(results[0].code, Some(21270202));
        assert_eq!(results[0].message.as_deref(), Some("unique id conflict"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/recurring_payment/batch_remove"
        );
    }

    /// record_ids 为必填（最多 500 个）
    #[tokio::test]
    async fn test_batch_remove_recurring_payment_validates_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = BatchRemoveRequest::new(config, vec![]).execute().await;
        assert!(result.is_err());
    }
}
