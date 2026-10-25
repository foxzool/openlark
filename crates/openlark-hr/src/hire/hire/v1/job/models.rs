//! hire v1 `job` 资源共享响应模型（官方 `job` 对象，`{id, zh_name, en_name, ...}` 嵌套形态）。
//!
//! 被 `job/list.rs`（获取职位列表 items 元素）与 `job/combined_create.rs`
//! （新建职位响应 `data.job`）共用；两者官方文档的 `job` 子树同构。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::shared_models::{CodeNameObject, IdNameObject};
use crate::hire::hire::common_models::JobCustomizedData;

/// 雇佣类型（官方 `job_recruitment_type`：`{id, zh_name, en_name, active_status}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobRecruitmentTypeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位部门（官方 `job_department`：`{id, zh_name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDepartmentInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门 ID，与入参 `department_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 工作城市（官方 `job_city`：`{city_code, zh_name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市码。
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

/// 职级（官方 `job_level`：`{id, zh_name, en_name, active_status}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobLevelInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级 ID，与入参 `job_level_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职级启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位亮点（官方 `job_highlight`：`{id, zh_name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobHighlightInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位序列（官方 `job_category`：`{id, zh_name, en_name, active_status}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobCategoryInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列 ID，与入参 `job_family_id_type` 类型一致。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位类别（官方 `job_type`：`{id, zh_name, en_name, active_status}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobTypeDetailInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别英文名称。
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 目标专业（官方 `target_major_info`：`{id, zh_name, en_name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TargetMajorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业 ID，「0」为不限专业。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业中文名称。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业英文名称。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位负责人（官方 `job.manager`，响应侧全可选形态）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobManagerInfo {
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

/// 信息登记表（官方 `registration_schema_info`：`{schema_id, name}`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RegistrationSchemaInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 登记表 ID。
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 登记表名称。
    pub name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 职位详情（官方 `job` 对象，获取职位列表 items 元素 / 新建职位响应 `data.job`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobDetail {
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
    /// 职位状态：1 启用 / 2 关闭。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 雇佣类型。
    pub recruitment_type: Option<JobRecruitmentTypeInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 部门。
    pub department: Option<JobDepartmentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 工作城市。
    pub city: Option<JobCityInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低职级。
    pub min_job_level: Option<JobLevelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高职级。
    pub max_job_level: Option<JobLevelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位亮点列表。
    pub highlight_list: Option<Vec<JobHighlightInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位序列。
    pub job_category: Option<JobCategoryInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位类别。
    pub job_type: Option<JobTypeDetailInfo>,
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
    /// 工作经验要求。
    pub experience: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 到期日期（已废弃，请使用 `expiry_timestamp`）。
    pub expiry_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最低月薪，单位：K。
    pub min_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最高月薪，单位：K。
    pub max_salary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 学历要求。
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
    /// 到期时间，毫秒时间戳。
    pub expiry_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 目标专业列表。
    pub target_major_list: Option<Vec<TargetMajorInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位负责人。
    pub job_manager: Option<JobManagerInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试登记表信息。
    pub interview_registration_schema_info: Option<RegistrationSchemaInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 入职登记表信息。
    pub onboard_registration_schema_info: Option<RegistrationSchemaInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网申请表信息。
    pub portal_website_apply_form_schema_info: Option<RegistrationSchemaInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}
