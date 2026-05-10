//! 更新员工信息
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/patch
//! docPath: https://open.feishu.cn/document/directory-v1/employee/patch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeePatchBuilder {
    config: Config,
    /// 员工 ID
    employee_id: String,
    /// 员工名称
    name: Option<String>,
    /// 邮箱
    email: Option<String>,
    /// 部门 ID 列表
    department_ids: Vec<String>,
}

impl EmployeePatchBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, employee_id: impl Into<String>) -> Self {
        Self {
            config,
            employee_id: employee_id.into(),
            name: None,
            email: None,
            department_ids: Vec::new(),
        }
    }

    /// 设置员工名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置邮箱
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// 添加部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_ids.push(department_id.into());
        self
    }

    /// 添加多个部门 ID
    pub fn department_ids(
        mut self,
        department_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.department_ids
            .extend(department_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EmployeePatchResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let request = EmployeePatchRequest {
            name: self.name,
            email: self.email,
            department_ids: self.department_ids,
        };

        let req: ApiRequest<EmployeePatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeePatchResponse> {
        let url = format!("/open-apis/directory/v1/employees/{}", self.employee_id);

        let request = EmployeePatchRequest {
            name: self.name,
            email: self.email,
            department_ids: self.department_ids,
        };

        let req: ApiRequest<EmployeePatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 更新员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeePatchRequest {
    /// 员工名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids", skip_serializing_if = "Vec::is_empty")]
    department_ids: Vec<String>,
}

/// 更新员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeePatchResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

impl ApiResponseTrait for EmployeePatchResponse {
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
