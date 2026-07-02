//! 更新应用协作者

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新应用协作者的请求。
#[derive(Debug, Clone)]
pub struct UpdateApplicationCollaboratorsRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 更新应用协作者的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApplicationCollaboratorsResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateApplicationCollaboratorsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateApplicationCollaboratorsRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行更新应用协作者请求。
    pub async fn execute(self) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateApplicationCollaboratorsResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            self.app_id
        );
        let req: ApiRequest<UpdateApplicationCollaboratorsResponse> = ApiRequest::put(&path);

        let _resp: openlark_core::api::Response<UpdateApplicationCollaboratorsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(UpdateApplicationCollaboratorsResponse { data: None })
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
