//! CoreHR 工时制度（working_hours_type）相关模型
//!
//! 字段表对齐官方文档（创建/查询/更新/批量查询/删除接口共用）。

use serde::{Deserialize, Serialize};

/// 工时制度
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct WorkingHoursType {
    /// 工时制度 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（多语种）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nItem>>,
    /// 国家/地区 ID 列表（可通过查询国家/地区信息接口获取）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_id_list: Option<Vec<String>>,
    /// 职务默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_job: Option<bool>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18nItem {
    /// 语言编码，如 `zh-CN`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方自定义字段结构：`{field_name, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值（json 转义后的字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
