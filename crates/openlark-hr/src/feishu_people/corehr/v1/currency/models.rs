//! CoreHR 货币（currency）相关模型
//!
//! 字段表对齐官方文档（批量查询/查询单条接口共用，items 元素与 currency 单体同形状）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 货币信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    /// 货币 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 货币所属国家/地区 ID，详细信息可通过【查询国家/地区信息】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 货币名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<Vec<I18n>>,
    /// 数字代码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    /// 三位字母代码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
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
