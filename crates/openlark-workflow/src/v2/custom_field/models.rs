//! 自定义字段 API v2 的数据模型

use serde::{Deserialize, Serialize};

/// 自定义字段类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CustomFieldType {
    /// 数字
    Number,
    /// 日期时间
    Datetime,
    /// 成员
    Member,
    /// 单选
    SingleSelect,
    /// 多选
    MultiSelect,
    /// 文本（默认）
    #[default]
    Text,
}

/// 数字类型字段设置（number_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NumberSetting {
    /// 数字展示格式：normal 常规数字 / percentage 百分比 / cny 人民币 / usd 美元 / custom 自定义符号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// format 为 custom 时的自定义符号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_symbol: Option<String>,
    /// 自定义符号显示位置：left / right（默认 right）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_symbol_position: Option<String>,
    /// 整数部分分隔符样式：none 无 / thousand 千分位（默认 none）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    /// 小数位数（0-6，默认 0）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_count: Option<i32>,
}

/// 人员类型字段设置（member_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MemberSetting {
    /// 是否支持多选（默认 false）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
}

/// 日期时间类型字段设置（datetime_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatetimeSetting {
    /// 日期时间格式（默认 yyyy-mm-dd）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// 单选/多选选项（option）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SelectOption {
    /// 选项名称（必填，最大 50 个字符）
    #[serde(default)]
    pub name: String,
    /// 选项颜色索引（0-54，不填随机分配）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    /// 选项是否隐藏
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

/// 单选/多选类型字段设置（select_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SelectSetting {
    /// 选项列表（0-100 个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption>>,
}

/// 文本类型字段设置（text_setting，目前无可设置项）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextSetting {}

/// 自定义字段配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldConfig {
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
    /// 字段选项（用于 Select 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

impl Default for CustomFieldConfig {
    fn default() -> Self {
        Self {
            field_type: CustomFieldType::Text,
            options: None,
        }
    }
}

/// 创建自定义字段请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCustomFieldBody {
    /// 资源类型（必填，目前仅支持 "tasklist"，默认 tasklist）
    pub resource_type: String,
    /// 资源 ID（必填，resource_type 为 "tasklist" 时填写清单 GUID）
    pub resource_id: String,
    /// 字段名称（必填，最大 50 个字符）
    pub name: String,
    /// 自定义字段类型（必填）：number / datetime / member / single_select / multi_select / text
    #[serde(rename = "type")]
    pub field_type: CustomFieldType,
    /// 数字类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_setting: Option<NumberSetting>,
    /// 人员类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_setting: Option<MemberSetting>,
    /// 日期时间类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_setting: Option<DatetimeSetting>,
    /// 单选类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_select_setting: Option<SelectSetting>,
    /// 多选类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_select_setting: Option<SelectSetting>,
    /// 文本类型字段设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_setting: Option<TextSetting>,
}

/// 更新自定义字段请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateCustomFieldBody {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CustomFieldConfig>,
}

/// 创建自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 更新自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCustomFieldResponse {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 更新时间
    pub updated_at: String,
}

/// 删除自定义字段响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCustomFieldResponse {
    /// 是否删除成功
    pub success: bool,
    /// 字段 GUID
    pub field_guid: String,
}

/// 自定义字段列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldItem {
    /// 字段 GUID
    pub field_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段配置
    pub config: CustomFieldConfig,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCustomFieldsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 列表项
    #[serde(default)]
    pub items: Vec<CustomFieldItem>,
}
