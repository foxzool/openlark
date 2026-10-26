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
    /// 经常性支付记录 id 列表（可选，最多 500 个）
    ids: Option<Vec<String>>,
    /// 经常性支付记录 unique_id 列表（可选，最多 500 个）
    unique_ids: Option<Vec<String>>,
    /// 员工 id 列表（可选，具体类型由 user_id_type 指定，最多 500 个）
    user_ids: Option<Vec<String>>,
    /// 薪酬项 id 列表（可选，最多 500 个）
    item_ids: Option<Vec<String>>,
    /// 发放开始日期大于等于（可选，如 "2023-04-01"）
    start_date_gte: Option<String>,
    /// 发放开始日期小于等于（可选）
    start_date_lte: Option<String>,
    /// 发放结束日期大于等于（可选）
    end_date_gte: Option<String>,
    /// 发放结束日期小于等于（可选）
    end_date_lte: Option<String>,
    /// 创建时间大于等于（可选，东八区，如 "2023-04-01 12:34:56"）
    create_time_gte: Option<String>,
    /// 创建时间小于等于（可选，东八区）
    create_time_lte: Option<String>,
    /// 更新时间大于等于（可选，东八区）
    modify_time_gte: Option<String>,
    /// 更新时间小于等于（可选，东八区）
    modify_time_lte: Option<String>,
    /// 合同主体 id 列表（可选）
    company_ids: Option<Vec<String>>,
    /// 任职公司 id 列表（可选）
    service_company_ids: Option<Vec<String>>,
    /// 部门 id 列表（可选）
    department_ids: Option<Vec<String>>,
    /// 序列 id 列表（可选）
    job_family_ids: Option<Vec<String>>,
    /// 职级 id 列表（可选）
    job_level_ids: Option<Vec<String>>,
    /// 工作地点 id 列表（可选）
    work_location_ids: Option<Vec<String>>,
    /// 员工类型 id 列表（可选）
    employee_type_ids: Option<Vec<String>>,
    /// 入职日期大于等于（可选）
    onboard_date_gte: Option<String>,
    /// 入职日期小于等于（可选）
    onboard_date_lte: Option<String>,
    /// 离职日期大于等于（可选）
    offboard_date_gte: Option<String>,
    /// 离职日期小于等于（可选）
    offboard_date_lte: Option<String>,
    /// 分页大小（查询参数，官方必填，取值范围 1～500）
    page_size: Option<i32>,
    /// 分页标记（查询参数，可选）
    page_token: Option<String>,
    /// 用户 ID 类型（查询参数：open_id/union_id/user_id/people_corehr_id，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            ids: None,
            unique_ids: None,
            user_ids: None,
            item_ids: None,
            start_date_gte: None,
            start_date_lte: None,
            end_date_gte: None,
            end_date_lte: None,
            create_time_gte: None,
            create_time_lte: None,
            modify_time_gte: None,
            modify_time_lte: None,
            company_ids: None,
            service_company_ids: None,
            department_ids: None,
            job_family_ids: None,
            job_level_ids: None,
            work_location_ids: None,
            employee_type_ids: None,
            onboard_date_gte: None,
            onboard_date_lte: None,
            offboard_date_gte: None,
            offboard_date_lte: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置经常性支付记录 id 列表（可选）
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.ids = Some(ids);
        self
    }

    /// 设置经常性支付记录 unique_id 列表（可选）
    pub fn unique_ids(mut self, unique_ids: Vec<String>) -> Self {
        self.unique_ids = Some(unique_ids);
        self
    }

    /// 设置员工 id 列表（可选，具体类型由 user_id_type 指定）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// 设置薪酬项 id 列表（可选）
    pub fn item_ids(mut self, item_ids: Vec<String>) -> Self {
        self.item_ids = Some(item_ids);
        self
    }

    /// 设置发放开始日期大于等于（可选）
    pub fn start_date_gte(mut self, start_date_gte: String) -> Self {
        self.start_date_gte = Some(start_date_gte);
        self
    }

    /// 设置发放开始日期小于等于（可选）
    pub fn start_date_lte(mut self, start_date_lte: String) -> Self {
        self.start_date_lte = Some(start_date_lte);
        self
    }

    /// 设置发放结束日期大于等于（可选）
    pub fn end_date_gte(mut self, end_date_gte: String) -> Self {
        self.end_date_gte = Some(end_date_gte);
        self
    }

    /// 设置发放结束日期小于等于（可选）
    pub fn end_date_lte(mut self, end_date_lte: String) -> Self {
        self.end_date_lte = Some(end_date_lte);
        self
    }

    /// 设置创建时间大于等于（可选，东八区）
    pub fn create_time_gte(mut self, create_time_gte: String) -> Self {
        self.create_time_gte = Some(create_time_gte);
        self
    }

    /// 设置创建时间小于等于（可选，东八区）
    pub fn create_time_lte(mut self, create_time_lte: String) -> Self {
        self.create_time_lte = Some(create_time_lte);
        self
    }

    /// 设置更新时间大于等于（可选，东八区）
    pub fn modify_time_gte(mut self, modify_time_gte: String) -> Self {
        self.modify_time_gte = Some(modify_time_gte);
        self
    }

    /// 设置更新时间小于等于（可选，东八区）
    pub fn modify_time_lte(mut self, modify_time_lte: String) -> Self {
        self.modify_time_lte = Some(modify_time_lte);
        self
    }

    /// 设置合同主体 id 列表（可选）
    pub fn company_ids(mut self, company_ids: Vec<String>) -> Self {
        self.company_ids = Some(company_ids);
        self
    }

    /// 设置任职公司 id 列表（可选）
    pub fn service_company_ids(mut self, service_company_ids: Vec<String>) -> Self {
        self.service_company_ids = Some(service_company_ids);
        self
    }

    /// 设置部门 id 列表（可选）
    pub fn department_ids(mut self, department_ids: Vec<String>) -> Self {
        self.department_ids = Some(department_ids);
        self
    }

    /// 设置序列 id 列表（可选）
    pub fn job_family_ids(mut self, job_family_ids: Vec<String>) -> Self {
        self.job_family_ids = Some(job_family_ids);
        self
    }

    /// 设置职级 id 列表（可选）
    pub fn job_level_ids(mut self, job_level_ids: Vec<String>) -> Self {
        self.job_level_ids = Some(job_level_ids);
        self
    }

    /// 设置工作地点 id 列表（可选）
    pub fn work_location_ids(mut self, work_location_ids: Vec<String>) -> Self {
        self.work_location_ids = Some(work_location_ids);
        self
    }

    /// 设置员工类型 id 列表（可选）
    pub fn employee_type_ids(mut self, employee_type_ids: Vec<String>) -> Self {
        self.employee_type_ids = Some(employee_type_ids);
        self
    }

    /// 设置入职日期大于等于（可选）
    pub fn onboard_date_gte(mut self, onboard_date_gte: String) -> Self {
        self.onboard_date_gte = Some(onboard_date_gte);
        self
    }

    /// 设置入职日期小于等于（可选）
    pub fn onboard_date_lte(mut self, onboard_date_lte: String) -> Self {
        self.onboard_date_lte = Some(onboard_date_lte);
        self
    }

    /// 设置离职日期大于等于（可选）
    pub fn offboard_date_gte(mut self, offboard_date_gte: String) -> Self {
        self.offboard_date_gte = Some(offboard_date_gte);
        self
    }

    /// 设置离职日期小于等于（可选）
    pub fn offboard_date_lte(mut self, offboard_date_lte: String) -> Self {
        self.offboard_date_lte = Some(offboard_date_lte);
        self
    }

    /// 设置分页大小（官方必填，取值范围 1～500）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
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

        // 1. 构建端点与查询参数
        let api_endpoint = CompensationApiV1::RecurringPaymentQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 2. 构建请求体（筛选项均可选）
        let request_body = QueryRequestBody {
            ids: self.ids,
            unique_ids: self.unique_ids,
            user_ids: self.user_ids,
            item_ids: self.item_ids,
            start_date_gte: self.start_date_gte,
            start_date_lte: self.start_date_lte,
            end_date_gte: self.end_date_gte,
            end_date_lte: self.end_date_lte,
            create_time_gte: self.create_time_gte,
            create_time_lte: self.create_time_lte,
            modify_time_gte: self.modify_time_gte,
            modify_time_lte: self.modify_time_lte,
            company_ids: self.company_ids,
            service_company_ids: self.service_company_ids,
            department_ids: self.department_ids,
            job_family_ids: self.job_family_ids,
            job_level_ids: self.job_level_ids,
            work_location_ids: self.work_location_ids,
            employee_type_ids: self.employee_type_ids,
            onboard_date_gte: self.onboard_date_gte,
            onboard_date_lte: self.onboard_date_lte,
            offboard_date_gte: self.offboard_date_gte,
            offboard_date_lte: self.offboard_date_lte,
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
            "查询经常性支付记录响应数据为空",
        )
        .await
    }
}

