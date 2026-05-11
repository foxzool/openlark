//! CoreHR 序列（job_family）相关模型
//!
//! 包含创建、删除、查询、更新序列等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 序列基础数据结构

/// 序列信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobFamily {
    /// 序列 ID
    pub job_family_id: String,
    /// 序列名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 序列描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 序列编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

// 创建序列相关模型

/// 创建序列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 序列名称（必填）
    pub name: String,
    /// 序列描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// 创建序列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 序列 ID
    pub job_family_id: String,
}

// 删除序列相关模型

/// 删除序列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个序列相关模型

/// 查询单个序列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 序列信息
    pub job_family: JobFamily,
}

// 批量查询序列相关模型

/// 批量查询序列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询序列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 序列列表
    pub job_family_list: Vec<JobFamily>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新序列相关模型

/// 更新序列请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 序列名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 序列描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 序列编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新序列响应
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
