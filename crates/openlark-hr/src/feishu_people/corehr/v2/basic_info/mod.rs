/// bank 子模块。
pub mod bank;
/// bank_branch 子模块。
pub mod bank_branch;
/// city 子模块。
pub mod city;
/// country_region 子模块。
pub mod country_region;
/// country_region_subdivision 子模块。
pub mod country_region_subdivision;
/// currency 子模块。
pub mod currency;
/// district 子模块。
pub mod district;
/// language 子模块。
pub mod language;
/// nationality 子模块。
pub mod nationality;
/// time_zone 子模块。
pub mod time_zone;

use serde::{Deserialize, Serialize};

/// 官方 i18n 多语文本结构：`{lang, value}`（basic_info 域响应通用）。
///
/// 注意与 `crate::common::shared_models::I18nText`（`{zh_cn, en_us}` 形态）不同，
/// corehr basic_info 线上响应的多语字段是 lang/value 数组格式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}
