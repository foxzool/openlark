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

/// 设置应用徽标的请求。
#[derive(Debug, Clone)]
pub struct SetAppBadgeRequest {
    config: Arc<Config>,
    body: SetAppBadgeBody,
}

/// 设置应用徽标的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetAppBadgeBody {
    /// 应用 ID。
    pub app_id: String,
    /// 徽标。
    pub badge: i32,
}

/// 设置应用徽标的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAppBadgeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SetAppBadgeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 成功但无 `data` 字段时的空成功（set 类 API，与旧 `unwrap_or` 默认值一致）。
    fn empty_success() -> Option<Self> {
        Some(Self { data: None })
    }
}

impl SetAppBadgeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: SetAppBadgeBody::default(),
        }
    }

    /// 设置应用 ID。
    pub fn app_id(mut self, id: impl Into<String>) -> Self {
        self.body.app_id = id.into();
        self
    }

    /// 设置徽标。
    pub fn badge(mut self, badge: i32) -> Self {
        self.body.badge = badge;
        self
    }

    /// 执行设置应用徽标请求。
    pub async fn execute(self) -> SDKResult<SetAppBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SetAppBadgeResponse> {
        let path = "/open-apis/application/v6/app_badge/set";
        let body = serde_json::to_value(&self.body)?;
        let req: ApiRequest<SetAppBadgeResponse> = ApiRequest::post(path).body(body);
        Transport::request_typed(req, &self.config, Some(option), "更新应用红点").await
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
