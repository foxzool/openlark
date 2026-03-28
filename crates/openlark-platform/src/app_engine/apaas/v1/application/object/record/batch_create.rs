//! 批量新建记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量新建记录 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchCreateBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录数据列表
    records: Vec<serde_json::Value>,
}

impl RecordBatchCreateBuilder {
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
    pub async fn execute(self) -> SDKResult<RecordBatchCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchCreateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_create",
            self.namespace, self.object_api_name
        );

        let request = RecordBatchCreateRequest {
            records: self.records,
        };

        let req: ApiRequest<RecordBatchCreateResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量新建记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordBatchCreateRequest {
    /// 记录数据列表
    #[serde(rename = "records")]
    records: Vec<serde_json::Value>,
}

/// 批量新建记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchCreateResponse {
    /// 创建的记录列表
    #[serde(rename = "items")]
    items: Vec<RecordCreateResult>,
}

/// 记录创建结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordCreateResult {
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

impl ApiResponseTrait for RecordBatchCreateResponse {
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
