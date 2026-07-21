//! 按条件更新数据表中的记录
//!
//! URL: PATCH:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 按条件更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsPatchRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 筛选条件
    filter: String,
    /// 更新的数据
    data: serde_json::Value,
}

impl TableRecordsPatchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
        filter: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
            filter: filter.into(),
            data: serde_json::json!({}),
        }
    }

    /// 设置更新的数据
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = data.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsPatchResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let request = TableRecordsPatchRequest {
            filter: self.filter,
            data: self.data,
        };

        let req: ApiRequest<TableRecordsPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(
            req,
            &self.config,
            Some(RequestOption::default()),
            "Operation",
        )
        .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsPatchResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        let request = TableRecordsPatchRequest {
            filter: self.filter,
            data: self.data,
        };

        let req: ApiRequest<TableRecordsPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 按条件更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsPatchRequest {
    /// 筛选条件
    #[serde(rename = "filter")]
    filter: String,
    /// 更新的数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 按条件更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsPatchResponse {
    /// 更新的记录数量
    #[serde(rename = "updated_count")]
    pub updated_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for TableRecordsPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsPatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsPatchBuilder = TableRecordsPatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../tables/{table}/records → 强类型 TableRecordsPatchResponse。
    #[tokio::test]
    async fn test_patch_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "updated_count": 3,
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

        let resp = TableRecordsPatchRequestBuilder::new(config, "ws_001", "user", "age > 18")
            .data(json!({"status": "active"}))
            .execute()
            .await
            .expect("按条件更新记录应成功");
        assert_eq!(resp.updated_count, 3);
        assert_eq!(resp.message, "OK");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
