//! 更新职位
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/combined_update>

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

use super::combined_create::{CombinedJobObjectValueMap, JobManager};
use crate::common::shared_models::{CodeNameObject, IdNameObject};

/// 更新职位请求
#[derive(Debug, Clone)]
pub struct CombinedUpdateRequest {
    job_id: Option<String>,
    request_body: CombinedUpdateRequestBody,
    /// 配置信息
    config: Config,
}

impl CombinedUpdateRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: CombinedUpdateRequestBody) -> Self {
        Self {
            job_id: None,
            request_body,
            config,
        }
    }

    /// 设置 `job_id`。
    pub fn job_id(mut self, job_id: impl Into<String>) -> Self {
        self.job_id = Some(job_id.into());
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: CombinedUpdateRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CombinedUpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CombinedUpdateResponse> {
        self.request_body.validate()?;
        let job_id = self.job_id.unwrap_or_default();
        validate_required!(job_id.trim(), "job_id 不能为空");

        let request = ApiRequest::<CombinedUpdateResponse>::post(format!(
            "/open-apis/hire/v1/jobs/{job_id}/combined_update"
        ));
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(request, &self.config, Some(option), "更新职位响应数据为空").await
    }
}

/// `CombinedUpdateRequestBody`。
///
/// 本接口为全量更新：未填写的字段原值会被清空。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombinedUpdateRequestBody {
    /// 职位 ID，可通过获取职位列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 工作经验要求：1 不限 / 2 应届 / 3 一年以内 / 4 一到三年 / 5 三到五年 ……
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience: Option<i32>,
    /// 到期日期（毫秒时间戳），已废弃，请使用 `expiry_timestamp`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<i64>,
    /// 自定义字段（注意：更新时会全量覆盖）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<CombinedJobObjectValueMap>>,
    /// 最低职级 ID，需与 `job_level_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_level_id: Option<String>,
    /// 最低薪资，单位：K。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_salary: Option<i32>,
    /// 职位名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 职位管理者（必填）。
    pub job_managers: JobManager,
    /// 招聘流程 ID，可通过获取招聘流程信息接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_process_id: Option<String>,
    /// 项目 ID，可通过获取项目列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    /// 职能分类 ID，可通过获取职能分类列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_function_id: Option<String>,
    /// 部门 ID，需与 `department_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 招聘数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_count: Option<i32>,
    /// 是否长期有效（必填）。
    pub is_never_expired: bool,
    /// 最高薪资，单位：K。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_salary: Option<i32>,
    /// 职位要求。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,
    /// 职位描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 职位亮点 ID 列表，见枚举常量介绍。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_list: Option<Vec<String>>,
    /// 职位类别 ID（必填），可通过获取职位类别列表接口获取。
    pub job_type_id: String,
    /// 最高职级 ID，需与 `job_level_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_level_id: Option<String>,
    /// 学历要求。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_degree: Option<i32>,
    /// 序列 ID，需与 `job_family_id_type` 查询参数类型一致。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_category_id: Option<String>,
    /// 工作地点 ID 列表，可通过获取地址列表接口获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id_list: Option<Vec<String>>,
    /// 职位属性：1 实体 / 2 虚拟。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_attribute: Option<i32>,
    /// 到期时间（毫秒时间戳）；`is_never_expired` 为 true 时该字段不生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_timestamp: Option<String>,
    /// 目标专业 ID 列表，"0" 表示专业不限。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_major_id_list: Option<Vec<String>>,
}

impl CombinedUpdateRequestBody {
    /// 创建新的请求实例（`job_managers`/`is_never_expired`/`job_type_id` 为官方必填字段）。
    pub fn new(
        job_managers: JobManager,
        is_never_expired: bool,
        job_type_id: impl Into<String>,
    ) -> Self {
        Self {
            id: None,
            experience: None,
            expiry_time: None,
            customized_data_list: None,
            min_level_id: None,
            min_salary: None,
            title: None,
            job_managers,
            job_process_id: None,
            subject_id: None,
            job_function_id: None,
            department_id: None,
            head_count: None,
            is_never_expired,
            max_salary: None,
            requirement: None,
            description: None,
            highlight_list: None,
            job_type_id: job_type_id.into(),
            max_level_id: None,
            required_degree: None,
            job_category_id: None,
            address_id_list: None,
            job_attribute: None,
            expiry_timestamp: None,
            target_major_id_list: None,
        }
    }

    /// 设置职位 ID。
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// 设置工作经验要求。
    pub fn experience(mut self, value: i32) -> Self {
        self.experience = Some(value);
        self
    }

    /// 设置到期日期（已废弃，请使用 `expiry_timestamp`）。
    pub fn expiry_time(mut self, value: i64) -> Self {
        self.expiry_time = Some(value);
        self
    }

