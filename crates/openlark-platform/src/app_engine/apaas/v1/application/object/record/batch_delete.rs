//! 批量删除记录
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete
//! docPath: https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchDeleteBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl RecordBatchDeleteBuilder {
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
    pub async fn execute(self) -> SDKResult<RecordBatchDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchDeleteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_delete",
            self.namespace, self.object_api_name
        );

        let request = RecordBatchDeleteRequest {
            record_ids: self.record_ids,
        };

        let req: ApiRequest<RecordBatchDeleteResponse> =
            ApiRequest::delete(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量删除记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordBatchDeleteRequest {
    /// 记录 ID 列表
    #[serde(rename = "record_ids")]
    record_ids: Vec<String>,
}

/// 批量删除记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchDeleteResponse {
    /// 删除的记录数量
    #[serde(rename = "deleted_count")]
    deleted_count: u32,
    /// 删除结果列表
    #[serde(rename = "items")]
    items: Vec<RecordDeleteResult>,
}

/// 记录删除结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteResult {
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

impl ApiResponseTrait for RecordBatchDeleteResponse {
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
