//! 查询一次性支付授予明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/lump_sum_payment/query_detail>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询一次性支付授予明细请求
#[derive(Debug, Clone)]
pub struct QueryDetailRequest {
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryDetailRequest {
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
    pub async fn execute(self) -> SDKResult<QueryDetailResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryDetailResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        let api_endpoint = CompensationApiV1::LumpSumPaymentQueryDetail;
        let mut request = ApiRequest::<QueryDetailResponse>::post(api_endpoint.to_url());

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
            "查询一次性支付授予明细响应数据为空",
        )
        .await
    }
}

/// 查询一次性支付授予明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryDetailResponse {
    /// 一次性支付授予明细列表
    pub records: Vec<LumpSumPaymentDetail>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 一次性支付明细（官方 lump_sum_payment_detail）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LumpSumPaymentDetail {
    /// 一次性支付记录明细 id
    pub id: String,
    /// 一次性支付记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 员工 id，具体类型由入参中的 user_id_type 指定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 一次性支付明细发放金额，可转数字的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_amount: Option<String>,
    /// 发放状态：to_be_issued-应发放 not_issued-不发放
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_status: Option<String>,
    /// 发放方式：with_salary-随工资发放 with_cash-现金发放 with_year_end_bonus-随年终奖发放
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_way: Option<String>,
    /// 发放日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_time: Option<String>,
    /// 币种 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 申请发放日期（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub belong_time: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
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

impl ApiResponseTrait for QueryDetailResponse {
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
    async fn test_query_detail_lump_sum_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.records[]（含全部明细字段）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"records": [{"id": "7395133551102200876", "record_id": "7395133551102168108", "user_id": "7337149697626801708", "issuance_amount": "2000.00", "issuance_status": "to_be_issued", "issuance_way": "with_salary", "issuance_time": "2024-08-01", "currency_id": "6863329932261459464", "belong_time": "2025-01-20", "create_time": "2024-08-01 12:34:56", "modify_time": "2024-08-01 12:34:56", "issuance_country_region_id": "6862995757234914824", "issuance_pay_group_id": "6862995757234914824", "detail_reference_period_start_date": "2024-08-01", "detail_reference_period_end_date": "2024-08-01"}], "has_more": true, "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/lump_sum_payment/query_detail",
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

        let data = QueryDetailRequest::new(config)
            .execute()
            .await
            .expect("查询一次性支付明细应成功");

        assert!(data.has_more);
        assert_eq!(data.records.len(), 1);
        let detail = &data.records[0];
        assert_eq!(detail.id, "7395133551102200876");
        assert_eq!(detail.record_id.as_deref(), Some("7395133551102168108"));
        assert_eq!(detail.issuance_amount.as_deref(), Some("2000.00"));
        assert_eq!(detail.issuance_status.as_deref(), Some("to_be_issued"));
        assert_eq!(detail.issuance_way.as_deref(), Some("with_salary"));
        assert_eq!(detail.issuance_time.as_deref(), Some("2024-08-01"));
        assert_eq!(detail.currency_id.as_deref(), Some("6863329932261459464"));
        assert_eq!(detail.belong_time.as_deref(), Some("2025-01-20"));
        assert_eq!(detail.create_time.as_deref(), Some("2024-08-01 12:34:56"));
        assert_eq!(detail.modify_time.as_deref(), Some("2024-08-01 12:34:56"));
        assert_eq!(
            detail.issuance_country_region_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            detail.issuance_pay_group_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            detail.detail_reference_period_start_date.as_deref(),
            Some("2024-08-01")
        );
        assert_eq!(
            detail.detail_reference_period_end_date.as_deref(),
            Some("2024-08-01")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/lump_sum_payment/query_detail"
        );
    }
}
