//! 职级相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 多语言内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语言区域。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 文本值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 职级信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobLevel {
    /// 职级 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 排序值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 国际化名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    /// 国际化描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<Vec<I18nContent>>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建/更新/获取单个职级响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobLevelResponse {
    /// 单个职级详情。
    pub job_level: JobLevel,
}

impl ApiResponseTrait for JobLevelResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户职级列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListJobLevelsResponse {
    /// 职级列表。
    #[serde(default)]
    pub items: Vec<JobLevel>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(default)]
    pub has_more: bool,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListJobLevelsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
