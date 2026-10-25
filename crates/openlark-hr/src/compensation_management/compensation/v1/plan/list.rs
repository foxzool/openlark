//! 批量查询薪资方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/plan/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::common::shared_models::I18nContent;

/// 批量查询薪资方案请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（可选）
    page_size: Option<i32>,
    /// 分页标记（可选）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
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
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::PlanList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询薪资方案响应数据为空",
        )
        .await
    }
}

/// 批量查询薪资方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 薪资方案列表
    pub items: Vec<Plan>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 薪资方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    /// 薪资方案 ID
    pub id: String,
    /// 薪资方案版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    /// 薪资方案名称
    pub name: String,
    /// 薪资方案描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 薪资方案生效时间
    pub effective_date: String,
    /// 薪资方案适用范围
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_scope: Option<PlanScope>,
    /// 币种 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 开启试用期薪酬状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_salary_status: Option<bool>,
    /// 方案关联的薪资项（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_items: Option<Vec<PlanItem>>,
    /// 方案关联的薪资统计指标（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_indicators: Option<Vec<PlanIndicator>>,
    /// 多语言名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<Vec<I18nContent>>,
    /// 多语言描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_descriptions: Option<Vec<I18nContent>>,
}

/// 薪资方案适用范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanScope {
    /// 是否全部范围
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all: Option<bool>,
    /// 方案适用范围条件组
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_conditions: Option<Vec<PlanCondition>>,
}

/// 方案适用范围条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanCondition {
    /// 适用范围左值：1-部门 2-部门（含下级） 3-工作地点 4-工作地点（含下级） 5-公司 6-公司（含下级） 7-序列 8-序列（含子序列） 9-职务 10-职级 11-人员类型 12-招聘类型 13-国家/地区 14-职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_type: Option<i32>,
    /// 适用范围操作：1-包含 2-不包含
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    /// 适用范围右值（ID 列表，语义随 left_type 变化）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right_value: Option<Vec<String>>,
}

/// 方案关联的薪资项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanItem {
    /// 定薪方式：manual-手动输入 formula-公式计算 fixed-固定值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<String>,
    /// 薪资项 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 方案关联薪资项逻辑配置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_item_logic: Option<AdjustmentLogic>,
    /// 试用期薪酬类型：percentum/manual_input/none/fixed/formula/not_set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_discount_type: Option<String>,
    /// 试用期薪酬百分比
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_discount_percentum: Option<String>,
}

/// 方案关联的薪资统计指标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanIndicator {
    /// 薪资统计指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    /// 方案关联薪资统计指标逻辑配置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_indicator_logic: Option<AdjustmentLogic>,
}

/// 定薪逻辑配置（固定值或公式）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdjustmentLogic {
    /// 固定值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed: Option<String>,
    /// 公式配置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula: Option<Formula>,
}

/// 公式配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Formula {
    /// 公式表达式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// 公式参数列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula_params: Option<Vec<FormulaParam>>,
}

/// 公式参数
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormulaParam {
    /// 引用类型：1-引用薪资项 2-引用薪资指标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_type: Option<i32>,
    /// 引用类型 ID
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
    async fn test_list_plans_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.items[]（含 plan_scope/plan_items/plan_indicators）+ 分页
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"items": [{"id": "2134193289", "tid": "129738122", "name": "基本月薪方案", "description": "基本月薪方案描述", "effective_date": "2022-10-20", "plan_scope": {"is_all": false, "plan_conditions": [{"left_type": 7, "operator": 1, "right_value": ["413431223"]}]}, "currency_id": "341324121", "probation_salary_status": true, "plan_items": [{"adjustment_type": "manual", "item_id": "21341234", "plan_item_logic": {"fixed": "60", "formula": {"expr": "${0} +${1}", "formula_params": [{"ref_type": 1, "id": "23143242"}]}}, "probation_discount_type": "percentum", "probation_discount_percentum": "80.00"}], "plan_indicators": [{"indicator_id": "13243432", "plan_indicator_logic": {"fixed": "60", "formula": {"expr": "${0} +${1}", "formula_params": [{"ref_type": 1, "id": "23143242"}]}}}], "i18n_names": [{"locale": "zh_cn", "value": "中文名称"}], "i18n_descriptions": [{"locale": "zh_cn", "value": "中文名称"}]}], "page_token": "123412344", "has_more": true}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/plans"))
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
            .execute()
            .await
            .expect("查询薪资方案应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "2134193289");
        assert_eq!(data.items[0].tid.as_deref(), Some("129738122"));
        assert_eq!(data.items[0].name, "基本月薪方案");
        assert_eq!(data.items[0].effective_date, "2022-10-20");
        let scope = data.items[0].plan_scope.as_ref().unwrap();
        assert_eq!(scope.is_all, Some(false));
        assert_eq!(
            scope.plan_conditions.as_ref().unwrap()[0].left_type,
            Some(7)
        );
        assert_eq!(scope.plan_conditions.as_ref().unwrap()[0].operator, Some(1));
        assert_eq!(
            scope.plan_conditions.as_ref().unwrap()[0].right_value,
            Some(vec!["413431223".to_string()])
        );
        assert_eq!(
            data.items[0].plan_items.as_ref().unwrap()[0].item_id,
            Some("21341234".to_string())
        );
        assert_eq!(
            data.items[0].plan_items.as_ref().unwrap()[0]
                .plan_item_logic
                .as_ref()
                .unwrap()
                .formula
                .as_ref()
                .unwrap()
                .expr,
            Some("${0} +${1}".to_string())
        );
        assert_eq!(
            data.items[0].plan_indicators.as_ref().unwrap()[0].indicator_id,
            Some("13243432".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/compensation/v1/plans");
    }
}
