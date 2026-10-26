//! 批量创建一次性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/lump_sum_payment/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量创建一次性支付记录请求
#[derive(Debug, Clone)]
pub struct BatchCreateRequest {
    /// 要创建的一次性支付记录列表（最多 500 条）
    records: Vec<LumpSumPaymentForCreate>,
    /// 用户 ID 类型（查询参数：open_id/union_id/user_id/people_corehr_id，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl BatchCreateRequest {
    /// 创建请求
    pub fn new(config: Config, records: Vec<LumpSumPaymentForCreate>) -> Self {
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
        let api_endpoint = CompensationApiV1::LumpSumPaymentBatchCreate;
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
            "批量创建一次性支付记录响应数据为空",
        )
        .await
    }
}

/// 批量创建一次性支付记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRequestBody {
    /// 要创建的一次性支付信息（最多 500 条）
    pub records: Vec<LumpSumPaymentForCreate>,
}

/// 待创建的一次性支付记录（官方 lump_sum_payment_for_create）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentForCreate {
    /// 外部幂等 id，由上游业务决定
    pub unique_id: String,
    /// 员工 id，具体类型由入参中的 user_id_type 指定
    pub user_id: String,
    /// 总金额，字符串表达的数字，单位为 currency_id 给定的币种
    pub total_amount: String,
    /// 绑定期，单位为月
    pub binding_period: i32,
    /// 币种 id
    pub currency_id: String,
    /// 发放次数，必须与 details 的长度一致
    pub issuance_frequency: i32,
    /// 薪酬项 id
    pub item_id: String,
    /// 一次性支付记录所属期开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_start_date: Option<String>,
    /// 一次性支付记录所属期结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_end_date: Option<String>,
    /// 发放明细列表（长度须与 issuance_frequency 一致）
    pub details: Vec<LumpSumPaymentDetailForCreate>,
    /// 备注（不超过 3000 字符）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 绑定期带小数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period_decimal: Option<String>,
}

/// 待创建的一次性支付明细（官方 lump_sum_payment_detail_for_create）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpSumPaymentDetailForCreate {
    /// 一次性支付明细发放金额，可转数字的字符串
    pub issuance_amount: String,
    /// 发放状态：to_be_issued-应发放 not_issued-不发放
    pub issuance_status: String,
    /// 发放方式：with_salary-随工资发放 with_cash-现金发放 with_year_end_bonus-随年终奖发放
    pub issuance_way: String,
    /// 发放时间，如 "2024-08-01"
    pub issuance_time: String,
    /// 申请发放日期，如 "2025-01-20"
    pub belong_time: String,
    /// 发放国家 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_country_region_id: Option<String>,
    /// 发放薪资组 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_pay_group_id: Option<String>,
    /// 一次性支付明细所属期开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_reference_period_start_date: Option<String>,
    /// 一次性支付明细所属期结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_reference_period_end_date: Option<String>,
}

/// 批量创建一次性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResponse {
    /// 每条记录的操作结果列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_results: Option<Vec<BatchCreateResult>>,
}

/// 批量创建操作结果（官方 lump_sum_payment_operate_result）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateResult {
    /// 操作的记录的 id（创建成功时返回创建后的一次性支付记录 id）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 操作的记录的 unique_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 操作结果状态码（0 表示成功，21270201 等为具体错误）
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
    async fn test_batch_create_lump_sum_payment_returns_data_on_success() {
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
                "/open-apis/compensation/v1/lump_sum_payment/batch_create",
            ))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "records": [{
                    "unique_id": "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000",
                    "user_id": "7337149697626801708",
                    "total_amount": "2000.00",
                    "binding_period": 2,
                    "currency_id": "6863329932261459464",
                    "issuance_frequency": 3,
                    "item_id": "7411039006180312620",
                    "reference_period_start_date": "2024-08-01",
                    "reference_period_end_date": "2024-08-01",
                    "details": [{
                        "issuance_amount": "2000.00",
                        "issuance_status": "to_be_issued",
                        "issuance_way": "with_salary",
                        "issuance_time": "2024-08-01",
                        "belong_time": "2025-01-20",
                        "issuance_country_region_id": "6862995757234914824",
                        "issuance_pay_group_id": "6862995757234914824",
                        "detail_reference_period_start_date": "2024-08-01",
                        "detail_reference_period_end_date": "2024-08-01"
                    }],
                    "remark": "该员工表现优异，为其发放一笔奖金",
                    "binding_period_decimal": "12.56"
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
            vec![LumpSumPaymentForCreate {
                unique_id: "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000".to_string(),
                user_id: "7337149697626801708".to_string(),
                total_amount: "2000.00".to_string(),
                binding_period: 2,
                currency_id: "6863329932261459464".to_string(),
                issuance_frequency: 3,
                item_id: "7411039006180312620".to_string(),
                reference_period_start_date: Some("2024-08-01".to_string()),
                reference_period_end_date: Some("2024-08-01".to_string()),
                details: vec![LumpSumPaymentDetailForCreate {
                    issuance_amount: "2000.00".to_string(),
                    issuance_status: "to_be_issued".to_string(),
                    issuance_way: "with_salary".to_string(),
                    issuance_time: "2024-08-01".to_string(),
                    belong_time: "2025-01-20".to_string(),
                    issuance_country_region_id: Some("6862995757234914824".to_string()),
                    issuance_pay_group_id: Some("6862995757234914824".to_string()),
                    detail_reference_period_start_date: Some("2024-08-01".to_string()),
                    detail_reference_period_end_date: Some("2024-08-01".to_string()),
                }],
                remark: Some("该员工表现优异，为其发放一笔奖金".to_string()),
                binding_period_decimal: Some("12.56".to_string()),
            }],
        )
        .user_id_type("open_id".to_string())
        .execute()
        .await
        .expect("批量创建一次性支付应成功");

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
            "/open-apis/compensation/v1/lump_sum_payment/batch_create"
        );
    }
}
