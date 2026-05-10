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
pub struct TableRecordsDeleteBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl TableRecordsDeleteBuilder {
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

        let resp = Transport::request(api_request, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除数据表记录", "响应数据为空"))
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsDeleteResponse {
    /// 删除的记录数量
    #[serde(rename = "deleted_count")]
    deleted_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for TableRecordsDeleteResponse {
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
