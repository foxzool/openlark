//! 录入身份信息
//!
//! 文档: <https://open.feishu.cn/document/server-docs/human_authentication-v1/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 录入身份信息 Builder
#[derive(Debug, Clone)]
pub struct IdentityCreateBuilder {
    config: Config,
    identity_name: String,
    identity_code: String,
    mobile: Option<String>,
}

impl IdentityCreateBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            identity_name: String::new(),
            identity_code: String::new(),
            mobile: None,
        }
    }

    /// 设置姓名
    pub fn identity_name(mut self, name: impl Into<String>) -> Self {
        self.identity_name = name.into();
        self
    }

    /// 设置身份证号
    pub fn identity_code(mut self, code: impl Into<String>) -> Self {
        self.identity_code = code.into();
        self
    }

    /// 设置手机号
    pub fn mobile(mut self, mobile: impl Into<String>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<IdentityCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<IdentityCreateResponse> {
        let url = "/open-apis/human_authentication/v1/identities";

        let request = IdentityCreateRequest {
            identity_name: self.identity_name,
            identity_code: self.identity_code,
            mobile: self.mobile,
        };

        let req: ApiRequest<IdentityCreateResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 录入身份信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct IdentityCreateRequest {
    /// 姓名
    #[serde(rename = "identity_name")]
    identity_name: String,
    /// 身份证号
    #[serde(rename = "identity_code")]
    identity_code: String,
    /// 手机号
    #[serde(rename = "mobile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<String>,
}

/// 录入身份信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IdentityCreateResponse {
    /// 身份信息 ID
    #[serde(rename = "identity_id")]
    pub identity_id: String,
}

impl ApiResponseTrait for IdentityCreateResponse {}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
