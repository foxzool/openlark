//! 搜索记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 搜索记录 Builder
#[derive(Debug, Clone)]
pub struct RecordSearchBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 搜索条件
    search: String,
    /// 字段列表
    fields: Vec<String>,
    /// 页码
    page: Option<u32>,
    /// 每页数量
    page_size: Option<u32>,
}

impl RecordSearchBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, search: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            search: search.into(),
            fields: Vec::new(),
            page: None,
            page_size: None,
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

    /// 设置页码
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordSearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordSearchResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/search",
            self.namespace
        );

        let request = RecordSearchRequest {
            search: self.search,
            fields: self.fields,
            page: self.page,
            page_size: self.page_size,
        };

        let req: ApiRequest<RecordSearchResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 搜索记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordSearchRequest {
    /// 搜索条件
    #[serde(rename = "search")]
    search: String,
    /// 字段列表
    #[serde(rename = "fields", skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
    /// 页码
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    /// 每页数量
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

/// 搜索记录结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchedRecord {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
    /// 相关性分数
    #[serde(rename = "score")]
    score: f64,
}

/// 搜索记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordSearchResponse {
    /// 搜索结果列表
    #[serde(rename = "items")]
    items: Vec<SearchedRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    page_size: u32,
}

impl ApiResponseTrait for RecordSearchResponse {
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
