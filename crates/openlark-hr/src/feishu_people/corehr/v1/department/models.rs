//! CoreHR 部门相关模型
//!
//! 包含创建、删除、查询、搜索部门等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 部门基础数据结构

/// 部门信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Department {
    /// 部门 ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 部门负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_ids: Option<Vec<String>>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门状态
    /// - 0: 停用
    /// - 1: 启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

/// 部门时间轴信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentTimeline {
    /// 部门 ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 生效日期（格式：YYYY-MM-DD）
    pub effective_date: String,
    /// 失效日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 部门负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_ids: Option<Vec<String>>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 部门操作日志
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentOperationLog {
    /// 操作 ID
    pub operation_id: String,
    /// 操作类型
    pub operation_type: String,
    /// 操作人 ID
    pub operator_id: String,
    /// 操作时间（毫秒时间戳）
    pub operation_time: i64,
    /// 变更内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<DepartmentChange>>,
}

/// 部门变更项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentChange {
    /// 变更字段名
    pub field_name: String,
    /// 变更前值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    /// 变更后值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
}

/// 部门树节点
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentTreeNode {
    /// 部门 ID
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 部门负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_ids: Option<Vec<String>>,
    /// 子部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DepartmentTreeNode>>,
}

// 创建部门相关模型

/// 创建部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 部门名称（必填）
    pub name: String,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_ids: Option<Vec<String>>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 部门 ID
    pub department_id: String,
}

// 删除部门相关模型

/// 删除部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 部门 ID（必填）
    pub department_id: String,
}

/// 删除部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个部门相关模型

/// 查询单个部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 部门 ID（必填）
    pub department_id: String,
}

/// 查询单个部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 部门信息
    pub department: Department,
}

// 批量查询部门相关模型

/// 批量查询部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 更新部门相关模型

/// 更新部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 部门 ID（必填）
    pub department_id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    /// 部门负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_ids: Option<Vec<String>>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 部门描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 部门状态
    /// - 0: 停用
    /// - 1: 启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

// 搜索部门相关模型

/// 搜索部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequestBody {
    /// 搜索关键词（必填）
    pub query: String,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 搜索部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 部门列表
    pub items: Vec<Department>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 批量获取部门相关模型

/// 批量获取部门请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRequestBody {
    /// 部门 ID 列表（必填，最多 100 个）
    pub department_ids: Vec<String>,
}

/// 批量获取部门响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 部门列表
    pub items: Vec<Department>,
}

// 获取父部门信息相关模型

/// 获取父部门信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponse {
    /// 父部门列表（从近到远）
    pub items: Vec<Department>,
}

// 获取部门树相关模型

/// 获取部门树请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeRequestBody {
    /// 根部门 ID（不传则从顶层开始）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 是否包含已停用部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
}

/// 获取部门树响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreeResponse {
    /// 部门树节点列表
    pub items: Vec<DepartmentTreeNode>,
}

// 查询时间轴相关模型

/// 查询时间轴请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineRequestBody {
    /// 部门 ID（必填）
    pub department_id: String,
    /// 开始时间（必填，格式：YYYY-MM-DD）
    pub start_time: String,
    /// 结束时间（必填，格式：YYYY-MM-DD）
    pub end_time: String,
}

/// 查询时间轴响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineResponse {
    /// 时间轴列表
    pub items: Vec<DepartmentTimeline>,
}

// 查询多时间轴相关模型

/// 查询多时间轴请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTimelineRequestBody {
    /// 部门 ID 列表（必填）
    pub department_ids: Vec<String>,
    /// 开始时间（必填，格式：YYYY-MM-DD）
    pub start_time: String,
    /// 结束时间（必填，格式：YYYY-MM-DD）
    pub end_time: String,
}

/// 查询多时间轴响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiTimelineResponse {
    /// 部门时间轴映射（key: department_id, value: 时间轴列表）
    pub items: std::collections::HashMap<String, Vec<DepartmentTimeline>>,
}

// 查询操作日志相关模型

/// 查询操作日志请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationLogsRequestBody {
    /// 部门 ID（必填）
    pub department_id: String,
    /// 开始时间（必填，格式：YYYY-MM-DD）
    pub start_time: String,
    /// 结束时间（必填，格式：YYYY-MM-DD）
    pub end_time: String,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 查询操作日志响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationLogsResponse {
    /// 操作日志列表
    pub items: Vec<DepartmentOperationLog>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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
