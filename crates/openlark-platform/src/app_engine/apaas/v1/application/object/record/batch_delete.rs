//! 批量删除记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete>

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
pub struct RecordBatchDeleteRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID 列表
    record_ids: Vec<String>,
}

impl RecordBatchDeleteRequestBuilder {
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
    pub deleted_count: u32,
    /// 删除结果列表
    #[serde(rename = "items")]
    pub items: Vec<RecordDeleteResult>,
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

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordBatchDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordBatchDeleteBuilder = RecordBatchDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../records/batch_delete → 强类型 RecordBatchDeleteResponse。
    #[tokio::test]
    async fn test_batch_delete_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "deleted_count": 2,
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

        let resp = RecordBatchDeleteRequestBuilder::new(config, "ns_test", "obj_test")
            .record_id("rec_001")
            .record_id("rec_002")
            .execute()
            .await
            .expect("批量删除记录应成功");
        assert_eq!(resp.deleted_count, 2);
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].id, "rec_001");
        assert!(resp.items[0].success);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/batch_delete"
        );
    }
}
