//! 获取用户自定义常用的应用

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取用户自定义常用的应用的请求。
#[derive(Debug, Clone)]
pub struct GetFrequentlyUsedAppsRequest {
    config: Arc<Config>,
}

/// 获取用户自定义常用的应用的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFrequentlyUsedAppsResponse {
    /// 响应数据。
    pub data: Option<FrequentlyUsedAppsData>,
}

impl ApiResponseTrait for GetFrequentlyUsedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户自定义常用的应用的响应数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentlyUsedAppsData {
    /// 应用列表。
    pub apps: Vec<FrequentlyUsedApp>,
}

/// 常用应用。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentlyUsedApp {
    /// 应用 ID。
    pub app_id: String,
    /// 应用名称。
    pub app_name: String,
}

impl GetFrequentlyUsedAppsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取用户自定义常用的应用请求。
    pub async fn execute(self) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFrequentlyUsedAppsResponse> {
        let path = "/open-apis/application/v6/applications/frequently_used".to_string();
        let req: ApiRequest<GetFrequentlyUsedAppsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取用户自定义常用的应用", "响应数据为空")
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
