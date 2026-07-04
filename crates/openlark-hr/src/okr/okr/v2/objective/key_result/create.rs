//! 在目标下创建关键结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.key_result/create>

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

/// 在目标下创建关键结果请求。
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
    ) -> SDKResult<CreateObjectiveKeyResultResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateObjectiveKeyResultResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = OkrApiV2::ObjectiveKeyResultCreate(self.objective_id).to_url();
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<CreateObjectiveKeyResultResponse> =
            ApiRequest::post(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("在目标下创建关键结果", "响应数据为空")
        })
    }
}

/// 在目标下创建关键结果响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateObjectiveKeyResultResponse {
    /// 关键结果 ID。
    #[serde(default)]
    pub key_result_id: Option<String>,
}

impl ApiResponseTrait for CreateObjectiveKeyResultResponse {
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
    fn test_create_objective_key_result_response_deserialize() {
        let json = serde_json::json!({
            "key_result_id": "KR-123"
        });
        let resp: CreateObjectiveKeyResultResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.key_result_id, Some("KR-123".to_string()));
    }
}
