//! 域无关的 HR 共享原型类型。
//!
//! 这些类型（国际化文本、ID+name 引用、分页响应壳、目录项等）不是 hire
//! 专属语义，而是所有 HR 域（attendance / corehr / payroll / performance /
//! compensation / ehr / hire）都可能复用的通用原语，因此提升到 crate root。
//!
//! 历史上它们曾定义在 `hire::hire::common_models`，6 个兄弟域要 typed i18n
//! 只能侧向伸手进 hire 子树；提升后 canonical 路径为
//! `crate::common::shared_models`，`hire` 反过来从此处 import。先例见
//! `okr::okr::v2::common::models` 的跨叶消重（#336），本次把同一模式抬到全局（#473）。
//!
//! 序列化契约：字段名与 serde 属性与原 hire 定义逐字一致，wire 格式不变。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 国际化文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct I18nText {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_cn` 字段。
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_us` 字段。
    pub en_us: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 可兼容普通字符串或多语言对象的文本字段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FlexibleText {
    /// `Plain` 变体。
    Plain(String),
    /// `I18n` 变体。
    I18n(I18nText),
}

impl FlexibleText {
    /// 提供 `zh_cn_or_plain` 能力。
    pub fn zh_cn_or_plain(&self) -> Option<&str> {
        match self {
            Self::Plain(value) => Some(value.as_str()),
            Self::I18n(value) => value.zh_cn.as_deref().or(value.en_us.as_deref()),
        }
    }
}

/// 常见的 ID + 名称引用对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IdNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 带 code + name 的区域对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 通用分页响应壳。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PaginatedResponse<T> {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 末尾长尾接口常见的目录/模板类对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CatalogItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `code` 字段。
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `i18n_name` 字段。
    pub i18n_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `description` 字段。
    pub description: Option<FlexibleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `status` 字段。
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `active_status` 字段。
    pub active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `version` 字段。
    pub version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `parent_id` 字段。
    pub parent_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `zh_name` / `en_name` 形式的双语文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocalizedLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `zh_name` 字段。
    pub zh_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `en_name` 字段。
    pub en_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}
