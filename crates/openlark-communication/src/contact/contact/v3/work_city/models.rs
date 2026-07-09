//! 工作城市相关模型（不算 API）

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

/// 工作城市信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkCity {
    /// 工作城市 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_city_id: Option<String>,
    /// 工作城市名称。
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

/// 获取单个工作城市信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkCityResponse {
    /// 单个工作城市详情。
    pub work_city: WorkCity,
}

impl ApiResponseTrait for WorkCityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取租户工作城市列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListWorkCitiesResponse {
    /// 工作城市列表。
    #[serde(default)]
    pub items: Vec<WorkCity>,
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

impl ApiResponseTrait for ListWorkCitiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
