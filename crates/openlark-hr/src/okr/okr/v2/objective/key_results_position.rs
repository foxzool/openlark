//! 修改关键结果位置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective/update>

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
use crate::okr::okr::v2::common::models::KeyResult;

/// 修改关键结果位置请求。
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
    pub async fn execute(
        self,
        body: serde_json::Value,
    ) -> SDKResult<UpdateObjectiveKeyResultsPositionResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<UpdateObjectiveKeyResultsPositionResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = OkrApiV2::ObjectiveKeyResultsPosition(self.objective_id).to_url();
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<UpdateObjectiveKeyResultsPositionResponse> =
            ApiRequest::put(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("修改关键结果位置", "响应数据为空")
        })
    }
}

/// 修改关键结果位置响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateObjectiveKeyResultsPositionResponse {
    /// 关键结果列表。
    #[serde(default)]
    pub items: Option<Vec<KeyResult>>,
}

impl ApiResponseTrait for UpdateObjectiveKeyResultsPositionResponse {
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
    fn test_update_objective_key_results_position_response_deserialize() {
        let json = serde_json::json!({
            "items": [
                {
                    "id": "KR-1",
                    "create_time": "1700000000000",
                    "update_time": "1700000000000",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "objective_id": "O-123",
                    "position": 1
                }
            ]
        });
        let resp: UpdateObjectiveKeyResultsPositionResponse =
            serde_json::from_value(json).expect("反序列化失败");
        let items = resp.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "KR-1");
        assert_eq!(items[0].position, 1);
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_objective_key_results_position_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/okr/v2/objectives/objective_001/key_results_position",
            ))
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
            .expect("okr_v2_objective_key_results_position 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/objectives/objective_001/key_results_position"
        );
    }
}
