//! 更新招聘需求
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job_requirement/update>

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

use super::create::JobRequirementCustomizedData;

/// 更新招聘需求请求体（官方必填 `name`/`display_progress`/`head_count`）
///
/// 除文档描述的必填字段外，其他字段是否必填以飞书招聘
/// 「招聘需求字段管理」中的设置为准。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRequestBody {
    /// 需求名称（必填）。
    pub name: String,
    /// 需求状态（必填）：1 待启动 / 2 进行中 / 3 已取消 / 4 已暂停 / 5 已完成 / 6 已超期。
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
    /// 学历要求：1 小学及以上 … 8 博士及以上 / 20 不限。
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
    /// 自定义字段，更新时会全量覆盖。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<JobRequirementCustomizedData>>,
    /// 招聘类型：1 社招 / 2 校招。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_type: Option<i32>,
    /// 职位类别 ID，可通过获取职位类别列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type_id: Option<String>,
    /// 关联的职位 ID 列表（最多 200 个，与 update_option.need_update_related_job 配合使用）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id_list: Option<Vec<String>>,
    /// 职务 ID（仅限飞书人事租户使用）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_job_id: Option<String>,
    /// 岗位 ID（仅限飞书人事租户使用）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 招聘需求修改确认控制。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_option: Option<JobRequirementUpdateOption>,
}

impl UpdateRequestBody {
    /// 创建新的请求实例（`name`/`display_progress`/`head_count` 为官方必填字段）。
    pub fn new(name: impl Into<String>, display_progress: i32, head_count: i32) -> Self {
        Self {
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
            update_option: None,
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

    /// 设置自定义字段列表（更新时全量覆盖）。
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

    /// 设置招聘需求修改确认控制。
    pub fn update_option(mut self, value: JobRequirementUpdateOption) -> Self {
        self.update_option = Some(value);
        self
    }
}

/// 招聘需求修改确认控制（官方 `job_requirement_update_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRequirementUpdateOption {
    /// 是否需要修改关联的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_update_related_job: Option<bool>,
}

/// 更新招聘需求请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 配置信息
    config: Config,
    job_requirement_id: String,
    request_body: UpdateRequestBody,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            job_requirement_id: String::new(),
            request_body: UpdateRequestBody::new(String::new(), 1, 0),
        }
    }

    /// 设置 `job_requirement_id`。
    pub fn job_requirement_id(mut self, job_requirement_id: impl Into<String>) -> Self {
        self.job_requirement_id = job_requirement_id.into();
        self
    }

    /// 设置请求体（`name`/`display_progress`/`head_count` 为官方必填字段）。
    pub fn request_body(mut self, request_body: UpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        validate_required!(
            self.job_requirement_id.trim(),
            "job_requirement_id 不能为空"
        );
        validate_required!(self.request_body.name.trim(), "name 不能为空");

        let request = ApiRequest::<UpdateResponse>::put(format!(
            "/open-apis/hire/v1/job_requirements/{}",
            self.job_requirement_id
        ))
        .body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新招聘需求响应数据为空",
        )
        .await
    }
}

/// 更新招聘需求响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_requirement_id` 字段。
    pub job_requirement_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for UpdateResponse {
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

    /// 端到端：PUT /open-apis/hire/v1/job_requirements/test001
    #[tokio::test]
    async fn test_update_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/hire/v1/job_requirements/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateRequest::new(config)
            .job_requirement_id("test001".to_string())
            .request_body(UpdateRequestBody::new("HR部门春季招聘需求", 1, 11))
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：name/display_progress/head_count 必填
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(
            sent,
            json!({ "name": "HR部门春季招聘需求", "display_progress": 1, "head_count": 11 })
        );
    }

    /// 缺必填 body 字段时快速失败，不发起请求。
    #[tokio::test]
    async fn test_update_rejects_blank_name() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = UpdateRequest::new(config)
            .job_requirement_id("test001".to_string())
            .request_body(UpdateRequestBody::new("  ", 1, 11))
            .execute()
            .await
            .expect_err("缺 name 应被拒绝");
        assert!(err.to_string().contains("name"), "实际: {err}");
    }
}
