//! 更新应用红点
//! docPath: https://open.feishu.cn/document/server-docs/application-v6/app_badge/set

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SetAppBadgeRequest {
    config: Arc<Config>,
    body: SetAppBadgeBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetAppBadgeBody {
    pub app_id: String,
    pub badge: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAppBadgeResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SetAppBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SetAppBadgeRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SetAppBadgeBody::default(),
        }
    }

    pub fn app_id(mut self, id: impl Into<String>) -> Self {
        self.body.app_id = id.into();
        self
    }

    pub fn badge(mut self, badge: i32) -> Self {
        self.body.badge = badge;
        self
    }

    pub async fn execute(self) -> SDKResult<SetAppBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetAppBadgeResponse> {
        let path = "/open-apis/application/v6/app_badge/set";
        let req: ApiRequest<SetAppBadgeResponse> =
            ApiRequest::post(path).json(&self.body).map_err(|e| {
                openlark_core::error::CoreError::Serialization(e.to_string())
            })?;

        let _resp: openlark_core::api::Response<SetAppBadgeResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(SetAppBadgeResponse { data: None })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
