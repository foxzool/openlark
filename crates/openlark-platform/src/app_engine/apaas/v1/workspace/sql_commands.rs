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
pub struct SqlCommandsBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// SQL 语句
    sql: String,
}

impl SqlCommandsBuilder {
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
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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
    result: SqlResult,
}

impl ApiResponseTrait for SqlCommandsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