    /// 设置自定义字段列表（更新时全量覆盖）。
    pub fn customized_data_list(mut self, value: Vec<CombinedJobObjectValueMap>) -> Self {
        self.customized_data_list = Some(value);
        self
    }

    /// 设置最低职级 ID。
    pub fn min_level_id(mut self, value: impl Into<String>) -> Self {
        self.min_level_id = Some(value.into());
        self
    }

    /// 设置最低薪资。
    pub fn min_salary(mut self, value: i32) -> Self {
        self.min_salary = Some(value);
        self
    }

    /// 设置职位名称。
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// 设置招聘流程 ID。
    pub fn job_process_id(mut self, value: impl Into<String>) -> Self {
        self.job_process_id = Some(value.into());
        self
    }

    /// 设置项目 ID。
    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    /// 设置职能分类 ID。
    pub fn job_function_id(mut self, value: impl Into<String>) -> Self {
        self.job_function_id = Some(value.into());
        self
    }

    /// 设置部门 ID。
    pub fn department_id(mut self, value: impl Into<String>) -> Self {
        self.department_id = Some(value.into());
        self
    }

    /// 设置招聘数量。
    pub fn head_count(mut self, value: i32) -> Self {
        self.head_count = Some(value);
        self
    }

    /// 设置最高薪资。
    pub fn max_salary(mut self, value: i32) -> Self {
        self.max_salary = Some(value);
        self
    }

    /// 设置职位要求。
    pub fn requirement(mut self, value: impl Into<String>) -> Self {
        self.requirement = Some(value.into());
        self
    }

    /// 设置职位描述。
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// 设置职位亮点 ID 列表。
    pub fn highlight_list(mut self, value: Vec<String>) -> Self {
        self.highlight_list = Some(value);
        self
    }

    /// 设置最高职级 ID。
    pub fn max_level_id(mut self, value: impl Into<String>) -> Self {
        self.max_level_id = Some(value.into());
        self
    }

    /// 设置学历要求。
    pub fn required_degree(mut self, value: i32) -> Self {
        self.required_degree = Some(value);
        self
    }

    /// 设置序列 ID。
    pub fn job_category_id(mut self, value: impl Into<String>) -> Self {
        self.job_category_id = Some(value.into());
        self
    }

    /// 设置工作地点 ID 列表。
    pub fn address_id_list(mut self, value: Vec<String>) -> Self {
        self.address_id_list = Some(value);
        self
    }

    /// 设置职位属性。
    pub fn job_attribute(mut self, value: i32) -> Self {
        self.job_attribute = Some(value);
        self
    }

    /// 设置到期时间（毫秒时间戳）。
    pub fn expiry_timestamp(mut self, value: impl Into<String>) -> Self {
        self.expiry_timestamp = Some(value.into());
        self
    }

    /// 设置目标专业 ID 列表。
    pub fn target_major_id_list(mut self, value: Vec<String>) -> Self {
        self.target_major_id_list = Some(value);
        self
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.job_type_id, "job_type_id 不能为空");
        validate_required!(
            self.job_managers.recruiter_id,
            "job_managers.recruiter_id 不能为空"
        );
        validate_required!(
            self.job_managers.hiring_manager_id_list,
            "job_managers.hiring_manager_id_list 不能为空"
        );

        Ok(())
    }
}

