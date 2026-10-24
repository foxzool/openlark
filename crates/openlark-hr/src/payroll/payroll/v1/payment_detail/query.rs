//! 批量查询发薪明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_detail/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量查询发薪明细请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 页码（必填，第一页从 1 开始，取值范围 1~100000）
    page_index: Option<i32>,
    /// 每页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 算薪项 ID 列表（可选）
    acct_item_ids: Option<Vec<String>>,
    /// 员工的飞书人事雇佣 ID 列表（必填）
    employee_ids: Option<Vec<String>>,
    /// 发薪日开始时间（可选，格式 YYYY-MM-dd；与结束时间或 activity_ids 至少传一组，否则服务端返回 2500006）
    pay_period_start_date: Option<String>,
    /// 发薪日结束时间（可选，格式 YYYY-MM-dd）
    pay_period_end_date: Option<String>,
    /// 发薪活动 ID 列表（可选）
    activity_ids: Option<Vec<String>>,
    /// 是否需要查询算薪明细的分段信息（可选）
    include_segment_data: Option<bool>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_index: None,
            page_size: None,
            acct_item_ids: None,
            employee_ids: None,
            pay_period_start_date: None,
            pay_period_end_date: None,
            activity_ids: None,
            include_segment_data: None,
            config,
        }
    }

    /// 设置页码（必填，第一页从 1 开始，取值范围 1~100000）
    pub fn page_index(mut self, page_index: i32) -> Self {
        self.page_index = Some(page_index);
        self
    }

    /// 设置每页大小（必填，取值范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置算薪项 ID 列表（可选）
    pub fn acct_item_ids(mut self, acct_item_ids: Vec<String>) -> Self {
        self.acct_item_ids = Some(acct_item_ids);
        self
    }

    /// 设置员工的飞书人事雇佣 ID 列表（必填）
    pub fn employee_ids(mut self, employee_ids: Vec<String>) -> Self {
        self.employee_ids = Some(employee_ids);
        self
    }

    /// 设置发薪日开始时间（可选，格式 YYYY-MM-dd）
    pub fn pay_period_start_date(mut self, pay_period_start_date: String) -> Self {
        self.pay_period_start_date = Some(pay_period_start_date);
        self
    }

    /// 设置发薪日结束时间（可选，格式 YYYY-MM-dd）
    pub fn pay_period_end_date(mut self, pay_period_end_date: String) -> Self {
        self.pay_period_end_date = Some(pay_period_end_date);
        self
    }

    /// 设置发薪活动 ID 列表（可选）
    pub fn activity_ids(mut self, activity_ids: Vec<String>) -> Self {
        self.activity_ids = Some(activity_ids);
        self
    }

    /// 设置是否需要查询算薪明细的分段信息（可选）
    pub fn include_segment_data(mut self, include_segment_data: bool) -> Self {
        self.include_segment_data = Some(include_segment_data);
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
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段（官方注意事项：employee_ids 为必填；page_index/page_size 必填）
        let page_index = self
            .page_index
            .ok_or_else(|| validation_error("page_index 不能为空", "页码必填，第一页从 1 开始"))?;
        if !(1..=100000).contains(&page_index) {
            return Err(validation_error(
                "page_index 超出范围",
                "页码取值范围为 1~100000",
            ));
        }
        let page_size = self.page_size.ok_or_else(|| {
            validation_error("page_size 不能为空", "每页大小必填，取值范围 1~100")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(validation_error(
                "page_size 超出范围",
                "每页大小取值范围为 1~100",
            ));
        }
        let employee_ids = self.employee_ids.ok_or_else(|| {
            validation_error("employee_ids 不能为空", "员工的飞书人事雇佣 ID 列表必填")
        })?;
        validate_required_list!(employee_ids, 50, "employee_ids 不能为空且不能超过 50 个");

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::PaymentDetailQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            page_index,
            page_size,
            acct_item_ids: self.acct_item_ids,
            employee_ids,
            pay_period_start_date: self.pay_period_start_date,
            pay_period_end_date: self.pay_period_end_date,
            activity_ids: self.activity_ids,
            include_segment_data: self.include_segment_data,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询发薪明细响应数据为空",
        )
        .await
    }
}

