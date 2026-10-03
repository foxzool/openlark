//! 单位相关模型（不算 API）

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 单位信息
///
/// 字段随文档演进，未显式建模字段使用 `extra` 透传。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    /// 单位 ID。
    pub unit_id: String,
    /// 单位名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 单位类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 创建单位响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateUnitResponse {
    /// 新建单位的 ID。
    pub unit_id: String,
}

impl ApiResponseTrait for CreateUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单位信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetUnitResponse {
    /// 单位详情。
    pub unit: Unit,
}

impl ApiResponseTrait for GetUnitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单位列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListUnitsResponse {
    /// 单位列表。
    #[serde(default)]
    pub unitlist: Vec<Unit>,
    /// 是否还有更多数据。
    #[serde(default)]
    pub has_more: bool,
    /// 下一页分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListUnitsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单位绑定的部门信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitDepartment {
    /// 单位 ID。
    pub unit_id: String,
    /// 部门 ID。
    pub department_id: String,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 获取单位绑定的部门列表响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListUnitDepartmentsResponse {
    /// 单位绑定的部门列表。
    #[serde(default)]
    pub departmentlist: Vec<UnitDepartment>,
    /// 是否还有更多数据。
    #[serde(default)]
    pub has_more: bool,
    /// 下一页分页标记。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 未显式建模的扩展字段。
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl ApiResponseTrait for ListUnitDepartmentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
