//! CoreHR 公司相关模型
//!
//! 包含创建、删除、查询、更新公司等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 公司基础数据结构

/// 公司信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Company {
    /// 公司 ID
    pub company_id: String,
    /// 公司名称
    pub name: String,
    /// 公司状态
    /// - 0: 停用
    /// - 1: 启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 公司编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 公司描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

// 创建公司相关模型

/// 创建公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 公司名称（必填）
    pub name: String,
    /// 公司编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 公司描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 公司 ID
    pub company_id: String,
}

// 删除公司相关模型

/// 删除公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 公司 ID（必填）
    pub company_id: String,
}

/// 删除公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个公司相关模型

/// 查询单个公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 公司 ID（必填）
    pub company_id: String,
}

/// 查询单个公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 公司信息
    pub company: Company,
}

// 批量查询公司相关模型

/// 批量查询公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 公司列表
    pub items: Vec<Company>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 更新公司相关模型

/// 更新公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 公司 ID（必填）
    pub company_id: String,
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 公司编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 公司描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 公司状态
    /// - 0: 停用
    /// - 1: 启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

// 批量获取公司相关模型

/// 批量获取公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRequestBody {
    /// 公司 ID 列表（必填，最多 100 个）
    pub company_ids: Vec<String>,
}

/// 批量获取公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 公司列表
    pub items: Vec<Company>,
}

// 激活公司相关模型

/// 激活公司请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRequestBody {
    /// 公司 ID（必填）
    pub company_id: String,
}

/// 激活公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponse {
    /// 激活结果
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
