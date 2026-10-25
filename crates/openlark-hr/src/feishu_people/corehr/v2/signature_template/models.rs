//! 电子签模板（signature_template）共享模型
//!
//! 官方类型 `signature_template`：`brief_info`（基本信息）+ `content_info`
//! （模板内容）。文档标注【已废弃，无需关注】的字段（`contents`、
//! `filter_fields`、`using_fields`）与【无需关注】/【弃用】的组合字段
//! （`combination_info`、`combination_info_v2`、`children`）不建模。

use serde::{Deserialize, Serialize};

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 电子签模板（官方 signature_template 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplate {
    /// 电子签模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 模板基本信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief_info: Option<SignatureTemplateBriefInfo>,
    /// 模板内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_info: Option<SignatureTemplateContentInfo>,
}

/// 模板基本信息（官方 signature_template_brief_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateBriefInfo {
    /// 电子签模板 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 模板名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 电子签模板分类（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<EnumValue>,
    /// 电子签模板用途（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<EnumValue>,
    /// 模版签署对象
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatory_labels: Option<Vec<SignatureSignatoryLabel>>,
    /// 是否激活
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 创建人的雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,
    /// 修改人的雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_by: Option<String>,
    /// 电子签模板适用范围（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicability: Option<EnumValue>,
    /// 模板创建方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_method: Option<String>,
    /// 版本
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 模板设置
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_setting: Option<SignatureTemplateSetting>,
    /// 模板适用区域
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_region_info: Option<SignatureTemplateRegionInfo>,
}

/// 模版签署对象（官方 signature_signatory_label 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureSignatoryLabel {
    /// 电子签模板签署对象类型（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_signatory_type: Option<EnumValue>,
    /// 字段多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 模板签署类型名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiname: Option<String>,
}

/// 模板设置（官方 signature_template_setting 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateSetting {
    /// 骑缝章类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_seal_types: Option<Vec<String>>,
}

/// 模板适用区域（官方 signature_template_region_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateRegionInfo {
    /// 是否全球适用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_global_scope: Option<String>,
    /// 适用区域名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta_infos: Option<Vec<SignatureMetaInfo>>,
    /// 模板编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// 模板描述（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_desc: Option<Vec<I18nText>>,
}

/// 适用区域名称（官方 signature_meta_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureMetaInfo {
    /// 区域名称 apiName
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 区域对应的唯一 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// 多语描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
}

/// 模板内容（官方 signature_template_content_info 结构）。
///
/// `contents` / `filter_fields` / `using_fields` 官方已标注废弃，不建模。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateContentInfo {
    /// 自定义字段列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<SignatureTemplateCustomField>>,
    /// 系统字段列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_setting_fields: Option<Vec<SignatureTemplateSystemSettingField>>,
}

/// 电子签模版自定义字段（官方 signature_template_custom_field 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateCustomField {
    /// 中英文描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiname: Option<String>,
    /// 用户自定义字段类型（枚举对象）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field_type: Option<EnumValue>,
    /// 是否使用到
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<bool>,
    /// 是否需要
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// 自定义描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_desc: Option<String>,
    /// 电子签模版公共字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_info: Option<SignatureTemplateCommonFieldInfo>,
}

/// 电子签模版公共字段信息（官方 signature_template_common_field_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateCommonFieldInfo {
    /// 模板字段源类型（枚举对象，如 System / MainData）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EnumValue>,
    /// 字段名（主数据 apiname）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiname: Option<String>,
}

/// 系统字段（官方 signature_template_field 结构）。
///
/// `combination_info` / `combination_info_v2` / `children` 官方标注无需关注，不建模。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateSystemSettingField {
    /// 模板字段值类型枚举
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<EnumValue>,
    /// 双语描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 通用字段信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_info: Option<SignatureTemplateCommonFieldInfo>,
}
