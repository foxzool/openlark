//! CoreHR 待入职信息（pre_hire）相关模型
//!
//! 字段表对齐官方文档（批量查询 / 更新接口共用；官方已推荐迁移 v2 pre_hire/query）。

use serde::{Deserialize, Serialize};

/// 待入职信息
///
/// `work_email_list` 与 `department_id` 仅批量查询（list）响应返回，更新（patch）响应中为 `None`。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct PreHire {
    /// 招聘投递 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ats_application_id: Option<String>,
    /// 待入职 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 入职日期，格式 `YYYY-MM-DD`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 雇佣类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<EnumValue>,
    /// 人员编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    /// 雇佣类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type_id: Option<String>,
    /// 个人信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 成本中心分摊信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_rate: Option<Vec<SupportCostCenterItem>>,
    /// 入职状态（preboarding：待入职；day_one：准备就绪；completed：已完成；withdrawn：已撤销；deleted：已删除）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<EnumValue>,
    /// 工作邮箱（仅批量查询响应返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_email_list: Option<Vec<WorkEmail>>,
    /// 部门 ID（仅批量查询响应返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

/// 成本中心分摊信息（support_cost_center_item）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SupportCostCenterItem {
    /// 支持的成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

/// 工作邮箱（email）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WorkEmail {
    /// 邮箱号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 是否为主要邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// 是否为公开邮箱
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    /// 邮箱用途
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_usage: Option<EnumValue>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18n {
    /// 语言编码，如 `zh-CN`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 结构：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18n>>,
}

/// 官方自定义字段结构：`{field_name, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值（json 转义后的字符串）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
