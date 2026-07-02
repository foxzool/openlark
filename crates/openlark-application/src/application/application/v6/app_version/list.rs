//! app_version list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用版本列表的请求。
#[derive(Debug, Clone)]
pub struct AppVersionListRequest {
    config: Arc<Config>,
}

/// 获取应用版本列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersionListResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for AppVersionListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppVersionListRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取应用版本列表请求。
    pub async fn execute(self) -> SDKResult<AppVersionListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AppVersionListResponse> {
        let path = "/open-apis/application/application/v1/app-version/list";
        let req: ApiRequest<AppVersionListResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("app_version list", "响应数据为空")
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
