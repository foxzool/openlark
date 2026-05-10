//! 创建员工
//!
//! 文档: https://open.feishu.cn/document/directory-v1/employee/create
//! docPath: https://open.feishu.cn/document/directory-v1/employee/create

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 创建员工 Builder
#[derive(Debug, Clone)]
pub struct EmployeeCreateBuilder {
    config: Config,
    /// 员工名称
    name: String,
    /// 手机号
    mobile: String,
    /// 邮箱
    email: Option<String>,
    /// 部门 ID 列表
    department_ids: Vec<String>,
}

impl EmployeeCreateBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, name: impl Into<String>, mobile: impl Into<String>) -> Self {
        Self {
            config,
            name: name.into(),
            mobile: mobile.into(),
            email: None,
            department_ids: Vec::new(),
        }
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
    pub async fn execute(self) -> SDKResult<EmployeeCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeCreateResponse> {
        let url = "/open-apis/directory/v1/employees".to_string();

        let request = EmployeeCreateRequest {
            name: self.name,
            mobile: self.mobile,
            email: self.email,
            department_ids: self.department_ids,
        };

        let req: ApiRequest<EmployeeCreateResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 创建员工请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeCreateRequest {
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    mobile: String,
    /// 邮箱
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 部门 ID 列表
    #[serde(rename = "department_ids", skip_serializing_if = "Vec::is_empty")]
    department_ids: Vec<String>,
}

/// 创建员工响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeCreateResponse {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
    /// 员工名称
    #[serde(rename = "name")]
    name: String,
    /// 手机号
    #[serde(rename = "mobile")]
    mobile: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
}

impl ApiResponseTrait for EmployeeCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request =
            EmployeeCreateBuilder::new(config.clone(), "test".to_string(), "test".to_string())
                .email("test".to_string())
                .department_id("test".to_string());
        let _ = request;
    }
}
