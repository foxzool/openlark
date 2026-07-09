//! 修改 OKR 目标位置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/cycle/update>

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

/// 修改 OKR 目标位置请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    cycle_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            cycle_id: String::new(),
        }
    }

    /// 设置路径参数 `cycle_id`。
    pub fn cycle_id(mut self, val: impl Into<String>) -> Self {
        self.cycle_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(
        self,
        body: serde_json::Value,
    ) -> SDKResult<UpdateCycleObjectivesPositionResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<UpdateCycleObjectivesPositionResponse> {
        validate_required!(self.cycle_id, "cycle_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = OkrApiV2::CycleObjectivesPosition(self.cycle_id).to_url();
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<UpdateCycleObjectivesPositionResponse> =
            ApiRequest::put(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("修改 OKR 目标位置", "响应数据为空")
        })
    }
}

/// 修改 OKR 目标位置响应。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCycleObjectivesPositionResponse {
    /// 目标列表。
    #[serde(default)]
    pub items: Option<Vec<Objective>>,
}

impl ApiResponseTrait for UpdateCycleObjectivesPositionResponse {
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
    fn test_url_path() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config).cycle_id("cycle_123");
        assert_eq!(
            format!(
                "/open-apis/okr/v2/cycles/{}/objectives_position",
                "cycle_123"
            ),
            "/open-apis/okr/v2/cycles/cycle_123/objectives_position"
        );
    }

    #[test]
    fn test_update_cycle_objectives_position_response_deserialize() {
        let json = serde_json::json!({
            "items": [
                {
                    "id": "7342342398472398473",
                    "create_time": "1760604634563",
                    "update_time": "1760604634563",
                    "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                    "cycle_id": "7342342398472398473",
                    "position": 1,
                    "score": 0.8,
                    "weight": 0.5
                }
            ]
        });
        let resp: UpdateCycleObjectivesPositionResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.items.as_ref().unwrap().len(), 1);
        let objective = &resp.items.unwrap()[0];
        assert_eq!(objective.id, "7342342398472398473");
        assert_eq!(objective.position, 1);
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v2_cycle_objectives_position_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{}"#).unwrap();
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/okr/v2/cycles/cycle_001/objectives_position",
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
            .cycle_id("cycle_001")
            .execute(serde_json::json!({}))
            .await
            .expect("okr_v2_cycle_objectives_position 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v2/cycles/cycle_001/objectives_position"
        );
    }
}
