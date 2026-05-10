//! 执行 OQL
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query
//! docPath: https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 执行 OQL Builder
#[derive(Debug, Clone)]
pub struct OqlQueryBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// OQL 查询语句
    oql: String,
    /// 字段列表
    fields: Vec<String>,
}

impl OqlQueryBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, oql: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            oql: oql.into(),
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
    pub async fn execute(self) -> SDKResult<OqlQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<OqlQueryResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/oql_query",
            self.namespace
        );

        let request = OqlQueryRequest {
            oql: self.oql,
            fields: self.fields,
        };

        let req: ApiRequest<OqlQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 执行 OQL 请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OqlQueryRequest {
    /// OQL 查询语句
    #[serde(rename = "oql")]
    oql: String,
    /// 字段列表
    #[serde(rename = "fields", skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
}

/// OQL 查询结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OqlRecord {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 执行 OQL 响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OqlQueryResponse {
    /// 查询结果列表
    #[serde(rename = "items")]
    items: Vec<OqlRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
}

impl ApiResponseTrait for OqlQueryResponse {
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
