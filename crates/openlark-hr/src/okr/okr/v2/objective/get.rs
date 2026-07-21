//! 获取目标详细信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective/get>

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
use crate::okr::okr::v2::common::models::Objective;

/// 获取目标详细信息请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    objective_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            objective_id: String::new(),
        }
    }

    /// 设置路径参数 `objective_id`。
    pub fn objective_id(mut self, val: impl Into<String>) -> Self {
        self.objective_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetObjectiveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetObjectiveResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = OkrApiV2::ObjectiveGet(self.objective_id).to_url();
        let req: ApiRequest<GetObjectiveResponse> = ApiRequest::get(path);
        Transport::request_typed(req, &self.config, Some(option), "获取目标详细信息").await
    }
}

/// 获取目标详细信息响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectiveResponse {
    /// 目标详情。
    pub objective: Objective,
}

impl ApiResponseTrait for GetObjectiveResponse {
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
    fn test_get_objective_response_deserialize() {
        let json = serde_json::json!({
            "objective": {
                "id": "O-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "cycle_id": "C-1",
                "position": 1,
                "score": 0.8,
                "weight": 0.5,
                "deadline": "1700000000000",
                "category_id": "cat-1"
            }
        });
        let resp: GetObjectiveResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.objective.id, "O-123");
        assert_eq!(resp.objective.position, 1);
        assert_eq!(resp.objective.owner.owner_type, "user");
        assert_eq!(resp.objective.score, Some(0.8));
        assert!(resp.objective.content.is_none());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_objective_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body = json!({
            "objective": {
                "id": "O-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "cycle_id": "C-1",
                "position": 1,
                "score": 0.8,
                "weight": 0.5,
                "deadline": "1700000000000",
                "category_id": "cat-1"
            }
        });
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v2/objectives/objective_001"))
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
            .objective_id("objective_001")
            .execute()
            .await
            .expect("okr_v2_objective_get 应成功");

        let _ = &data.objective;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/objectives/objective_001"
        );
    }
}
