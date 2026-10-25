//! CoreHR 城市/区域（subregion）相关模型
//!
//! 字段表对齐官方文档（批量查询/查询单条接口共用，items 元素与 subregion 单体同形状）。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 城市/区域信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Subregion {
    /// 城市/区域 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 城市/区域名称（多语种）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    /// 所属省份/行政区 ID，详细信息可通过【查询省份/行政区信息】接口查询获得
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision_id: Option<String>,
    /// 上级城市/区域 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superior_subregion_id: Option<String>,
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
