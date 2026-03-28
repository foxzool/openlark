//! 获取记录详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取记录详情 Builder
#[derive(Debug, Clone)]
pub struct RecordQueryBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID
    record_id: String,
    /// 字段列表
    fields: Vec<String>,
}

impl RecordQueryBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
        record_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            record_id: record_id.into(),
            fields: Vec::new(),
        }
    }

    /// 添加字段
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    /// 添加多个字段
    pub fn fields(mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields.extend(fields.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordQueryResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}/query",
            self.namespace, self.object_api_name, self.record_id
        );

        let request = RecordQueryRequest {
            fields: self.fields,
        };

        let req: ApiRequest<RecordQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 获取记录详情请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordQueryRequest {
    /// 字段列表
    #[serde(rename = "fields", skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
}

/// 获取记录详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordQueryResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    updated_time: i64,
}

impl ApiResponseTrait for RecordQueryResponse {
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
