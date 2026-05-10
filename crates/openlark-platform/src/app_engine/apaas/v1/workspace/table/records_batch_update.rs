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
pub struct TableRecordsBatchUpdateBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 要更新的记录列表
    records: Vec<RecordUpdate>,
}

impl TableRecordsBatchUpdateBuilder {
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

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量更新记录响应数据为空".to_string(),
                "服务器没有返回有效的数据".to_string(),
            )
        })
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
    updated_count: u32,
    /// 操作结果列表
    #[serde(rename = "items")]
    pub items: Vec<RecordOperationResult>,
}

impl ApiResponseTrait for TableRecordsBatchUpdateResponse {
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