/// 批量查询发薪明细请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 页码，第一页从 1 开始，取值范围 1~100000
    pub page_index: i32,
    /// 每页大小，取值范围 1~100
    pub page_size: i32,
    /// 算薪项 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_item_ids: Option<Vec<String>>,
    /// 员工的飞书人事雇佣 ID 列表（必填）
    pub employee_ids: Vec<String>,
    /// 发薪日开始时间，格式 YYYY-MM-dd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_period_start_date: Option<String>,
    /// 发薪日结束时间，格式 YYYY-MM-dd
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_period_end_date: Option<String>,
    /// 发薪活动 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ids: Option<Vec<String>>,
    /// 是否需要查询算薪明细的分段信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_segment_data: Option<bool>,
}

/// 批量查询发薪明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 发薪明细列表
    pub payment_details: Vec<PaymentDetail>,
    /// 发薪明细总数
    pub total: i64,
}

/// 发薪明细
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentDetail {
    /// 员工的飞书人事雇佣 ID
    pub employee_id: String,
    /// 发薪明细所在的发薪活动 ID
    pub activity_id: String,
    /// 发薪明细详情
    pub payment_accounting_items: Vec<PaymentAccountingItem>,
}

/// 发薪明细详情（算薪项维度）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentAccountingItem {
    /// 算薪项 ID
    pub id: String,
    /// 算薪项名称（多语种）
    pub accounting_item_names: Vec<I18nContent>,
    /// 算薪项值
    pub accounting_item_value: AccountingItemValue,
    /// 算薪项分段数据
    pub segment_values: Vec<SegmentValue>,
    /// 算薪项类型：1-文本 2-金额 3-数值 4-百分比 5-日期 6-引用
    pub accounting_item_type: i32,
}

/// 算薪项值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountingItemValue {
    /// 算薪项数据原始值；数据来源为「人工导入」且算薪项为引用类型时可能为空
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_value: Option<String>,
    /// 引用类型算薪项展示值
    pub reference_values: Vec<I18nContent>,
}

/// 算薪项分段数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SegmentValue {
    /// 分段开始时间-毫秒级时间戳，[start_time, end_time] 是左闭右闭区间
    pub start_time: String,
    /// 分段结束时间-毫秒级时间戳，[start_time, end_time] 是左闭右闭区间
    pub end_time: String,
    /// 引用类型算薪项分段展示值
    pub reference_values: Vec<I18nContent>,
    /// 算薪项分段原始值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_value: Option<String>,
}

/// 多语种内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语种，如 zh_cn、en_us
    pub locale: String,
    /// 语种对应的值
    pub value: String,
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
    async fn test_query_payment_details_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应形状：payment_details + total
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "payment_details": [{
                    "employee_id": "7202076988667019308",
                    "activity_id": "7202076988667019308",
                    "payment_accounting_items": [{
                        "id": "7202076988667019333",
                        "accounting_item_names": [{"locale": "zh_cn", "value": "基本工资"}],
                        "accounting_item_value": {
                            "original_value": "100",
                            "reference_values": [{"locale": "zh_cn", "value": "名称"}]
                        },
                        "segment_values": [{
                            "start_time": "7220356259681386540",
                            "end_time": "7220356259681386540",
                            "reference_values": [{"locale": "zh_cn", "value": "名称"}],
                            "original_value": "10000"
                        }],
                        "accounting_item_type": 1
                    }]
                }],
                "total": 50000
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_detail/query"))
            .and(body_json(json!({
                "page_index": 1,
                "page_size": 10,
                "employee_ids": ["7202076988667019222"],
                "activity_ids": ["7202076988667019308"]
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
            .page_index(1)
            .page_size(10)
            .employee_ids(vec!["7202076988667019222".to_string()])
            .activity_ids(vec!["7202076988667019308".to_string()])
            .execute()
            .await
            .expect("查询发薪明细应成功");

        assert_eq!(data.total, 50000);
        assert_eq!(data.payment_details.len(), 1);
        assert_eq!(data.payment_details[0].employee_id, "7202076988667019308");
        let item = &data.payment_details[0].payment_accounting_items[0];
        assert_eq!(item.id, "7202076988667019333");
        assert_eq!(item.accounting_item_names[0].value, "基本工资");
        assert_eq!(item.accounting_item_type, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/payment_detail/query"
        );
    }

    /// page_index / page_size / employee_ids 为必填，缺失时应校验失败
    #[tokio::test]
    async fn test_query_payment_details_requires_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = QueryRequest::new(config.clone()).execute().await;
        assert!(result.is_err());

        let result = QueryRequest::new(config)
            .page_index(1)
            .page_size(10)
            .execute()
            .await;
        assert!(result.is_err());
    }
}
