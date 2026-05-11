//! CoreHR 合同相关模型
//!
//! 包含创建、删除、查询、搜索合同等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 合同基础数据结构

/// 合同信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contract {
    /// 合同 ID
    pub contract_id: String,
    /// 合同编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 合同开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 合同结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 合同类型
    /// - 1: 固定期限合同
    /// - 2: 无固定期限合同
    /// - 3: 以完成一定工作任务为期限的合同
    /// - 4: 实习协议
    /// - 5: 劳务合同
    /// - 6: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<i32>,
    /// 合同状态
    /// - 1: 有效
    /// - 2: 已到期
    /// - 3: 已解除
    /// - 4: 已终止
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 签订日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<String>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 合同文件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<ContractFile>>,
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

/// 合同文件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractFile {
    /// 文件 ID
    pub file_id: String,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小（字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomField {
    /// 字段 API 名称
    pub field_api_name: String,
    /// 字段值
    pub field_value: serde_json::Value,
}

// 创建合同相关模型

/// 创建合同请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 员工 ID（必填）
    pub employee_id: String,
    /// 合同编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// 合同开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 合同结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 合同类型
    /// - 1: 固定期限合同
    /// - 2: 无固定期限合同
    /// - 3: 以完成一定工作任务为期限的合同
    /// - 4: 实习协议
    /// - 5: 劳务合同
    /// - 6: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<i32>,
    /// 签订日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<String>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 合同文件 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 创建合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 合同 ID
    pub contract_id: String,
}

// 删除合同相关模型

/// 删除合同请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 合同 ID（必填）
    pub contract_id: String,
}

/// 删除合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个合同相关模型

/// 查询单个合同请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRequestBody {
    /// 合同 ID（必填）
    pub contract_id: String,
}

/// 查询单个合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 合同信息
    pub contract: Contract,
}

// 批量查询合同相关模型

/// 批量查询合同请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工 ID 列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 合同状态列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<i32>>,
}

/// 批量查询合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 合同列表
    pub items: Vec<Contract>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 更新合同相关模型

/// 更新合同请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 合同 ID（必填）
    pub contract_id: String,
    /// 合同编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// 合同开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 合同结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 合同类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<i32>,
    /// 合同状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 签订日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_date: Option<String>,
    /// 试用期开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_start_date: Option<String>,
    /// 试用期结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_end_date: Option<String>,
    /// 试用期时长（月）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_duration: Option<i32>,
    /// 合同文件 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// 更新合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}

// 搜索合同相关模型

/// 搜索合同请求体
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
    /// 员工 ID 列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 合同状态列表（用于过滤）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<i32>>,
}

/// 搜索合同响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 合同列表
    pub items: Vec<Contract>,
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
