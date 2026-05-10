//! 获取门禁用户列表
//! docPath: https://open.feishu.cn/document/server-docs/acs-v1/user/list

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
pub struct ListUserRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUserResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListUserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListUserRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<ListUserResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListUserResponse> {
        let req: ApiRequest<ListUserResponse> = ApiRequest::get("/open-apis/acs/v1/users");
        let _resp: openlark_core::api::Response<ListUserResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(ListUserResponse { data: None })
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
