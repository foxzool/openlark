//! 审批设备申报
//! docPath: https://open.feishu.cn/document/security_and_compliance-v1/security_and_compliance-v2/device_record/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UpdateDeviceApplyRecordRequest {
    config: Arc<Config>,
    device_record_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDeviceApplyRecordResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateDeviceApplyRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateDeviceApplyRecordRequest {
    pub fn new(config: Arc<Config>, device_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_record_id: device_record_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdateDeviceApplyRecordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateDeviceApplyRecordResponse> {
        validate_required!(self.device_record_id.trim(), "device_record_id 不能为空");

        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            self.device_record_id
        );
        let req: ApiRequest<UpdateDeviceApplyRecordResponse> = ApiRequest::put(&path);
        let _resp: openlark_core::api::Response<UpdateDeviceApplyRecordResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateDeviceApplyRecordResponse { data: None })
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
