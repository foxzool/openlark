//! 查询发薪活动明细列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/payment_activity_detail/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询发薪活动明细列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 页码（必填，第一页从 1 开始，取值范围 1~100000）
    page_index: Option<i32>,
    /// 每页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 发薪活动 ID（必填，可通过「查询发薪活动列表」获取）
    activity_id: Option<String>,
    /// 是否需要查询算薪明细的分段信息（可选）
    include_segment_data: Option<bool>,
    /// 算薪项 ID 列表（可选，0~100 个）
    acct_item_ids: Option<Vec<String>>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_index: None,
            page_size: None,
            activity_id: None,
            include_segment_data: None,
            acct_item_ids: None,
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

    /// 设置发薪活动 ID（必填）
    pub fn activity_id(mut self, activity_id: String) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// 设置是否需要查询算薪明细的分段信息（可选）
    pub fn include_segment_data(mut self, include_segment_data: bool) -> Self {
        self.include_segment_data = Some(include_segment_data);
        self
    }

    /// 设置算薪项 ID 列表（可选，0~100 个）
    pub fn acct_item_ids(mut self, acct_item_ids: Vec<String>) -> Self {
        self.acct_item_ids = Some(acct_item_ids);
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
        let activity_id = self
            .activity_id
            .ok_or_else(|| validation_error("activity_id 不能为空", "发薪活动 ID 必填"))?;
        validate_required!(activity_id, "activity_id 不能为空");
        if let Some(ref acct_item_ids) = self.acct_item_ids
            && acct_item_ids.len() > 100
        {
            return Err(validation_error(
                "acct_item_ids 超出范围",
                "算薪项 ID 列表最多 100 个",
            ));
        }

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::PaymentActivityDetailList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request
            .query("page_index", page_index.to_string())
            .query("page_size", page_size.to_string())
            .query("activity_id", &activity_id);
        if let Some(include_segment_data) = self.include_segment_data {
            request = request.query("include_segment_data", include_segment_data.to_string());
        }
        if let Some(ref acct_item_ids) = self.acct_item_ids {
            request = request.query("acct_item_ids", acct_item_ids.join(","));
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询发薪活动明细列表响应数据为空",
        )
        .await
    }
}

/// 查询发薪活动明细列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 发薪明细列表
    pub payment_activity_details: Vec<PaymentActivityDetail>,
    /// 发薪明细总数
    pub total: i64,
}

/// 发薪活动明细
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentActivityDetail {
    /// 员工的飞书人事雇佣 ID
    pub employee_id: String,
    /// 发薪明细详情
    pub payment_details: Vec<PaymentAccountingItem>,
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
    async fn test_list_payment_activity_details_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应形状：payment_activity_details + total
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "payment_activity_details": [{
                    "employee_id": "7202076988667019308",
                    "payment_details": [{
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
                        "accounting_item_type": 2
                    }]
                }],
                "total": 50000
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activity_details"))
            .and(query_param("page_index", "1"))
            .and(query_param("page_size", "20"))
            .and(query_param("activity_id", "7202076988667019308"))
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
            .page_index(1)
            .page_size(20)
            .activity_id("7202076988667019308".to_string())
            .execute()
            .await
            .expect("查询发薪活动明细应成功");

        assert_eq!(data.total, 50000);
        assert_eq!(data.payment_activity_details.len(), 1);
        assert_eq!(
            data.payment_activity_details[0].employee_id,
            "7202076988667019308"
        );
        let item = &data.payment_activity_details[0].payment_details[0];
        assert_eq!(item.id, "7202076988667019333");
        assert_eq!(item.accounting_item_names[0].value, "基本工资");
        assert_eq!(
            item.accounting_item_value.original_value,
            Some("100".to_string())
        );
        assert_eq!(
            item.segment_values[0].original_value,
            Some("10000".to_string())
        );
        assert_eq!(item.accounting_item_type, 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/payment_activity_details"
        );
    }

    /// page_index / page_size / activity_id 为必填，缺失时应校验失败
    #[tokio::test]
    async fn test_list_payment_activity_details_requires_required_params() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = ListRequest::new(config.clone()).execute().await;
        assert!(result.is_err());

        let result = ListRequest::new(config)
            .page_size(20)
            .activity_id("pa_1".to_string())
            .execute()
            .await;
        assert!(result.is_err());
    }
}
