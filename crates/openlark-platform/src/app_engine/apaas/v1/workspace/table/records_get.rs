//! 查询数据表数据记录
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
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
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
    /// 筛选条件
    filter: Option<String>,
    /// 排序
    order_by: Option<String>,
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
            page: None,
            page_size: None,
            filter: None,
            order_by: None,
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

    /// 设置排序
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
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
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let mut req: ApiRequest<TableRecordsGetResponse> = ApiRequest::get(&url);
        if let Some(page) = self.page {
            req = req.query("page", page.to_string());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(filter) = self.filter {
            req = req.query("filter", &filter);
        }
        if let Some(order_by) = self.order_by {
            req = req.query("order_by", &order_by);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询数据表记录").await
    }
}

/// 记录信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecord {
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

/// 查询数据表记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsGetResponse {
    /// 记录列表
    #[serde(rename = "items")]
    pub items: Vec<TableRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
    /// 总数
    #[serde(rename = "total_count")]
    pub total_count: u32,
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
    #[tokio::test]
    async fn test_get_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {"id": "r1", "data": {"name": "alice"}, "created_time": 1700000000, "updated_time": 1700000050}
                    ],
                    "has_more": true,
                    "page": 1,
                    "page_size": 20,
                    "total_count": 35
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
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("查询数据表记录应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id, "r1");
        assert!(resp.has_more);
        assert_eq!(resp.total_count, 35);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "GET");
    }
}
