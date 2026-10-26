//! 批量创建经常性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/recurring_payment/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量创建经常性支付记录请求
#[derive(Debug, Clone)]
pub struct BatchCreateRequest {
    /// 要创建的经常性支付记录列表（最多 500 条）
    records: Vec<RecurringPaymentForCreate>,
    /// 用户 ID 类型（查询参数：open_id/union_id/user_id/people_corehr_id，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl BatchCreateRequest {
    /// 创建请求
    pub fn new(config: Config, records: Vec<RecurringPaymentForCreate>) -> Self {
        Self {
            records,
            user_id_type: None,
            config,
        }
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 构建端点与查询参数
        let api_endpoint = CompensationApiV1::RecurringPaymentBatchCreate;
        let mut request = ApiRequest::<BatchCreateResponse>::post(api_endpoint.to_url());
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 2. 构建请求体
        let request_body = BatchCreateRequestBody {
            records: self.records,
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
            "批量创建经常性支付记录响应数据为空",
        )
        .await
    }
}

/// 批量创建经常性支付记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRequestBody {
    /// 要创建的经常性支付记录（最多 500 条）
    pub records: Vec<RecurringPaymentForCreate>,
}

/// 待创建的经常性支付记录（官方 recurring_payment_for_create）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPaymentForCreate {
    /// 经常性支付记录 unique_id，在创建时由上游指定
    pub unique_id: String,
    /// 员工 id，具体类型由入参中的 user_id_type 指定
    pub user_id: String,
    /// 薪酬项 id
    pub item_id: String,
    /// 每次发放金额，字符串表达的数字
    pub each_amount: String,
    /// 发放开始时间，格式 YYYY-MM-DD
    pub start_date: String,
    /// 发放结束时间，格式 YYYY-MM-DD
    pub end_date: String,
    /// 币种 id
    pub currency_id: String,
    /// 发放方式：with_salary-随工资发放 with_cash-现金发放 with_physical_distribution-实物发放 with_year_end_bonus-随年终奖发放
    pub issuance_type: String,
    /// 发放频率：year/half_year/quarterly/bimonthly/month/biweekly/week/day/hour
    pub issuance_period: String,
    /// 备注（不超过 3000 字符）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 发放国家 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_country_region_id: Option<String>,
}

/// 批量创建经常性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResponse {
    /// 每条记录的操作结果列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_results: Option<Vec<BatchCreateResult>>,
}

/// 批量创建操作结果（官方 recurring_payment_operate_result）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResult {
    /// 操作记录的 id（创建成功时返回创建后的经常性支付记录 id）
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

impl ApiResponseTrait for BatchCreateResponse {
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
    async fn test_batch_create_recurring_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.operate_results[]（含 code/unique_id/message）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"operate_results": [{"id": "7390583861280556588", "unique_id": "7390583861280556588", "code": 21270202, "message": "uqniue id conflict"}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_create",
            ))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "records": [{
                    "unique_id": "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000",
                    "user_id": "7337149697626801708",
                    "item_id": "7411039006180312620",
                    "each_amount": "20.00",
                    "start_date": "2024-08-01",
                    "end_date": "2025-08-01",
                    "currency_id": "6863329932261459464",
                    "issuance_type": "with_salary",
                    "issuance_period": "year",
                    "remark": "这是个备注",
                    "issuance_country_region_id": "6862995745046267400"
                }]
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

        let data = BatchCreateRequest::new(
            config,
            vec![RecurringPaymentForCreate {
                unique_id: "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000".to_string(),
                user_id: "7337149697626801708".to_string(),
                item_id: "7411039006180312620".to_string(),
                each_amount: "20.00".to_string(),
                start_date: "2024-08-01".to_string(),
                end_date: "2025-08-01".to_string(),
                currency_id: "6863329932261459464".to_string(),
                issuance_type: "with_salary".to_string(),
                issuance_period: "year".to_string(),
                remark: Some("这是个备注".to_string()),
                issuance_country_region_id: Some("6862995745046267400".to_string()),
            }],
        )
        .user_id_type("open_id".to_string())
        .execute()
        .await
        .expect("批量创建经常性支付应成功");

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
            "/open-apis/compensation/v1/recurring_payment/batch_create"
        );
    }
}
