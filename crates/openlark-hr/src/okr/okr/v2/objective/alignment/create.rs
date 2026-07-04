//! 创建目标对齐关系
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.alignment/create>

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

/// 创建目标对齐关系请求。
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
    ) -> SDKResult<CreateObjectiveAlignmentResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<CreateObjectiveAlignmentResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = format!(
            "/open-apis/okr/v2/objectives/{}/alignments",
            self.objective_id
        );
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<CreateObjectiveAlignmentResponse> =
            ApiRequest::post(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("创建目标对齐关系", "响应数据为空")
        })
    }
}

/// 创建目标对齐关系响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateObjectiveAlignmentResponse {
    /// 对齐 ID。
    #[serde(default)]
    pub alignment_id: Option<String>,
}

impl ApiResponseTrait for CreateObjectiveAlignmentResponse {
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
    fn test_create_objective_alignment_response_deserialize() {
        let json = serde_json::json!({
            "alignment_id": "A-123"
        });
        let resp: CreateObjectiveAlignmentResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(resp.alignment_id, Some("A-123".to_string()));
    }
}
