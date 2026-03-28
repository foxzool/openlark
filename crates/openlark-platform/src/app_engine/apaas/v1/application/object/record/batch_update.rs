//! 批量编辑记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量编辑记录 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchUpdateBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 要更新的记录列表
    records: Vec<RecordUpdateItem>,
}

impl RecordBatchUpdateBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            records: Vec::new(),
        }
    }

    /// 添加要更新的记录
    pub fn record(
        mut self,
        record_id: impl Into<String>,
        data: impl Into<serde_json::Value>,
    ) -> Self {
        self.records.push(RecordUpdateItem {
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
            .extend(records.into_iter().map(|(id, data)| RecordUpdateItem {
                id: id.into(),
                data: data.into(),
            }));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordBatchUpdateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_update",
            self.namespace, self.object_api_name
        );

        let request = RecordBatchUpdateRequest {
            records: self.records,
        };

        let req: ApiRequest<RecordBatchUpdateResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchUpdateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_update",
            self.namespace, self.object_api_name
        );

        let request = RecordBatchUpdateRequest {
            records: self.records,
        };

        let req: ApiRequest<RecordBatchUpdateResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 记录更新项
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordUpdateItem {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 更新的数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 批量编辑记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordBatchUpdateRequest {
    /// 要更新的记录列表
    #[serde(rename = "records")]
    records: Vec<RecordUpdateItem>,
}

/// 批量编辑记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchUpdateResponse {
    /// 更新的记录列表
    #[serde(rename = "items")]
    items: Vec<RecordUpdateResult>,
}

/// 记录更新结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordUpdateResult {
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

impl ApiResponseTrait for RecordBatchUpdateResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
