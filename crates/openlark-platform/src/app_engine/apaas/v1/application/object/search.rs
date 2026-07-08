//! 搜索记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/search>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 搜索记录 Builder
#[derive(Debug, Clone)]
pub struct RecordSearchRequestBuilder {
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

impl RecordSearchRequestBuilder {
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
    pub items: Vec<SearchedRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for RecordSearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordSearchRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordSearchBuilder = RecordSearchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{ns}/objects/search → 强类型 RecordSearchResponse。
    #[tokio::test]
    async fn test_search_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/search",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "id": "rec_001", "data": { "name": "记录一" }, "score": 0.95 }
                    ],
                    "has_more": false,
                    "page": 1,
                    "page_size": 20
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RecordSearchRequestBuilder::new(config, "ns_test", "关键字")
            .page(1)
            .page_size(20)
            .execute()
            .await
            .expect("搜索记录应成功");
        assert_eq!(resp.items.len(), 1);
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);
        assert_eq!(resp.page_size, 20);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/search"
        );
    }
}
