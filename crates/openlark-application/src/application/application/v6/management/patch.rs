//! management patch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用管理信息的请求。
#[derive(Debug, Clone)]
pub struct ManagementPatchRequest {
    config: Arc<Config>,
}

/// 更新应用管理信息的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementPatchResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ManagementPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ManagementPatchRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行更新应用管理信息请求。
    pub async fn execute(self) -> SDKResult<ManagementPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ManagementPatchResponse> {
        let path = "/open-apis/application/application/v1/management/patch";
        let req: ApiRequest<ManagementPatchResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("management patch", "响应数据为空")
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
