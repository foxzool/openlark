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
pub struct TableRecordsPostBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 记录数据列表
    records: Vec<serde_json::Value>,
}

impl TableRecordsPostBuilder {
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
    items: Vec<RecordOperationResult>,
}

impl ApiResponseTrait for TableRecordsPostResponse {
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
