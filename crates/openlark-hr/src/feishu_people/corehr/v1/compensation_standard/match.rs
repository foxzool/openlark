//! 获取员工薪资标准
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/compensation_standard/match>

use openlark_core::{
    SDKResult,
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取员工薪资标准请求
#[derive(Debug, Clone, Default)]
pub struct MatchRequest {
    config: Config,
    /// 薪资标准表关联对象 ID（必填，即薪资项目/统计指标 ID）
    reference_object_id: Option<String>,
    /// 薪资标准的关联对象（可选：cpst_item 薪资项目 / cpst_indicator 薪资统计指标）
    reference_object_api: Option<String>,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 部门 ID 类型（可选，默认 people_corehr_department_id）
    department_id_type: Option<String>,
    /// 雇员 ID（可选）
    employment_id: Option<String>,
    /// 部门 ID（可选）
    department_id: Option<String>,
    /// 工作地点 ID（可选）
    work_location_id: Option<String>,
    /// 公司 ID（可选）
    company_id: Option<String>,
    /// 职务序列 ID（可选）
    job_family_id: Option<String>,
    /// 职务 ID（可选）
    job_id: Option<String>,
    /// 职级 ID（可选）
    job_level_id: Option<String>,
    /// 人员类型 ID（可选）
    employee_type_id: Option<String>,
    /// 招聘类型（可选：experienced_professionals 社招 / recent_graduates 校招 / routine_intern 日常实习）
    recruitment_type: Option<String>,
    /// 定调薪原因 ID（可选）
    cpst_change_reason_id: Option<String>,
    /// 薪资方案 ID（可选）
    cpst_plan_id: Option<String>,
    /// 薪级薪等 ID（可选）
    cpst_salary_level_id: Option<String>,
    /// 生效时间（可选，毫秒级时间戳）
    effective_time: Option<String>,
}

impl MatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// 设置薪资标准表关联对象 ID（必填，可通过【批量查询薪资项】/【批量查询薪资统计指标】获取）。
    pub fn reference_object_id(mut self, reference_object_id: String) -> Self {
        self.reference_object_id = Some(reference_object_id);
        self
    }

    /// 设置薪资标准的关联对象（可选：cpst_item / cpst_indicator）。
    pub fn reference_object_api(mut self, reference_object_api: String) -> Self {
        self.reference_object_api = Some(reference_object_api);
        self
    }

    /// 设置用户 ID 类型（可选）。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置部门 ID 类型（可选）。
    pub fn department_id_type(mut self, department_id_type: String) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 设置雇员 ID（可选）。
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.employment_id = Some(employment_id);
        self
    }

    /// 设置部门 ID（可选）。
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置工作地点 ID（可选）。
    pub fn work_location_id(mut self, work_location_id: String) -> Self {
        self.work_location_id = Some(work_location_id);
        self
    }

    /// 设置公司 ID（可选）。
    pub fn company_id(mut self, company_id: String) -> Self {
        self.company_id = Some(company_id);
        self
    }

    /// 设置职务序列 ID（可选）。
    pub fn job_family_id(mut self, job_family_id: String) -> Self {
        self.job_family_id = Some(job_family_id);
        self
    }

    /// 设置职务 ID（可选）。
    pub fn job_id(mut self, job_id: String) -> Self {
        self.job_id = Some(job_id);
        self
    }

    /// 设置职级 ID（可选）。
    pub fn job_level_id(mut self, job_level_id: String) -> Self {
        self.job_level_id = Some(job_level_id);
        self
    }

    /// 设置人员类型 ID（可选）。
    pub fn employee_type_id(mut self, employee_type_id: String) -> Self {
        self.employee_type_id = Some(employee_type_id);
        self
    }

    /// 设置招聘类型（可选）。
    pub fn recruitment_type(mut self, recruitment_type: String) -> Self {
        self.recruitment_type = Some(recruitment_type);
        self
    }

    /// 设置定调薪原因 ID（可选）。
    pub fn cpst_change_reason_id(mut self, cpst_change_reason_id: String) -> Self {
        self.cpst_change_reason_id = Some(cpst_change_reason_id);
        self
    }

    /// 设置薪资方案 ID（可选）。
    pub fn cpst_plan_id(mut self, cpst_plan_id: String) -> Self {
        self.cpst_plan_id = Some(cpst_plan_id);
        self
    }

    /// 设置薪级薪等 ID（可选）。
    pub fn cpst_salary_level_id(mut self, cpst_salary_level_id: String) -> Self {
        self.cpst_salary_level_id = Some(cpst_salary_level_id);
        self
    }

    /// 设置生效时间（可选，毫秒级时间戳）。
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = Some(effective_time);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let reference_object_id = self.reference_object_id.unwrap_or_default();
        openlark_core::validate_required!(reference_object_id, "reference_object_id 不能为空");

        let api_endpoint = FeishuPeopleApiV1::CompensationStandardMatch;
        let request = ApiRequest::<MatchResponse>::get(api_endpoint.to_url())
            .query("reference_object_id", reference_object_id)
            .query_opt("reference_object_api", self.reference_object_api)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type)
            .query_opt("employment_id", self.employment_id)
            .query_opt("department_id", self.department_id)
            .query_opt("work_location_id", self.work_location_id)
            .query_opt("company_id", self.company_id)
            .query_opt("job_family_id", self.job_family_id)
            .query_opt("job_id", self.job_id)
            .query_opt("job_level_id", self.job_level_id)
            .query_opt("employee_type_id", self.employee_type_id)
            .query_opt("recruitment_type", self.recruitment_type)
            .query_opt("cpst_change_reason_id", self.cpst_change_reason_id)
            .query_opt("cpst_plan_id", self.cpst_plan_id)
            .query_opt("cpst_salary_level_id", self.cpst_salary_level_id)
            .query_opt("effective_time", self.effective_time);

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取员工薪资标准响应数据为空",
        )
        .await
    }
}

