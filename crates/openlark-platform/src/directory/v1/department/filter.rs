//! 获取部门列表
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/filter>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/filter>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取部门列表 Builder
#[derive(Debug, Clone)]
pub struct DepartmentFilterRequestBuilder {
    config: Config,
    /// 父部门 ID
    parent_id: Option<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl DepartmentFilterRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            parent_id: None,
            page: None,
            page_size: None,
        }
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
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
    pub async fn execute(self) -> SDKResult<DepartmentFilterResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentFilterResponse> {
        let url = "/open-apis/directory/v1/departments/filter".to_string();

        let request = DepartmentFilterRequest {
            parent_id: self.parent_id,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<DepartmentFilterResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 获取部门列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentFilterRequest {
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 部门简要信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentBrief {
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

/// 获取部门列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentFilterResponse {
    /// 部门列表
    #[serde(rename = "items")]
    pub items: Vec<DepartmentBrief>,
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

impl ApiResponseTrait for DepartmentFilterResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentFilterRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentFilterBuilder = DepartmentFilterRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../directory/v1/departments/filter → 强类型 DepartmentFilterResponse。
    #[tokio::test]
    async fn test_filter_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/departments/filter"))
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

        let resp = DepartmentFilterRequestBuilder::new(config)
            .parent_id("dept_000")
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("获取部门列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].department_id, "dept_001");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/filter"
        );
        assert_eq!(received[0].method, "POST");
    }
}
