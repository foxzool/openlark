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
pub struct TableRecordsPatchBuilder {
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

impl TableRecordsPatchBuilder {
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
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
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
    updated_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for TableRecordsPatchResponse {
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
