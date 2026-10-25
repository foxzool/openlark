//! 兼职（additional_job）共享模型
//!
//! 官方类型 `employees.additional_job_write_resp`（create/patch 响应的
//! `additional_job` 容器）与 `employees.additional_job[]`（batch 响应的
//! `items` 元素）字段表一致，共用同一结构体。

use serde::{Deserialize, Serialize};

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 兼职信息（官方 employees.additional_job_write_resp / employees.additional_job）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalJob {
    /// 兼职记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 人员类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 工作地点 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职务 ID（需字段权限 corehr:additional_job.job:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职级 ID（需字段权限 corehr:additional_job.job_level:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 序列 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 兼职开始日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 兼职结束日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 直属上级的雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<String>,
    /// 虚线上级的雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_manager_id: Option<String>,
    /// 排班类型（枚举对象，需字段权限 corehr:additional_job.work_shift:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_shift: Option<EnumValue>,
    /// 薪资类型（枚举对象，需字段权限 corehr:additional_job.compensation_type:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_type: Option<EnumValue>,
    /// 任职公司（需字段权限 corehr:additional_job.service_company:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_company: Option<String>,
    /// 周工作时长【0~168】
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_working_hours: Option<String>,
    /// 工作日历 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_calendar_id: Option<String>,
    /// 岗位 ID（需字段权限 corehr:additional_job.position:read/write）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 人员子类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_subtype_id: Option<String>,
}