/// `{id, zh_name, en_name, active_status}` 形式的职位引用对象
/// （官方 `job_recruitment_type` / `job_department` / `job_level` / `job_category` /
/// `job_type` / `job_highlight` / `target_major_info` 共享形状）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobNamedRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 工作城市（官方 `job_city`，`{city_code, zh_name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCityRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市码，详情请查看查询地点列表接口。
    pub city_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段选项（官方 `job_customized_option`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项 ID。
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 选项名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段时间段（官方 `job_customized_time_range`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 开始时间，毫秒时间戳。
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 结束时间，毫秒时间戳。
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段值（官方 `job_customized_value`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为单行文本、多行文本、模块、默认字段时，从此字段取值。
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为单选时，从此字段取值。
    pub option: Option<JobCustomizedOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为多选时，从此字段取值。
    pub option_list: Option<Vec<JobCustomizedOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为时间段时，从此字段取值。
    pub time_range: Option<JobCustomizedTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为日期选择、月份选择、年份选择时，从此字段取值，毫秒级时间戳。
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 当字段类型为数字时，从此字段取值。
    pub number: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位自定义字段（官方 `job_customized_data`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCustomizedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段 ID。
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 字段类型：1 单行文本 / 2 多行文本 / 3 单选 / 4 多选 / 5 日期 / 6 月份选择 /
    /// 7 年份选择 / 8 时间段 / 9 数字 / 10 默认字段 / 11 模块。
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段值。
    pub value: Option<JobCustomizedValue>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位（官方 `job`，`combined_update` 响应内嵌容器）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位名称。
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位描述。
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位编号。
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位要求。
    pub requirement: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型。
    pub recruitment_type: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门。
    pub department: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市。
    pub city: Option<JobCityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_job_level: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_job_level: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点。
    pub highlight_list: Option<Vec<JobNamedRef>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列。
    pub job_category: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<JobNamedRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人 ID，若为空则为系统或其他对接系统创建。
    pub create_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间（已废弃，请使用 `create_timestamp`）。
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间（已废弃，请使用 `update_timestamp`）。
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程类型：1 社招流程 / 2 校招流程。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程 ID。
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程中文名称。
    pub process_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘流程英文名称。
    pub process_en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 自定义字段列表。
    pub customized_data_list: Option<Vec<JobCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职能分类。
    pub job_function: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 项目。
    pub subject: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘数量。
    pub head_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作经验要求（1-8 枚举，详见官方文档）。
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 到期日期（已废弃，请使用 `expiry_timestamp`）。
    pub expiry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低薪资，单位：K。
    pub min_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高薪资，单位：K。
    pub max_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求（1-8/20 枚举，详见官方文档）。
    pub required_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市列表。
    pub city_list: Option<Vec<CodeNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位属性：1 实体职位 / 2 虚拟职位。
    pub job_attribute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳。
    pub create_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 更新时间，毫秒时间戳。
    pub update_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 到期时间，毫秒时间戳；`is_never_expired` 为 true 时该字段不生效。
    pub expiry_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业列表。
    pub target_major_list: Option<Vec<JobNamedRef>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 默认职位广告（官方 `combined_job_result_default_job_post`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DefaultJobPostRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 默认职位广告的 ID，可通过职位发布至官网接口发布至官网。
    pub id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位负责人（官方 `job.manager`，响应视图：字段全部可选）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 招聘负责人 ID，与入参 `user_id_type` 类型一致。
    pub recruiter_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 用人经理 ID 列表，与入参 `user_id_type` 类型一致。
    pub hiring_manager_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 协助人 ID 列表，与入参 `user_id_type` 类型一致。
    pub assistant_id_list: Option<Vec<String>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 更新职位响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CombinedUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `result` 字段。
    pub result: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `success` 字段。
    pub success: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位广告。
    pub default_job_post: Option<DefaultJobPostRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位。
    pub job: Option<CombinedJob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位负责人。
    pub job_manager: Option<JobManagerView>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for CombinedUpdateResponse {
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

    /// 端到端：POST /open-apis/hire/v1/jobs/test001/combined_update
    #[tokio::test]
    async fn test_combined_update_returns_data_on_success() {
        // 响应示例的 `job` 对象键较多，单独构造以避免 `json!` 宏超过默认递归深度。
        let customized_value = json!({
            "content": "这是一个单行文本",
            "option": {
                "key": "7281257045172308287",
                "name": { "zh_cn": "选项 A", "en_us": "Option A" }
            },
            "option_list": [
                { "key": "7281257045172308287", "name": { "zh_cn": "选项 A", "en_us": "Option A" } }
            ],
            "time_range": { "start_time": "1622484739955", "end_time": "1622484739955" },
            "time": "1625456721000",
            "number": "111"
        });
        let job = json!({
            "id": "7281257045172308287",
            "title": "测试职位",
            "description": "这是一个测试职位",
            "code": "Z88",
            "requirement": "要求高学历人才",
            "recruitment_type": {
                "id": "7281257045172308287",
                "zh_name": "全职",
                "en_name": "FullTime",
                "active_status": 1
            },
            "department": {
                "id": "7281257045172308287",
                "zh_name": "字节跳动",
                "en_name": "Bytedance"
            },
            "city": { "city_code": "CT_20", "zh_name": "成都", "en_name": "Chengdu" },
            "min_job_level": {
                "id": "7281257045172308287",
                "zh_name": "级别-2",
                "en_name": "level-2",
                "active_status": 1
            },
            "max_job_level": {
                "id": "7281257045172308287",
                "zh_name": "级别-2",
                "en_name": "level-2",
                "active_status": 1
            },
            "highlight_list": [
                { "id": "7281257045172308287", "zh_name": "团队氛围好", "en_name": "Positive team atmosphere" }
            ],
            "job_category": {
                "id": "7281257045172308287",
                "zh_name": "序列-A",
                "en_name": "category-A",
                "active_status": 1
            },
            "job_type": { "id": "6890840777044265230", "zh_name": "金融", "en_name": "Finance" },
            "create_user_id": "7281257045172308287",
            "create_time": 1617170925462i64,
            "update_time": 1617170925462i64,
            "process_type": 1,
            "process_id": "1",
            "process_name": "流程中文名",
            "process_en_name": "流程英文名",
            "customized_data_list": [
                {
                    "object_id": "7281257045172308287",
                    "name": { "zh_cn": "字段 1", "en_us": "field 1" },
                    "object_type": 1,
                    "value": customized_value
                }
            ],
            "job_function": { "id": "7281257045172308287", "name": { "zh_cn": "测试职能", "en_us": "test job function" } },
            "subject": { "id": "7281257045172308287", "name": { "zh_cn": "测试项目", "en_us": "test subject" } },
            "head_count": 100,
            "experience": 1,
            "expiry_time": 1622484739955i64,
            "min_salary": 10,
            "max_salary": 20,
            "required_degree": 1,
            "city_list": [
                { "code": "CT_223", "name": { "zh_cn": "成都", "en_us": "Chengdu" } }
            ],
            "job_attribute": 1,
            "create_timestamp": "1617170925462",
            "update_timestamp": "1617170925462",
            "expiry_timestamp": "1622484739955",
            "target_major_list": [
                { "id": "MDMJ00000067", "zh_name": "考古", "en_name": "archeology" }
            ]
        });
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/jobs/test001/combined_update"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "default_job_post": { "id": "6960663240925956568" },
                    "job": job,
                    "job_manager": {
                        "id": "1618209327096",
                        "recruiter_id": "ou_efk39117c300506837def50545420c6a",
                        "hiring_manager_id_list": ["on_94a1ee5551019f18cd73d9f111898cf2"],
                        "assistant_id_list": ["on_94a1ee5551019f18cd73d9f111898cf2"]
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

        let body = CombinedUpdateRequestBody::new(
            JobManager {
                id: None,
                recruiter_id: "ou_efk39117c300506837def50545420c6a".to_string(),
                hiring_manager_id_list: vec!["ou_808dad5b8fb21416b4ff242a5c616016".to_string()],
                assistant_id_list: Some(vec!["ou_808dad5b8fb21416b4ff242a5c616017".to_string()]),
            },
            false,
            "6960663240925956551",
        )
        .title("后端研发")
        .head_count(100)
        .expiry_timestamp("1622484739955");

        let resp = CombinedUpdateRequest::new(config, body)
            .job_id("test001")
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(
            resp.default_job_post.as_ref().and_then(|v| v.id.as_deref()),
            Some("6960663240925956568")
        );
        let job = resp.job.as_ref().expect("job 应存在");
        assert_eq!(job.id.as_deref(), Some("7281257045172308287"));
        assert_eq!(job.title.as_deref(), Some("测试职位"));
        assert_eq!(
            job.recruitment_type
                .as_ref()
                .and_then(|v| v.zh_name.as_deref()),
            Some("全职")
        );
        assert_eq!(
            job.recruitment_type.as_ref().unwrap().active_status,
            Some(1)
        );
        assert_eq!(
            job.department.as_ref().and_then(|v| v.en_name.as_deref()),
            Some("Bytedance")
        );
        assert_eq!(
            job.city.as_ref().unwrap().city_code.as_deref(),
            Some("CT_20")
        );
        assert_eq!(job.head_count, Some(100));
        assert_eq!(job.experience, Some(1));
        assert_eq!(job.customized_data_list.as_ref().unwrap().len(), 1);
        assert_eq!(
            job.city_list
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .code
                .as_deref(),
            Some("CT_223")
        );
        assert_eq!(
            job.target_major_list
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .zh_name
                .as_deref(),
            Some("考古")
        );
        let job_manager = resp.job_manager.as_ref().expect("job_manager 应存在");
        assert_eq!(
            job_manager.recruiter_id.as_deref(),
            Some("ou_efk39117c300506837def50545420c6a")
        );
        assert_eq!(
            job_manager.hiring_manager_id_list.as_ref().unwrap().len(),
            1
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({
                "title": "后端研发",
                "job_managers": {
                    "recruiter_id": "ou_efk39117c300506837def50545420c6a",
                    "hiring_manager_id_list": ["ou_808dad5b8fb21416b4ff242a5c616016"],
                    "assistant_id_list": ["ou_808dad5b8fb21416b4ff242a5c616017"]
                },
                "head_count": 100,
                "is_never_expired": false,
                "job_type_id": "6960663240925956551",
                "expiry_timestamp": "1622484739955"
            })
        );
    }

    /// 必填字段缺失时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_combined_update_rejects_blank_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = CombinedUpdateRequest::new(
            config,
            CombinedUpdateRequestBody::new(
                JobManager {
                    recruiter_id: "  ".to_string(),
                    hiring_manager_id_list: Vec::new(),
                    ..JobManager::default()
                },
                false,
                "6960663240925956551",
            ),
        )
        .job_id("test001")
        .execute()
        .await;
        assert!(result.is_err());
    }
}
