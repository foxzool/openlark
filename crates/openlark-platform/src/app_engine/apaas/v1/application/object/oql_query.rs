//! 执行 OQL
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query>

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
pub struct OqlQueryRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// OQL 查询语句
    oql: String,
    /// 字段列表
    fields: Vec<String>,
}

impl OqlQueryRequestBuilder {
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
    pub items: Vec<OqlRecord>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

impl ApiResponseTrait for OqlQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to OqlQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type OqlQueryBuilder = OqlQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{ns}/objects/oql_query → 强类型 OqlQueryResponse。
    #[tokio::test]
    async fn test_oql_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/oql_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "id": "rec_001", "data": { "name": "记录一" } }
                    ],
                    "has_more": false
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

        let resp = OqlQueryRequestBuilder::new(config, "ns_test", "SELECT * FROM User")
            .execute()
            .await
            .expect("执行 OQL 查询应成功");
        assert_eq!(resp.items.len(), 1);
        assert!(!resp.has_more);
        assert_eq!(resp.items[0].id, "rec_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/oql_query"
        );
    }
}
