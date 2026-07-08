//! 向数据表中添加或更新记录
//!
//! URL: POST:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 添加或更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsPostRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 记录数据列表
    records: Vec<serde_json::Value>,
}

impl TableRecordsPostRequestBuilder {
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
            records: Vec::new(),
        }
    }

    /// 添加记录数据
    pub fn record(mut self, record: impl Into<serde_json::Value>) -> Self {
        self.records.push(record.into());
        self
    }

    /// 添加多条记录数据
    pub fn records(
        mut self,
        records: impl IntoIterator<Item = impl Into<serde_json::Value>>,
    ) -> Self {
        self.records.extend(records.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsPostResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsPostResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let request = TableRecordsPostRequest {
            records: self.records,
        };

        let req: ApiRequest<TableRecordsPostResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 添加或更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsPostRequest {
    /// 记录数据列表
    #[serde(rename = "records")]
    records: Vec<serde_json::Value>,
}

/// 操作结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordOperationResult {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 是否成功
    #[serde(rename = "success")]
    success: bool,
    /// 错误信息
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// 添加或更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsPostResponse {
    /// 操作结果列表
    #[serde(rename = "items")]
    pub items: Vec<RecordOperationResult>,
}

impl ApiResponseTrait for TableRecordsPostResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsPostRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsPostBuilder = TableRecordsPostRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../tables/{table}/records → 强类型 TableRecordsPostResponse。
    #[tokio::test]
    async fn test_post_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {"id": "r1", "success": true},
                        {"id": "r2", "success": false, "error": "冲突"}
                    ]
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

        let resp = TableRecordsPostRequestBuilder::new(config, "ws_001", "user")
            .record(json!({"id": "r1", "name": "alice"}))
            .record(json!({"id": "r2", "name": "bob"}))
            .execute()
            .await
            .expect("添加或更新记录应成功");
        assert_eq!(resp.items.len(), 2);
        assert!(resp.items[0].success);
        assert!(!resp.items[1].success);
        assert_eq!(resp.items[1].error.as_deref(), Some("冲突"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "POST");
    }
}
