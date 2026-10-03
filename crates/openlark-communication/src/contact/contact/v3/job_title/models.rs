//! 职务相关模型（不算 API）

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

/// 职务信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobTitle {
    /// 职务 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title_id: Option<String>,
    /// 职务名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 国际化名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<Vec<I18nContent>>,
    /// 状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取单个职务信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobTitleResponse {
    /// 单个职务详情。
    pub job_title: JobTitle,
}

impl ApiResponseTrait for JobTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户职务列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListJobTitlesResponse {
    /// 职务列表。
    #[serde(default)]
    pub items: Vec<JobTitle>,
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

impl ApiResponseTrait for ListJobTitlesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
