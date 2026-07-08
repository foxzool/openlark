//! 删除记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/delete>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除记录 Builder
#[derive(Debug, Clone)]
pub struct RecordDeleteRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID
    record_id: String,
}

impl RecordDeleteRequestBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordDeleteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}",
            self.namespace, self.object_api_name, self.record_id
        );

        let req: ApiRequest<RecordDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除记录", "响应数据为空"))
    }
}

/// 删除记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    pub id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for RecordDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordDeleteBuilder = RecordDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../records/{record_id} → 强类型 RecordDeleteResponse。
    #[tokio::test]
    async fn test_delete_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "rec_001", "message": "删除成功" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RecordDeleteRequestBuilder::new(config, "ns_test", "obj_test", "rec_001")
            .execute()
            .await
            .expect("删除记录应成功");
        assert_eq!(resp.id, "rec_001");
        assert_eq!(resp.message, "删除成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001"
        );
    }
}
