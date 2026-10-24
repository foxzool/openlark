//! 根据方案ID和生效日期批量查询参保方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/social_plan/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 根据方案ID和生效日期批量查询参保方案请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 参保方案 ID 列表（必填，最少 1 个、最大 200 个）
    plan_ids: Vec<String>,
    /// 生效日期（必填，查询在该日期生效的参保方案数据，格式 YYYY-mm-dd，长度 10 字符）
    effective_date: String,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, plan_ids: Vec<String>, effective_date: String) -> Self {
        Self {
            plan_ids,
            effective_date,
            config,
        }
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required_list!(self.plan_ids, 200, "plan_ids 不能为空且不能超过 200 个");
        // 官方要求：日期格式为 YYYY-mm-dd，长度为 10 字符
        let date = self.effective_date.trim();
        let bytes = date.as_bytes();
        if date.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(validation_error(
                "effective_date 无效",
                "effective_date 必须为 YYYY-mm-dd 格式的 10 字符日期，如 2024-01-01",
            ));
        }
        Ok(())
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

        self.validate()?;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::SocialPlanQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            plan_ids: self.plan_ids,
            effective_date: self.effective_date,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询参保方案响应数据为空",
        )
        .await
    }
}

/// 查询请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 参保方案 ID 列表
    pub plan_ids: Vec<String>,
    /// 生效日期，格式 YYYY-mm-dd
    pub effective_date: String,
}

/// 根据方案ID和生效日期批量查询参保方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 方案列表
    pub plans: Vec<SocialPlan>,
}

/// 参保方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlan {
    /// 参保方案 ID
    pub plan_id: String,
    /// 参保方案版本 ID
    pub plan_tid: String,
    /// 参保方案名称（多语种）
    pub name: I18nName,
    /// 生效时间（yyyy-MM-dd）
    pub effective_date: String,
    /// 是否启用
    pub active: bool,
    /// 险种类型：social_insurance-社保 provident_fund-公积金
    pub insurance_type: String,
    /// 参保方案适用范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<SocialPlanScope>,
    /// 参保信息（险种维度）
    pub item_detail: Vec<SocialPlanItemDetail>,
    /// 备注（多语种）
    pub remark: I18nName,
}

/// 参保方案适用范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlanScope {
    /// 是否适用于全部
    pub is_all: bool,
    /// 条件规则，两层结构 [条件组[条件]]：多个条件组之间为「或」关系，条件组内多个条件为「且」关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Vec<SocialPlanCondition>>>,
}

/// 适用范围条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlanCondition {
    /// 适用范围左值：1-参保城市 2-缴纳主体
    pub left_type: i32,
    /// 适用范围操作：1-包含 2-不包含
    pub operator: i32,
    /// 适用范围右值（left_type=1 时为参保城市 ID，left_type=2 时为缴纳主体 ID）
    pub right_values: Vec<String>,
}

/// 参保信息（险种维度）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlanItemDetail {
    /// 险种 ID
    pub item_id: String,
    /// 险种名（多语种）
    pub item_name: I18nName,
    /// 个人缴纳配置
    pub item_setting_of_person: SocialPlanItemSetting,
    /// 企业缴纳配置
    pub item_setting_of_company: SocialPlanItemSetting,
    /// 缴纳频率：annually-年缴 monthly-月缴 quarterly-季缴
    pub payment_frequency: String,
    /// 缴纳月份，1 月～12 月
    pub payment_months: Vec<i32>,
}

/// 缴纳配置（基数上下限、缴纳比例、舍入规则等）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlanItemSetting {
    /// 基数下限，浮点数保留二位小数，单位：元
    pub lower_limit: String,
    /// 基数上限，浮点数保留二位小数，单位：元
    pub upper_limit: String,
    /// 缴纳比例，浮点数默认填充到二位小数（支持四位），单位 %
    pub payment_ratio: String,
    /// 缴纳金舍入规则：rounding-四舍五入 round_up-向上舍入 round_down-向下舍入
    pub payment_rounding_rule: String,
    /// 缴纳金小数位数（0~6 位）
    pub payment_decimals: i32,
    /// 附加固定金额，浮点数保留二位小数，单位：元
    pub fixed_payment: String,
}

