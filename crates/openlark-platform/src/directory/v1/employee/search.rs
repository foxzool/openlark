//! 搜索员工信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/employee/search>
//! docPath: <https://open.feishu.cn/document/directory-v1/employee/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 搜索员工信息 Builder
#[derive(Debug, Clone)]
pub struct EmployeeSearchRequestBuilder {
    config: Config,
    /// 搜索关键词
    keyword: String,
    /// 部门 ID
    department_id: Option<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl EmployeeSearchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, keyword: impl Into<String>) -> Self {
        Self {
            config,
            keyword: keyword.into(),
            department_id: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
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
    pub async fn execute(self) -> SDKResult<EmployeeSearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<EmployeeSearchResponse> {
        let url = "/open-apis/directory/v1/employees/search".to_string();

        let request = EmployeeSearchRequest {
            keyword: self.keyword,
            department_id: self.department_id,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<EmployeeSearchResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 搜索员工信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct EmployeeSearchRequest {
    /// 搜索关键词
    #[serde(rename = "keyword")]
    keyword: String,
    /// 部门 ID
    #[serde(rename = "department_id", skip_serializing_if = "Option::is_none")]
    department_id: Option<String>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 搜索到的员工信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedEmployee {
    /// 员工 ID
    #[serde(rename = "employee_id")]
    employee_id: String,
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
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 搜索员工信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmployeeSearchResponse {
    /// 搜索结果列表
    #[serde(rename = "items")]
    pub items: Vec<SearchedEmployee>,
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

impl ApiResponseTrait for EmployeeSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EmployeeSearchRequestBuilder, will be removed in v1.0 (#271)")]
pub type EmployeeSearchBuilder = EmployeeSearchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../employees/search → 强类型 EmployeeSearchResponse。
    #[tokio::test]
    async fn test_search_employees_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/employees/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "employee_id": "emp_001",
                            "name": "alice",
                            "mobile": "13800000001",
                            "email": "alice@example.com",
                            "department_ids": ["dept_001"]
                        }
                    ],
                    "has_more": false,
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

        let resp = EmployeeSearchRequestBuilder::new(config, "alice")
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("搜索员工信息应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].employee_id, "emp_001");
        assert_eq!(resp.items[0].name, "alice");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/employees/search"
        );
        assert_eq!(received[0].method, "POST");
    }
}