/// 查询经常性支付记录请求体（筛选项均可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 经常性支付记录 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// 经常性支付记录 unique_id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_ids: Option<Vec<String>>,
    /// 员工 id 列表，具体类型由入参中的 user_id_type 指定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 薪酬项 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    /// 发放开始日期大于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_gte: Option<String>,
    /// 发放开始日期小于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_lte: Option<String>,
    /// 发放结束日期大于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_gte: Option<String>,
    /// 发放结束日期小于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_lte: Option<String>,
    /// 创建时间大于等于（东八区）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time_gte: Option<String>,
    /// 创建时间小于等于（东八区）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time_lte: Option<String>,
    /// 更新时间大于等于（东八区）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time_gte: Option<String>,
    /// 更新时间小于等于（东八区）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time_lte: Option<String>,
    /// 合同主体 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_ids: Option<Vec<String>>,
    /// 任职公司 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company_ids: Option<Vec<String>>,
    /// 部门 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 序列 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_ids: Option<Vec<String>>,
    /// 职级 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_ids: Option<Vec<String>>,
    /// 工作地点 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_ids: Option<Vec<String>>,
    /// 员工类型 id 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_ids: Option<Vec<String>>,
    /// 入职日期大于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_date_gte: Option<String>,
    /// 入职日期小于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboard_date_lte: Option<String>,
    /// 离职日期大于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboard_date_gte: Option<String>,
    /// 离职日期小于等于
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboard_date_lte: Option<String>,
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
        use wiremock::matchers::{body_partial_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.records[]（含 12 字段）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"records": [{"id": "7397033607132351532", "unique_id": "7402510801304718380_7309316347007764012_7402523725868058156_1726070400000_10000", "user_id": "7337149697626801708", "item_id": "7411039006180312620", "issuance_type": "with_salary", "each_amount": "24.00", "start_date": "2024-01-10", "end_date": "2025-01-20", "issuance_period": "year", "currency_id": "6863329932261459464", "remark": "这是个备注", "issuance_country_region_id": "6862995745046267400"}], "has_more": true, "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0="}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/recurring_payment/query"))
            .and(query_param("page_size", "10"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_partial_json(json!({
                "ids": ["7337149697626801708"],
                "user_ids": ["7337149697626801708"],
                "start_date_gte": "2023-04-01"
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

        let data = QueryRequest::new(config)
            .page_size(10)
            .user_id_type("open_id".to_string())
            .ids(vec!["7337149697626801708".to_string()])
            .user_ids(vec!["7337149697626801708".to_string()])
            .start_date_gte("2023-04-01".to_string())
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
