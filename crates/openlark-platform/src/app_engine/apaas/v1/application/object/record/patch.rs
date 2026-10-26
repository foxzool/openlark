//! 编辑记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/patch>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 编辑记录 Builder
#[derive(Debug, Clone)]
pub struct RecordPatchRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录 ID
    record_id: String,
    /// 请求体
    body: RecordPatchBody,
}

impl RecordPatchRequestBuilder {
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
            body: RecordPatchBody {
                record: String::new(),
            },
        }
    }

    /// 设置记录数据（JSON 字符串，键为字段 API 名称，值为字段值）
    pub fn record(mut self, record: impl Into<String>) -> Self {
        self.body.record = record.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordPatchResponse> {
        validate_required!(self.namespace, "应用命名空间不能为空");
        validate_required!(self.object_api_name, "对象唯一标识不能为空");
        validate_required!(self.record_id, "记录ID不能为空");
        validate_required!(self.body.record, "记录数据不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/{}",
            self.namespace, self.object_api_name, self.record_id
        );

        let req: ApiRequest<RecordPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "编辑记录").await
    }
}

/// 编辑记录请求体（官方字段：record 必填，为 JSON 字符串）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPatchBody {
    /// 编辑对象使用的数据（JSON 字符串，键为字段 API 名称）
    pub record: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// 编辑记录的响应（官方成功响应 data 为空对象，无业务字段）。
pub struct RecordPatchResponse {}

impl ApiResponseTrait for RecordPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordPatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordPatchBuilder = RecordPatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../records/{record_id} → 官方形状（data 为空对象）。
    #[tokio::test]
    async fn test_patch_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _resp = RecordPatchRequestBuilder::new(config, "ns_test", "obj_test", "rec_001")
            .record("{\"book_count\":\"3\",\"book_name\":\"showcase2\"}")
            .execute()
            .await
            .expect("编辑记录应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/obj_test/records/rec_001"
        );
        // 请求体须与官方文档一致：record 为 JSON 字符串
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({ "record": "{\"book_count\":\"3\",\"book_name\":\"showcase2\"}" })
        );
    }
}
