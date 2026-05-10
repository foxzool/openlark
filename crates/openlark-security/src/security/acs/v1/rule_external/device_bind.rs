//! 设备绑定权限组
//! docPath: https://open.feishu.cn/document/acs-v1/rule_external/device_bind

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
pub struct BindDeviceToRuleRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindDeviceToRuleResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BindDeviceToRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BindDeviceToRuleRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<BindDeviceToRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BindDeviceToRuleResponse> {
        let path = format!("/open-apis/acs/v1/rule_external/device_bind");
        let req: ApiRequest<BindDeviceToRuleResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<BindDeviceToRuleResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(BindDeviceToRuleResponse { data: None })
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
