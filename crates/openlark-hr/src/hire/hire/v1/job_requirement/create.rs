//! 创建招聘需求
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job_requirement/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRequirementDto;

/// 创建招聘需求请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    request_body: CreateRequestBody,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: CreateRequestBody) -> Self {
        Self {
            request_body,
            config,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: CreateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobRequirementCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建招聘需求响应数据为空",
        )
        .await
    }
}

/// `CreateRequestBody`。
///
/// 除招聘需求编号等官方必填字段外，其余字段是否必填以飞书招聘
/// 「招聘需求字段管理」中的设置为准。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRequestBody {
    /// 招聘需求编号，需传入唯一的编号（必填）。
    pub short_code: String,
    /// 需求名称（必填）。
    pub name: String,
    /// 需求状态（必填）：1 待开始 / 2 进行中 / 3 已取消 / 4 已暂停 / 5 已完成。
    pub display_progress: i32,
    /// 需求人数（必填）。
    pub head_count: i32,
    /// 职位性质 ID（即将下线，建议改用 `employee_type_id`）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruitment_type_id: Option<String>,
    /// 人员类型 ID，类型需与 `employee_type_id_type` 查询参数保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 最高职级 ID，需与 `job_level_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level_id: Option<String>,
    /// 最低职级 ID，需与 `job_level_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_level_id: Option<String>,
    /// 职位序列 ID，需与 `job_family_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<String>,
    /// 需求类型：1 新增 / 2 替换。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    /// 需求部门 ID，需与 `department_id_type` 类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 需求负责人 ID 列表，需与 `user_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recruiter_id_list: Option<Vec<String>>,
    /// 需求用人经理 ID 列表，需与 `user_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jr_hiring_manager_id_list: Option<Vec<String>>,
    /// 直属上级 ID 列表，需与 `user_id_type` 类型保持一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_leader_id_list: Option<Vec<String>>,
    /// 开始日期，毫秒时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 预计完成日期，毫秒时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 招聘优先级：1 高 / 2 中 / 3 低。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 学历要求。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    /// 月薪范围-最高薪资，单位：K。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<String>,
    /// 月薪范围-最低薪资，单位：K。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<String>,
    /// 工作地址 ID，可通过获取地址列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// 需求描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 自定义字段，可通过获取招聘需求模板接口获取字段列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<JobRequirementCustomizedData>>,
    /// 招聘类型：1 社招 / 2 校招。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_type: Option<i32>,
    /// 职位类别 ID，可通过获取职位类别列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type_id: Option<String>,
    /// 关联的职位 ID 列表（最多 200 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id_list: Option<Vec<String>>,
    /// 职务 ID，可通过获取租户职务列表接口获取（仅限飞书人事租户使用）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_job_id: Option<String>,
    /// 岗位 ID，可通过查询岗位信息接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
}

impl CreateRequestBody {
    /// 创建新的请求实例（`short_code`/`name`/`display_progress`/`head_count` 为官方必填字段）。
    pub fn new(
        short_code: impl Into<String>,
        name: impl Into<String>,
        display_progress: i32,
        head_count: i32,
    ) -> Self {
        Self {
            short_code: short_code.into(),
            name: name.into(),
            display_progress,
            head_count,
            recruitment_type_id: None,
            employee_type_id: None,
            max_level_id: None,
            min_level_id: None,
            sequence_id: None,
            category: None,
            department_id: None,
            recruiter_id_list: None,
            jr_hiring_manager_id_list: None,
            direct_leader_id_list: None,
            start_time: None,
            deadline: None,
            priority: None,
            required_degree: None,
            max_salary: None,
            min_salary: None,
            address_id: None,
            description: None,
            customized_data_list: None,
            process_type: None,
            job_type_id: None,
            job_id_list: None,
            employment_job_id: None,
            position_id: None,
        }
    }

    /// 设置职位性质 ID。
    pub fn recruitment_type_id(mut self, value: impl Into<String>) -> Self {
        self.recruitment_type_id = Some(value.into());
        self
    }

    /// 设置人员类型 ID。
    pub fn employee_type_id(mut self, value: impl Into<String>) -> Self {
        self.employee_type_id = Some(value.into());
        self
    }

    /// 设置最高职级 ID。
    pub fn max_level_id(mut self, value: impl Into<String>) -> Self {
        self.max_level_id = Some(value.into());
        self
    }

    /// 设置最低职级 ID。
    pub fn min_level_id(mut self, value: impl Into<String>) -> Self {
        self.min_level_id = Some(value.into());
        self
    }

    /// 设置职位序列 ID。
    pub fn sequence_id(mut self, value: impl Into<String>) -> Self {
        self.sequence_id = Some(value.into());
        self
    }

    /// 设置需求类型。
    pub fn category(mut self, value: i32) -> Self {
        self.category = Some(value);
        self
    }

    /// 设置需求部门 ID。
    pub fn department_id(mut self, value: impl Into<String>) -> Self {
        self.department_id = Some(value.into());
        self
    }

    /// 设置需求负责人 ID 列表。
    pub fn recruiter_id_list(mut self, value: Vec<String>) -> Self {
        self.recruiter_id_list = Some(value);
        self
    }

    /// 设置需求用人经理 ID 列表。
    pub fn jr_hiring_manager_id_list(mut self, value: Vec<String>) -> Self {
        self.jr_hiring_manager_id_list = Some(value);
        self
    }

    /// 设置直属上级 ID 列表。
    pub fn direct_leader_id_list(mut self, value: Vec<String>) -> Self {
        self.direct_leader_id_list = Some(value);
        self
    }

