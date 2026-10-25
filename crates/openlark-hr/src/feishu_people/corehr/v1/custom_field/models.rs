//! corehr v1 自定义字段（custom_field）共享响应模型。
//!
//! 被 `query`（获取自定义字段列表）与 `get_by_param`（获取字段详情）共用，
//! 字段按官方响应体字段表建模（`CustomFieldItem` 顶层 + `common_schema_config`
//! 各字段类型配置子结构）。

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::shared_models::I18nText;

/// 自定义字段条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CustomFieldItem {
    /// 自定义字段 API name，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 描述（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    /// 是否必填
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// 是否唯一
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    /// 所属对象 apiname
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    /// 字段类型（1-11 枚举：1 文本 / 2 布尔 / 3 数字 / 4 枚举 / 5 查找 / 6 自动编码 / 7 日期时间 / 8 附件 / 9 图片 / 10 计算字段 / 11 反向查找）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 字段类型配置信息，仅字段类型为「文本」「布尔」「数字」「枚举」「日期时间」「附件」「图片」等时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_schema_config: Option<CommonSchemaConfig>,
    /// 创建时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 字段类型配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaConfig {
    /// 文本配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_field_setting: Option<TextFieldSetting>,
    /// 数字配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_field_setting: Option<NumberFieldSetting>,
    /// 枚举配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_field_setting: Option<EnumFieldSetting>,
    /// 查找字段配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_field_setting: Option<LookupFieldSetting>,
    /// 日期时间配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_field_setting: Option<DateTimeFieldSetting>,
    /// 附件配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_field_setting: Option<AttachmentFieldSetting>,
    /// 图片配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_field_setting: Option<ImageFieldSetting>,
    /// 计算字段配置信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated_field_setting: Option<CalculatedFieldSetting>,
}

/// 文本配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TextFieldSetting {
    /// 是否多语言
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multilingual: Option<bool>,
    /// 是否多行
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiline: Option<bool>,
    /// 最大长度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    /// 是否是“超链接”类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_url_type: Option<bool>,
}

/// 数字配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NumberFieldSetting {
    /// 数字类型（1 百分比 / 2 整数 / 3 数值 / 4 金额）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_field_type: Option<i32>,
    /// 小数点后的位数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    /// 四舍五入规则（0 四舍五入 / 1 向上舍入 / 2 向下舍入）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round_type: Option<i32>,
    /// 整数+小数总位数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_total_places: Option<i32>,
}

/// 枚举配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EnumFieldSetting {
    /// 枚举选项信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_field_option_list: Option<Vec<CommonSchemaOption>>,
    /// 是否为多选
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
}

/// 枚举选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CommonSchemaOption {
    /// 枚举常量集 API name，即一组选项集合的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 选项名称（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 选项描述（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

/// 查找字段配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LookupFieldSetting {
    /// 查找字段所引用对象的 API name（“人员（单选/多选）”为 `employment`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookup_obj_api_name: Option<String>,
    /// 是否为多值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
}

/// 日期时间配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DateTimeFieldSetting {
    /// 时间类型枚举（1 日期 / 2 时间 / 3 日期时间 / 4 时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time_type: Option<i32>,
}

/// 附件配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttachmentFieldSetting {
    /// 是否支持多个文件
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_multiple: Option<bool>,
    /// 废弃属性，不建议使用，通常为空值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<i32>,
}

/// 图片配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ImageFieldSetting {
    /// 图片类型枚举（1 头像 / 2 工卡照片 / 3 标志）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<i32>,
    /// 显示样式（1 方形 / 2 圆形）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_style: Option<i32>,
}

/// 计算字段配置信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CalculatedFieldSetting {
    /// 字段类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}
