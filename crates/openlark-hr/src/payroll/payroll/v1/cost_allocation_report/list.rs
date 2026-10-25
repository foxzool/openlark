//! 查询成本分摊报表汇总数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/cost_allocation_report/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询成本分摊报表汇总数据请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 成本分摊方案 ID（必填，通过批量查询成本分摊方案获取）
    cost_allocation_plan_id: Option<String>,
    /// 期间（必填，成本分摊数据对应的年月，格式 yyyy-MM）
    pay_period: Option<String>,
    /// 报表类型（必填：0-默认 1-计提 2-实发）
    report_type: Option<i32>,
    /// 分页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填，表示从头开始遍历）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            cost_allocation_plan_id: None,
            pay_period: None,
            report_type: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置成本分摊方案 ID（必填）
    pub fn cost_allocation_plan_id(mut self, cost_allocation_plan_id: String) -> Self {
        self.cost_allocation_plan_id = Some(cost_allocation_plan_id);
        self
    }

    /// 设置期间（必填，格式 yyyy-MM）
    pub fn pay_period(mut self, pay_period: String) -> Self {
        self.pay_period = Some(pay_period);
        self
    }

    /// 设置报表类型（必填：0-默认 1-计提 2-实发）
    pub fn report_type(mut self, report_type: i32) -> Self {
        self.report_type = Some(report_type);
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
        let cost_allocation_plan_id = self.cost_allocation_plan_id.ok_or_else(|| {
            validation_error(
                "cost_allocation_plan_id 不能为空",
                "成本分摊方案 ID 必填，通过批量查询成本分摊方案获取",
            )
        })?;
        validate_required!(cost_allocation_plan_id, "cost_allocation_plan_id 不能为空");
        let pay_period = self
            .pay_period
            .ok_or_else(|| validation_error("pay_period 不能为空", "期间必填，格式 yyyy-MM"))?;
        validate_required!(pay_period, "pay_period 不能为空");
        let report_type = self.report_type.ok_or_else(|| {
            validation_error("report_type 不能为空", "报表类型必填：0-默认 1-计提 2-实发")
        })?;
        if !(0..=2).contains(&report_type) {
            return Err(validation_error(
                "report_type 超出范围",
                "报表类型取值范围为 0~2",
            ));
        }
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
        let api_endpoint = PayrollApiV1::CostAllocationReportList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("cost_allocation_plan_id", cost_allocation_plan_id);
        request = request.query("pay_period", pay_period);
        request = request.query("report_type", report_type.to_string());
        request = request.query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询成本分摊报表汇总数据响应数据为空",
        )
        .await
    }
}

/// 查询成本分摊报表汇总数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 期间，成本分摊报表对应的年月
    pub pay_period: String,
    /// 报表名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation_report_names: Option<Vec<I18nContent>>,
    /// 汇总数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation_report_datas: Option<Vec<CostAllocationReportData>>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 成本分摊报表汇总数据（官方 cost_allocation_report_data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostAllocationReportData {
    /// 数据维度汇总
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_summary_dimensions: Option<Vec<DataSummaryDimension>>,
    /// 成本项数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_cost_item: Option<CompensationCostItem>,
}

/// 数据维度汇总（官方 data_summary_dimension）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataSummaryDimension {
    /// 层级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_level: Option<i32>,
    /// 类型：1-公司主体 2-成本中心 3-部门 4-薪资组 5-人员类型 6-雇佣状态 7-转正状态 8-职务 9-序列 10-职级 11-工时制度 12-合同类型 13-算薪项 100-自定义维度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<i32>,
    /// 维度 ID（需根据 dimension_type 转换为对应对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_value_id: Option<String>,
    /// 算薪项为特定枚举值时的枚举维度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_dimension: Option<EnumDimension>,
    /// 维度引用对象的基础信息（维度为引用类型字段时返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_value_lookup_info: Option<DimensionValueLookupInfo>,
    /// 维度名称（算薪项、自定义维度使用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_names: Option<Vec<I18nContent>>,
    /// 数据维度表头（算薪项、自定义维度使用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_titles: Option<Vec<I18nContent>>,
}

