/// create 子模块。
pub mod create;
/// delete 子模块。
pub mod delete;
/// patch 子模块。
pub mod patch;
/// query_multi_timeline 模块。
pub mod query_multi_timeline;
/// query_recent_change 子模块。
pub mod query_recent_change;
/// search 子模块。
pub mod search;
/// tree 模块。
pub mod tree;
/// version 子模块。
pub mod version;

use serde::{Deserialize, Serialize};

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言（如 zh-CN / en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 成本中心版本信息（官方 `cost_center.version` 结构）。
///
/// 用于 [`search`] 的 `data.items[]` 元素、[`version::create`] /
/// [`version::patch`] 的 `data.version` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostCenterVersion {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 成本中心版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 成本中心名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级成本中心 ID（一级成本中心不返回该字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    /// 成本中心负责人 ID 列表（受 contact:user.employee_id:readonly 字段权限门控）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    /// 成本中心描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 版本生效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 版本失效日期（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 当前实体是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// 成本中心信息（官方 `cost_center` 结构）。
///
/// 用于 [`create`] 的 `data.cost_center`（含 `cost_center_version_id`）与
/// [`patch`]（启用/停用）的 `data.cost_center` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 成本中心版本 ID（创建成本中心响应返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_version_id: Option<String>,
    /// 成本中心名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    /// 成本中心负责人 ID 列表（受 contact:user.employee_id:readonly 字段权限门控）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    /// 成本中心描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 成本中心启用状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
