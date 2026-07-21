//! 删除数据表中的记录
//!
//! URL: DELETE:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除数据表记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsDeleteRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl TableRecordsDeleteRequestBuilder {
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
            record_ids: Vec::new(),
        }
    }

    /// 添加记录 ID
    pub fn record_id(mut self, record_id: impl Into<String>) -> Self {
        self.record_ids.push(record_id.into());
        self
    }

    /// 添加多个记录 ID
    pub fn record_ids(mut self, record_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.record_ids
            .extend(record_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsDeleteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        use serde_json::json;

        let request = json!({
            "record_ids": self.record_ids,
        });

        let mut api_request = ApiRequest::<TableRecordsDeleteResponse>::delete(&url);
        api_request = api_request.body(request);

        Transport::request_typed(api_request, &self.config, Some(option), "删除数据表记录").await
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsDeleteResponse {
    /// 删除的记录数量
    #[serde(rename = "deleted_count")]
    pub deleted_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for TableRecordsDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsDeleteBuilder = TableRecordsDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../tables/{table}/records → 强类型 TableRecordsDeleteResponse。
    #[tokio::test]
    async fn test_delete_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "deleted_count": 2,
                    "message": "OK"
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

        let resp = TableRecordsDeleteRequestBuilder::new(config, "ws_001", "user")
            .record_id("r1")
            .record_id("r2")
            .execute()
            .await
            .expect("删除数据表记录应成功");
        assert_eq!(resp.deleted_count, 2);
        assert_eq!(resp.message, "OK");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
