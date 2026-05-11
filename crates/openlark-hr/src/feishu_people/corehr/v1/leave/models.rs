//! CoreHR 假期（leave）相关模型
//!
//! 包含查询工作日历、假期余额等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 假期余额基础数据结构

/// 假期余额信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveBalance {
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 假期类型 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 假期类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_type_name: Option<String>,
    /// 总余额（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<f64>,
    /// 已用余额（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_balance: Option<f64>,
    /// 可用余额（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usable_balance: Option<f64>,
    /// 冻结余额（小时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen_balance: Option<f64>,
    /// 余额单位
    /// - 1: 天
    /// - 2: 小时
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
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

// 根据适用条件获取工作日历 ID 相关模型

/// 根据适用条件获取工作日历 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarByScopeRequestBody {
    /// 员工 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 开始日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// 根据适用条件获取工作日历 ID 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarByScopeResponse {
    /// 工作日历 ID
    pub calendar_id: String,
}

// 批量查询员工假期余额相关模型

/// 批量查询员工假期余额请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalancesRequestBody {
    /// 员工 ID 列表（与 department_id 至少提供一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<String>>,
    /// 部门 ID（与 employee_ids 至少提供一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 假期类型 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询员工假期余额响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveBalancesResponse {
    /// 假期余额列表
    pub leave_balances: Vec<LeaveBalance>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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
