//! 查询发薪活动列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询发薪活动列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 发薪日开始时间（必填，格式 YYYY-MM-dd，与结束时间构成闭区间）
    pay_period_start_date: Option<String>,
    /// 发薪日结束时间（必填，格式 YYYY-MM-dd，与开始时间构成闭区间）
    pay_period_end_date: Option<String>,
    /// 分页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填，表示从头开始遍历）
    page_token: Option<String>,
    /// 发薪活动审批状态列表（可选）
    statuses: Option<Vec<i32>>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            pay_period_start_date: None,
            pay_period_end_date: None,
            page_size: None,
            page_token: None,
            statuses: None,
            config,
        }
    }

    /// 设置发薪日开始时间（必填，格式 YYYY-MM-dd）
    pub fn pay_period_start_date(mut self, pay_period_start_date: String) -> Self {
        self.pay_period_start_date = Some(pay_period_start_date);
        self
    }

    /// 设置发薪日结束时间（必填，格式 YYYY-MM-dd）
    pub fn pay_period_end_date(mut self, pay_period_end_date: String) -> Self {
        self.pay_period_end_date = Some(pay_period_end_date);
        self
    }

    /// 设置分页大小（必填，取值范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置发薪活动审批状态列表（可选，如 100-待确认名单、200-审批中、400-已封存）
    pub fn statuses(mut self, statuses: Vec<i32>) -> Self {
        self.statuses = Some(statuses);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段
        let pay_period_start_date = self.pay_period_start_date.ok_or_else(|| {
            validation_error(
                "pay_period_start_date 不能为空",
                "发薪日开始时间必填，格式 YYYY-MM-dd",
            )
        })?;
        validate_required!(pay_period_start_date, "pay_period_start_date 不能为空");
        let pay_period_end_date = self.pay_period_end_date.ok_or_else(|| {
            validation_error(
                "pay_period_end_date 不能为空",
                "发薪日结束时间必填，格式 YYYY-MM-dd",
            )
        })?;
        validate_required!(pay_period_end_date, "pay_period_end_date 不能为空");
        let page_size = self.page_size.ok_or_else(|| {
            validation_error("page_size 不能为空", "分页大小必填，取值范围 1~100")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(validation_error(
                "page_size 超出范围",
                "分页大小取值范围为 1~100",
            ));
        }

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request
            .query("pay_period_start_date", &pay_period_start_date)
            .query("pay_period_end_date", &pay_period_end_date)
            .query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(ref statuses) = self.statuses {
            let statuses_str = statuses
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(",");
            request = request.query("statuses", statuses_str);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询发薪活动列表响应数据为空",
        )
        .await
    }
}

/// 查询发薪活动列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 发薪活动列表
    pub payment_activitys: Vec<PaymentActivity>,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

/// 发薪活动
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentActivity {
    /// 发薪活动唯一标识
    pub activity_id: String,
    /// 发薪活动名称（多语种）
    pub activity_names: Vec<I18nContent>,
    /// 发薪活动发薪日期（yyyy-MM-dd）
    pub pay_date: String,
    /// 发薪总笔数
    pub total_number_of_payroll: i32,
    /// 关联的算薪活动个数
    pub number_of_calculation_activities: i32,
    /// 发薪活动关联的算薪活动详情
    pub calculation_activities: Vec<CalculationActivity>,
    /// 发薪活动审批状态：100-待确认发薪名单 150-待提交审批 200-审批中 300-审批被拒绝 350-审批被撤回 400-已封存
    pub activity_status: i32,
}

/// 算薪活动
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalculationActivity {
    /// 算薪活动唯一标识
    pub calculation_activity_id: String,
    /// 算薪活动名称（多语种）
    pub calculation_activity_names: Vec<I18nContent>,
}

/// 多语种内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语种，如 zh_cn、en_us
    pub locale: String,
    /// 语种对应的值
    pub value: String,
}

impl ApiResponseTrait for ListResponse {
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
    async fn test_list_payment_activities_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应形状：payment_activitys + 分页字段
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "payment_activitys": [{
                    "activity_id": "7202076988667019308",
                    "activity_names": [{"locale": "zh_cn", "value": "2024-01 发薪"}],
                    "pay_date": "2020-10-31",
                    "total_number_of_payroll": 100,
                    "number_of_calculation_activities": 1000,
                    "calculation_activities": [{
                        "calculation_activity_id": "7371828319837012009",
                        "calculation_activity_names": [{"locale": "zh_cn", "value": "算薪活动"}]
                    }],
                    "activity_status": 400
                }],
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activitys"))
            .and(query_param("pay_period_start_date", "2024-01-01"))
            .and(query_param("pay_period_end_date", "2024-01-31"))
            .and(query_param("page_size", "20"))
            .and(query_param("statuses", "100,200"))
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

        let data = ListRequest::new(config)
            .pay_period_start_date("2024-01-01".to_string())
            .pay_period_end_date("2024-01-31".to_string())
            .page_size(20)
            .statuses(vec![100, 200])
            .execute()
            .await
            .expect("查询发薪活动应成功");

        assert!(data.has_more);
        assert_eq!(data.payment_activitys.len(), 1);
        let activity = &data.payment_activitys[0];
        assert_eq!(activity.activity_id, "7202076988667019308");
        assert_eq!(activity.activity_names[0].value, "2024-01 发薪");
        assert_eq!(activity.pay_date, "2020-10-31");
        assert_eq!(activity.total_number_of_payroll, 100);
        assert_eq!(activity.number_of_calculation_activities, 1000);
        assert_eq!(
            activity.calculation_activities[0].calculation_activity_id,
            "7371828319837012009"
        );
        assert_eq!(activity.activity_status, 400);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/payment_activitys"
        );
    }

    /// pay_period_start_date / pay_period_end_date / page_size 为必填，缺失时应校验失败
    #[tokio::test]
    async fn test_list_payment_activities_requires_required_params() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = ListRequest::new(config.clone()).execute().await;
        assert!(result.is_err());

        let result = ListRequest::new(config)
            .pay_period_start_date("2024-01-01".to_string())
            .execute()
            .await;
        assert!(result.is_err());
    }
}
