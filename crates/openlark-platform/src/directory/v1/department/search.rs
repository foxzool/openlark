//! 搜索部门
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/search>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 搜索部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentSearchRequestBuilder {
    config: Config,
    /// 搜索关键词
    keyword: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl DepartmentSearchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, keyword: impl Into<String>) -> Self {
        Self {
            config,
            keyword: keyword.into(),
            page: None,
            page_size: None,
        }
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
    pub async fn execute(self) -> SDKResult<DepartmentSearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentSearchResponse> {
        let url = "/open-apis/directory/v1/departments/search".to_string();

        let request = DepartmentSearchRequest {
            keyword: self.keyword,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<DepartmentSearchResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 搜索部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentSearchRequest {
    /// 搜索关键词
    #[serde(rename = "keyword")]
    keyword: String,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 搜索到的部门信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedDepartment {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
}

/// 搜索部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentSearchResponse {
    /// 搜索结果列表
    #[serde(rename = "items")]
    pub items: Vec<SearchedDepartment>,
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

impl ApiResponseTrait for DepartmentSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentSearchRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentSearchBuilder = DepartmentSearchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../directory/v1/departments/search → 强类型 DepartmentSearchResponse。
    #[tokio::test]
    async fn test_search_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/departments/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "department_id": "dept_001",
                            "name": "Engineering",
                            "parent_id": "dept_000"
                        }
                    ],
                    "has_more": true,
                    "page": 1,
                    "page_size": 10
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

        let resp = DepartmentSearchRequestBuilder::new(config, "Eng")
            .page(1)
            .page_size(10)
            .execute()
            .await
            .expect("搜索部门应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].department_id, "dept_001");
        assert!(resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 10);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/search"
        );
        assert_eq!(received[0].method, "POST");
    }
}
