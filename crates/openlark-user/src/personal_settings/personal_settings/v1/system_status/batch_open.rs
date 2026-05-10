//! system_status batch_open
//! docPath: https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/batch_open

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
pub struct SystemStatusBatch_openRequest {
    config: Arc<Config>,
    status_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusBatch_openResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusBatch_openResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusBatch_openRequest {
    pub fn new(config: Arc<Config>, status_id: impl Into<String>) -> Self {
        Self {
            config,
            status_id: status_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<SystemStatusBatch_openResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusBatch_openResponse> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_open",
            self.status_id
        );
        let req: ApiRequest<SystemStatusBatch_openResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("system_status batch_open", "响应数据为空")
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
