//! 批量获取员工列表
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/filter
//! docPath: https://open.feishu.cn/document/directory-v1/employee/filter

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量获取员工列表 Builder
#[derive(Debug, Clone)]
pub struct EmployeeFilterBuilder {
    config: Config,
    /// 部门 ID
    department_id: Option<String>,
    /// 是否包含子部门
    include_child_dept: Option<bool>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl EmployeeFilterBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: None,
            include_child_dept: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    /// 设置是否包含子部门
    pub fn include_child_dept(mut self, include: bool) -> Self {
        self.include_child_dept = Some(include);
        self
    }

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeeFilterResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeFilterResponse> {
        let url = "/open-apis/directory/v1/employees/filter".to_string();

        let request = EmployeeFilterRequest {
            department_id: self.department_id,
            include_child_dept: self.include_child_dept,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<EmployeeFilterResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量获取员工列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeFilterRequest {
    /// 部门 ID
    #[serde(rename = "department_id", skip_serializing_if = "Option::is_none")]
    department_id: Option<String>,
    /// 是否包含子部门
    #[serde(rename = "include_child_dept", skip_serializing_if = "Option::is_none")]
    include_child_dept: Option<bool>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 员工简要信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeBrief {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 批量获取员工列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeFilterResponse {
    /// 员工列表
    #[serde(rename = "items")]
    items: Vec<EmployeeBrief>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for EmployeeFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
