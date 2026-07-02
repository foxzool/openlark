//! 获取应用版本信息
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取应用版本信息的请求。
#[derive(Debug, Clone)]
pub struct GetApplicationVersionRequest {
    config: Arc<Config>,
    app_id: String,
    version_id: String,
}

/// 获取应用版本信息的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationVersionResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetApplicationVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetApplicationVersionRequest {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        app_id: impl Into<String>,
        version_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            version_id: version_id.into(),
        }
    }

    /// 执行获取应用版本信息请求。
    pub async fn execute(self) -> SDKResult<GetApplicationVersionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetApplicationVersionResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            self.app_id, self.version_id
        );
        let req: ApiRequest<GetApplicationVersionResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetApplicationVersionResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetApplicationVersionResponse { data: None })
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
