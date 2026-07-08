//! 获取记录详情
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取记录详情 Builder
#[derive(Debug, Clone)]
pub struct RecordQueryRequestBuilder {
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

impl RecordQueryRequestBuilder {
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
    pub id: String,
    /// 记录数据
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    pub updated_time: i64,
}

impl ApiResponseTrait for RecordQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordQueryBuilder = RecordQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../records/{record_id}/query → 强类型 RecordQueryResponse。
    #[tokio::test]
    async fn test_query_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001/query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "id": "rec_001",
                    "data": { "name": "记录一" },
                    "created_time": 1700000000,
                    "updated_time": 1700000001
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

        let resp = RecordQueryRequestBuilder::new(config, "ns_test", "obj_test", "rec_001")
            .field("name")
            .execute()
            .await
            .expect("获取记录详情应成功");
        assert_eq!(resp.id, "rec_001");
        assert_eq!(resp.created_time, 1700000000);
        assert_eq!(resp.updated_time, 1700000001);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001/query"
        );
    }
}
