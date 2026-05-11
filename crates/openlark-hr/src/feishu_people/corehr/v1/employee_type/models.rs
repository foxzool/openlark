//! CoreHR 人员类型（employee_type）相关模型
//!
//! 包含创建、删除、查询、更新人员类型等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 人员类型基础数据结构

/// 人员类型信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeType {
    /// 人员类型 ID
    pub employee_type_id: String,
    /// 人员类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

// 创建人员类型相关模型

/// 创建人员类型请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 人员类型名称（必填）
    pub name: String,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 人员类型 ID
    pub employee_type_id: String,
}

// 删除人员类型相关模型

/// 删除人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个人员类型相关模型

/// 查询单个人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 人员类型信息
    pub employee_type: EmployeeType,
}

// 批量查询人员类型相关模型

/// 批量查询人员类型请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 人员类型列表
    pub employee_type_list: Vec<EmployeeType>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新人员类型相关模型

/// 更新人员类型请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 人员类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 人员类型描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新人员类型响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
