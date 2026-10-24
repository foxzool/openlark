//! 查询视图数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/views/:view_name/records
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-view/views_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
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
    /// 分页大小，默认 10，最大 500
    page_size: Option<u32>,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 返回的列，默认为 *
    select: Option<String>,
    /// 筛选条件（PostgREST 语法）
    filter: Option<String>,
    /// 排序条件（PostgREST 语法）
    order: Option<String>,
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
            page_size: None,
            page_token: None,
            select: None,
            filter: None,
            order: None,
        }
    }

    /// 设置分页大小（取值范围 1-500，默认 10）
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记，第一次请求不填，表示从头开始遍历
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置返回的列，默认为 *
    pub fn select(mut self, select: impl Into<String>) -> Self {
        self.select = Some(select.into());
        self
    }

    /// 设置筛选条件（PostgREST 语法）
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 设置排序条件（PostgREST 语法）
    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ViewsGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ViewsGetResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.view_name, "view_name 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/views/{}/records",
            self.workspace_id, self.view_name
        );

        let mut req: ApiRequest<ViewsGetResponse> = ApiRequest::get(&url);
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", &page_token);
        }
        if let Some(select) = self.select {
            req = req.query("select", &select);
        }
        if let Some(filter) = self.filter {
            req = req.query("filter", &filter);
        }
        if let Some(order) = self.order {
            req = req.query("order", &order);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询视图记录").await
    }
}

/// 查询视图记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewsGetResponse {
    /// 数据记录列表，格式为数组序列化后的 JSONString，
    /// 调用方可自行 `serde_json::from_str::<Vec<serde_json::Value>>` 解码行数据
    #[serde(rename = "items")]
    pub items: String,
    /// 是否还有更多项
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时会同时返回新的 page_token
    #[serde(rename = "page_token", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 符合条件的记录总数
    #[serde(rename = "total")]
    pub total: i64,
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
    /// 官方契约：查询参数 page_size/page_token/select/filter/order，data 含 items(JSONString)/has_more/page_token/total。
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
                    "items": "[{\"name\":\"王一一\",\"gender\":\"male\",\"age\":10}]",
                    "has_more": false,
                    "total": 1
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
            .page_size(20)
            .select("*")
            .order("age desc")
            .execute()
            .await
            .expect("查询视图记录应成功");
        assert!(!resp.has_more);
        assert_eq!(resp.page_token, None);
        assert_eq!(resp.total, 1);
        let rows: Vec<serde_json::Value> =
            serde_json::from_str(&resp.items).expect("items 应为 JSONString");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["name"], "王一一");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/views/active/records"
        );
        assert_eq!(received[0].method, "GET");
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=20"),
            "query should contain page_size=20: {query}"
        );
        assert!(
            query.contains("select="),
            "query should contain select: {query}"
        );
        assert!(
            query.contains("order="),
            "query should contain order: {query}"
        );
        assert!(!query.contains("page="), "官方无 page 查询参数: {query}");
    }
}
