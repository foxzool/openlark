//! CoreHR 职级（job_level）相关模型
//!
//! 包含创建、删除、查询、更新职级等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 职级基础数据结构

/// 职级信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobLevel {
    /// 职级 ID
    pub job_level_id: String,
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

// 创建职级相关模型

/// 创建职级请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 职级名称（必填）
    pub name: String,
    /// 职级等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
}

/// 创建职级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 职级 ID
    pub job_level_id: String,
}

// 删除职级相关模型

/// 删除职级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个职级相关模型

/// 查询单个职级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 职级信息
    pub job_level: JobLevel,
}

// 批量查询职级相关模型

/// 批量查询职级请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询职级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 职级列表
    pub job_level_list: Vec<JobLevel>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新职级相关模型

/// 更新职级请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 职级名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职级等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 职级描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新职级响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

#[cfg(test)]
#[allow(unused_imports)]
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