    /// 设置开始日期（毫秒时间戳）。
    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    /// 设置预计完成日期（毫秒时间戳）。
    pub fn deadline(mut self, value: impl Into<String>) -> Self {
        self.deadline = Some(value.into());
        self
    }

    /// 设置招聘优先级。
    pub fn priority(mut self, value: i32) -> Self {
        self.priority = Some(value);
        self
    }

    /// 设置学历要求。
    pub fn required_degree(mut self, value: i32) -> Self {
        self.required_degree = Some(value);
        self
    }

    /// 设置月薪范围-最高薪资。
    pub fn max_salary(mut self, value: impl Into<String>) -> Self {
        self.max_salary = Some(value.into());
        self
    }

    /// 设置月薪范围-最低薪资。
    pub fn min_salary(mut self, value: impl Into<String>) -> Self {
        self.min_salary = Some(value.into());
        self
    }

    /// 设置工作地址 ID。
    pub fn address_id(mut self, value: impl Into<String>) -> Self {
        self.address_id = Some(value.into());
        self
    }

    /// 设置需求描述。
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// 设置自定义字段列表。
    pub fn customized_data_list(mut self, value: Vec<JobRequirementCustomizedData>) -> Self {
        self.customized_data_list = Some(value);
        self
    }

    /// 设置招聘类型。
    pub fn process_type(mut self, value: i32) -> Self {
        self.process_type = Some(value);
        self
    }

    /// 设置职位类别 ID。
    pub fn job_type_id(mut self, value: impl Into<String>) -> Self {
        self.job_type_id = Some(value.into());
        self
    }

    /// 设置关联的职位 ID 列表。
    pub fn job_id_list(mut self, value: Vec<String>) -> Self {
        self.job_id_list = Some(value);
        self
    }

    /// 设置职务 ID。
    pub fn employment_job_id(mut self, value: impl Into<String>) -> Self {
        self.employment_job_id = Some(value.into());
        self
    }

    /// 设置岗位 ID。
    pub fn position_id(mut self, value: impl Into<String>) -> Self {
        self.position_id = Some(value.into());
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.short_code, "short_code 不能为空");
        validate_required!(self.name, "name 不能为空");

        Ok(())
    }
}

/// 招聘需求自定义字段（`job_requirement_customized_data`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementCustomizedData {
    /// 自定义字段 ID，可通过获取招聘需求模板接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 自定义字段值，格式见官方文档「自定义字段数据格式说明」。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 创建招聘需求响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘需求信息。
    pub job_requirement: Option<JobRequirementDto>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/job_requirements，响应按官方 `data.job_requirement` 形状解析。
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/job_requirements"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "SUCCESS",
                "data": {
                    "job_requirement": {
                        "id": "6949805467799537964",
                        "short_code": "R01905",
                        "name": "HR部门春季招聘需求",
                        "display_progress": 1,
                        "head_count": 10,
                        "recruitment_type": {
                            "id": "101",
                            "name": {
                                "zh_cn": "社招-全职",
                                "en_us": "Social recruitment - Full time"
                            }
                        },
                        "employee_type": {
                            "id": "6807409776231254285",
                            "name": { "zh_cn": "专家", "en_us": "specialist" }
                        },
                        "category": 1,
                        "priority": 1,
                        "process_type": 1,
                        "job_type": {
                            "id": "6920523524664496628",
                            "name": { "zh_cn": "技术", "en_us": "Tech" },
                            "parent_id": "0"
                        },
                        "approval_status": 3
                    }
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

        let body = CreateRequestBody::new("xx1", "test", 1, 11)
            .priority(1)
            .description("11")
            .recruiter_id_list(vec!["ou_0c9b1b7b9b94146b9df142c349e3c4bf".to_string()])
            .customized_data_list(vec![JobRequirementCustomizedData {
                object_id: Some("1213213123123".to_string()),
                value: Some("简单文本".to_string()),
            }]);

        let resp = CreateRequest::new(config, body)
            .execute()
            .await
            .expect("请求应成功");
        let requirement = resp.job_requirement.expect("应解析出 job_requirement");
        assert_eq!(requirement.id.as_deref(), Some("6949805467799537964"));
        assert_eq!(requirement.short_code.as_deref(), Some("R01905"));
        assert_eq!(requirement.head_count, Some(10));
        assert_eq!(
            requirement
                .recruitment_type
                .as_ref()
                .and_then(|r| r.id.as_deref()),
            Some("101")
        );
        assert_eq!(
            requirement
                .employee_type
                .as_ref()
                .and_then(|e| e.name.as_ref().and_then(|n| n.zh_cn.as_deref())),
            Some("专家")
        );
        assert_eq!(requirement.approval_status, Some(3));
        assert_eq!(
            requirement
                .job_type
                .as_ref()
                .and_then(|t| t.parent_id.as_deref()),
            Some("0")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({
                "short_code": "xx1",
                "name": "test",
                "display_progress": 1,
                "head_count": 11,
                "priority": 1,
                "description": "11",
                "recruiter_id_list": ["ou_0c9b1b7b9b94146b9df142c349e3c4bf"],
                "customized_data_list": [
                    { "object_id": "1213213123123", "value": "简单文本" }
                ]
            })
        );
    }

    /// 必填字段缺失时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_create_rejects_blank_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CreateRequest::new(config, CreateRequestBody::new("  ", "test", 1, 11))
            .execute()
            .await;
        assert!(result.is_err());
    }
}
