//! 员工任职信息（job_data）v2 共享响应模型
//!
//! 供 `employees/job_data/batch_get.rs` 与 `employees/job_data/query.rs` 复用。
//! 字段形状与 v1 `corehr/v1/job_data/models.rs` 的 `JobData` 完全不同，勿混用。

use serde::{Deserialize, Serialize};

/// 员工任职信息（官方 `employee_job_data` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeJobData {
    /// 员工雇佣 ID，类型与 user_id_type 一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 任职记录列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_datas: Option<Vec<JobData>>,
}

/// 任职记录（官方 `job_data` 结构，v2 形状）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobData {
    /// 任职信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    /// 任职记录版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 人员类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 工作地点 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    /// 部门 ID，与 department_id_type 类型一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 岗位 ID（需 corehr:employment.position:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 职务 ID（需 corehr:employment.job:read 等权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职级 ID（需 corehr:employment.job_level:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职等 ID（需 corehr:employment.job_grade:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade_id: Option<String>,
    /// 序列 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 试用期开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（实际结束日期）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 是否为主任职：true 主职，false 兼职
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_job_data: Option<bool>,
    /// 雇佣 ID，与 user_id_type 类型一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 任职记录版本的生效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 任职记录版本的失效时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 业务类型（原：任职原因）（需 corehr:job_data.assignment_start_reason:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignment_start_reason: Option<EnumValue>,
    /// 预计试用期结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_expected_end_date: Option<String>,
    /// 试用期结果
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probation_outcome: Option<EnumValue>,
    /// 直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager: Option<BasicJobData>,
    /// 虚线上级列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_managers: Option<Vec<BasicJobData>>,
    /// 第二实线主管
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_direct_manager: Option<BasicJobData>,
    /// 成本中心分摊信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_rates: Option<Vec<JobDataCostCenter>>,
    /// 排班类型（需 corehr:job_data.work_shift:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<EnumValue>,
    /// 薪资类型（需 corehr:job_data.compensation_type:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<EnumValue>,
    /// 任职公司（需 corehr:job_data.service_company:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    /// 创建时间（仅 batch_get 返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 周工作时长 V2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours_v2: Option<f64>,
    /// 周工作时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<i64>,
    /// 人员子类型 ID（功能灰度中）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
    /// 任职原因（需 corehr:job_data.job_data_reason:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_reason: Option<EnumValue>,
    /// 通道 ID（需 corehr:employment.pathway:read / write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
}

/// 基础任职信息（官方 `basic_job_data` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasicJobData {
    /// 任职信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    /// 雇佣 ID，与 user_id_type 类型一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
}

/// 成本中心分摊信息（官方 `job_data_cost_center` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobDataCostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    /// 分摊比例
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_rate: Option<f64>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}
