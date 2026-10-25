//! 查询一次性支付授予记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/lump_sum_payment/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::common::shared_models::I18nText;

use super::query_detail::LumpSumPaymentDetail;

/// 查询一次性支付授予记录请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::LumpSumPaymentQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询一次性支付授予记录响应数据为空",
        )
        .await
    }
}

/// 查询一次性支付授予记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 一次性支付授予记录列表
    pub records: Vec<LumpSumPayment>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 一次性支付记录（官方 lump_sum_payment）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LumpSumPayment {
    /// 一次性支付记录 id
    pub id: String,
    /// 外部幂等 id，由上游业务自由决定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 员工 id，具体类型由入参中的 user_id_type 指定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 总金额，字符串表达的数字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    /// 绑定期，单位为月
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period: Option<i32>,
    /// 币种 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 发放次数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_frequency: Option<i32>,
    /// 薪酬项 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 发放规则描述文本（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_detail_text: Option<I18nText>,
    /// 申请来源：1-Offer薪酬创建 2-导入 3-调级调薪创建 4-填报创建 5-开放平台创建 6-招聘内推创建 7-自动化规则创建
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_source: Option<i32>,
    /// 应退回金额（税前）（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amount_before_tax: Option<String>,
    /// 应退回金额（税后）（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_amount_after_tax: Option<String>,
    /// 绑定期内离职类型：yes-绑定期内离职 no-绑定期外离职 default-无绑定期离职标识（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period_offboarding_type: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 一次性支付记录所属期开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_start_date: Option<String>,
    /// 一次性支付记录所属期结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_period_end_date: Option<String>,
    /// 发放明细列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<LumpSumPaymentDetail>>,
    /// 绑定期带小数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding_period_decimal: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
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
    async fn test_query_lump_sum_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.records[]（含 details 明细）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"records": [{"id": "7397033607132351532", "unique_id": "7402510801304718380_7309316347007764012", "user_id": "7337149697626801708", "total_amount": "2000.00", "binding_period": 2, "currency_id": "6863329932261459464", "issuance_frequency": 3, "item_id": "7411039006180312620", "remark": "备注", "issuance_detail_text": {"zh_cn": "分2次发放", "en_us": "It will be issued in 2 installments."}, "apply_source": 1, "return_amount_before_tax": "2000.00", "return_amount_after_tax": "2000.00", "binding_period_offboarding_type": "yes", "create_time": "2024-08-01 12:34:56", "modify_time": "2024-08-01 12:34:56", "reference_period_start_date": "2024-08-01", "reference_period_end_date": "2024-08-01", "details": [{"id": "7395133551102200876", "record_id": "7395133551102168108", "user_id": "7337149697626801708", "issuance_amount": "2000.00", "issuance_status": "to_be_issued", "issuance_way": "with_salary", "issuance_time": "2024-08-01", "currency_id": "6863329932261459464", "belong_time": "2025-01-20", "create_time": "2024-08-01 12:34:56", "modify_time": "2024-08-01 12:34:56", "issuance_country_region_id": "6862995757234914824", "issuance_pay_group_id": "6862995757234914824", "detail_reference_period_start_date": "2024-08-01", "detail_reference_period_end_date": "2024-08-01"}], "binding_period_decimal": "12"}], "has_more": true, "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/lump_sum_payment/query"))
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

        let data = QueryRequest::new(config)
            .execute()
            .await
            .expect("查询一次性支付应成功");

        assert!(data.has_more);
        assert_eq!(data.records.len(), 1);
        let record = &data.records[0];
        assert_eq!(record.id, "7397033607132351532");
        assert_eq!(record.total_amount.as_deref(), Some("2000.00"));
        assert_eq!(record.binding_period, Some(2));
        assert_eq!(record.issuance_frequency, Some(3));
        assert_eq!(record.apply_source, Some(1));
        assert_eq!(record.binding_period_decimal.as_deref(), Some("12"));
        assert_eq!(
            record.issuance_detail_text.as_ref().unwrap().zh_cn,
            Some("分2次发放".to_string())
        );
        let details = record.details.as_ref().unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].record_id.as_deref(), Some("7395133551102168108"));
        assert_eq!(details[0].issuance_amount.as_deref(), Some("2000.00"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/lump_sum_payment/query"
        );
    }
}
