//! CoreHR 员工相关模型
//!
//! 包含创建、删除、查询、搜索员工等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 员工基础数据结构

/// 员工信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Employee {
    /// 员工 ID
    pub employee_id: String,
    /// 员工姓名
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工状态
    /// - 0: 未入职
    /// - 1: 在职
    /// - 2: 离职
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// 离职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

/// 员工花名册信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeRoster {
    /// 员工 ID
    pub employee_id: String,
    /// 员工姓名
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_name: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 入职日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

// 创建员工相关模型

/// 创建员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 员工姓名（必填）
    pub name: String,
    /// 员工邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 职位 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 入职日期（格式：YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

/// 创建员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 员工 ID
    pub employee_id: String,
}

// 删除员工相关模型

/// 删除员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 员工 ID（必填）
    pub employee_id: String,
}

/// 删除员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 批量查询员工花名册相关模型

/// 批量查询员工花名册请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询员工花名册响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 员工花名册列表
    pub items: Vec<EmployeeRoster>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 搜索员工相关模型

/// 搜索员工请求体
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

/// 搜索员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 员工列表
    pub items: Vec<Employee>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

// 批量获取员工相关模型

/// 批量获取员工请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRequestBody {
    /// 员工 ID 列表（必填，最多 100 个）
    pub employee_ids: Vec<String>,
}

/// 批量获取员工响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 员工列表
    pub items: Vec<Employee>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_employee_serialization() {
        let employee = Employee {
            employee_id: "emp_001".to_string(),
            name: "张三".to_string(),
            email: Some("zhangsan@example.com".to_string()),
            phone: Some("13800138000".to_string()),
            department_id: Some("dept_001".to_string()),
            position_id: Some("pos_001".to_string()),
            employee_no: Some("10001".to_string()),
            status: Some(1),
            hire_date: Some("2024-01-01".to_string()),
            termination_date: None,
            created_time: Some(1704067200000),
            updated_time: Some(1704067200000),
        };

        let json = serde_json::to_string(&employee).unwrap();
        let parsed: Employee = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(employee.employee_id, parsed.employee_id);
        assert_eq!(employee.name, parsed.name);
        assert_eq!(employee.status, parsed.status);
    }

    #[test]
    fn test_employee_roster_deserialization() {
        let json = r#"{
            "employee_id": "emp_002",
            "name": "李四",
            "email": "lisi@example.com",
            "department_name": "研发部",
            "position_name": "工程师",
            "employee_no": "10002",
            "status": 1,
            "hire_date": "2024-02-01"
        }"#;

        let roster: EmployeeRoster = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(roster.employee_id, "emp_002");
        assert_eq!(roster.name, "李四");
        assert_eq!(roster.department_name, Some("研发部".to_string()));
    }

    #[test]
    fn test_create_request_body() {
        let request = CreateRequestBody {
            name: "王五".to_string(),
            email: Some("wangwu@example.com".to_string()),
            phone: Some("13900139000".to_string()),
            department_id: Some("dept_001".to_string()),
            position_id: Some("pos_001".to_string()),
            employee_no: Some("10003".to_string()),
            hire_date: Some("2024-03-01".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: CreateRequestBody = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(request.name, parsed.name);
        assert_eq!(request.email, parsed.email);
    }

    #[test]
    fn test_create_response() {
        let response = CreateResponse {
            employee_id: "emp_new".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: CreateResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(response.employee_id, parsed.employee_id);
    }

    #[test]
    fn test_delete_response() {
        let response = DeleteResponse { result: true };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: DeleteResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(response.result, parsed.result);
    }

    #[test]
    fn test_list_response() {
        let response = ListResponse {
            items: vec![EmployeeRoster {
                employee_id: "emp_001".to_string(),
                name: "张三".to_string(),
                email: None,
                phone: None,
                department_name: None,
                position_name: None,
                employee_no: None,
                status: None,
                hire_date: None,
            }],
            has_more: false,
            page_token: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ListResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(response.items.len(), parsed.items.len());
        assert_eq!(response.has_more, parsed.has_more);
    }

    #[test]
    fn test_search_request_body() {
        let request = SearchRequestBody {
            query: "张三".to_string(),
            page_size: Some(50),
            page_token: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: SearchRequestBody = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(request.query, parsed.query);
        assert_eq!(request.page_size, parsed.page_size);
    }

    #[test]
    fn test_batch_get_request_body() {
        let request = BatchGetRequestBody {
            employee_ids: vec!["emp_001".to_string(), "emp_002".to_string()],
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: BatchGetRequestBody = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(request.employee_ids.len(), parsed.employee_ids.len());
    }

    #[test]
    fn test_batch_get_response() {
        let response = BatchGetResponse {
            items: vec![Employee {
                employee_id: "emp_001".to_string(),
                name: "张三".to_string(),
                email: None,
                phone: None,
                department_id: None,
                position_id: None,
                employee_no: None,
                status: None,
                hire_date: None,
                termination_date: None,
                created_time: None,
                updated_time: None,
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: BatchGetResponse = serde_json::from_str(&json).expect("JSON 反序列化失败");
        assert_eq!(response.items.len(), parsed.items.len());
    }
}