/// 获取员工薪资标准响应
///
/// 对应官方响应体 `data` 节点（`cpst_match_item`，`ResponseFormat::Data`
/// 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchResponse {
    /// 薪资标准表 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_id: Option<String>,
    /// 薪资等级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<CpstGrade>,
    /// 生效时间（毫秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 薪资等级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstGrade {
    /// 薪资等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_id: Option<String>,
    /// 薪资等级时间轴 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_tid: Option<String>,
    /// 带宽上下限和标准值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_standard_value: Option<CpstGradeStandardValue>,
    /// 币种
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<CpstCurrency>,
    /// 薪资标准描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::common::shared_models::I18nText>,
}

/// 带宽上下限和标准值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstGradeStandardValue {
    /// 薪资标准的关联对象，项目或者指标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_object: Option<CpstReferenceObject>,
    /// 薪资标准类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_type: Option<CpstStandardType>,
    /// 上下限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band_width: Option<CpstBandWidth>,
    /// 标准值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_value: Option<String>,
}

/// 薪资标准的关联对象（项目或指标）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstReferenceObject {
    /// cpst_item（项目）/ cpst_indicator（指标）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 值列表，例如薪资项和薪资统计指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 薪资标准类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstStandardType {
    /// standard_value（标准值）/ bandwidth_and_standard_value（上下限和标准值）/ bandwidth_upper_and_lower_limit（上下限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
}

/// 上下限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstBandWidth {
    /// 上限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<String>,
    /// 下限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<String>,
}

/// 币种
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstCurrency {
    /// 币种 ID，可通过【查询货币信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 币种 code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 币种名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::common::shared_models::I18nText>,
}

impl ApiResponseTrait for MatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v1/compensation_standards/match
    #[tokio::test]
    async fn test_match_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/compensation_standards/match"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "effective_time": "1660924800000",
                    "grade": {
                        "currency": {
                            "code": "CNY",
                            "currency_id": "6863329932261459464",
                            "name": {"en_us": "Renminbi (Chinese) yuan", "zh_cn": "人民币元"}
                        },
                        "grade_id": "7174758360888215084",
                        "grade_standard_value": {
                            "band_width": {"lower_limit": "5000", "upper_limit": "10000"},
                            "reference_object": {"api_name": "cpst_item", "id": "7156853394442044972"},
                            "standard_type": {"api_name": "bandwidth_and_standard_value"},
                            "standard_value": "8888"
                        },
                        "grade_tid": "7174758360888247852"
                    },
                    "standard_id": "7174758593538295340"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = MatchRequest::new(config)
            .reference_object_id("7156853394442044972".to_string())
            .reference_object_api("cpst_item".to_string())
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.standard_id.as_deref(), Some("7174758593538295340"));
        assert_eq!(resp.effective_time.as_deref(), Some("1660924800000"));
        let grade = resp.grade.as_ref().expect("grade 应解析");
        assert_eq!(grade.grade_id.as_deref(), Some("7174758360888215084"));
        assert_eq!(
            grade
                .currency
                .as_ref()
                .and_then(|c| c.currency_id.as_deref()),
            Some("6863329932261459464")
        );
        let standard_value = grade
            .grade_standard_value
            .as_ref()
            .expect("grade_standard_value 应解析");
        assert_eq!(standard_value.standard_value.as_deref(), Some("8888"));
        assert_eq!(
            standard_value
                .band_width
                .as_ref()
                .and_then(|b| b.upper_limit.as_deref()),
            Some("10000")
        );
        assert_eq!(
            standard_value
                .reference_object
                .as_ref()
                .and_then(|r| r.api_name.as_deref()),
            Some("cpst_item")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // reference_object_id 为官方必填查询参数
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("reference_object_id=7156853394442044972")
                && query.contains("reference_object_api=cpst_item"),
            "查询串应含 reference_object_id 与 reference_object_api，实际 {query}"
        );
    }

    /// 必填校验：缺 reference_object_id 快速失败
    #[tokio::test]
    async fn test_match_missing_reference_object_id_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = MatchRequest::new(config)
            .execute()
            .await
            .expect_err("缺 reference_object_id 应被拒绝");
        assert!(
            err.to_string().contains("reference_object_id"),
            "实际: {err}"
        );
    }
}
