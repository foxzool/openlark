//! 查询记录列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询记录列表 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchQueryRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID 列表
    record_ids: Vec<String>,
    /// 字段列表
    fields: Vec<String>,
}

impl RecordBatchQueryRequestBuilder {
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
            fields: Vec::new(),
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
    pub async fn execute(self) -> SDKResult<RecordBatchQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchQueryResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_query",
            self.namespace, self.object_api_name
        );

        let request = RecordBatchQueryRequest {
            record_ids: self.record_ids,
            fields: self.fields,
        };

        let req: ApiRequest<RecordBatchQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 查询记录列表请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordBatchQueryRequest {
    /// 记录 ID 列表
    #[serde(rename = "record_ids")]
    record_ids: Vec<String>,
    /// 字段列表
    #[serde(rename = "fields", skip_serializing_if = "Vec::is_empty")]
    fields: Vec<String>,
}

/// 记录信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordInfo {
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

/// 查询记录列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchQueryResponse {
    /// 记录列表
    #[serde(rename = "items")]
    pub items: Vec<RecordInfo>,
}

impl ApiResponseTrait for RecordBatchQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordBatchQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordBatchQueryBuilder = RecordBatchQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../records/batch_query → 强类型 RecordBatchQueryResponse。
    #[tokio::test]
    async fn test_batch_query_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "rec_001",
                            "data": { "name": "记录一" },
                            "created_time": 1700000000,
                            "updated_time": 1700000001
                        }
                    ]
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

        let resp = RecordBatchQueryRequestBuilder::new(config, "ns_test", "obj_test")
            .record_id("rec_001")
            .execute()
            .await
            .expect("批量查询记录应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id, "rec_001");
        assert_eq!(resp.items[0].created_time, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_query"
        );
    }
}
