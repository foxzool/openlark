pub mod create;
pub mod patch;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 官方 i18n 多语文本结构：`{lang, value}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：`{enum_name, display: i18n[]}`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 个人信息（官方 `person_info` 结构）。
///
/// 官方响应为字段级权限门控的超大对象（gender/national_id/education 等数十组，
/// 未授权不返回）；此处仅类型化常用叶子，其余字段经 `extra` 原样透传。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonInfo {
    /// 个人信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 个人电话（需 corehr:person.phone:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// 法定姓名（需 corehr:person.legal_name:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// 常用名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_name: Option<String>,
    /// 常用本地全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_local_full_name: Option<String>,
    /// 常用英文全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_english_full_name: Option<String>,
    /// 姓名列表（需 corehr:person.legal_name:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_list: Option<Vec<PersonName>>,
    /// 其余权限门控字段原样透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 姓名（官方 `person_name` 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonName {
    /// 姓 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary: Option<String>,
    /// 名 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name: Option<String>,
    /// 国家/地区 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    /// 姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<EnumValue>,
    /// 名 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_first_name_2: Option<String>,
    /// 姓 - 第二本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_primary_2: Option<String>,
    /// 补充姓名类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name_type: Option<EnumValue>,
    /// 名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// 全名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// 姓氏称谓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hereditary: Option<String>,
    /// 自定义姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// 本地文字的自定义姓名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_local_name: Option<String>,
    /// 中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// 姓
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_primary: Option<String>,
    /// 第二姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
    /// 尊称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social: Option<EnumValue>,
    /// 婚后姓氏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tertiary: Option<String>,
    /// 头衔
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<EnumValue>,
    /// 本地中间名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_middle_name: Option<String>,
    /// 第二姓氏 - 本地文字
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_secondary: Option<String>,
    /// 展示姓名（本地和西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_and_western_script: Option<String>,
    /// 展示姓名（本地文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_local_script: Option<String>,
    /// 展示姓名（西方文字）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name_western_script: Option<String>,
}
