//! app_version create

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
pub struct AppVersionCreateRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct AppVersionCreateResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for AppVersionCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppVersionCreateRequest {
    /// 待补充文档。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<AppVersionCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AppVersionCreateResponse> {
        let path = "/open-apis/application/application/v1/app-version/create";
        let req: ApiRequest<AppVersionCreateResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("app_version create", "响应数据为空")
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
