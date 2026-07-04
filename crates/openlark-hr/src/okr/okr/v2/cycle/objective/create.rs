//! 创建 OKR 目标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/cycle.objective/create>

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

/// 创建 OKR 目标请求。
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
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<CreateCycleObjectiveResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateCycleObjectiveResponse> {
        validate_required!(self.cycle_id, "cycle_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = format!("/open-apis/okr/v2/cycles/{}/objectives", self.cycle_id);
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<CreateCycleObjectiveResponse> = ApiRequest::post(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("创建 OKR 目标", "响应数据为空"))
    }
}

/// 创建 OKR 目标响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCycleObjectiveResponse {
    /// 目标 ID。
    #[serde(default)]
    pub objective_id: Option<String>,
}

impl ApiResponseTrait for CreateCycleObjectiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            format!("/open-apis/okr/v2/cycles/{}/objectives", "cycle_123"),
            "/open-apis/okr/v2/cycles/cycle_123/objectives"
        );
    }

    #[test]
    fn test_create_cycle_objective_response_deserialize() {
        let json = serde_json::json!({"objective_id": "7342342398472398473"});
        let resp: CreateCycleObjectiveResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.objective_id, Some("7342342398472398473".to_string()));
    }
}
