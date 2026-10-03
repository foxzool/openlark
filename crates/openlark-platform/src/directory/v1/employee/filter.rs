//! 批量获取员工列表
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/filter>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/filter>

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
pub struct EmployeeFilterRequestBuilder {
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

impl EmployeeFilterRequestBuilder {
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
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
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
    pub items: Vec<EmployeeBrief>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for EmployeeFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeFilterRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeFilterBuilder = EmployeeFilterRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../employees/filter → 强类型 EmployeeFilterResponse。
    #[tokio::test]
    async fn test_filter_employees_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/filter"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employee_id": "emp_001",
                            "name": "alice",
                            "department_ids": ["dept_001"]
                        }
                    ],
                    "has_more": true,
                    "page": 1,
                    "page_size": 20
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = EmployeeFilterRequestBuilder::new(config)
            .department_id("dept_001")
            .include_child_dept(true)
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("批量获取员工列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].employee_id, "emp_001");
        assert_eq!(resp.items[0].name, "alice");
        assert!(resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/filter"
        );
        assert_eq!(received[0].method, "POST");
    }
}
