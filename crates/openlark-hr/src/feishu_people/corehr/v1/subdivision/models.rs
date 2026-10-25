//! CoreHR 省份/行政区（subdivision）相关模型
//!
//! 字段表对齐官方文档（批量查询/查询单条接口共用，items 元素与 subdivision 单体同形状）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 省份/行政区信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Subdivision {
    /// 省份/行政区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 省份/行政区名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    /// 所属国家/地区 ID，详细信息可通过【查询国家/地区信息】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 行政区类型，枚举值可通过文档【飞书人事枚举常量】行政区类型（subdivision_type）枚举定义部分获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision_type: Option<SubdivisionType>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 行政区类型枚举：`{enum_name, display: i18n[]}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SubdivisionType {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18n>>,
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
