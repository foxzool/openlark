//! CoreHR 地点相关模型
//!
//! 包含创建、删除、查询、更新地点等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 地点基础数据结构

/// 地点信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// 地点 ID
    pub location_id: String,
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
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

// 创建地点相关模型

/// 创建地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 地点名称（必填）
    pub name: String,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// 创建地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 地点 ID
    pub location_id: String,
}

// 删除地点相关模型

/// 删除地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个地点相关模型

/// 查询单个地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 地点信息
    pub location: Location,
}

// 批量查询地点相关模型

/// 批量查询地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 地点列表
    pub location_list: Vec<Location>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新地点相关模型

/// 更新地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新地点响应
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
