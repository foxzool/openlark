//! 批量新建记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_create>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量新建记录 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchCreateRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录数据列表
    records: Vec<serde_json::Value>,
}

impl RecordBatchCreateRequestBuilder {
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
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
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
    pub items: Vec<RecordCreateResult>,
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

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordBatchCreateRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordBatchCreateBuilder = RecordBatchCreateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../records/batch_create → 强类型 RecordBatchCreateResponse。
    #[tokio::test]
    async fn test_batch_create_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_create",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "id": "rec_001", "success": true },
                        { "id": "rec_002", "success": true }
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

        let resp = RecordBatchCreateRequestBuilder::new(config, "ns_test", "obj_test")
            .record(json!({ "name": "记录一" }))
            .record(json!({ "name": "记录二" }))
            .execute()
            .await
            .expect("批量新建记录应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].id, "rec_001");
        assert!(resp.items[0].success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_create"
        );
    }
}
