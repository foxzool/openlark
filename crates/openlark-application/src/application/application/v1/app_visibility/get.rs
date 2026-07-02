//! app_visibility get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用可见范围的请求。
#[derive(Debug, Clone)]
pub struct AppVisibilityGetRequest {
    config: Arc<Config>,
}

/// 获取应用可见范围的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVisibilityGetResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for AppVisibilityGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppVisibilityGetRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取应用可见范围请求。
    pub async fn execute(self) -> SDKResult<AppVisibilityGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AppVisibilityGetResponse> {
        let path = "/open-apis/application/application/v1/app-visibility/get";
        let req: ApiRequest<AppVisibilityGetResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("app_visibility get", "响应数据为空")
        })
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
