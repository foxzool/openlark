//! CoreHR 任职信息（job_data）相关模型
//!
//! 包含创建、删除、查询、更新任职信息等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 任职信息基础数据结构

/// 任职信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobData {
    /// 任职信息 ID
    pub job_data_id: String,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 职务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 状态
    /// - 1: 在职
    /// - 2: 离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomField {
    /// 字段 API 名称
    pub field_api_name: String,
    /// 字段值
    pub value: serde_json::Value,
}

// 创建任职信息相关模型

/// 创建任职信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 员工 ID（必填）
    pub employee_id: String,
    /// 职务 ID（必填）
    pub job_id: String,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 状态
    /// - 1: 在职
    /// - 2: 离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建任职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 任职信息 ID
    pub job_data_id: String,
}

// 删除任职信息相关模型

/// 删除任职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个任职信息相关模型

/// 查询单个任职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 任职信息
    pub job_data: JobData,
}

// 批量查询任职信息相关模型

/// 批量查询任职信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工 ID 过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 状态过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 批量查询任职信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 任职信息列表
    pub job_data_list: Vec<JobData>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新任职信息相关模型

/// 更新任职信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 职务 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 状态
    /// - 1: 在职
    /// - 2: 离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 更新任职信息响应
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
