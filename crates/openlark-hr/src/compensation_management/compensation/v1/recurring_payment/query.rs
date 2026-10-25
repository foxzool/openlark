//! 查询经常性支付记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/recurring_payment/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询经常性支付记录请求
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

        let api_endpoint = CompensationApiV1::RecurringPaymentQuery;
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
            "查询经常性支付记录响应数据为空",
        )
        .await
    }
}

/// 查询经常性支付记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 经常性支付授予记录列表
    pub records: Vec<RecurringPayment>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 经常性支付记录（官方 recurring_payment）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecurringPayment {
    /// 经常性支付记录 id
    pub id: String,
    /// 外部幂等 id，由上游业务决定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 员工 id，具体类型由入参中的 user_id_type 指定（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 薪酬项 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 发放方式：with_salary-随工资发放 with_cash-现金发放 with_physical_distribution-实物发放 with_year_end_bonus-随年终奖发放
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_type: Option<String>,
    /// 单次发放金额，字符串表达的数字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub each_amount: Option<String>,
    /// 发放开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 发放结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 发放频率：year/half_year/quarterly/bimonthly/month/biweekly/week/day/hour
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_period: Option<String>,
    /// 币种 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 发放国家 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_country_region_id: Option<String>,
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
    async fn test_query_recurring_payment_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.records[]（含 12 字段）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"records": [{"id": "7397033607132351532", "unique_id": "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000", "user_id": "7337149697626801708", "item_id": "7411039006180312620", "issuance_type": "with_salary", "each_amount": "24.00", "start_date": "2024-01-10", "end_date": "2025-01-20", "issuance_period": "year", "currency_id": "6863329932261459464", "remark": "这是个备注", "issuance_country_region_id": "6862995745046267400"}], "has_more": true, "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/recurring_payment/query"))
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
            .expect("查询经常性支付应成功");

        assert!(data.has_more);
        assert_eq!(data.records.len(), 1);
        let record = &data.records[0];
        assert_eq!(record.id, "7397033607132351532");
        assert_eq!(record.each_amount.as_deref(), Some("24.00"));
        assert_eq!(record.issuance_type.as_deref(), Some("with_salary"));
        assert_eq!(record.issuance_period.as_deref(), Some("year"));
        assert_eq!(record.start_date.as_deref(), Some("2024-01-10"));
        assert_eq!(record.end_date.as_deref(), Some("2025-01-20"));
        assert_eq!(record.currency_id.as_deref(), Some("6863329932261459464"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/recurring_payment/query"
        );
    }
}