/// 枚举维度（官方 enum_object）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumDimension {
    /// 枚举对象 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_value_id: Option<String>,
    /// 枚举对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_key: Option<String>,
}

/// 维度引用对象的基础信息（官方 dimension_value_lookup_info）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DimensionValueLookupInfo {
    /// 引用对象类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 引用对象的 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 引用对象的 code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// 成本项数据（官方 compensation_cost_item）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompensationCostItem {
    /// 发薪人数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_individuals_for_payment: Option<i32>,
    /// 成本项数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_costs: Option<Vec<CompensationCost>>,
}

/// 成本项（官方 compensation_cost）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompensationCost {
    /// 成本项值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_cost_value: Option<String>,
    /// 成本项名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
}

/// 多语种内容（官方 i18n_content，id 为名称对应的实体 id）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语种，如 zh_cn、en_us
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 语种对应的值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 名称对应的实体 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    async fn test_list_cost_allocation_reports_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data 为 pay_period/cost_allocation_report_names/cost_allocation_report_datas 单对象
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "pay_period": "2023-11",
                "page_token": "6823630319749580302",
                "has_more": true,
                "cost_allocation_report_names": [
                    {"locale": "zh_cn", "value": "名称"}
                ],
                "cost_allocation_report_datas": [
                    {
                        "data_summary_dimensions": [
                            {
                                "dimension_level": 1,
                                "dimension_type": 1,
                                "dimension_value_id": "6823630319749580306",
                                "enum_dimension": {
                                    "enum_value_id": "7188920315914207276",
                                    "enum_key": "company"
                                },
                                "dimension_value_lookup_info": {
                                    "type": "work_calendar",
                                    "id": "6961286846093788621",
                                    "code": "D1230011115"
                                },
                                "dimension_names": [
                                    {"locale": "zh_cn", "value": "名称"}
                                ],
                                "dimension_titles": [
                                    {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                                ]
                            }
                        ],
                        "compensation_cost_item": {
                            "number_of_individuals_for_payment": 100,
                            "compensation_costs": [
                                {
                                    "compensation_cost_value": "123456.78",
                                    "i18n_names": [
                                        {"locale": "zh_cn", "value": "名称"}
                                    ]
                                }
                            ]
                        }
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_reports"))
            .and(query_param(
                "cost_allocation_plan_id",
                "6823630319749580304",
            ))
            .and(query_param("pay_period", "2023-11"))
            .and(query_param("report_type", "1"))
            .and(query_param("page_size", "50"))
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
            .cost_allocation_plan_id("6823630319749580304".to_string())
            .pay_period("2023-11".to_string())
            .report_type(1)
            .page_size(50)
            .execute()
            .await
            .expect("查询成本分摊报表汇总数据应成功");

        assert_eq!(data.pay_period, "2023-11");
        assert!(data.has_more);
        let report_datas = data
            .cost_allocation_report_datas
            .as_ref()
            .expect("cost_allocation_report_datas 应反序列化");
        assert_eq!(report_datas.len(), 1);
        let dimension = report_datas[0]
            .data_summary_dimensions
            .as_ref()
            .expect("data_summary_dimensions 应反序列化")[0]
            .clone();
        assert_eq!(dimension.dimension_level, Some(1));
        assert_eq!(
            dimension.dimension_value_id.as_deref(),
            Some("6823630319749580306")
        );
        assert_eq!(
            dimension
                .enum_dimension
                .as_ref()
                .and_then(|e| e.enum_key.as_deref()),
            Some("company")
        );
        assert_eq!(
            dimension
                .dimension_value_lookup_info
                .as_ref()
                .and_then(|l| l.code.as_deref()),
            Some("D1230011115")
        );
        assert_eq!(
            dimension.dimension_titles.as_ref().unwrap()[0]
                .id
                .as_deref(),
            Some("723123123123123213")
        );
        let cost_item = report_datas[0]
            .compensation_cost_item
            .as_ref()
            .expect("compensation_cost_item 应反序列化");
        assert_eq!(cost_item.number_of_individuals_for_payment, Some(100));
        assert_eq!(
            cost_item.compensation_costs.as_ref().unwrap()[0]
                .compensation_cost_value
                .as_deref(),
            Some("123456.78")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/cost_allocation_reports"
        );
    }
}
