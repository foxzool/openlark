//! 批量查询成本分摊方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/cost_allocation_plan/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量查询成本分摊方案请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 期间（必填，生成成本分摊报表对应的年月，格式 yyyy-MM，仅返回期间内生效的方案）
    pay_period: Option<String>,
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
            pay_period: None,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置期间（必填，格式 yyyy-MM）
    pub fn pay_period(mut self, pay_period: String) -> Self {
        self.pay_period = Some(pay_period);
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
        let pay_period = self
            .pay_period
            .ok_or_else(|| validation_error("pay_period 不能为空", "期间必填，格式 yyyy-MM"))?;
        validate_required!(pay_period, "pay_period 不能为空");
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
        let api_endpoint = PayrollApiV1::CostAllocationPlanList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("pay_period", pay_period);
        request = request.query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询成本分摊方案响应数据为空",
        )
        .await
    }
}

/// 批量查询成本分摊方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 成本分摊方案列表
    pub items: Vec<CostAllocationPlan>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 成本分摊方案（官方 cost_allocation_plan）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostAllocationPlan {
    /// 成本分摊方案 id，唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 方案名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nContent>>,
    /// 适用国家 ID（通过查询国家/地区信息获取）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_country_region: Option<String>,
    /// 成本分摊方案对应的汇总维度列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<Dimension>>,
    /// 成本分摊方案对应的成本项列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_items: Option<Vec<CostItem>>,
}

/// 汇总维度（官方 dimension）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dimension {
    /// 汇总维度信息（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
    /// 汇总维度字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 汇总维度对象名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_api_name: Option<String>,
}

/// 成本项（官方 cost_item）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostItem {
    /// 成本项的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 成本项名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nContent>>,
    /// 成本项是否启用更正
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_correct: Option<bool>,
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
    async fn test_list_cost_allocation_plans_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：items[]{id, names, applicable_country_region, dimensions, cost_items}
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "id": "6823630319749593096",
                        "names": [
                            {"locale": "zh_cn", "value": "名称"}
                        ],
                        "applicable_country_region": "6823630319749593389",
                        "dimensions": [
                            {
                                "i18n_names": [
                                    {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                                ],
                                "api_name": "company",
                                "obj_api_name": "jobData"
                            }
                        ],
                        "cost_items": [
                            {
                                "id": "7433424967234601004",
                                "name": [
                                    {"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}
                                ],
                                "enable_correct": true
                            }
                        ]
                    }
                ],
                "page_token": "6823630319749592467",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_plans"))
            .and(query_param("pay_period", "2023-11"))
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
            .pay_period("2023-11".to_string())
            .page_size(50)
            .execute()
            .await
            .expect("批量查询成本分摊方案应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        let plan = &data.items[0];
        assert_eq!(plan.id.as_deref(), Some("6823630319749593096"));
        assert_eq!(
            plan.names.as_ref().unwrap()[0].value.as_deref(),
            Some("名称")
        );
        assert_eq!(
            plan.applicable_country_region.as_deref(),
            Some("6823630319749593389")
        );
        assert_eq!(
            plan.dimensions.as_ref().unwrap()[0].api_name.as_deref(),
            Some("company")
        );
        assert_eq!(
            plan.cost_items.as_ref().unwrap()[0].enable_correct,
            Some(true)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/payroll/v1/cost_allocation_plans"
        );
    }
}
