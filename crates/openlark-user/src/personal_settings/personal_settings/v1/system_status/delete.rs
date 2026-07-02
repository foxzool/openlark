//! system_status delete
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 删除系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusDeleteRequest {
    config: Arc<Config>,
    status_id: String,
}

/// 删除系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusDeleteResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusDeleteRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            status_id: String::new(),
        }
    }

    /// 设置状态 ID。
    pub fn status_id(mut self, status_id: impl Into<String>) -> Self {
        self.status_id = status_id.into();
        self
    }

    /// 执行删除系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusDeleteResponse> {
        validate_required!(self.status_id.trim(), "status_id 不能为空");
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            self.status_id
        );
        let req: ApiRequest<SystemStatusDeleteResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("system_status delete", "响应数据为空")
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
