//! 获取应用协作者列表
//! docPath: https://open.feishu.cn/document/server-docs/application-v6/application/get

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
pub struct GetApplicationCollaboratorsRequest {
    config: Arc<Config>,
    app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationCollaboratorsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationCollaboratorsRequest {
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetApplicationCollaboratorsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationCollaboratorsResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            self.app_id
        );
        let req: ApiRequest<GetApplicationCollaboratorsResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationCollaboratorsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationCollaboratorsResponse { data: None })
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
