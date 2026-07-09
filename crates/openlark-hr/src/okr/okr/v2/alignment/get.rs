//! 获取 OKR 对齐
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/alignment/get>

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
use crate::okr::okr::v2::common::models::Alignment;

/// 获取 OKR 对齐请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    alignment_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            alignment_id: String::new(),
        }
    }

    /// 设置路径参数 `alignment_id`。
    pub fn alignment_id(mut self, val: impl Into<String>) -> Self {
        self.alignment_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetAlignmentResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAlignmentResponse> {
        validate_required!(self.alignment_id, "alignment_id 不能为空");
        let path = OkrApiV2::AlignmentGet(self.alignment_id).to_url();
        let req: ApiRequest<GetAlignmentResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取 OKR 对齐", "响应数据为空"))
    }
}

/// 获取 OKR 对齐响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAlignmentResponse {
    /// 对齐详情。
    pub alignment: Alignment,
}

impl ApiResponseTrait for GetAlignmentResponse {
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
    fn test_get_alignment_response_deserialize() {
        let json = serde_json::json!({
            "alignment": {
                "id": "A-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "from_owner": {"owner_type": "user", "user_id": "ou_from"},
                "to_owner": {"owner_type": "user", "user_id": "ou_to"},
                "from_entity_type": 1,
                "from_entity_id": "E-1",
                "to_entity_type": 2,
                "to_entity_id": "E-2"
            }
        });
        let resp: GetAlignmentResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.alignment.id, "A-123");
        assert_eq!(resp.alignment.from_entity_type, 1);
        assert_eq!(resp.alignment.to_owner.owner_type, "user");
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_alignment_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body = json!({
            "alignment": {
                "id": "A-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "from_owner": {"owner_type": "user", "user_id": "ou_from"},
                "to_owner": {"owner_type": "user", "user_id": "ou_to"},
                "from_entity_type": 1,
                "from_entity_id": "E-1",
                "to_entity_type": 2,
                "to_entity_id": "E-2"
            }
        });
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v2/alignments/alignment_001"))
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
            .alignment_id("alignment_001")
            .execute()
            .await
            .expect("okr_v2_alignment_get 应成功");

        let _ = &data.alignment;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/alignments/alignment_001"
        );
    }
}
