//! 编辑 OKR 目标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective/patch>

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

/// 编辑 OKR 目标请求。
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
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchObjectiveResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<PatchObjectiveResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = OkrApiV2::ObjectivePatch(self.objective_id).to_url();
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<PatchObjectiveResponse> = ApiRequest::patch(path).body(body_val);
        Transport::request_typed(req, &self.config, Some(option), "编辑 OKR 目标").await
    }
}

/// 编辑 OKR 目标响应。
#[derive(Debug, Clone, Deserialize)]
pub struct PatchObjectiveResponse {
    /// 目标详情。
    #[serde(default)]
    pub objective: Option<Objective>,
}

impl ApiResponseTrait for PatchObjectiveResponse {
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
    fn test_patch_objective_response_deserialize() {
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
        let resp: PatchObjectiveResponse = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.objective.as_ref().unwrap().id, "O-123");
        assert_eq!(resp.objective.as_ref().unwrap().position, 1);
        assert_eq!(resp.objective.as_ref().unwrap().owner.owner_type, "user");
        assert_eq!(resp.objective.as_ref().unwrap().score, Some(0.8));
        assert!(resp.objective.as_ref().unwrap().content.is_none());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_objective_patch_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("PATCH"))
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
            .execute(serde_json::json!({}))
            .await
            .expect("okr_v2_objective_patch 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/objectives/objective_001"
        );
    }
}
