//! 批量更新数据表中的记录
//!
//! URL: PATCH:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records_batch_update
//! docPath:

use crate::app_engine::apaas::v1::workspace::table::records_post::RecordOperationResult;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsBatchUpdateRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 要更新的记录列表
    records: Vec<RecordUpdate>,
}

impl TableRecordsBatchUpdateRequestBuilder {
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

    /// 添加要更新的记录
    pub fn record(
        mut self,
        record_id: impl Into<String>,
        data: impl Into<serde_json::Value>,
    ) -> Self {
        self.records.push(RecordUpdate {
            id: record_id.into(),
            data: data.into(),
        });
        self
    }

    /// 添加多条要更新的记录
    pub fn records(
        mut self,
        records: impl IntoIterator<Item = (impl Into<String>, impl Into<serde_json::Value>)>,
    ) -> Self {
        self.records
            .extend(records.into_iter().map(|(id, data)| RecordUpdate {
                id: id.into(),
                data: data.into(),
            }));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsBatchUpdateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsBatchUpdateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records_batch_update",
            self.workspace_id, self.table_name
        );

        let request = TableRecordsBatchUpdateRequest {
            records: self.records,
        };

        // 使用新的 Transport API
        let req: ApiRequest<TableRecordsBatchUpdateResponse> =
            ApiRequest::patch(url).body(serde_json::to_value(&request)?);

        Transport::request_typed(req, &self.config, Some(option), "批量更新记录响应数据为空").await
    }
}

/// 记录更新
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordUpdate {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 更新的数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 批量更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsBatchUpdateRequest {
    /// 要更新的记录列表
    #[serde(rename = "records")]
    records: Vec<RecordUpdate>,
}

/// 批量更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsBatchUpdateResponse {
    /// 更新的记录数量
    #[serde(rename = "updated_count")]
    pub updated_count: u32,
    /// 操作结果列表
    #[serde(rename = "items")]
    pub items: Vec<RecordOperationResult>,
}

impl ApiResponseTrait for TableRecordsBatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to TableRecordsBatchUpdateRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type TableRecordsBatchUpdateBuilder = TableRecordsBatchUpdateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../tables/{table}/records_batch_update → 强类型 TableRecordsBatchUpdateResponse。
    #[tokio::test]
    async fn test_batch_update_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records_batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "updated_count": 2,
                    "items": [
                        {"id": "r1", "success": true},
                        {"id": "r2", "success": true}
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

        let resp = TableRecordsBatchUpdateRequestBuilder::new(config, "ws_001", "user")
            .record("r1", json!({"name": "alice"}))
            .record("r2", json!({"name": "bob"}))
            .execute()
            .await
            .expect("批量更新记录应成功");
        assert_eq!(resp.updated_count, 2);
        assert_eq!(resp.items.len(), 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records_batch_update"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