/// 多语种名称
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nName {
    /// 中文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
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
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_social_plan_query_request_builder() {
        let request = QueryRequest::new(
            create_test_config(),
            vec!["plan_1".to_string(), "plan_2".to_string()],
            "2024-01-01".to_string(),
        );

        assert_eq!(request.plan_ids.len(), 2);
        assert_eq!(request.effective_date, "2024-01-01");
    }

    #[test]
    fn test_social_plan_query_request_body_serialize() {
        let body = QueryRequestBody {
            plan_ids: vec!["10001".to_string()],
            effective_date: "2024-01-01".to_string(),
        };
        let value = serde_json::to_value(body).expect("序列化请求体失败");

        assert_eq!(value["plan_ids"], json!(["10001"]));
        assert_eq!(value["effective_date"], json!("2024-01-01"));
    }

    #[test]
    fn test_social_plan_query_response_deserialize() {
        // 官方响应示例形状：data.plans[]
        let value = json!({
            "plans": [{
                "plan_id": "7198138281677325868",
                "plan_tid": "7198138281677358636",
                "name": {"zh_cn": "sky-公积金参保方案", "en_us": "sky-provident fund plan test"},
                "effective_date": "2024-01-01",
                "active": false,
                "insurance_type": "provident_fund",
                "scope": {
                    "is_all": true,
                    "rules": [[{"left_type": 1, "operator": 1, "right_values": ["1"]}]]
                },
                "item_detail": [{
                    "item_id": "7200726176044680748",
                    "item_name": {"zh_cn": "养老保险", "en_us": "endowment insurance"},
                    "item_setting_of_person": {
                        "lower_limit": "1000.00",
                        "upper_limit": "2000.00",
                        "payment_ratio": "8.00",
                        "payment_rounding_rule": "rounding",
                        "payment_decimals": 2,
                        "fixed_payment": "200.00"
                    },
                    "item_setting_of_company": {
                        "lower_limit": "1000.00",
                        "upper_limit": "2000.00",
                        "payment_ratio": "8.00",
                        "payment_rounding_rule": "round_down",
                        "payment_decimals": 2,
                        "fixed_payment": "200.00"
                    },
                    "payment_frequency": "quarterly",
                    "payment_months": [1, 4]
                }],
                "remark": {"zh_cn": "中文备注", "en_us": "english remark"}
            }]
        });
        let response: QueryResponse = serde_json::from_value(value).expect("反序列化响应失败");

        assert_eq!(response.plans.len(), 1);
        let plan = &response.plans[0];
        assert_eq!(plan.plan_id, "7198138281677325868");
        assert_eq!(plan.name.zh_cn, Some("sky-公积金参保方案".to_string()));
        assert_eq!(plan.effective_date, "2024-01-01");
        assert!(!plan.active);
        assert_eq!(plan.insurance_type, "provident_fund");
        assert_eq!(
            plan.scope.as_ref().unwrap().rules.as_ref().unwrap()[0][0].left_type,
            1
        );
        assert_eq!(plan.item_detail[0].item_id, "7200726176044680748");
        assert_eq!(plan.item_detail[0].payment_months, vec![1, 4]);
        assert_eq!(plan.remark.en_us, Some("english remark".to_string()));
    }

    #[test]
    fn test_social_plan_query_validation() {
        let request = QueryRequest::new(create_test_config(), vec![], "2024-01-01".to_string());
        assert!(request.validate().is_err());

        // effective_date 必须为 10 字符日期字符串（官方 YYYY-mm-dd）
        let invalid_date_request = QueryRequest::new(
            create_test_config(),
            vec!["plan_1".to_string()],
            "1707000000".to_string(),
        );
        assert!(invalid_date_request.validate().is_err());

        let valid_request = QueryRequest::new(
            create_test_config(),
            vec!["plan_1".to_string()],
            "2024-01-01".to_string(),
        );
        assert!(valid_request.validate().is_ok());
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_query_social_plans_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"plans": []}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_plans/query"))
            .and(body_json(
                json!({"plan_ids": ["10001"], "effective_date": "2024-01-01"}),
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

        let data = QueryRequest::new(config, vec!["10001".to_string()], "2024-01-01".to_string())
            .execute()
            .await
            .expect("批量查询参保方案应成功");

        assert!(data.plans.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/social_plans/query"
        );
    }
}
