//! 人员类型相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 国际化内容
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

/// 人员类型枚举值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeTypeEnum {
    /// 枚举 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_id: Option<String>,
    /// 枚举值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_value: Option<String>,
    /// 展示内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 枚举类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_type: Option<i32>,
    /// 枚举状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_status: Option<i32>,
    /// 国际化内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<Vec<I18nContent>>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建/更新人员类型响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeTypeEnumResponse {
    /// 单个人员类型详情。
    pub employee_type_enum: EmployeeTypeEnum,
}

impl ApiResponseTrait for EmployeeTypeEnumResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询人员类型响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListEmployeeTypeEnumsResponse {
    /// 人员类型列表。
    #[serde(default)]
    pub items: Vec<EmployeeTypeEnum>,
    /// 分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListEmployeeTypeEnumsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
