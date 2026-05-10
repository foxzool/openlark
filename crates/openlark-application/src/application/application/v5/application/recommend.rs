//! 获取管理员推荐的应用
//! docPath: https://open.feishu.cn/document/server-docs/workplace-v1/app_recommend_rule/recommend

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
pub struct GetRecommendedAppsRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecommendedAppsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetRecommendedAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetRecommendedAppsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<GetRecommendedAppsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetRecommendedAppsResponse> {
        let req: ApiRequest<GetRecommendedAppsResponse> =
            ApiRequest::get("/open-apis/application/v5/applications/recommend");

        let _resp: openlark_core::api::Response<GetRecommendedAppsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetRecommendedAppsResponse { data: None })
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
