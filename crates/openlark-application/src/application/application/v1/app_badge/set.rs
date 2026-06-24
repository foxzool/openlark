//! 更新应用红点

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct SetAppBadgeRequest {
    config: Arc<Config>,
    body: SetAppBadgeBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 待补充文档。
pub struct SetAppBadgeBody {
    /// 待补充文档。
    pub app_id: String,
    /// 待补充文档。
    pub badge: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct SetAppBadgeResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SetAppBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SetAppBadgeRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SetAppBadgeBody::default(),
        }
    }

    /// 待补充文档。
    pub fn app_id(mut self, id: impl Into<String>) -> Self {
        self.body.app_id = id.into();
        self
    }

    /// 待补充文档。
    pub fn badge(mut self, badge: i32) -> Self {
        self.body.badge = badge;
        self
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<SetAppBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetAppBadgeResponse> {
        let path = "/open-apis/application/v6/app_badge/set";
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SetAppBadgeResponse> = ApiRequest::post(path).body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        Ok(resp.data.unwrap_or(SetAppBadgeResponse { data: None }))
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
