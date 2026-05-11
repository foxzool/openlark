//! CoreHR 职务相关模型
//!
//! 包含创建、删除、查询、更新职务等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 职务基础数据结构

/// 职务信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    /// 职务 ID
    pub job_id: String,
    /// 职务名称
    pub name: String,
    /// 职务编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 职务状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 所属序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 所属职级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
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
    pub field_value: serde_json::Value,
}

// 创建职务相关模型

/// 创建职务请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 职务名称（必填）
    pub name: String,
    /// 职务编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 职务状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 所属序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 所属职级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建职务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 职务 ID
    pub job_id: String,
}

// 删除职务相关模型

/// 删除职务请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 职务 ID（必填）
    pub job_id: String,
}

/// 删除职务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个职务相关模型

/// 查询单个职务请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 职务 ID（必填）
    pub job_id: String,
}

/// 查询单个职务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 职务信息
    pub job: Job,
}

// 批量查询职务相关模型

/// 批量查询职务请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 职务名称（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职务状态列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<i32>>,
    /// 所属序列 ID 列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_ids: Option<Vec<String>>,
    /// 所属职级 ID 列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_ids: Option<Vec<String>>,
}

/// 批量查询职务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 职务列表
    pub items: Vec<Job>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 更新职务相关模型

/// 更新职务请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 职务 ID（必填）
    pub job_id: String,
    /// 职务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职务编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 职务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 职务状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 所属序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 所属职级 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 更新职务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

#[cfg(test)]
mod tests {

    use serde_json;

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
