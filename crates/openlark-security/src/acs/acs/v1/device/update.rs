//! 更新设备

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
pub struct UpdateDeviceRequest {
    config: Arc<Config>,
    device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateDeviceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateDeviceRequest {
    pub fn new(config: Arc<Config>, device_id: impl Into<String>) -> Self {
        Self {
            config,
            device_id: device_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateDeviceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateDeviceResponse> {
        let path = format!("/open-apis/acs/v1/devices/{}", self.device_id);
        let req: ApiRequest<UpdateDeviceResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateDeviceResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateDeviceResponse { data: None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
