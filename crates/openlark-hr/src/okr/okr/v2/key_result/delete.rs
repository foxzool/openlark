//! 删除关键结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/key_result/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_endpoints::OkrApiV2;

/// 删除关键结果请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    key_result_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            key_result_id: String::new(),
        }
    }

    /// 设置路径参数 `key_result_id`。
    pub fn key_result_id(mut self, val: impl Into<String>) -> Self {
        self.key_result_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteKeyResultResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteKeyResultResponse> {
        validate_required!(self.key_result_id, "key_result_id 不能为空");
        let path = OkrApiV2::KeyResultDelete(self.key_result_id).to_url();
        let req: ApiRequest<DeleteKeyResultResponse> = ApiRequest::delete(path);
        Transport::request_typed(req, &self.config, Some(option), "删除关键结果").await
    }
}

/// 删除关键结果响应。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteKeyResultResponse {}

impl ApiResponseTrait for DeleteKeyResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_delete_key_result_response_deserialize() {
        let json = serde_json::json!({});
        let resp: DeleteKeyResultResponse = serde_json::from_value(json).expect("反序列化失败");
        let _ = resp;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_key_result_delete_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("DELETE"))
            .and(path("/open-apis/okr/v2/key_results/key_result_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = Request::new(std::sync::Arc::new(config))
            .key_result_id("key_result_001")
            .execute()
            .await
            .expect("okr_v2_key_result_delete 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/key_results/key_result_001"
        );
    }
}
