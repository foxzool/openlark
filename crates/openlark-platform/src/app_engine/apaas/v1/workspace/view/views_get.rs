//! 查询视图数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/views/:view_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询视图记录 Builder
#[derive(Debug, Clone)]
pub struct ViewsGetRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 视图名称
    view_name: String,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 筛选条件
    filter: Option<String>,
}

impl ViewsGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        view_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            view_name: view_name.into(),
            page: None,
            page_size: None,
            filter: None,
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

    /// 设置筛选条件
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ViewsGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ViewsGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/views/{}/records",
            self.workspace_id, self.view_name
        );

        let mut req: ApiRequest<ViewsGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(filter) = self.filter {
            req = req.query("filter", &filter);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("查询视图记录", "响应数据为空"))
    }
}

/// 视图记录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewRecord {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

/// 查询视图记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewsGetResponse {
    /// 记录列表
    #[serde(rename = "items")]
    pub items: Vec<ViewRecord>,
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

impl ApiResponseTrait for ViewsGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ViewsGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type ViewsGetBuilder = ViewsGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../apaas/v1/workspaces/{ws}/views/{view_name}/records → 强类型 ViewsGetResponse。
    #[tokio::test]
    async fn test_get_views_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/views/active/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {"id": "r1", "data": {"name": "alice"}, "created_time": 1700000000, "updated_time": 1700000050}
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

        let resp = ViewsGetRequestBuilder::new(config, "ws_001", "active")
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("查询视图记录应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id, "r1");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/views/active/records"
        );
        assert_eq!(received[0].method, "GET");
        let query = received[0].url.query().unwrap_or("");
        assert!(
            query.contains("page=1"),
            "query should contain page=1: {query}"
        );
        assert!(
            query.contains("page_size=20"),
            "query should contain page_size=20: {query}"
        );
    }
}
