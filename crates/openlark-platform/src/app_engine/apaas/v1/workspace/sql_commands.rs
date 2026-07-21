//! 执行SQL
//!
//! URL: POST:/open-apis/apaas/v1/workspaces/:workspace_id/sql_commands
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 执行SQL Builder
#[derive(Debug, Clone)]
pub struct SqlCommandsRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// SQL 语句
    sql: String,
}

impl SqlCommandsRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>, sql: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            sql: sql.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SqlCommandsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SqlCommandsResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/sql_commands",
            self.workspace_id
        );

        let request = SqlCommandsRequest { sql: self.sql };

        let req: ApiRequest<SqlCommandsResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 执行SQL请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct SqlCommandsRequest {
    /// SQL 语句
    #[serde(rename = "sql")]
    sql: String,
}

/// SQL 执行结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlResult {
    /// 结果数据
    #[serde(rename = "data")]
    data: Vec<serde_json::Value>,
    /// 影响行数
    #[serde(rename = "affected_rows")]
    affected_rows: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

/// 执行SQL响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SqlCommandsResponse {
    /// SQL 执行结果
    #[serde(rename = "result")]
    pub result: SqlResult,
}

impl ApiResponseTrait for SqlCommandsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to SqlCommandsRequestBuilder, will be removed in v1.0 (#271)")]
pub type SqlCommandsBuilder = SqlCommandsRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../apaas/v1/workspaces/{ws}/sql_commands → 强类型 SqlCommandsResponse。
    #[tokio::test]
    async fn test_execute_sql_commands_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/workspaces/ws_001/sql_commands"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "result": {
                        "data": [{"id": 1}, {"id": 2}],
                        "affected_rows": 2,
                        "message": "OK"
                    }
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

        let resp = SqlCommandsRequestBuilder::new(config, "ws_001", "SELECT * FROM t")
            .execute()
            .await
            .expect("执行 SQL 应成功");
        assert_eq!(resp.result.affected_rows, 2);
        assert_eq!(resp.result.message, "OK");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/sql_commands"
        );
        assert_eq!(received[0].method, "POST");
    }
}
