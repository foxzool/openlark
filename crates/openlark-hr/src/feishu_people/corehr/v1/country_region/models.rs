//! CoreHR 国家/地区（country_region）相关模型
//!
//! 字段表对齐官方文档（查询单条/批量查询接口共用，items 元素与 country_region 单体同形状）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 国家/地区信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct CountryRegion {
    /// 国家/地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 国家/地区名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    /// 国家地区三字码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_3_code: Option<String>,
    /// 国家地区二字码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_2_code: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
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
