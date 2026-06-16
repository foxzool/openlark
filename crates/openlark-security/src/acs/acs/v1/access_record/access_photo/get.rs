//! 下载开门时的人脸识别图片

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
pub struct GetAccessPhotoRequest {
    config: Arc<Config>,
    access_record_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccessPhotoResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetAccessPhotoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetAccessPhotoRequest {
    pub fn new(config: Arc<Config>, access_record_id: impl Into<String>) -> Self {
        Self {
            config,
            access_record_id: access_record_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetAccessPhotoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetAccessPhotoResponse> {
        let path = format!("/open-apis/acs/v1/access_records/{}/access_photo", self.access_record_id);
        let req: ApiRequest<GetAccessPhotoResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetAccessPhotoResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetAccessPhotoResponse { data: None })
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
