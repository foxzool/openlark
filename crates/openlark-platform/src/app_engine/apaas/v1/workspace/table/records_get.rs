//! 查询数据表数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/records_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询数据表记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsGetRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 分页标记，第一次请求不填，表示从头开始遍历
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<u32>,
    /// 返回的列，默认为 *
    select: Option<String>,
    /// 筛选条件（PostgREST 语法）
    filter: Option<String>,
    /// 排序条件（PostgREST 语法）
    order: Option<String>,
}

impl TableRecordsGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
            page_token: None,
            page_size: None,
            select: None,
            filter: None,
            order: None,
        }
    }

    /// 设置分页标记，第一次请求不填，表示从头开始遍历
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置返回的列，默认为 *
    pub fn select(mut self, select: impl Into<String>) -> Self {
        self.select = Some(select.into());
        self
    }

    /// 设置筛选条件（PostgREST 语法，如 `age=gt.10`）
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
    pub async fn execute(self) -> SDKResult<TableRecordsGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsGetResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let mut req: ApiRequest<TableRecordsGetResponse> = ApiRequest::get(&url);
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", &page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
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
        Transport::request_typed(req, &self.config, Some(option), "查询数据表记录").await
    }
}

/// 查询数据表记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsGetResponse {
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

impl ApiResponseTrait for TableRecordsGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsGetBuilder = TableRecordsGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../tables/{table}/records → 强类型 TableRecordsGetResponse。
    /// 官方示例：items 为数组序列化后的 JSONString，data 下含 has_more/page_token/total。
    #[tokio::test]
    async fn test_get_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": "[{\"name\":\"王一一\",\"gender\":\"male\",\"age\":10}]",
                    "has_more": true,
                    "page_token": "next_token",
                    "total": 35
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

        let resp = TableRecordsGetRequestBuilder::new(config, "ws_001", "user")
            .page_size(20)
            .filter("age=gt.10")
            .select("*")
            .order("age desc")
            .execute()
            .await
            .expect("查询数据表记录应成功");
        assert!(resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("next_token"));
        assert_eq!(resp.total, 35);
        let rows: Vec<serde_json::Value> =
            serde_json::from_str(&resp.items).expect("items 应为 JSONString");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["name"], "王一一");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "GET");
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=20"),
            "query should contain page_size=20: {query}"
        );
        assert!(
            query.contains("filter="),
            "query should contain filter: {query}"
        );
        assert!(
            query.contains("select="),
            "query should contain select: {query}"
        );
        assert!(
            query.contains("order="),
            "query should contain order: {query}"
        );
    }
}
