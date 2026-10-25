//! 职务（job）v2 共享响应模型
//!
//! 供 `job/get.rs` 与 `job/list.rs` 复用。字段形状与 v1 `corehr/v1/job/models.rs`
//! 的 `Job` 不同，勿混用。

use serde::{Deserialize, Serialize};

/// 职务信息（官方 `job` 结构，v2 形状）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CorehrV2Job {
    /// 职务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 启用状态，true 为启用，false 为停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 职务头衔（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<Vec<I18nText>>,
    /// 通道 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    /// 职务关联序列 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id_list: Option<Vec<String>>,
    /// 职务关联职级 ID 列表（需 corehr:job.job_level:read 字段权限，未授权时不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id_list: Option<Vec<String>>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 当前版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 当前版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 `object_field_data` 结构（`{field_name, value}`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
