//! 新建记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/create>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 新建记录 Builder
#[derive(Debug, Clone)]
pub struct RecordCreateRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 请求体
    body: RecordCreateBody,
}

impl RecordCreateRequestBuilder {
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
            body: RecordCreateBody {
                record: String::new(),
            },
        }
    }

    /// 设置记录数据（JSON 字符串，键为字段 API 名称，值为字段值，如 "{\"book_name\":\"test\"}"）
    pub fn record(mut self, record: impl Into<String>) -> Self {
        self.body.record = record.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordCreateResponse> {
        validate_required!(self.namespace, "应用命名空间不能为空");
        validate_required!(self.object_api_name, "对象唯一标识不能为空");
        validate_required!(self.body.record, "记录数据不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records",
            self.namespace, self.object_api_name
        );

        let req: ApiRequest<RecordCreateResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "新建记录").await
    }
}

/// 新建记录请求体（官方字段：record 必填，为 JSON 字符串）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordCreateBody {
    /// 创建对象使用的数据（JSON 字符串，键为字段 API 名称）
    pub record: String,
}

/// 新建记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordCreateResponse {
    /// 记录 ID
    pub id: String,
}

impl ApiResponseTrait for RecordCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordCreateRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordCreateBuilder = RecordCreateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../objects/{obj}/records → 官方形状（data 仅含 id）。
    #[tokio::test]
    async fn test_create_record_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "id": "1764024447556775" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = RecordCreateRequestBuilder::new(config, "ns_test", "user")
            .record("{\"book_name\":\"test\"}")
            .execute()
            .await
            .expect("新建记录应成功");
        assert_eq!(resp.id, "1764024447556775");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/user/records"
        );
        // 请求体须与官方文档一致：record 为 JSON 字符串
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(sent, json!({ "record": "{\"book_name\":\"test\"}" }));
    }
}
