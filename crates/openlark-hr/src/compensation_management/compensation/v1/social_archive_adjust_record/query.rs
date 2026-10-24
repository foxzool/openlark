//! 通过员工ID批量获取社保增减员记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/social_archive_adjust_record/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 通过员工ID批量获取社保增减员记录请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 用户 ID 列表（必填，与入参 user_id_type 类型一致，最多 200 个）
    user_id_list: Vec<String>,
    /// 增减员类型（必填）：increase-增员 attrition-减员
    record_type: String,
    /// 用户 ID 类型（查询参数，默认 open_id：open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, user_id_list: Vec<String>, record_type: String) -> Self {
        Self {
            user_id_list,
            record_type,
            user_id_type: None,
            config,
        }
    }

    /// 设置用户 ID 类型（可选，默认 open_id）
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

        // 1. 验证必填字段
        validate_required_list!(
            self.user_id_list,
            200,
            "user_id_list 不能为空且不能超过 200 个"
        );
        validate_required!(self.record_type, "record_type 不能为空");
        if self.record_type != "increase" && self.record_type != "attrition" {
            return Err(validation_error(
                "record_type 取值无效",
                "record_type 仅支持 increase（增员）或 attrition（减员）",
            ));
        }

        // 2. 构建端点与查询参数
        let api_endpoint = CompensationApiV1::SocialArchiveAdjustRecordQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_id_list: self.user_id_list,
            record_type: self.record_type,
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
            "通过员工ID批量获取社保增减员记录响应数据为空",
        )
        .await
    }
}

/// 通过员工ID批量获取社保增减员记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表，与入参 user_id_type 类型一致
    pub user_id_list: Vec<String>,
    /// 增减员类型：increase-增员 attrition-减员
    pub record_type: String,
}

/// 通过员工ID批量获取社保增减员记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 增/减员记录列表
    pub records: Vec<SocialArchiveAdjustRecord>,
}

/// 社保增减员记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialArchiveAdjustRecord {
    /// 员工 ID，与入参 user_id_type 类型一致
    pub user_id: String,
    /// 类型：increase-增员 attrition-减员
    pub record_type: String,
    /// 员工增减员记录，包括社保、公积金记录
    pub details: Vec<SocialArchiveDetail>,
}

/// 员工参保档案明细（社保/公积金）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialArchiveDetail {
    /// 调整说明（多语种）
    pub description: I18nDescription,
    /// 类型：social_insurance-社保 provident_fund-公积金
    pub insurance_type: String,
    /// 参保状态：contribution-参保 not_contribution-未参保 stopped_contribution-停缴；
    /// 非「参保」状态下，基数、险种数据等为空
    pub insurance_status: String,
    /// 员工参保档案 ID（增减员记录中返回为空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 员工参保档案调整记录 ID（增减员记录中返回为空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    /// 参保方案 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// 参保方案版本 ID（record_type 为 attrition 时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_tid: Option<String>,
    /// 参保城市 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// 社保缴纳主体 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// 社保/公积金账户类型（record_type 为 attrition 时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 社保/公积金账号（record_type 为 attrition 时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_account: Option<String>,
    /// 申报缴纳基数，单位：元（record_type 为 attrition 时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_salary: Option<String>,
    /// 险种数据详情（record_type 为 attrition 时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance_details: Option<Vec<SocialArchiveItem>>,
    /// 待增员参保年月/待减员停缴年月（yyyy-MM-dd）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}

/// 险种数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialArchiveItem {
    /// 险种 ID
    pub insurance_id: String,
    /// 企业缴纳金额，单位：元
    pub company_deduction: String,
    /// 企业缴纳配置
    pub company_setting: SocialPlanItemSetting,
    /// 个人缴纳金额，单位：元
    pub personal_deduction: String,
    /// 个人缴纳配置
    pub personal_setting: SocialPlanItemSetting,
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

/// 多语种描述
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nDescription {
    /// 中文描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文描述
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

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_query_social_archive_adjust_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应形状：data.records[]
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "records": [{
                    "user_id": "9960875",
                    "record_type": "increase",
                    "details": [{
                        "description": {"zh_cn": "2024年社保基数调整", "en_us": "2024 Social Insurance Base Adjustment"},
                        "insurance_type": "social_insurance",
                        "insurance_status": "contribution",
                        "id": null,
                        "tid": null,
                        "plan_id": "7199837930276308524",
                        "plan_tid": "7199643907243361836",
                        "location_id": "7152398273176192556",
                        "company_id": "7096372135576618540",
                        "account_type": "associated_company",
                        "insurance_account": "ac123456",
                        "base_salary": "1000.10",
                        "insurance_details": [{
                            "insurance_id": "7200725830765790764",
                            "company_deduction": "2000.20",
                            "company_setting": {
                                "lower_limit": "1000.00",
                                "upper_limit": "2000.00",
                                "payment_ratio": "8.00",
                                "payment_rounding_rule": "rounding",
                                "payment_decimals": 2,
                                "fixed_payment": "200.00"
                            },
                            "personal_deduction": "1000.20",
                            "personal_setting": {
                                "lower_limit": "1000.00",
                                "upper_limit": "2000.00",
                                "payment_ratio": "8.00",
                                "payment_rounding_rule": "round_down",
                                "payment_decimals": 2,
                                "fixed_payment": "200.00"
                            },
                            "payment_frequency": "monthly",
                            "payment_months": [1, 2, 3]
                        }],
                        "effective_date": "2024-01-01"
                    }]
                }]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/social_archive_adjust_record/query",
            ))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "user_id_list": ["7134995965475620396"],
                "record_type": "increase"
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

        let data = QueryRequest::new(
            config,
            vec!["7134995965475620396".to_string()],
            "increase".to_string(),
        )
        .user_id_type("open_id".to_string())
        .execute()
        .await
        .expect("查询社保档案调整记录应成功");

        assert_eq!(data.records.len(), 1);
        assert_eq!(data.records[0].user_id, "9960875");
        assert_eq!(data.records[0].record_type, "increase");
        let detail = &data.records[0].details[0];
        assert_eq!(detail.insurance_type, "social_insurance");
        assert_eq!(detail.insurance_status, "contribution");
        assert_eq!(detail.id, None);
        assert_eq!(detail.plan_id, Some("7199837930276308524".to_string()));
        assert_eq!(detail.base_salary, Some("1000.10".to_string()));
        let item = &detail.insurance_details.as_ref().unwrap()[0];
        assert_eq!(item.insurance_id, "7200725830765790764");
        assert_eq!(item.company_setting.payment_ratio, "8.00");
        assert_eq!(item.payment_months, vec![1, 2, 3]);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/social_archive_adjust_record/query"
        );
    }

    /// user_id_list / record_type 为必填，record_type 仅支持 increase/attrition
    #[tokio::test]
    async fn test_query_social_archive_adjust_record_validates_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = QueryRequest::new(config.clone(), vec![], "increase".to_string())
            .execute()
            .await;
        assert!(result.is_err());

        let result = QueryRequest::new(
            config,
            vec!["7134995965475620396".to_string()],
            "invalid".to_string(),
        )
        .execute()
        .await;
        assert!(result.is_err());
    }
}
