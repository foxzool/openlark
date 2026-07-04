//! 更新量化指标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/indicator/patch>

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

use crate::okr::okr::v2::common::models::Indicator;

/// 更新量化指标请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    indicator_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            indicator_id: String::new(),
        }
    }

    /// 设置路径参数 `indicator_id`。
    pub fn indicator_id(mut self, val: impl Into<String>) -> Self {
        self.indicator_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<PatchIndicatorResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<PatchIndicatorResponse> {
        validate_required!(self.indicator_id, "indicator_id 不能为空");
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }
        let path = format!("/open-apis/okr/v2/indicators/{}", self.indicator_id);
        let body_val = serde_json::to_value(&body).map_err(|e| {
            openlark_core::error::validation_error("请求体序列化失败", format!("无法序列化: {e}"))
        })?;
        let req: ApiRequest<PatchIndicatorResponse> = ApiRequest::patch(path).body(body_val);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("更新量化指标", "响应数据为空"))
    }
}

/// 更新量化指标响应。
#[derive(Debug, Clone, Deserialize)]
pub struct PatchIndicatorResponse {
    /// 指标详情。
    #[serde(default)]
    pub indicator: Option<Indicator>,
}

impl ApiResponseTrait for PatchIndicatorResponse {
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
    fn test_patch_indicator_response_deserialize() {
        let json = serde_json::json!({
            "indicator": {
                "id": "I-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "entity_type": 1,
                "entity_id": "E-1",
                "indicator_status": 0,
                "status_calculate_type": 0,
                "start_value": 0.0,
                "target_value": 100.0,
                "current_value": 50.0,
                "current_value_calculate_type": 0,
                "unit": {"unit_type": 1, "unit_value": "PERCENT"}
            }
        });
        let resp: PatchIndicatorResponse = serde_json::from_value(json).expect("反序列化失败");
        let indicator = resp.indicator.expect("指标不应为空");
        assert_eq!(indicator.id, "I-123");
        assert_eq!(indicator.entity_type, 1);
        assert_eq!(indicator.current_value, Some(50.0));
        assert_eq!(indicator.unit.as_ref().unwrap().unit_value, "PERCENT");
        assert_eq!(indicator.owner.user_id, Some("ou_xxx".to_string()));
    }

    #[test]
    fn test_patch_indicator_response_deserialize_empty() {
        let json = serde_json::json!({});
        let resp: PatchIndicatorResponse = serde_json::from_value(json).expect("反序列化失败");
        assert!(resp.indicator.is_none());
    }
}
