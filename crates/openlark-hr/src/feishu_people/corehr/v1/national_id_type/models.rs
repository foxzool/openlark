//! CoreHR 国家证件类型（national_id_type）相关模型
//!
//! 字段表对齐官方文档（创建/查询/更新/批量查询/删除接口共用）。

use serde::{Deserialize, Serialize};

/// 国家证件类型
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct NationalIdType {
    /// 证件类型 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 国家 / 地区 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 名称（多语种）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 校验规则（正则表达式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rule: Option<String>,
    /// 校验规则描述（多语种）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_rule_description: Option<Vec<I18n>>,
    /// 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 证件类型枚举
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<EnumValue>,
    /// 自定义字段（暂不支持）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18n {
    /// 语言编码，如 `zh-CN`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 结构：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18n>>,
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
